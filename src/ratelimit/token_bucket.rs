use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u64,
    tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate: refill_rate as f64,
            last_refill: Instant::now(),
        }
    }

    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.refill();

        let tokens_needed = tokens as f64;
        if self.tokens >= tokens_needed {
            self.tokens -= tokens_needed;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let tokens_to_add = elapsed.as_secs_f64() * self.refill_rate;

        self.tokens = (self.tokens + tokens_to_add).min(self.capacity as f64);
        self.last_refill = now;
    }

    pub fn available_tokens(&mut self) -> u64 {
        self.refill();
        self.tokens as u64
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn time_until_available(&mut self, tokens: u64) -> Duration {
        self.refill();
        let tokens_needed = tokens as f64;
        if self.tokens >= tokens_needed {
            Duration::from_secs(0)
        } else {
            let tokens_deficit = tokens_needed - self.tokens;
            let seconds = tokens_deficit / self.refill_rate;
            Duration::from_secs_f64(seconds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_creation() {
        let bucket = TokenBucket::new(100, 10);
        assert_eq!(bucket.capacity(), 100);
    }

    #[test]
    fn test_consume_tokens() {
        let mut bucket = TokenBucket::new(100, 10);
        assert!(bucket.try_consume(50));
        assert_eq!(bucket.available_tokens(), 50);
    }

    #[test]
    fn test_consume_more_than_available() {
        let mut bucket = TokenBucket::new(100, 10);
        assert!(bucket.try_consume(50));
        assert!(!bucket.try_consume(60));
    }

    #[test]
    fn test_token_refill() {
        let mut bucket = TokenBucket::new(100, 1000);
        bucket.try_consume(100);
        std::thread::sleep(Duration::from_millis(100));
        assert!(bucket.available_tokens() > 0);
    }
}
