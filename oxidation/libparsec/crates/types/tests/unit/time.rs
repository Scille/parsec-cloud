// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use chrono::Timelike;

use crate::{DateTime, TimeProvider};

#[test]
fn test_datetime_parse_has_microsecond_precision() {
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
fn test_datetime_now_has_microsecond_precision() {
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
