// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use chrono::{Datelike, TimeZone, Timelike};
use core::ops::Sub;

pub use chrono::Duration; // Reexported

/*
 * DateTime
 */

// DateTime with microsecond precision.
//
// Python's datetime uses microseconds precision unlike chrono::Datetime (which goes
// up to the nanosecond).
// In theory this is not a big deal given microsecond is good enough for our needs.
// However things get ugly given our serialization protocol encode the datetime
// as a 64bits floating number.
//
// Floating point numbers have a step depending of the size of the number. This may
// cause rounding issues when converting into datetime if we try to retrieve
// too much precision.
// Typically if we consider a 1e9 timestamp (representing 2001-9-9T1:46:40.0Z) the
// floating point atomic step is 1e-7, hence we are safe to represent microseconds,
// but not nanoseconds.
// This property is kept up until ~4e9 (so around year 2096). In our case this is
// "fine enough" given we use datetime to store events that have occurred (hence
// further fix is required in 70years ^^).
//
// Hence we choose to use microsecond in Rust to avoid potential tenacious bugs due
// to a datetime with sub-microsecond precision not equal to itself after being
// serialized (i.e. `dt = now(); dt != load(dump(dt))`).
//
// Aaaaaand we've learn a lesson here, next time we will stick with good old integer
// instead of playing smart with float !
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub fn from_ymd_hms_us(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> Self {
        Self(
            chrono::Utc
                .ymd(year, month, day)
                .and_hms_micro(hour, minute, second, microsecond),
        )
    }

    // Don't implement this as `From<f64>` to prevent misunderstanding on precision
    pub fn from_f64_with_us_precision(ts: f64) -> Self {
        let mut t = ts.trunc() as i64;
        let mut us = (ts.fract() * 1e6).round() as i32;
        if us >= 1_000_000 {
            t += 1;
            us -= 1_000_000;
        } else if us < 0 {
            t -= 1;
            us += 1_000_000;
        }

        Self(chrono::Utc.timestamp_opt(t, (us as u32) * 1000).unwrap())
    }

    // Don't implement this as `Into<f64>` to prevent misunderstanding on precision
    pub fn get_f64_with_us_precision(&self) -> f64 {
        let ts_us = self.0.timestamp_nanos() / 1000;
        ts_us as f64 / 1e6
    }

    pub fn add_us(&self, us: i64) -> Self {
        Self(self.0 + Duration::microseconds(us))
    }

    pub fn year(&self) -> i32 {
        self.0.year()
    }

    pub fn month(&self) -> u32 {
        self.0.month()
    }

    pub fn day(&self) -> u32 {
        self.0.day()
    }

    pub fn hour(&self) -> u32 {
        self.0.hour()
    }

    pub fn minute(&self) -> u32 {
        self.0.minute()
    }

    pub fn second(&self) -> u32 {
        self.0.second()
    }

    pub fn to_local(self) -> LocalDateTime {
        self.into()
    }

    // TODO: remove me and only rely on TimeProvider instead !
    #[cfg(not(feature = "mock-time"))]
    #[inline]
    pub fn now_legacy() -> Self {
        let now = chrono::Utc::now();
        now.into()
    }


    /// Use RFC3339 format when stable serialization to text is needed (e.g. in `OrganizationFileLink`)
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
    }

    /// Use RFC3339 format when stable serialization to text is needed (e.g. in `OrganizationFileLink`)
    pub fn from_rfc3339(s: &str) -> Result<Self, chrono::ParseError> {
        s.parse().map(|dt: chrono::DateTime<chrono::Utc>| dt.into())
    }
}

impl std::convert::AsRef<chrono::DateTime<chrono::Utc>> for DateTime {
    #[inline]
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DateTime").field(&self.to_rfc3339()).finish()
    }
}

// `std::fmt::Display` is convenient but ambiguous about it format (and the
// guarantee it won't change !), so:
// - display should only be used for human display (e.g. CLI output, error messages)
// - for actual serialization `DateTime::to_rfc3339` must be used instead
impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

// `FromStr` is a convenient but ambiguous shortcut, we should only use it
// for tests and prefere `DateTime::from_rfc3339` elsewhere
// TODO: prevent me from being used outside of test code
impl std::str::FromStr for DateTime {
    type Err = chrono::format::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_rfc3339(s)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        // Force precision to the microsecond
        Self(
            chrono::Utc
                .timestamp_opt(dt.timestamp(), dt.timestamp_subsec_micros() * 1000)
                .unwrap(),
        )
    }
}

