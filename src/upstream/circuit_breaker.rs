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

    fn check_reset_timeout(&self) -> bool {
        let last_failure = self.last_failure_time.lock().unwrap();
        if let Some(last) = *last_failure {
            last.elapsed() > self.reset_timeout
        } else {
            false
        }
    }

    pub async fn is_call_allowed(&self) -> bool {
        let state = *self.state.lock().await;
        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if self.check_reset_timeout() {
                    let mut state = self.state.lock().await;
                    *state = CircuitState::HalfOpen;
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub async fn record_success(&self) {
        let mut state = self.state.lock().await;
        *state = CircuitState::Closed;
        self.failure_count.store(0, Ordering::SeqCst);
    }

    pub async fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        {
            let mut last_failure = self.last_failure_time.lock().unwrap();
            *last_failure = Some(Instant::now());
        }

        if count >= self.failure_threshold {
            let mut state = self.state.lock().await;
            *state = CircuitState::Open;
        }
    }

    pub async fn state(&self) -> CircuitState {
        *self.state.lock().await
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5, Duration::from_secs(60))
    }
}
