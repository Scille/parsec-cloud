// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

macro_rules! new_uuid_type {
    (pub $name:ident) => {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
        #[serde(transparent)]
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
            fn default() -> Self {
                Self(uuid::Uuid::new_v4())
            }
        }

        impl std::convert::AsRef<uuid::Uuid> for $name {
            #[inline]
            fn as_ref(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::convert::From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl std::convert::From<uuid::Bytes> for $name {
            fn from(bytes: uuid::Bytes) -> Self {
                Self(uuid::Uuid::from_bytes(bytes))
            }
        }

        impl std::str::FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                uuid::Uuid::parse_str(s)
                    .map(Self)
                    .or(Err(concat!("Invalid ", stringify!($name))))
            }
        }
    };
}

pub(crate) use new_uuid_type;

// TODO: move to it own crate ?
/// Serialize/Deserialize DateTime as f64 unix timestamp with nanosecond precision
pub mod ts_with_nanoseconds_as_double {
    use chrono::{DateTime, TimeZone, Utc};
    use core::fmt;
    use serde::{de, ser};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_f64(dt.timestamp() as f64 + (dt.timestamp_subsec_nanos() as f64 / 1e9))
    }

    struct TimestampVisitor;

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_f64(TimestampVisitor)
    }

    impl<'de> de::Visitor<'de> for TimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unix timestamp as 64 bits floating point number")
        }

        /// Deserialize a timestamp in milliseconds since the epoch
        fn visit_f64<E>(self, value: f64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(Utc.timestamp_nanos((value * 1e9) as i64))
        }
    }
}