impl From<LocalDateTime> for DateTime {
    fn from(ldt: LocalDateTime) -> Self {
        Self(ldt.0.into())
    }
}

impl Sub for DateTime {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

/*
 * MockedTime
 */

#[derive(Debug, Clone, PartialEq)]
pub enum MockedTime {
    RealTime,
    FrozenTime(DateTime),
    ShiftedTime {
        microseconds: i64,
    },
    FasterTime {
        reference: DateTime,
        microseconds: i64,
        speed_factor: f64,
    },
}

// TODO: remove me and only rely on TimeProvider instead !
#[cfg(feature = "mock-time")]
mod mock_time {
    use super::{DateTime, MockedTime};
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<MockedTime> = RefCell::new(MockedTime::RealTime);
    }

    impl DateTime {
        pub fn now_legacy() -> Self {
            MOCK_TIME.with(|cell| match *cell.borrow() {
                MockedTime::RealTime => chrono::Utc::now().into(),
                MockedTime::FrozenTime(dt) => dt,
                MockedTime::ShiftedTime { microseconds: us } => {
                    DateTime::from(chrono::Utc::now()).add_us(us)
                }
                MockedTime::FasterTime {
                    reference,
                    microseconds: us,
                    speed_factor,
                } => {
                    let delta = DateTime::from(chrono::Utc::now()) - reference;
                    let delta_us = delta
                        .num_microseconds()
                        .expect("No reason to get an overflow");
                    let speed_shift = speed_factor * delta_us as f64;
                    reference.add_us(us + speed_shift as i64)
                }
            })
        }

        pub fn mock_time(time: MockedTime) {
            MOCK_TIME.with(|cell| *cell.borrow_mut() = time)
        }
    }
}

/*
 * TimeProvider
 */

// Taking the current time constitutes a side effect, on top of that we want to be able
// to simulate in our tests complex behavior where different Parsec client/server have
// shifting clocks.
// So the solution here is to force the current time to be taken from a non-global object
// (typically each client/server should have it own) that can be independantly mocked.

#[cfg(not(feature = "mock-time"))]
mod time_provider {

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct TimeProvider;

    impl TimeProvider {
        pub fn now(&self) -> super::DateTime {
            let now = chrono::Utc::now();
            now.into()
        }

        pub async fn sleep(&self, time: std::time::Duration) {
            libparsec_platform_async::native::sleep(time).await
        }
    }
}

#[cfg(feature = "mock-time")]
mod time_provider {
    use std::sync::{Arc, Mutex};

    use libparsec_platform_async::future::FutureExt;

    use super::DateTime;
    use super::MockedTime;

    // For simplicity we only keep as single global event that will be triggered for any
    // instead of giving each
    // In theory each TimeProviderAgent should have it own event triggered when it
    // mock config has been changed.
    // However it is much simpler to have instead a single global event. This way any change
    // in mock config will wake up all the TimeProviderAgent currently sleeping, but it's
    // not a big deal given:
    // 1) this is only for test where it's most likely they will be only one thing to wakeup anyway
    // 2) Any wrongly waked up coroutine will just realized it is too soon and go back to sleep
    lazy_static! {
        static ref MOCK_CONFIG_HAS_CHANGED: (
            libparsec_platform_async::watch::Sender<()>,
            libparsec_platform_async::watch::Receiver<()>
        ) = libparsec_platform_async::watch::channel(());
    }

    /// Time provider system consist of a hierarchy of agents, each one able to mock it
    /// time while taking into account it parent's mock time:
    /// - RealTime agent will use it parent's time if it has one or the actual real time otherwise
    /// - FrozenTime agent will always use the same time configured for it
    /// - ShiftedTime agent work as the RealTime agent but add it configured shifted value on top
    #[derive(Debug)]
    struct TimeProviderAgent {
        parent: Option<Arc<Mutex<TimeProviderAgent>>>,
        time: MockedTime,
        sleeping_stats: u64,
    }

    impl TimeProviderAgent {
        pub fn new(parent: Option<Arc<Mutex<TimeProviderAgent>>>) -> Self {
            Self {
                parent,
                time: MockedTime::RealTime,
                sleeping_stats: 0,
            }
        }

