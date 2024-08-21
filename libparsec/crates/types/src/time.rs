// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use chrono::{Datelike, LocalResult, TimeZone, Timelike};
use core::ops::{Add, AddAssign, Sub, SubAssign};

pub use chrono::Duration; // Reexported

/*
 * DateTime
 */

// DateTime with microsecond precision.
//
// Why using microsecond precision which `chrono::Datetime` goes up to the nanosecond ?
// The reason is that PostgreSQL's TIMESTAMPZ and Python's datetime types both have
// a microsecond precision, and losing precision on round trip would be very error prone.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(Debug, thiserror::Error)]
pub enum DatetimeFromTimestampMicrosError {
    #[error("Out-of-range timestamp")]
    OutOfRange,
}

impl DateTime {
    /// Create a `DateTime` at the current time.
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }

    /// Create a `DateTime` at the Unix epoch.
    pub fn epoch() -> Self {
        Self(chrono::Utc.timestamp_nanos(0))
    }

    pub fn from_ymd_hms_us(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> Result<Self, DatetimeFromTimestampMicrosError> {
        let outcome = chrono::Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .map(|x| x + Duration::microseconds(microsecond as i64))
            .map(Self);
        match outcome {
            LocalResult::Single(dt) => Ok(dt),
            _ => Err(DatetimeFromTimestampMicrosError::OutOfRange),
        }
    }

    // Don't implement this as `From<i64>` to prevent misunderstanding on precision
    pub fn from_timestamp_micros(ts: i64) -> Result<Self, DatetimeFromTimestampMicrosError> {
        match chrono::Utc.timestamp_micros(ts).map(Self) {
            LocalResult::Single(dt) => Ok(dt),
            _ => Err(DatetimeFromTimestampMicrosError::OutOfRange),
        }
    }

    // Don't implement this as `Into<i64>` to prevent misunderstanding on precision
    pub fn as_timestamp_micros(&self) -> i64 {
        self.0.timestamp_micros()
    }

    // Don't implement this as `From<i64>` to prevent misunderstanding on precision
    pub fn from_timestamp_seconds(ts: i64) -> Result<Self, DatetimeFromTimestampMicrosError> {
        Self::from_timestamp_micros(ts * 1_000_000)
    }

    // Don't implement this as `Into<i64>` to prevent misunderstanding on precision
    pub fn as_timestamp_seconds(&self) -> i64 {
        self.0.timestamp()
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

    pub fn microsecond(&self) -> u32 {
        self.0.nanosecond() / 1000
    }

    /// Use RFC3339 format when stable serialization to text is needed (e.g. in the administration REST API)
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
    }

    /// Use RFC3339 format when stable serialization to text is needed (e.g. in the administration REST API)
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
                // Always valid as params comes from a `chrono::DateTime`
                .unwrap(),
        )
    }
}

impl From<DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(dt: DateTime) -> Self {
        dt.0
    }
}

impl From<DateTime> for std::time::SystemTime {
    fn from(dt: DateTime) -> Self {
        dt.0.into()
    }
}

impl Sub for DateTime {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl AddAssign<Duration> for DateTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs;
    }
}

impl Add<Duration> for DateTime {
    type Output = DateTime;
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl SubAssign<Duration> for DateTime {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs;
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
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
        /// Offset in microseconds between the reference time and the actual
        /// time we had before the mock. The speed factor should not impact this
        /// offset.
        microseconds: i64,
        speed_factor: f64,
    },
}

/*
 * TimeProvider
 */

// Taking the current time constitutes a side effect, on top of that we want to be able
// to simulate in our tests complex behavior where different Parsec client/server have
// shifting clocks.
// So the solution here is to force the current time to be taken from a non-global object
// (typically each client/server should have it own) that can be independently mocked.

#[cfg(not(feature = "test-mock-time"))]
mod time_provider {

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    // Avoid having an unit struct here otherwise clippy will complain about
    // `TimeProvider::default()`
    pub struct TimeProvider(());

    impl TimeProvider {
        pub fn now(&self) -> super::DateTime {
            let now = chrono::Utc::now();
            now.into()
        }

        pub async fn sleep(&self, time: super::Duration) {
            // Err in case time is negative, if such no need to sleep !
            if let Ok(time) = time.to_std() {
                libparsec_platform_async::sleep(time).await;
            }
        }

