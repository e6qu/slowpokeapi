use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreaker {
    failure_threshold: u64,
    reset_timeout: Duration,
    failure_count: AtomicU64,
    #[allow(clippy::disallowed_types)]
    last_failure_time: std::sync::Mutex<Option<Instant>>,
    state: Mutex<CircuitState>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u64, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            reset_timeout,
            failure_count: AtomicU64::new(0),
            #[allow(clippy::disallowed_types)]
            last_failure_time: std::sync::Mutex::new(None),
            state: Mutex::new(CircuitState::Closed),
        }
    }

    pub fn is_call_allowed(&self) -> bool {
        let state = self.state.blocking_lock();
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                let last_failure = self.last_failure_time.lock().unwrap();
                if let Some(last) = *last_failure {
                    if last.elapsed() > self.reset_timeout {
                        drop(last_failure);
                        drop(state);
                        let mut state = self.state.blocking_lock();
                        *state = CircuitState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub fn record_success(&self) {
        let mut state = self.state.blocking_lock();
        *state = CircuitState::Closed;
        self.failure_count.store(0, Ordering::SeqCst);
    }

    pub fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut last_failure = self.last_failure_time.lock().unwrap();
        *last_failure = Some(Instant::now());
        drop(last_failure);

        if count >= self.failure_threshold {
            let mut state = self.state.blocking_lock();
            *state = CircuitState::Open;
        }
    }

    pub fn state(&self) -> CircuitState {
        *self.state.blocking_lock()
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5, Duration::from_secs(60))
    }
}

#[allow(clippy::disallowed_types)]
impl Clone for CircuitBreaker {
    fn clone(&self) -> Self {
        Self {
            failure_threshold: self.failure_threshold,
            reset_timeout: self.reset_timeout,
            failure_count: AtomicU64::new(self.failure_count.load(Ordering::SeqCst)),
            last_failure_time: std::sync::Mutex::new(*self.last_failure_time.lock().unwrap()),
            state: Mutex::new(*self.state.blocking_lock()),
        }
    }
}
