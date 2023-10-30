// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::{Duration, RateLimiter};

#[test]
fn test_rate_limiter() {
    let mut rate = RateLimiter::new();

    assert_eq!(rate.get_duration_to_wait(), Duration::ZERO);

    rate.attempt = 1;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(1));

    rate.attempt = 2;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(5));

    rate.attempt = 3;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(10));

    rate.attempt = 4;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(30));

    rate.attempt = 5;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(60));

    rate.attempt = 10;
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(60));

    rate.set_desired_duration(Duration::from_secs(42));
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(42));

    rate.reset();
    assert_eq!(rate.get_duration_to_wait(), Duration::from_secs(42));
}
