//! Lab 4: Circuit Breaker
//!
//! ## Goal
//! Implement circuit breaker pattern with three states
//!
//! ## Requirements
//! 1. Three states: Closed, Open, HalfOpen
//! 2. Open after N consecutive failures
//! 3. Transition to HalfOpen after timeout
//! 4. Close after success in HalfOpen
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === Circuit Breaker Demo ===
//!
//! State: CLOSED
//! Call 1: Success
//! Call 2: Success
//! Call 3: Failure
//! Call 4: Failure
//! Call 5: Failure -> Circuit OPENS
//!
//! State: OPEN
//! Call 6: Rejected (circuit open)
//!
//! (wait for timeout)
//!
//! State: HALF_OPEN
//! Call 7: Testing...
//! Call 7: Success -> Circuit CLOSES
//! ```
//!
//! ## Hints
//! - Use enum for state (with timestamp for Open)
//! - Track consecutive failures
//! - Reset failure count on success
//! - In HalfOpen, single success closes, single failure opens
//!
//! ## Acceptance Criteria
//! - [ ] Starts in Closed state
//! - [ ] Opens after failure threshold
//! - [ ] Rejects calls when Open
//! - [ ] Transitions to HalfOpen after timeout
//! - [ ] Closes on HalfOpen success

use std::time::{Duration, Instant};

// ============================================================
// TODO: Implement circuit breaker
// ============================================================

/// Circuit breaker states
enum State {
    Closed,
    Open { until: Instant },
    HalfOpen,
}

/// Circuit breaker error
enum CircuitError {
    Open,
    Failed(String),
}

/// Circuit breaker
struct CircuitBreaker {
    state: State,
    failure_count: u32,
    failure_threshold: u32,
    reset_timeout: Duration,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, reset_timeout: Duration) -> Self {
        // TODO: Initialize circuit breaker

        todo!("Implement CircuitBreaker::new")
    }

    /// Get current state name
    fn state_name(&self) -> &str {
        // TODO: Return state name

        todo!("Implement state_name")
    }

    /// Execute function through circuit breaker
    fn call<F, T>(&mut self, f: F) -> Result<T, CircuitError>
    where
        F: FnOnce() -> Result<T, String>,
    {
        // TODO: Implement
        // 1. Check if Open and expired -> transition to HalfOpen
        // 2. If Open -> return error
        // 3. Execute function
        // 4. On success -> handle based on state
        // 5. On failure -> handle based on state

        todo!("Implement CircuitBreaker::call")
    }

    /// Record success
    fn on_success(&mut self) {
        // TODO: Reset failure count, close if HalfOpen

        todo!("Implement on_success")
    }

    /// Record failure
    fn on_failure(&mut self) {
        // TODO: Increment failures, open if threshold reached

        todo!("Implement on_failure")
    }
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    // 1. Create circuit breaker
    // 2. Simulate successful calls
    // 3. Simulate failures until circuit opens
    // 4. Show rejected calls
    // 5. Wait for timeout
    // 6. Show recovery

    todo!("Implement main")
}
