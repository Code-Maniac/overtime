use std::time::{Duration, Instant};

const SECONDS_IN_10_YEARS: u64 = 315_360_000u64;

/// duration that never expires under normal circumstances
const NEVER: Duration = Duration::from_secs(SECONDS_IN_10_YEARS);

pub struct Timeout {
    duration: Duration,
    expire_time: Instant,
}

impl Timeout {
    /// create a timeout that will expire after the given duration
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            expire_time: Instant::now() + duration,
        }
    }

    /// Timeout that expires now
    pub fn now() -> Self {
        Timeout::new(Duration::from_millis(0))
    }

    /// create a timeout that is far in the future and will never timeout
    pub fn never() -> Self {
        Timeout::new(NEVER)
    }

    /// Reset the timer by adding the initial duration to the current time
    pub fn reset(&mut self) {
        self.expire_time = Instant::now() + self.duration;
    }

    /// Return true if the timeout has expired else return false
    pub fn expired(&self) -> bool {
        return Instant::now() >= self.expire_time;
    }

    /// Return the number of milliseconds remaining until expiry
    pub fn remaining(&self) -> u128 {
        self.remaining_ms()
    }

    /// Return the number of seconds remaining until expiry
    pub fn remaining_s(&self) -> u64 {
        if self.expired() {
            0
        } else {
            (self.expire_time - Instant::now()).as_secs()
        }
    }

    /// Return the number of milliseconds remaining until expiry (more verbose)
    pub fn remaining_ms(&self) -> u128 {
        if self.expired() {
            0
        } else {
            (self.expire_time - Instant::now()).as_millis()
        }
    }

    /// Return the number of microseconds remaining until expiry
    pub fn remaining_us(&self) -> u128 {
        if self.expired() {
            0
        } else {
            (self.expire_time - Instant::now()).as_micros()
        }
    }

    /// Return the number of nanoseconds remaining until expiry
    pub fn remaining_ns(&self) -> u128 {
        if self.expired() {
            0
        } else {
            (self.expire_time - Instant::now()).as_nanos()
        }
    }
}