        pub fn get_speed_factor(&self) -> Option<f64> {
            match (&self.parent, &self.time) {
                (None, MockedTime::FasterTime { speed_factor, .. }) => Some(*speed_factor),
                (None, _) => None,
                (Some(parent), MockedTime::FasterTime { speed_factor, .. }) => Some(
                    speed_factor
                        * parent
                            .lock()
                            .expect("Mutex is poisoned")
                            .get_speed_factor()
                            .unwrap_or(1.0),
                ),
                (Some(parent), _) => parent.lock().expect("Mutex is poisoned").get_speed_factor(),
            }
        }

        pub fn mock_time(&mut self, time: MockedTime) {
            self.time = time;
            // Broadcast the config change given it impact everybody waiting
            MOCK_CONFIG_HAS_CHANGED
                .0
                .send(())
                .expect("The channel is closed");
        }

        pub fn parent_now_or_realtime(&self) -> DateTime {
            match &self.parent {
                // TODO: remove me and only rely on `chrono::Utc::now().into()` instead !
                None => DateTime::now_legacy(),
                Some(parent) => parent.lock().expect("Mutex is poisoned").now(),
            }
        }

        pub fn now(&self) -> DateTime {
            match self.time {
                MockedTime::RealTime => self.parent_now_or_realtime(),
                MockedTime::FrozenTime(dt) => dt,
                MockedTime::ShiftedTime { microseconds: us } => {
                    self.parent_now_or_realtime().add_us(us)
                }
                MockedTime::FasterTime {
                    reference,
                    microseconds: us,
                    speed_factor,
                } => {
                    let delta = self.parent_now_or_realtime() - reference;
                    let delta_us = delta
                        .num_microseconds()
                        .expect("No reason to get an overflow");
                    let speed_shift = speed_factor * delta_us as f64;
                    reference.add_us(us + speed_shift as i64)
                }
            }
        }
    }

    /// Wrap the `TimeProviderAgent` into `TimeProvider` to hide the sync stuff
    /// and provide the same simple API as the non-test `TimeProvider`
    #[derive(Debug, Clone)]
    pub struct TimeProvider(Arc<Mutex<TimeProviderAgent>>);

    // Implement Eq&PartialEq as noop to keep the same behavior than in prod
    // where TimeProvider is an empty structure
    impl Eq for TimeProvider {}
    impl PartialEq for TimeProvider {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    impl Default for TimeProvider {
        fn default() -> Self {
            Self(Arc::new(Mutex::new(TimeProviderAgent::new(None))))
        }
    }

    /// Ensure the sleep
    struct SleepingStatPlusOne(Arc<Mutex<TimeProviderAgent>>);
    impl SleepingStatPlusOne {
        pub fn new(time_provider: Arc<Mutex<TimeProviderAgent>>) -> Self {
            time_provider
                .lock()
                .expect("Mutex is poisoned")
                .sleeping_stats += 1;
            Self(time_provider)
        }
    }
    impl Drop for SleepingStatPlusOne {
        fn drop(&mut self) {
            self.0.lock().expect("Mutex is poisoned").sleeping_stats -= 1;
        }
    }

    impl TimeProvider {
        pub fn now(&self) -> DateTime {
            self.0.lock().expect("Mutex is poisoned").now()
        }

        pub fn parent_now_or_realtime(&self) -> DateTime {
            self.0
                .lock()
                .expect("Mutex is poisoned")
                .parent_now_or_realtime()
        }

        /// When we call sleep, there is no guarantee when the starting time will be taken.
        /// Hence it is possible this time is taken after we have call the mock_time (this
        /// is typically the case when calling sleep from Python where the actual sleep is
        /// scheduled on a tokio thread while directly returning a fake coroutine).
        /// So the solution is to use a busyloop in the testing code that watch over the
        /// number of tasks currently sleeping on our time provider.
        pub fn sleeping_stats(&self) -> u64 {
            self.0.lock().expect("Mutex is poisoned").sleeping_stats
        }