        pub fn new_child(&self) -> Self {
            Self(())
        }
    }
}

#[cfg(feature = "test-mock-time")]
mod time_provider {
    use std::sync::{Arc, Mutex};

    use super::DateTime;
    use super::MockedTime;

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
            let parent = parent.unwrap_or_else(Self::root_provider);
            Self {
                parent: Some(parent),
                time: MockedTime::RealTime,
                sleeping_stats: 0,
            }
        }

        /// Root provider allows to globally mock time
        pub fn root_provider() -> Arc<Mutex<TimeProviderAgent>> {
            static ROOT_PROVIDER: std::sync::OnceLock<Arc<Mutex<TimeProviderAgent>>> =
                std::sync::OnceLock::new();
            ROOT_PROVIDER
                .get_or_init(|| {
                    Arc::new(Mutex::new(TimeProviderAgent {
                        parent: None,
                        time: MockedTime::RealTime,
                        sleeping_stats: 0,
                    }))
                })
                .to_owned()
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
                None => chrono::Utc::now().into(),
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

        /// Root provider allows to globally mock time
        pub fn root_provider() -> Self {
            Self(TimeProviderAgent::root_provider())
        }

        fn parent_now_or_realtime(&self) -> DateTime {
            self.0
                .lock()
                .expect("Mutex is poisoned")
                .parent_now_or_realtime()
        }

        /// When we call sleep, there is no guarantee when the starting time will be taken.
        /// Hence it is possible this time is taken after we have call the mock_time (this
        /// is typically the case when calling sleep from Python where the actual sleep is
        /// scheduled on a tokio thread while directly returning a fake coroutine).
        /// So the solution is to use a busy loop in the testing code that watch over the
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
            while let Ok(to_sleep) = remaining_time.to_std() {
                let to_sleep = if let Some(speed_factor) =
                    self.0.lock().expect("Mutex is poisoned").get_speed_factor()
                {
                    if speed_factor > 0. {
                        Some(to_sleep.div_f64(speed_factor))
                    } else {
                        None
                    }
                } else {
                    Some(to_sleep)
                };

                // Recompute the time we still have to sleep
                let mut recompute_time_we_have_to_sleep = || {
                    let now = self.now();
                    remaining_time -= now - sleep_started_at;
                    sleep_started_at = now;
                };

                // Small weirdness here: given `config` has just been cloned, it is
                // considered it has not acknowledged the data in the watch. Hence
                // the first call to `config.changed()` will always finish right away.
                // This is okay given the code is designed around the fact config can be
                // fired randomly (we just recompute the remaining time and go back to sleep).
                if let Some(to_sleep) = to_sleep {
                    libparsec_platform_async::select2_biased!(
                        _ = libparsec_platform_async::sleep(to_sleep) => break,
                        _ = config.changed() => {
                            recompute_time_we_have_to_sleep();
                        }
                    );
                } else {
                    // Time is frozen
                    let _ = config.changed().await;
                    recompute_time_we_have_to_sleep();
                }
            }
        }

        pub fn new_child(&self) -> Self {
            Self(Arc::new(Mutex::new(TimeProviderAgent::new(Some(
                self.0.clone(),
            )))))
        }

        // Following methods are only implemented for testing purpose

        fn mock_time(&self, time: MockedTime) {
            self.0.lock().expect("Mutex is poisoned").mock_time(time);
        }

        /// Switch back to real time
        pub fn unmock_time(&self) {
            self.mock_time(MockedTime::RealTime);
        }

        pub fn mock_time_frozen(&self, time: DateTime) {
            self.mock_time(MockedTime::FrozenTime(time));
        }

        pub fn mock_time_shifted(&self, microseconds: i64) {
            self.mock_time(MockedTime::ShiftedTime { microseconds });
        }

        pub fn mock_time_faster(&self, speed_factor: f64) {
            let reference = self.parent_now_or_realtime();
            let microseconds = (self.now() - reference)
                .num_microseconds()
                .expect("No reason to overflow");
            self.mock_time(MockedTime::FasterTime {
                reference,
                microseconds,
                speed_factor,
            });
        }
    }
}

pub use time_provider::TimeProvider;

#[cfg(test)]
#[path = "../tests/unit/time.rs"]
mod tests;
