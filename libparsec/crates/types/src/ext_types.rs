// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::{DateTime, DatetimeFromTimestampMicrosError};

// Note msgpack already define a extension type for timestamp
// (see https://github.com/msgpack/msgpack/blob/master/spec.md#timestamp-extension-type)
// However is not a good fit for our use case:
// - it is more complex than needed with 3 different ways to encode the timestamp.
// - it precision is in nanosecond, we only need microsecond (this is important since
//   PostgreSQL's TIMESTAMPZ type has a microsecond precision, and losing precision on
//   round trip would be very error prone).
pub(crate) const DATETIME_EXT_ID: i8 = 1;
pub(crate) const UUID_EXT_ID: i8 = 2;

/*
 * DateTime
 */

impl serde::Serialize for DateTime {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let buf = self.as_timestamp_micros().to_be_bytes();
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
            let unexpected = serde::de::Unexpected::Signed(tag as i64);
            return Err(serde::de::Error::invalid_value(unexpected, &self));
        }

        const I64_SIZE: usize = std::mem::size_of::<i64>();

        if data.len() != I64_SIZE {
            return Err(serde::de::Error::custom(
                "invalid data for datetime extension",
            ));
        }

        let ts = i64::from_be_bytes(
            data[..I64_SIZE]
                .try_into()
                .expect("data.len() should be equal to I64_SIZE"),
        );

        Self::Value::from_timestamp_micros(ts).map_err(|err| match err {
            DatetimeFromTimestampMicrosError::OutOfRange => {
                serde::de::Error::custom("out-of-range datetime")
            }
        })
    }
}

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
            let unexpected = serde::de::Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexpected, &self))
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
        self.unwrap_or_else(|| default)
    }
    pub fn unwrap_or_else(self, default: impl FnOnce() -> T) -> T {
        match self {
            Self::Present(data) => data,
            Self::Absent => default(),
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
{
    fn deserialize_as<D>(deserializer: D) -> Result<Maybe<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Maybe::Present(
            serde_with::de::DeserializeAsWrap::<T, U>::deserialize(deserializer)?.into_inner(),
        ))
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
