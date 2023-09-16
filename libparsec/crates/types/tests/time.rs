// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

type Part = (Box<dyn Fn(DateTime) -> u32>, i64);

#[test]
fn datetime() {
    let dt = DateTime::from_ymd_hms_us(2000, 1, 2, 12, 30, 45, 123456).unwrap();

    p_assert_eq!(dt.year(), 2000);
    p_assert_eq!(dt.month(), 1);
    p_assert_eq!(dt.day(), 2);
    p_assert_eq!(dt.hour(), 12);
    p_assert_eq!(dt.minute(), 30);
    p_assert_eq!(dt.second(), 45);
    p_assert_eq!(dt.microsecond(), 123456);

    let parts: [Part; 5] = [
        (Box::new(|dt: DateTime| dt.microsecond()), 1),
        (Box::new(|dt: DateTime| dt.second()), 1_000_000),
        (Box::new(|dt: DateTime| dt.minute()), 60 * 1_000_000),
        (Box::new(|dt: DateTime| dt.hour()), 60 * 60 * 1_000_000),
        (Box::new(|dt: DateTime| dt.day()), 24 * 60 * 60 * 1_000_000),
    ];

    for (f, us) in parts {
        p_assert_eq!(f(dt.add_us(us)), f(dt) + 1);
        p_assert_eq!(f(dt.add_us(-us)), f(dt) - 1);
    }

    p_assert_eq!(dt, dt);
    p_assert_ne!(dt.add_us(1), dt);
    p_assert_eq!(dt.add_us(1).add_us(-1), dt);

    assert!(dt < dt.add_us(1));
    assert!(dt > dt.add_us(-1));
    assert!(dt <= dt);
    assert!(dt >= dt);

    p_assert_eq!(dt - dt.add_us(-1), Duration::microseconds(1));

    p_assert_eq!(dt.to_rfc3339(), "2000-01-02T12:30:45.123456Z");
    p_assert_eq!(DateTime::from_rfc3339(&dt.to_rfc3339()).unwrap(), dt);

    p_assert_eq!(dt.get_f64_with_us_precision(), 946816245.123456);
    p_assert_eq!(
        DateTime::from_f64_with_us_precision(dt.get_f64_with_us_precision()),
        dt
    );
}
