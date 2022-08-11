// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use chrono::{Datelike, Duration, TimeZone, Timelike};
use core::ops::Sub;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::ops::Add;

pub(crate) const DATETIME_EXT_ID: i8 = 1;
pub(crate) const UUID_EXT_ID: i8 = 2;
use crate::VlobID;

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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
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

    #[cfg(not(feature = "mock-time"))]
    #[inline]
    pub fn now() -> Self {
        let now = chrono::Utc::now();
        now.into()
    }
}

pub enum MockedTime {
    RealTime,
    FrozenTime(DateTime),
    ShiftedTime(i64),
}

#[cfg(feature = "mock-time")]
mod mock_time {
    use super::{DateTime, MockedTime};
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<MockedTime> = RefCell::new(MockedTime::RealTime);
    }

    impl DateTime {
        pub fn now() -> Self {
            MOCK_TIME.with(|cell| match *cell.borrow() {
                MockedTime::RealTime => chrono::Utc::now().into(),
                MockedTime::FrozenTime(dt) => dt,
                MockedTime::ShiftedTime(dt) => DateTime::from(chrono::Utc::now()) + dt,
            })
        }

        pub fn mock_time(time: MockedTime) {
            MOCK_TIME.with(|cell| *cell.borrow_mut() = time)
        }
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
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
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

impl serde::Serialize for DateTime {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let buf = self.get_f64_with_us_precision().to_be_bytes();
        // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
        // rmp_serde this should be treated as an extension type
        serializer.serialize_newtype_struct(
            rmp_serde::MSGPACK_EXT_STRUCT_NAME,
            &(DATETIME_EXT_ID, ByteBuf::from(buf)),
        )
    }
}

impl<'de> serde::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
        // rmp_serde this should be treated as an extension type
        deserializer
            .deserialize_newtype_struct(rmp_serde::MSGPACK_EXT_STRUCT_NAME, DateTimeExtVisitor)
    }
}

impl Sub for DateTime {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<i64> for DateTime {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        Self(self.0 + Duration::microseconds(rhs))
    }
}

impl Sub<i64> for DateTime {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        Self(self.0 - Duration::microseconds(rhs))
    }
}

pub(crate) struct DateTimeExtVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeExtVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a datetime as extension 1 format")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let tag: i8 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let data: ByteBuf = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        if tag != DATETIME_EXT_ID {
            let unexp = serde::de::Unexpected::Signed(tag as i64);
            return Err(serde::de::Error::invalid_value(unexp, &self));
        }

        const F64_SIZE: usize = std::mem::size_of::<f64>();

        if data.len() != F64_SIZE {
            return Err(serde::de::Error::custom(
                "invalid data for datetime extension",
            ));
        }

        let ts = f64::from_be_bytes(
            data[..F64_SIZE]
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
        );

        Ok(Self::Value::from_f64_with_us_precision(ts))
    }
}

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
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_datetime_deserialize_has_ms_precision() {
        let serialized = &hex!("d70141d86ad584cd5d4f")[..];
        let expected_timestamp_nanos = 1638618643208820000;

        let dt: DateTime = rmp_serde::from_slice(serialized).unwrap();
        assert_eq!(dt.as_ref().timestamp_nanos(), expected_timestamp_nanos);

        // Round trip
        let serialized2 = rmp_serde::to_vec_named(&dt).unwrap();
        assert_eq!(serialized2, serialized);
    }

    #[test]
    fn test_datetime_parse_has_ms_precision() {
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
    fn test_datetime_now_has_ms_precision() {
        // The inner loop is much faster than a us, so if we run it multiple
        // times we are sure we won't by hazard end up with a now datetime
        // with nanoseconds last 3 digits at 0.
        for _ in 0..10 {
            let dt = DateTime::now();
            let ns = dt.0.nanosecond();
            assert_eq!(ns % 1000, 0);
        }
    }
}

/*
 * UUID
 */

