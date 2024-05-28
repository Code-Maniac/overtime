fn main() { }

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use timeouts::Timeout;

    #[test]
    fn expiry_test() {
        let t = Timeout::new(Duration::from_millis(100));

        let now = Instant::now();
        // sleep for less than the timeout and check expiry
        spin_sleep::sleep(Duration::from_millis(10));
        assert!(!t.expired());

        // sleep for more than the timeout and then check expiry
        spin_sleep::sleep(Duration::from_millis(100));
        assert!(t.expired())
    }

    #[test]
    fn reset_test() {
        let mut t = Timeout::new(Duration::from_millis(100));

        // sleep for more than the timeout and check expiry
        spin_sleep::sleep(Duration::from_millis(100));
        assert!(t.expired());

        // reset the timer and check expiry again
        t.reset();
        assert!(!t.expired());

        // wait again for more than the timeout and check expired again
        spin_sleep::sleep(Duration::from_millis(100));
        assert!(t.expired());
    }

    #[test]
    fn now_test() {
        // Check that a timeout created with Timeout::now() is immediately expired
        let t = Timeout::now();
        assert!(t.expired());
    }

    #[test]
    fn never_test() {
        // Check that a timeout created with Timeout::never is not immediately expired
        let t = Timeout::never();
        assert!(!t.expired());

        // approximately
        const SECONDS_IN_10_YEARS_MINUS_5: u64 = (315_360_000u64) - 5;

        // Check that the timeout from Timeout::never() has a timeout of > than 1 year
        assert!(t.remaining_s() > SECONDS_IN_10_YEARS_MINUS_5);
    }

    macro_rules! assert_match {
        ($val: expr, $mat: pat) => {
            match $val {
                $mat => {
                    // do nothing because we passed
                }
                _ => assert!(false)
            }
        }
    }

    #[test]
    fn remaining_test() {
        let mut t = Timeout::new(Duration::from_secs(1000));
        assert_match!(t.remaining_s(), 999..=1000);

        t.reset();
        assert_match!(t.remaining(), 999_990..=1_000_000);
        assert_match!(t.remaining_ms(), 999_990..=1_000_000);

        t.reset();
        assert_match!(t.remaining_us(), 999_999_990..=1_000_000_000);

        t.reset();
        assert_match!(t.remaining_ns(), 999_999_999_000..=1_000_000_000_000);
    }
}
