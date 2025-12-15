//! Lab 3 Reference Answer

use std::time::{Duration, Instant};

/// Token bucket rate limiter
struct RateLimiter {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,
    last_refill: Instant,
    // Statistics
    allowed: u64,
    denied: u64,
}

impl RateLimiter {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        RateLimiter {
            tokens: capacity,  // Start with full bucket
            capacity,
            refill_rate,
            last_refill: Instant::now(),
            allowed: 0,
            denied: 0,
        }
    }

    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();

        // Add tokens based on elapsed time
        self.tokens += elapsed * self.refill_rate;

        // Cap at capacity
        if self.tokens > self.capacity {
            self.tokens = self.capacity;
        }

        self.last_refill = now;
    }

    /// Check if request is allowed (consumes 1 token)
    fn allow(&mut self) -> bool {
        self.allow_n(1.0)
    }

    /// Check if request is allowed (consumes n tokens)
    fn allow_n(&mut self, n: f64) -> bool {
        self.refill();

        if self.tokens >= n {
            self.tokens -= n;
            self.allowed += 1;
            true
        } else {
            self.denied += 1;
            false
        }
    }

    /// Get current token count
    fn tokens(&mut self) -> f64 {
        self.refill();
        self.tokens
    }

    /// Get statistics
    fn stats(&self) -> (u64, u64) {
        (self.allowed, self.denied)
    }
}

#[tokio::main]
async fn main() {
    println!("=== Token Bucket Rate Limiter Demo ===\n");

    // Create rate limiter: 10 requests/second, burst of 5
    let mut limiter = RateLimiter::new(5.0, 10.0);

    println!("Configuration:");
    println!("  Capacity (burst): 5 tokens");
    println!("  Refill rate: 10 tokens/second");
    println!("  Initial tokens: {:.1}\n", limiter.tokens());

    // Test 1: Burst
    println!("Test 1: Rapid burst (8 requests)");
    println!("---------------------------------");
    for i in 1..=8 {
        let allowed = limiter.allow();
        let status = if allowed { "ALLOWED" } else { "DENIED " };
        println!(
            "  Request {:2}: {} (tokens remaining: {:.1})",
            i,
            status,
            limiter.tokens
        );
    }

    let (allowed, denied) = limiter.stats();
    println!("  Results: {} allowed, {} denied\n", allowed, denied);

    // Test 2: Wait and retry
    println!("Test 2: Wait 500ms and retry");
    println!("-----------------------------");
    println!("  Waiting 500ms (should gain ~5 tokens)...");
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("  Tokens after wait: {:.1}", limiter.tokens());

    for i in 1..=3 {
        let allowed = limiter.allow();
        let status = if allowed { "ALLOWED" } else { "DENIED " };
        println!(
            "  Request {:2}: {} (tokens: {:.1})",
            i,
            status,
            limiter.tokens
        );
    }

    // Test 3: Steady rate
    println!("\nTest 3: Steady rate (1 request every 150ms)");
    println!("--------------------------------------------");
    println!("  At 10 tokens/sec, 1 req/150ms should mostly succeed");

    let mut steady_allowed = 0;
    let mut steady_denied = 0;

    for i in 1..=10 {
        tokio::time::sleep(Duration::from_millis(150)).await;
        let allowed = limiter.allow();
        if allowed {
            steady_allowed += 1;
        } else {
            steady_denied += 1;
        }
        let status = if allowed { "ALLOWED" } else { "DENIED " };
        println!(
            "  Request {:2}: {} (tokens: {:.1})",
            i,
            status,
            limiter.tokens
        );
    }
    println!(
        "  Results: {} allowed, {} denied",
        steady_allowed, steady_denied
    );

    // Test 4: Multi-token requests
    println!("\nTest 4: Multi-token requests");
    println!("-----------------------------");

    // Reset limiter
    let mut limiter = RateLimiter::new(10.0, 5.0);
    println!("  New limiter: capacity=10, rate=5/sec");
    println!("  Initial tokens: {:.1}", limiter.tokens());

    println!("  Large request (3 tokens): {}",
             if limiter.allow_n(3.0) { "ALLOWED" } else { "DENIED" });
    println!("  Tokens: {:.1}", limiter.tokens);

    println!("  Large request (5 tokens): {}",
             if limiter.allow_n(5.0) { "ALLOWED" } else { "DENIED" });
    println!("  Tokens: {:.1}", limiter.tokens);

    println!("  Large request (5 tokens): {}",
             if limiter.allow_n(5.0) { "ALLOWED" } else { "DENIED" });
    println!("  Tokens: {:.1}", limiter.tokens);

    // Final stats
    println!("\n=== Summary ===");
    let (total_allowed, total_denied) = limiter.stats();
    println!("Total requests: {}", total_allowed + total_denied);
    println!("Allowed: {}", total_allowed);
    println!("Denied: {}", total_denied);
    if total_allowed + total_denied > 0 {
        println!(
            "Allow rate: {:.1}%",
            (total_allowed as f64 / (total_allowed + total_denied) as f64) * 100.0
        );
    }

    println!("\n=== Key Concepts ===");
    println!("- Token bucket allows bursts up to capacity");
    println!("- Refills at constant rate (smooths out traffic)");
    println!("- No tokens = request denied");
    println!("- Good for API rate limiting");
}

// Key concepts demonstrated:
//
// 1. TOKEN BUCKET:
//    - Bucket holds tokens up to capacity
//    - Tokens refill at constant rate
//    - Each request consumes tokens
//    - Allows bursts, then rate-limits
//
// 2. LAZY REFILL:
//    - Refill calculated on demand
//    - No background thread needed
//    - Efficient for sparse traffic
//
// 3. BURST HANDLING:
//    - capacity = max burst size
//    - refill_rate = sustained rate
//    - Burst followed by steady rate = OK
//
// 4. MULTI-TOKEN:
//    - Some operations cost more
//    - E.g., expensive queries cost 5 tokens
//    - Allows flexible rate limiting

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_within_capacity() {
        let mut limiter = RateLimiter::new(5.0, 10.0);

        for _ in 0..5 {
            assert!(limiter.allow());
        }
    }

    #[test]
    fn test_deny_over_capacity() {
        let mut limiter = RateLimiter::new(3.0, 10.0);

        assert!(limiter.allow());
        assert!(limiter.allow());
        assert!(limiter.allow());
        assert!(!limiter.allow());  // Denied
    }

    #[test]
    fn test_refill() {
        let mut limiter = RateLimiter::new(5.0, 100.0);  // Fast refill

        // Consume all tokens
        while limiter.allow() {}

        // Wait for refill (10ms at 100/sec = 1 token)
        std::thread::sleep(Duration::from_millis(20));

        assert!(limiter.allow());
    }

    #[test]
    fn test_capacity_limit() {
        let mut limiter = RateLimiter::new(5.0, 1000.0);  // Very fast refill

        // Wait (would refill way more than capacity)
        std::thread::sleep(Duration::from_millis(100));

        // Should still only have capacity tokens
        assert!(limiter.tokens() <= 5.0);
    }

    #[test]
    fn test_allow_n() {
        let mut limiter = RateLimiter::new(10.0, 10.0);

        assert!(limiter.allow_n(5.0));
        assert!(limiter.allow_n(5.0));
        assert!(!limiter.allow_n(1.0));  // No tokens left
    }
}
