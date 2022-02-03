// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

// use byteorder::{ByteOrder, NetworkEndian};
// use chrono::prelude::*;
// use serde::{de, Deserializer, Serializer};
// use serde_bytes::ByteBuf;
// use serde_with::{DeserializeAs, SerializeAs};

// macro_rules! new_uuid_type {
//     (pub $name:ident) => {
//         #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
//         #[serde(transparent)]
//         pub struct $name(uuid::Uuid);

//         impl $name {
//             pub fn as_bytes(&self) -> &uuid::Bytes {
//                 self.0.as_bytes()
//             }

//             pub fn as_hyphenated(&self) -> String {
//                 self.0.as_hyphenated().to_string()
//             }
//         }

//         impl std::fmt::Display for $name {
//             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 write!(f, "{}", self.0.as_simple())
//             }
//         }

//         impl Default for $name {
//             fn default() -> Self {
//                 Self(uuid::Uuid::new_v4())
//             }
//         }

//         impl std::ops::Deref for $name {
//             type Target = uuid::Uuid;

//             fn deref(&self) -> &Self::Target {
//                 &self.0
//             }
//         }

//         impl std::convert::AsRef<uuid::Uuid> for $name {
//             #[inline]
//             fn as_ref(&self) -> &uuid::Uuid {
//                 &self.0
//             }
//         }

//         impl std::convert::From<uuid::Uuid> for $name {
//             fn from(id: uuid::Uuid) -> Self {
//                 Self(id)
//             }
//         }

//         impl std::convert::From<uuid::Bytes> for $name {
//             fn from(bytes: uuid::Bytes) -> Self {
//                 Self(uuid::Uuid::from_bytes(bytes))
//             }
//         }

//         impl std::str::FromStr for $name {
//             type Err = &'static str;

//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 uuid::Uuid::parse_str(s)
//                     .map(Self)
//                     .or(Err(concat!("Invalid ", stringify!($name))))
//             }
//         }
//     };
// }

// pub(crate) use new_uuid_type;

// const DATETIME_EXT_ID: i8 = 1;
// const UUID_EXT_ID: i8 = 2;

// #[derive(Debug, PartialEq)]
// pub struct UuidExtFormat((i8, ByteBuf));

// impl SerializeAs<uuid::Uuid> for UuidExtFormat {
//     fn serialize_as<S>(id: &uuid::Uuid, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
//         // rmp_serde this should be treated as an extension type
//         serializer.serialize_newtype_struct(
//             rmp_serde::MSGPACK_EXT_STRUCT_NAME,
//             &(UUID_EXT_ID, id.as_ref()),
//         )
//     }
// }

// impl<'de> DeserializeAs<'de, uuid::Uuid> for UuidExtFormat {
//     fn deserialize_as<D>(deserializer: D) -> Result<uuid::Uuid, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct Visitor;

//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = uuid::Uuid;

//             fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
//                 formatter.write_str("a sequence of tag & bytes")
//             }

//             fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 deserializer.deserialize_tuple(2, self)
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::SeqAccess<'de>,
//             {
//                 let tag: i8 = seq
//                     .next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
//                 let data: ByteBuf = seq
//                     .next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
//                 if tag == UUID_EXT_ID {
//                     uuid::Uuid::from_slice(&data)
//                         .map_err(|_| serde::de::Error::custom("invalid size of data extension"))
//                 } else {
//                     let unexp = de::Unexpected::Signed(tag as i64);
//                     Err(serde::de::Error::invalid_value(unexp, &self))
//                 }
//             }
//         }

//         // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
//         // rmp_serde this should be treated as an extension type
//         deserializer.deserialize_newtype_struct(rmp_serde::MSGPACK_EXT_STRUCT_NAME, Visitor)
//     }
// }

// #[derive(Debug, PartialEq)]
// pub struct DateTimeExtFormat((i8, f64));

// impl SerializeAs<DateTime<Utc>> for DateTimeExtFormat {
//     fn serialize_as<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let timestamp = dt.timestamp() as f64 + (dt.timestamp_subsec_nanos() as f64 / 1e9);
//         // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
//         // rmp_serde this should be treated as an extension type
//         serializer.serialize_newtype_struct(
//             rmp_serde::MSGPACK_EXT_STRUCT_NAME,
//             &(DATETIME_EXT_ID, timestamp),
//         )
//     }
// }

// impl<'de> DeserializeAs<'de, DateTime<Utc>> for DateTimeExtFormat {
//     fn deserialize_as<D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct Visitor;

//         impl<'de> de::Visitor<'de> for Visitor {
//             type Value = DateTime<Utc>;

//             fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
//                 formatter.write_str("a sequence of tag & double")
//             }

//             fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//             where
//                 D: Deserializer<'de>,
//             {
//                 deserializer.deserialize_tuple(2, self)
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::SeqAccess<'de>,
//             {
//                 let tag: i8 = seq
//                     .next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
//                 let ts_raw: ByteBuf = seq
//                     .next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
//                 if ts_raw.len() != 8 {
//                     return Err(serde::de::Error::custom("invalid size of data extension"));
//                 }
//                 let ts = NetworkEndian::read_f64(&ts_raw);

//                 if tag == DATETIME_EXT_ID {
//                     Ok(Utc.timestamp_nanos((ts * 1e9) as i64))
//                 } else {
//                     let unexp = de::Unexpected::Signed(tag as i64);
//                     Err(serde::de::Error::invalid_value(unexp, &self))
//                 }
//             }
//         }

//         // `rmp_serde::MSGPACK_EXT_STRUCT_NAME` is a magic value to tell
//         // rmp_serde this should be treated as an extension type
//         deserializer.deserialize_newtype_struct(rmp_serde::MSGPACK_EXT_STRUCT_NAME, Visitor)
//     }
// }
