// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use chrono::Timelike;

use libparsec_tests_lite::prelude::*;

use super::*;

#[test]
fn datetime_deserialize_has_microsecond_precision() {
    let serialized = &hex!("d70141d86ad584cd5d4f")[..];
    let expected_timestamp_nanos = 1638618643208820000;

    let dt: DateTime = rmp_serde::from_slice(serialized).unwrap();
    p_assert_eq!(
        dt.as_ref().timestamp_nanos_opt(),
        Some(expected_timestamp_nanos)
    );

    // Round trip
    let serialized2 = rmp_serde::to_vec_named(&dt).unwrap();
    p_assert_eq!(serialized2, serialized);
}

#[test]
fn datetime_parse_has_microsecond_precision() {
    let dt1 = DateTime::from_rfc3339("2021-12-04T11:50:43.208820992Z").unwrap();
    let dt2 = DateTime::from_rfc3339("2021-12-04T11:50:43.208820Z").unwrap();
    p_assert_eq!(dt1, dt2);
    let dt3 = DateTime::from_rfc3339("2021-12-04T11:50:43.208821Z").unwrap();
    p_assert_ne!(dt1, dt3);

    p_assert_eq!(dt1.0.nanosecond() % 1000, 0);
    p_assert_eq!(dt2.0.nanosecond() % 1000, 0);
    p_assert_eq!(dt3.0.nanosecond() % 1000, 0);
}

#[test]
fn datetime_attributes() {
    let dt = DateTime::from_rfc3339("2000-1-2T12:30:59.123456Z").unwrap();
    p_assert_eq!(dt.year(), 2000);
    p_assert_eq!(dt.month(), 1);
    p_assert_eq!(dt.day(), 2);
    p_assert_eq!(dt.hour(), 12);
    p_assert_eq!(dt.minute(), 30);
    p_assert_eq!(dt.second(), 59);
    p_assert_eq!(dt.microsecond(), 123456);
}

#[test]
fn datetime_now_has_microsecond_precision() {
    let time_provider = TimeProvider::default();
    // The inner loop is much faster than a microsecond, so if we run it multiple
    // times we are sure we won't by hazard end up with a now datetime
    // with nanoseconds last 3 digits at 0.
    for _ in 0..10 {
        let dt = time_provider.now();
        let ns = dt.0.nanosecond();
        p_assert_eq!(ns % 1000, 0);
    }
}

#[rstest]
#[case("2000-01-01T00:00:00Z", 0)]
#[case("2000-01-01T00:00:00+00:00", 0)]
#[case("2000-01-01T01:00:00+01:00", 0)]
#[case("2000-01-01T01:25:00+01:25", 0)]
#[case("1999-12-31T23:00:00-01:00", 0)]
// Test with milliseconds
#[case("2000-01-01T00:00:00.123+00:00", 123000)]
#[case("2000-01-01T01:00:00.123+01:00", 123000)]
#[case("2000-01-01T01:25:00.123+01:25", 123000)]
#[case("1999-12-31T23:00:00.123-01:00", 123000)]
// Test with microseconds
#[case("2000-01-01T00:00:00.123456Z", 123456)]
#[case("2000-01-01T00:00:00.123456+00:00", 123456)]
#[case("2000-01-01T01:00:00.123456+01:00", 123456)]
#[case("2000-01-01T01:25:00.123456+01:25", 123456)]
#[case("1999-12-31T23:00:00.123456-01:00", 123456)]
fn rfc3339_conversion(#[case] raw: &str, #[case] micro: u32) {
    let dt = DateTime::from_rfc3339(raw);
    p_assert_eq!(
        dt,
        Ok(DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, micro).unwrap())
    );

    let dt = dt.unwrap();

    let expected = match micro {
        0 => "2000-01-01T00:00:00Z".to_owned(),
        x if x % 1000 == 0 => format!("2000-01-01T00:00:00.{}Z", micro / 1000),
        _ => format!("2000-01-01T00:00:00.{micro}Z"),
    };
    p_assert_eq!(dt.to_rfc3339(), expected);

    // Finally cheap test on idempotence
    p_assert_eq!(DateTime::from_rfc3339(&dt.to_rfc3339()), Ok(dt));
}

#[rstest]
#[case("2000-01-01")] // Missing time part
#[case("2000-01-01T00:00:00")] // Missing tz
#[case("2000-01-01T00:00:00+42:00")]
#[case("2000-01-01T01:00:")]
#[case("2000-01-01T01")]
#[case("2000-01-01T")]
#[case("whatever")]
fn datetime_from_rfc3339_bad_parsing(#[case] raw: &str) {
    let ret = DateTime::from_rfc3339(raw);
    p_assert_eq!(ret.is_err(), true);
}

type Part = (Box<dyn Fn(DateTime) -> u32>, i64);

#[test]
fn arithmetic() {
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
