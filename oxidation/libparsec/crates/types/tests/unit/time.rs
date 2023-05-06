// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use chrono::Timelike;
use hex_literal::hex;
use pretty_assertions::{assert_eq, assert_ne};
use rstest::rstest;

use super::*;

#[test]
fn datetime_deserialize_has_microsecond_precision() {
    let serialized = &hex!("d70141d86ad584cd5d4f")[..];
    let expected_timestamp_nanos = 1638618643208820000;

    let dt: DateTime = rmp_serde::from_slice(serialized).unwrap();
    assert_eq!(dt.as_ref().timestamp_nanos(), expected_timestamp_nanos);

    // Round trip
    let serialized2 = rmp_serde::to_vec_named(&dt).unwrap();
    assert_eq!(serialized2, serialized);
}

#[test]
fn datetime_parse_has_microsecond_precision() {
    let dt1 = DateTime::from_rfc3339("2021-12-04T11:50:43.208820992Z").unwrap();
    let dt2 = DateTime::from_rfc3339("2021-12-04T11:50:43.208820Z").unwrap();
    assert_eq!(dt1, dt2);
    let dt3 = DateTime::from_rfc3339("2021-12-04T11:50:43.208821Z").unwrap();
    assert_ne!(dt1, dt3);

    assert_eq!(dt1.0.nanosecond() % 1000, 0);
    assert_eq!(dt2.0.nanosecond() % 1000, 0);
    assert_eq!(dt3.0.nanosecond() % 1000, 0);
}

#[test]
fn datetime_attributes() {
    let dt = DateTime::from_rfc3339("2000-1-2T12:30:59.123456Z").unwrap();
    assert_eq!(dt.year(), 2000);
    assert_eq!(dt.month(), 1);
    assert_eq!(dt.day(), 2);
    assert_eq!(dt.hour(), 12);
    assert_eq!(dt.minute(), 30);
    assert_eq!(dt.second(), 59);
    assert_eq!(dt.microsecond(), 123456);
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
        assert_eq!(ns % 1000, 0);
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
    assert_eq!(
        dt,
        Ok(DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, micro).unwrap())
    );

    let dt = dt.unwrap();

    let expected = match micro {
        0 => "2000-01-01T00:00:00Z".to_owned(),
        x if x % 1000 == 0 => format!("2000-01-01T00:00:00.{}Z", micro / 1000),
        _ => format!("2000-01-01T00:00:00.{micro}Z"),
    };
    assert_eq!(dt.to_rfc3339(), expected);

    // Finally cheap test on idempotence
    assert_eq!(DateTime::from_rfc3339(&dt.to_rfc3339()), Ok(dt));
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
    assert_eq!(ret.is_err(), true);
}
