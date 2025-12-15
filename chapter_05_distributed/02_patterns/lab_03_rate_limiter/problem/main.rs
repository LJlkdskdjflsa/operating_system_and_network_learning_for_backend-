//! Lab 3: Rate Limiter (Token Bucket)
//!
//! ## Goal
//! Implement token bucket rate limiter
//!
//! ## Requirements
//! 1. Configurable rate (tokens per second)
//! 2. Configurable burst capacity
//! 3. Allow/deny requests based on available tokens
//! 4. Automatic token refill over time
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! Rate Limiter: 10 req/sec, burst: 5
//!
//! Burst test (5 rapid requests):
//! Request 1: ALLOWED (tokens: 4)
//! Request 2: ALLOWED (tokens: 3)
//! ...
//! Request 6: DENIED (no tokens)
//!
//! After 1 second (10 tokens refilled):
//! Request 7: ALLOWED
//! ```
//!
//! ## Hints
//! - Track tokens as f64 for partial refills
//! - Refill on each check, not in background
//! - tokens = min(tokens + elapsed * rate, capacity)
//!
//! ## Acceptance Criteria
//! - [ ] Allows requests when tokens available
//! - [ ] Denies requests when no tokens
//! - [ ] Refills tokens over time
//! - [ ] Respects capacity limit

use std::time::Instant;

// ============================================================
// TODO: Implement token bucket rate limiter
// ============================================================

/// Token bucket rate limiter
struct RateLimiter {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,  // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        // TODO: Initialize with full bucket

        todo!("Implement RateLimiter::new")
    }

    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        // TODO: Calculate elapsed time and add tokens
        // Don't exceed capacity

        todo!("Implement RateLimiter::refill")
    }

    /// Check if request is allowed (consumes 1 token)
    fn allow(&mut self) -> bool {
        // TODO: Refill, then check and consume token

        todo!("Implement RateLimiter::allow")
    }

    /// Check if request is allowed (consumes n tokens)
    fn allow_n(&mut self, n: f64) -> bool {
        // TODO: Allow requests that need multiple tokens

        todo!("Implement RateLimiter::allow_n")
    }

    /// Get current token count
    fn tokens(&mut self) -> f64 {
        // TODO: Refill and return current tokens

        todo!("Implement RateLimiter::tokens")
    }
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    // 1. Create rate limiter
    // 2. Test burst (rapid requests)
    // 3. Test refill (wait and retry)
    // 4. Show statistics

    todo!("Implement main")
}
