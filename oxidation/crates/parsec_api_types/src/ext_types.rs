// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use byteorder::{ByteOrder, NetworkEndian};
use chrono::prelude::*;
use serde::{de, Deserializer, Serializer};
use serde_bytes::ByteBuf;
use serde_with::{DeserializeAs, SerializeAs};

pub(crate) const DATETIME_EXT_ID: i8 = 1;
pub(crate) const UUID_EXT_ID: i8 = 2;

/*
 * DateTime
 */

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DateTimeExtFormat;

impl SerializeAs<DateTime<Utc>> for DateTimeExtFormat {
    fn serialize_as<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = dt.timestamp() as f64 + (dt.timestamp_subsec_nanos() as f64 / 1e9);
        let mut buf = [0u8; 8];
        NetworkEndian::write_f64(&mut buf, timestamp);
        // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
        // rmp_serde this should be treated as an extension type
        serializer.serialize_newtype_struct(
            rmp_serde::MSGPACK_EXT_STRUCT_NAME,
            &(DATETIME_EXT_ID, ByteBuf::from(buf)),
        )
    }
}

impl<'de> DeserializeAs<'de, DateTime<Utc>> for DateTimeExtFormat {
    fn deserialize_as<D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DateTime<Utc>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a datetime as extension 1 format")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
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
                let ts_raw: ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                if ts_raw.len() != 8 {
                    return Err(serde::de::Error::custom("invalid size of data extension"));
                }
                let ts = NetworkEndian::read_f64(&ts_raw);

                if tag == DATETIME_EXT_ID {
                    Ok(Utc.timestamp_nanos((ts * 1e9) as i64))
                } else {
                    let unexp = de::Unexpected::Signed(tag as i64);
                    Err(serde::de::Error::invalid_value(unexp, &self))
                }
            }
        }

        // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
        // rmp_serde this should be treated as an extension type
        deserializer.deserialize_newtype_struct(rmp_serde::MSGPACK_EXT_STRUCT_NAME, Visitor)
    }
}

/*
 * UUID
 */

macro_rules! new_uuid_type {
    (pub $name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
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
            let unexp = de::Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexp, &self))
        }
    }
}
