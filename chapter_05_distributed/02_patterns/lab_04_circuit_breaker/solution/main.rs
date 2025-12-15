//! Lab 4 Reference Answer

use std::fmt;
use std::time::{Duration, Instant};

/// Circuit breaker states
#[derive(Debug)]
enum State {
    Closed,
    Open { until: Instant },
    HalfOpen,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Closed => write!(f, "CLOSED"),
            State::Open { .. } => write!(f, "OPEN"),
            State::HalfOpen => write!(f, "HALF_OPEN"),
        }
    }
}

/// Circuit breaker error
#[derive(Debug)]
enum CircuitError {
    Open,
    Failed(String),
}

impl fmt::Display for CircuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitError::Open => write!(f, "Circuit is open"),
            CircuitError::Failed(e) => write!(f, "Call failed: {}", e),
        }
    }
}

/// Circuit breaker
struct CircuitBreaker {
    state: State,
    failure_count: u32,
    failure_threshold: u32,
    reset_timeout: Duration,
    // Statistics
    total_calls: u64,
    successful_calls: u64,
    failed_calls: u64,
    rejected_calls: u64,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, reset_timeout: Duration) -> Self {
        CircuitBreaker {
            state: State::Closed,
            failure_count: 0,
            failure_threshold,
            reset_timeout,
            total_calls: 0,
            successful_calls: 0,
            failed_calls: 0,
            rejected_calls: 0,
        }
    }

    /// Get current state
    fn state(&self) -> &State {
        &self.state
    }

    /// Check and update state (for timeout transition)
    fn check_state(&mut self) {
        if let State::Open { until } = self.state {
            if Instant::now() >= until {
                println!("  [Circuit] Timeout expired, transitioning to HALF_OPEN");
                self.state = State::HalfOpen;
            }
        }
    }

    /// Execute function through circuit breaker
    fn call<F, T>(&mut self, f: F) -> Result<T, CircuitError>
    where
        F: FnOnce() -> Result<T, String>,
    {
        self.total_calls += 1;

        // Check for state transition
        self.check_state();

        // If open, reject immediately
        if let State::Open { .. } = self.state {
            self.rejected_calls += 1;
            return Err(CircuitError::Open);
        }

        // Execute the function
        match f() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(CircuitError::Failed(e))
            }
        }
    }

    /// Record success
    fn on_success(&mut self) {
        self.successful_calls += 1;

        match self.state {
            State::Closed => {
                // Reset consecutive failures
                self.failure_count = 0;
            }
            State::HalfOpen => {
                // Success in half-open closes the circuit
                println!("  [Circuit] Success in HALF_OPEN, closing circuit");
                self.state = State::Closed;
                self.failure_count = 0;
            }
            State::Open { .. } => {
                // Shouldn't happen, but handle gracefully
            }
        }
    }

    /// Record failure
    fn on_failure(&mut self) {
        self.failed_calls += 1;

        match self.state {
            State::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    println!(
                        "  [Circuit] {} consecutive failures, opening circuit",
                        self.failure_count
                    );
                    self.state = State::Open {
                        until: Instant::now() + self.reset_timeout,
                    };
                }
            }
            State::HalfOpen => {
                // Failure in half-open opens the circuit again
                println!("  [Circuit] Failure in HALF_OPEN, opening circuit");
                self.state = State::Open {
                    until: Instant::now() + self.reset_timeout,
                };
            }
            State::Open { .. } => {
                // Shouldn't happen
            }
        }
    }

    /// Get statistics
    fn stats(&self) -> (u64, u64, u64, u64) {
        (
            self.total_calls,
            self.successful_calls,
            self.failed_calls,
            self.rejected_calls,
        )
    }
}

/// Simulated external service
struct UnreliableService {
    should_fail: bool,
}

impl UnreliableService {
    fn new() -> Self {
        UnreliableService { should_fail: false }
    }

    fn set_failing(&mut self, failing: bool) {
        self.should_fail = failing;
    }

