use std::time::{Duration, Instant};

pub(crate) struct FrequencyControl {
    interval: Duration,
    interval_increment: Duration,
    min_interval: Duration,
    max_interval: Duration,
    last_called_at: Instant,
}

impl FrequencyControl {
    pub fn new(
        interval: Duration,
        interval_decrement: Duration,
        min_interval: Duration,
        max_interval: Duration,
    ) -> Self {
        Self {
            interval,
            interval_increment: interval_decrement,
            min_interval,
            max_interval,
            last_called_at: Instant::now(),
        }
    }

    pub fn call_if_allowed(&mut self) -> bool {
        let now = Instant::now();

        if now.duration_since(self.last_called_at) >= self.interval {
            if now.duration_since(self.last_called_at) >= self.max_interval {
                self.interval = self.min_interval;
            } else {
                self.interval = self
                    .max_interval
                    .min(self.interval.saturating_add(self.interval_increment));
            }
            self.last_called_at = now;
            true
        } else {
            false
        }
    }
}