        pub async fn sleep(&self, time: super::Duration) {
            // The order is important here: we first clone the configuration
            // to make sure we won't miss a configuration change while we're
            // taking a time reference using now()
            let mut config = MOCK_CONFIG_HAS_CHANGED.1.clone();
            let mut sleep_started_at = self.now();

            // Increase the `TimeProvider.sleeping_stats` counter and ensure
            // it will be correctly decreased even if the future is aborted
            let _updated_stats = SleepingStatPlusOne::new(self.0.clone());

            let mut remaining_time = time;

            // Err is `to_sleep` gets negative, if such we are done with sleeping !
            while let Ok(mut to_sleep) = remaining_time.to_std() {
                if let Some(speed_factor) =
                    self.0.lock().expect("Mutex is poisoned").get_speed_factor()
                {
                    to_sleep = to_sleep.div_f64(speed_factor);
                }
                libparsec_platform_async::select!(
                    _ = libparsec_platform_async::native::sleep(to_sleep).fuse() => break,
                    _ = config.changed().fuse() => {
                        // Recompute the time we still have to sleep
                        let now = self.now();
                        remaining_time = remaining_time - (now - sleep_started_at);
                        sleep_started_at = now;
                    }
                );
            }
        }

        // Following methods are only implemented for testing purpose

        pub fn new_child(&self) -> Self {
            Self(Arc::new(Mutex::new(TimeProviderAgent::new(Some(
                self.0.clone(),
            )))))
        }

        pub fn mock_time(&mut self, time: MockedTime) {
            self.0.lock().expect("Mutex is poisoned").mock_time(time);
        }
    }
}

pub use time_provider::TimeProvider;

/*
 * LocalDateTime
 */

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct LocalDateTime(chrono::DateTime<chrono::Local>);

impl From<DateTime> for LocalDateTime {
    fn from(dt: DateTime) -> Self {
        Self(dt.0.into())
    }
}

impl LocalDateTime {
    pub fn from_ymd_hms_us(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> Self {
        Self(
            chrono::Local
                .ymd(year, month, day)
                .and_hms_micro(hour, minute, second, microsecond),
        )
    }

    // Don't implement this as `From<f64>` to keep it private
    pub fn from_f64_with_us_precision(ts: f64) -> Self {
        let mut t = ts.trunc() as i64;
        let mut us = (ts.fract() * 1e6).round() as i32;
        if us >= 1_000_000 {
            t += 1;
            us -= 1_000_000;
        } else if us < 0 {
            t -= 1;
            us += 1_000_000;
        }

        Self(chrono::Local.timestamp_opt(t, (us as u32) * 1000).unwrap())
    }

    // Don't implement this as `Into<f64>` to keep it private
    pub fn get_f64_with_us_precision(&self) -> f64 {
        let ts_us = self.0.timestamp_nanos() / 1000;
        ts_us as f64 / 1e6
    }

    pub fn year(&self) -> u64 {
        self.0.year() as u64
    }

    pub fn month(&self) -> u64 {
        self.0.month() as u64
    }

    pub fn day(&self) -> u64 {
        self.0.day() as u64
    }

    pub fn hour(&self) -> u64 {
        self.0.hour() as u64
    }

    pub fn minute(&self) -> u64 {
        self.0.minute() as u64
    }

    pub fn second(&self) -> u64 {
        self.0.second() as u64
    }

    pub fn to_utc(self) -> DateTime {
        self.into()
    }

    pub fn format(&self, fmt: &str) -> String {
        self.0.format(fmt).to_string()
    }
}

impl std::fmt::Debug for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LocalDateTime")
            .field(&self.to_string())
            .field(&self.0.nanosecond())
            .finish()
    }
}

impl std::fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, false)
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use hex_literal::hex;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_datetime_deserialize_has_microsecond_precision() {
        let serialized = &hex!("d70141d86ad584cd5d4f")[..];
        let expected_timestamp_nanos = 1638618643208820000;

        let dt: DateTime = rmp_serde::from_slice(serialized).unwrap();
        assert_eq!(dt.as_ref().timestamp_nanos(), expected_timestamp_nanos);

        // Round trip
        let serialized2 = rmp_serde::to_vec_named(&dt).unwrap();
        assert_eq!(serialized2, serialized);
    }

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
    fn test_rfc3339_conversion(#[case] raw: &str, #[case] micro: u32) {
        let dt = DateTime::from_rfc3339(raw);
        assert_eq!(
            dt,
            Ok(DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, micro))
        );

        let dt = dt.unwrap();

        let expected = match micro {
            0 => "2000-01-01T00:00:00Z".to_owned(),
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
    fn test_datetime_from_rfc3339_bad_parsing(#[case] raw: &str) {
        let ret = DateTime::from_rfc3339(raw);
        assert_eq!(ret.is_err(), true);
    }
}