macro_rules! new_uuid_type {
    (pub $name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn as_bytes(&self) -> &uuid::Bytes {
                self.0.as_bytes()
            }

            pub fn as_hyphenated(&self) -> String {
                self.0.as_hyphenated().to_string()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0.as_simple())
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self(uuid::Uuid::new_v4())
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::convert::AsRef<uuid::Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::convert::From<uuid::Uuid> for $name {
            #[inline]
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl std::convert::From<uuid::Bytes> for $name {
            #[inline]
            fn from(bytes: uuid::Bytes) -> Self {
                Self(uuid::Uuid::from_bytes(bytes))
            }
        }

        impl std::str::FromStr for $name {
            type Err = &'static str;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                uuid::Uuid::parse_str(s)
                    .map(Self)
                    .or(Err(concat!("Invalid ", stringify!($name))))
            }
        }

        impl serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
                // rmp_serde this should be treated as an extension type
                serializer.serialize_newtype_struct(
                    rmp_serde::MSGPACK_EXT_STRUCT_NAME,
                    &(
                        $crate::ext_types::UUID_EXT_ID,
                        serde_bytes::Bytes::new(self.as_bytes()),
                    ),
                )
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
                // rmp_serde this should be treated as an extension type
                deserializer
                    .deserialize_newtype_struct(
                        rmp_serde::MSGPACK_EXT_STRUCT_NAME,
                        $crate::ext_types::UuidExtVisitor,
                    )
                    .map($name)
            }
        }
    };
}

pub(crate) use new_uuid_type;

pub(crate) struct UuidExtVisitor;

impl<'de> serde::de::Visitor<'de> for UuidExtVisitor {
    type Value = uuid::Uuid;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an UUID as extension 2 format")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let tag: i8 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let data: ByteBuf = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        if tag == UUID_EXT_ID {
            uuid::Uuid::from_slice(&data)
                .map_err(|_| serde::de::Error::custom("invalid size of data extension"))
        } else {
            let unexp = serde::de::Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexp, &self))
        }
    }
}

/*
 * Optional field helper (used for backward compatibility)
 */

pub mod maybe_field {
    use serde::{Deserialize, Deserializer};

    /// Any value that is present is considered Some value, including null.
    pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Some)
    }

    // serialize is not needed given we never omit fields
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Maybe<T> {
    Present(T),
    Absent,
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Self::Absent
    }
}

impl<T> Maybe<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Present(data) => data,
            Self::Absent => default,
        }
    }
}

impl<T> From<T> for Maybe<T> {
    fn from(data: T) -> Self {
        Self::Present(data)
    }
}

impl<T, U> serde_with::SerializeAs<Maybe<T>> for Maybe<U>
where
    U: serde_with::SerializeAs<T>,
{
    fn serialize_as<S>(source: &Maybe<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *source {
            Maybe::Present(ref value) => {
                serializer.serialize_some(&serde_with::ser::SerializeAsWrap::<T, U>::new(value))
            }
            Maybe::Absent => serializer.serialize_none(),
        }
    }
}

impl<'de, T, U> serde_with::DeserializeAs<'de, Maybe<T>> for Maybe<U>
where
    U: serde_with::DeserializeAs<'de, T>,
    T: Default,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Maybe<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MaybeVisitor<T, U>(std::marker::PhantomData<(T, U)>);

        impl<'de, T, U> serde::de::Visitor<'de> for MaybeVisitor<T, U>
        where
            U: serde_with::DeserializeAs<'de, T>,
            T: Default,
        {
            type Value = Maybe<T>;

            fn expecting(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                unreachable!()
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Maybe::Present(T::default()))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Maybe::Present(T::default()))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                U::deserialize_as(deserializer).map(Maybe::Present)
            }
        }

        deserializer.deserialize_option(MaybeVisitor::<T, U>(std::marker::PhantomData))
    }
}

macro_rules! impl_from_maybe {
    ($name: ty) => {
        impl From<crate::Maybe<$name>> for $name {
            fn from(data: crate::Maybe<$name>) -> Self {
                match data {
                    crate::Maybe::Present(data) => data,
                    crate::Maybe::Absent => Default::default(),
                }
            }
        }
    };
}

impl_from_maybe!(bool);

pub(crate) use impl_from_maybe;

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReencryptionBatchEntry {
    pub vlob_id: VlobID,
    pub version: u64,
    #[serde_as(as = "serde_with::Bytes")]
    pub blob: Vec<u8>,
}
