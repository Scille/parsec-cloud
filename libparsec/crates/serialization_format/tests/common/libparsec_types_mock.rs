// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// We don't want to have this crate depend on `libparsec_types` for it test
// given `libparsec_types` itself uses this crate... hence in case of bug in
// this crate we wouldn't be able to use the tests !
// So here we simulate `libparsec_types` by implementing the bare minimum that
// our macros need.

pub type Integer = i64;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DeviceID(pub String);

pub mod rmp_serialize {
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;

    pub use rmp::encode; // Re-expose for convenience

    #[derive(Debug)]
    pub enum SerializeError {
        Io(std::io::Error),
        LengthOverflow {
            kind: &'static str,
            len: usize,
        },
        /// `Maybe::Absent` only exist to represent a missing field when deserializing
        /// a payload generated with an older version of the schema.
        /// Hence it should never be serialized (besides, skipping a field during
        /// deserialization complexifies structure serialization since it number
        /// of fields must be known before each field is actually serialized).
        MaybeFieldCannotBeAbsent,
    }

    pub trait Serialize {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError>;
    }

    impl From<rmp::encode::ValueWriteError<std::io::Error>> for SerializeError {
        fn from(value: rmp::encode::ValueWriteError<std::io::Error>) -> Self {
            // We don't care where the IO error occurred
            match value {
                rmp::encode::ValueWriteError::InvalidMarkerWrite(err) => Self::Io(err),
                rmp::encode::ValueWriteError::InvalidDataWrite(err) => Self::Io(err),
            }
        }
    }

    impl From<std::io::Error> for SerializeError {
        fn from(value: std::io::Error) -> Self {
            Self::Io(value)
        }
    }

    impl<T: Serialize + ?Sized> Serialize for &T {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            (*self).serialize(writer)
        }
    }

    impl Serialize for bool {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_bool(writer, *self)?;
            Ok(())
        }
    }

    macro_rules! impl_signed_serialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Serialize for $ty {
                    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
                        rmp::encode::write_sint(writer, (*self).into())?;
                        Ok(())
                    }
                }
            )+
        };
    }

    macro_rules! impl_unsigned_serialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Serialize for $ty {
                    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
                        rmp::encode::write_uint(writer, (*self).into())?;
                        Ok(())
                    }
                }
            )+
        };
    }

    impl_signed_serialize!(i8, i16, i32, i64);
    impl_unsigned_serialize!(u8, u16, u32, u64);

    impl Serialize for f32 {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_f32(writer, *self)?;
            Ok(())
        }
    }

    impl Serialize for f64 {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_f64(writer, *self)?;
            Ok(())
        }
    }

    impl Serialize for str {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_str(writer, self)?;
            Ok(())
        }
    }

    impl Serialize for String {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_str(writer, self)?;
            Ok(())
        }
    }

    impl Serialize for bytes::Bytes {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_bin(writer, self.as_ref())?;
            Ok(())
        }
    }

    impl<T> Serialize for Option<T>
    where
        T: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            match self {
                Some(value) => value.serialize(writer),
                None => {
                    rmp::encode::write_nil(writer)?;
                    Ok(())
                }
            }
        }
    }

    fn usize_to_u32(kind: &'static str, len: usize) -> Result<u32, SerializeError> {
        u32::try_from(len).map_err(|_| SerializeError::LengthOverflow { kind, len })
    }

    impl<T> Serialize for Vec<T>
    where
        T: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            let len = usize_to_u32("array", self.len())?;
            rmp::encode::write_array_len(writer, len)?;
            for item in self {
                item.serialize(writer)?;
            }
            Ok(())
        }
    }

    impl<T> Serialize for HashSet<T>
    where
        T: Serialize + Eq + Hash,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            let len = usize_to_u32("array", self.len())?;
            rmp::encode::write_array_len(writer, len)?;
            for item in self {
                item.serialize(writer)?;
            }
            Ok(())
        }
    }

    impl<T> Serialize for HashMap<String, T>
    where
        T: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            let len = usize_to_u32("map", self.len())?;
            rmp::encode::write_map_len(writer, len)?;
            for (key, value) in self {
                rmp::encode::write_str(writer, key)?;
                value.serialize(writer)?;
            }
            Ok(())
        }
    }

    impl<T> Serialize for super::Maybe<T>
    where
        T: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            match self {
                super::Maybe::Present(value) => value.serialize(writer),
                super::Maybe::Absent => Err(SerializeError::MaybeFieldCannotBeAbsent),
            }
        }
    }
}

impl rmp_serialize::Serialize for DeviceID {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), rmp_serialize::SerializeError> {
        rmp_serialize::Serialize::serialize(self.0.as_str(), writer)
    }
}

#[allow(unused)]
pub enum ProtocolFamily {
    Family,
}

#[expect(dead_code)]
pub trait ProtocolRequest<const V: u32> {
    const FAMILY: ProtocolFamily;
    type Response: for<'de> Deserialize<'de>;

    fn api_dump(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, rmp_serde::decode::Error>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[expect(dead_code)]
pub struct ApiVersion {
    pub version: u32,
    pub revision: u32,
}

// Copy/paste of Maybe field... cannot go around that :(

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

use serde::Deserialize;
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
