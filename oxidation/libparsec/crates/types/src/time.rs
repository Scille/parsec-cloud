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
    pub fn from_ymd_and_hms(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) -> Self {
        Self(
            chrono::Utc
                .ymd(year as i32, month as u32, day as u32)
                .and_hms(hour as u32, minute as u32, second as u32),
        )
    }

    // Don't implement this as `From<f64>` to keep it private
    pub fn from_f64_with_us_precision(ts: f64) -> Self {
        let mut t = ts.trunc() as i64;
        let mut us = (ts.fract() * 1e6).round() as i32;
        if us >= 1000000 {
            t += 1;
            us -= 1000000;
        } else if us < 0 {
            t -= 1;
            us += 1000000;
        }

        Self(chrono::Utc.timestamp_opt(t, (us as u32) * 1000).unwrap())
    }

    // Don't implement this as `Into<f64>` to keep it private
    pub fn get_f64_with_us_precision(&self) -> f64 {
        let ts_us = self.0.timestamp_nanos() / 1000;
        ts_us as f64 / 1e6
    }

    pub fn add_us(&self, us: i64) -> Self {
        Self(self.0 + Duration::microseconds(us))
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

    /// Return a date-time formatted string in the rfc3339 format with a precision update to milliseconds.
    /// Equivalent to ISO-8601
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockedTime {
    RealTime,
    FrozenTime(DateTime),
    ShiftedTime { microseconds: i64 },
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
        static ref MOCK_CONFIG_HAS_CHANGED: libparsec_platform_async::Notify =
            libparsec_platform_async::Notify::new();
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

        pub fn mock_time(&mut self, time: MockedTime) {
            self.time = time;
            // Broadcast the config change given it impact everybody waiting
            MOCK_CONFIG_HAS_CHANGED.notify_waiters();
        }

        fn parent_now_or_realtime(&self) -> DateTime {
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
            // Increase the `TimeProvider.sleeping_stats` counter and ensure
            // it will be correctly decreased even if the future is aborted
            let _updated_stats = SleepingStatPlusOne::new(self.0.clone());

            let mut remaining_time = time;
            // Err is `to_sleep` gets negative, if such we are done with sleeping !
            while let Ok(to_sleep) = remaining_time.to_std() {
                let sleep_started_at = self.now();
                libparsec_platform_async::select!(
                    _ = libparsec_platform_async::native::sleep(to_sleep).fuse() => break,
                    _ = MOCK_CONFIG_HAS_CHANGED.notified().fuse() => {
                        // Recompute the time we still have to sleep
                        remaining_time = remaining_time - (self.now() - sleep_started_at);
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
 * DateTime
 */

impl std::convert::AsRef<chrono::DateTime<chrono::Utc>> for DateTime {
    #[inline]
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DateTime")
            .field(&self.to_string())
            .field(&self.0.nanosecond())
            .finish()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, false)
        )
    }
}

impl std::str::FromStr for DateTime {
    type Err = chrono::format::ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|dt: chrono::DateTime<chrono::Utc>| dt.into())
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
 * LocalDateTime
 */

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct LocalDateTime(chrono::DateTime<chrono::Local>);

impl From<DateTime> for LocalDateTime {
    fn from(dt: DateTime) -> Self {
        Self(dt.0.into())
    }
}

impl LocalDateTime {
    pub fn from_ymd_and_hms(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) -> Self {
        Self(
            chrono::Local
                .ymd(year as i32, month as u32, day as u32)
                .and_hms(hour as u32, minute as u32, second as u32),
        )
    }

    // Don't implement this as `From<f64>` to keep it private
    pub fn from_f64_with_us_precision(ts: f64) -> Self {
        let mut t = ts.trunc() as i64;
        let mut us = (ts.fract() * 1e6).round() as i32;
        if us >= 1000000 {
            t += 1;
            us -= 1000000;
        } else if us < 0 {
            t -= 1;
            us += 1000000;
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
        let dt1: DateTime = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
        let dt2: DateTime = "2021-12-04T11:50:43.208820Z".parse().unwrap();
        assert_eq!(dt1, dt2);
        let dt3: DateTime = "2021-12-04T11:50:43.208821Z".parse().unwrap();
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
}