    fn call(&self) -> Result<String, String> {
        if self.should_fail {
            Err("Service unavailable".to_string())
        } else {
            Ok("Success".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Circuit Breaker Demo ===\n");

    // Create circuit breaker: opens after 3 failures, 2 second timeout
    let mut breaker = CircuitBreaker::new(3, Duration::from_secs(2));
    let mut service = UnreliableService::new();

    // Test 1: Normal operation
    println!("Test 1: Normal operation (service healthy)");
    println!("------------------------------------------");
    println!("  State: {}", breaker.state());

    for i in 1..=3 {
        match breaker.call(|| service.call()) {
            Ok(result) => println!("  Call {}: {}", i, result),
            Err(e) => println!("  Call {}: Error - {}", i, e),
        }
    }

    // Test 2: Service starts failing
    println!("\nTest 2: Service starts failing");
    println!("-------------------------------");
    service.set_failing(true);

    for i in 1..=5 {
        println!("  State: {}, Failures: {}", breaker.state(), breaker.failure_count);
        match breaker.call(|| service.call()) {
            Ok(result) => println!("  Call {}: {}", i, result),
            Err(e) => println!("  Call {}: Error - {}", i, e),
        }
    }

    // Test 3: Circuit is open - calls rejected
    println!("\nTest 3: Circuit open - calls rejected immediately");
    println!("-------------------------------------------------");
    println!("  State: {}", breaker.state());

    for i in 1..=3 {
        match breaker.call(|| service.call()) {
            Ok(result) => println!("  Call {}: {}", i, result),
            Err(CircuitError::Open) => println!("  Call {}: REJECTED (circuit open)", i),
            Err(e) => println!("  Call {}: Error - {}", i, e),
        }
    }

    // Test 4: Wait for timeout
    println!("\nTest 4: Wait for timeout (2 seconds)");
    println!("-------------------------------------");
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Service is still failing - should open again after one failure
    println!("  Service still failing...");
    println!("  State before call: {}", breaker.state());

    match breaker.call(|| service.call()) {
        Ok(result) => println!("  Test call: {}", result),
        Err(e) => println!("  Test call: Error - {} (circuit opens again)", e),
    }

    println!("  State after call: {}", breaker.state());

    // Test 5: Service recovers
    println!("\nTest 5: Service recovers");
    println!("------------------------");
    tokio::time::sleep(Duration::from_secs(3)).await;

    service.set_failing(false);
    println!("  Service is healthy again");
    println!("  State before call: {}", breaker.state());

    match breaker.call(|| service.call()) {
        Ok(result) => println!("  Test call: {} (circuit closes)", result),
        Err(e) => println!("  Test call: Error - {}", e),
    }

    println!("  State after call: {}", breaker.state());

    // Verify circuit is closed
    println!("\nTest 6: Verify circuit is closed");
    println!("---------------------------------");
    for i in 1..=3 {
        match breaker.call(|| service.call()) {
            Ok(result) => println!("  Call {}: {}", i, result),
            Err(e) => println!("  Call {}: Error - {}", i, e),
        }
    }

    // Statistics
    let (total, success, failed, rejected) = breaker.stats();
    println!("\n=== Statistics ===");
    println!("Total calls: {}", total);
    println!("Successful:  {}", success);
    println!("Failed:      {}", failed);
    println!("Rejected:    {}", rejected);

    println!("\n=== Key Concepts ===");
    println!("- CLOSED: Normal operation, counting failures");
    println!("- OPEN: Failing fast, rejecting all calls");
    println!("- HALF_OPEN: Testing if service recovered");
    println!("- Prevents cascade failures");
    println!("- Gives failing service time to recover");
}

// Key concepts demonstrated:
//
// 1. STATE MACHINE:
//    - CLOSED -> OPEN: After failure_threshold failures
//    - OPEN -> HALF_OPEN: After reset_timeout
//    - HALF_OPEN -> CLOSED: On success
//    - HALF_OPEN -> OPEN: On failure
//
// 2. FAIL FAST:
//    - When open, reject immediately
//    - Don't waste resources on failing calls
//    - Return error quickly to caller
//
// 3. RECOVERY TESTING:
//    - Half-open allows test calls
//    - Single success closes circuit
//    - Single failure opens again
//
// 4. TIMEOUT:
//    - Give service time to recover
//    - Don't test constantly
//    - Configurable based on service

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_closed() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(30));
        assert!(matches!(breaker.state(), State::Closed));
    }

    #[test]
    fn test_opens_after_threshold() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(30));

        // 3 failures should open
        for _ in 0..3 {
            let _ = breaker.call(|| Err::<(), _>("fail".to_string()));
        }

        assert!(matches!(breaker.state(), State::Open { .. }));
    }

    #[test]
    fn test_rejects_when_open() {
        let mut breaker = CircuitBreaker::new(1, Duration::from_secs(30));

        // Open the circuit
        let _ = breaker.call(|| Err::<(), _>("fail".to_string()));

        // Next call should be rejected
        let result = breaker.call(|| Ok::<_, String>("success"));
        assert!(matches!(result, Err(CircuitError::Open)));
    }

    #[test]
    fn test_success_resets_failures() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(30));

        // 2 failures
        let _ = breaker.call(|| Err::<(), _>("fail".to_string()));
        let _ = breaker.call(|| Err::<(), _>("fail".to_string()));

        // 1 success resets
        let _ = breaker.call(|| Ok::<_, String>("success"));

        // 2 more failures shouldn't open
        let _ = breaker.call(|| Err::<(), _>("fail".to_string()));
        let _ = breaker.call(|| Err::<(), _>("fail".to_string()));

        assert!(matches!(breaker.state(), State::Closed));
    }
}
