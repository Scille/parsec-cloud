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
    pub use rmpv::ValueRef;

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

    #[derive(Debug)]
    pub enum DeserializeError {
        Decode(rmpv::decode::Error),
        InvalidType {
            expected: &'static str,
            got: &'static str,
        },
        InvalidValue(String),
        MissingField(&'static str),
    }

    pub trait Deserialize: Sized {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError>;
    }

    pub fn value_kind(value: &ValueRef<'_>) -> &'static str {
        match value {
            ValueRef::Nil => "nil",
            ValueRef::Boolean(_) => "boolean",
            ValueRef::Integer(_) => "integer",
            ValueRef::F32(_) => "f32",
            ValueRef::F64(_) => "f64",
            ValueRef::String(_) => "string",
            ValueRef::Binary(_) => "binary",
            ValueRef::Array(_) => "array",
            ValueRef::Map(_) => "map",
            ValueRef::Ext(_, _) => "ext",
        }
    }

    pub fn from_slice<T>(buf: &[u8]) -> Result<T, DeserializeError>
    where
        T: Deserialize,
    {
        let mut rd = buf;
        let value =
            rmpv::decode::value_ref::read_value_ref(&mut rd).map_err(DeserializeError::Decode)?;
        T::deserialize(value)
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

    impl<T: Deserialize> Deserialize for &T {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            let _ = value;
            Err(DeserializeError::InvalidValue(
                "cannot deserialize a borrowed reference".to_owned(),
            ))
        }
    }

    impl Serialize for bool {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_bool(writer, *self)?;
            Ok(())
        }
    }

    impl Deserialize for bool {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Boolean(v) => Ok(v),
                other => Err(DeserializeError::InvalidType {
                    expected: "boolean",
                    got: value_kind(&other),
                }),
            }
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

    macro_rules! impl_signed_deserialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Deserialize for $ty {
                    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
                        match value {
                            ValueRef::Integer(n) => n
                                .as_i64()
                                .ok_or_else(|| DeserializeError::InvalidValue(
                                    "integer out of range for signed type".to_owned(),
                                ))
                                .and_then(|v| {
                                    <$ty>::try_from(v).map_err(|_| {
                                        DeserializeError::InvalidValue(
                                            "integer out of range for signed type".to_owned(),
                                        )
                                    })
                                }),
                            other => Err(DeserializeError::InvalidType {
                                expected: "integer",
                                got: value_kind(&other),
                            }),
                        }
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

    macro_rules! impl_unsigned_deserialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Deserialize for $ty {
                    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
                        match value {
                            ValueRef::Integer(n) => n
                                .as_u64()
                                .ok_or_else(|| DeserializeError::InvalidValue(
                                    "integer out of range for unsigned type".to_owned(),
                                ))
                                .and_then(|v| {
                                    <$ty>::try_from(v).map_err(|_| {
                                        DeserializeError::InvalidValue(
                                            "integer out of range for unsigned type".to_owned(),
                                        )
                                    })
                                }),
                            other => Err(DeserializeError::InvalidType {
                                expected: "integer",
                                got: value_kind(&other),
                            }),
                        }
                    }
                }
            )+
        };
    }

    impl_signed_serialize!(i8, i16, i32, i64);
    impl_signed_deserialize!(i8, i16, i32, i64);
    impl_unsigned_serialize!(u8, u16, u32, u64);
    impl_unsigned_deserialize!(u8, u16, u32, u64);

    impl Serialize for f32 {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_f32(writer, *self)?;
            Ok(())
        }
    }

    impl Deserialize for f32 {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::F32(v) => Ok(v),
                ValueRef::F64(v) => Ok(v as f32),
                ValueRef::Integer(n) => n.as_f64().map(|v| v as f32).ok_or_else(|| {
                    DeserializeError::InvalidValue(
                        "integer cannot be represented as f32".to_owned(),
                    )
                }),
                other => Err(DeserializeError::InvalidType {
                    expected: "number",
                    got: value_kind(&other),
                }),
            }
        }
    }

    impl Serialize for f64 {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_f64(writer, *self)?;
            Ok(())
        }
    }

    impl Deserialize for f64 {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::F32(v) => Ok(v.into()),
                ValueRef::F64(v) => Ok(v),
                ValueRef::Integer(n) => n.as_f64().ok_or_else(|| {
                    DeserializeError::InvalidValue(
                        "integer cannot be represented as f64".to_owned(),
                    )
                }),
                other => Err(DeserializeError::InvalidType {
                    expected: "number",
                    got: value_kind(&other),
                }),
            }
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

    impl Deserialize for String {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::String(s) => s.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                    DeserializeError::InvalidValue("invalid UTF-8 string".to_owned())
                }),
                other => Err(DeserializeError::InvalidType {
                    expected: "string",
                    got: value_kind(&other),
                }),
            }
        }
    }

    impl Serialize for bytes::Bytes {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_bin(writer, self.as_ref())?;
            Ok(())
        }
    }

    impl Deserialize for bytes::Bytes {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Binary(data) => Ok(bytes::Bytes::copy_from_slice(data)),
                other => Err(DeserializeError::InvalidType {
                    expected: "binary",
                    got: value_kind(&other),
                }),
            }
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

    impl<T> Deserialize for Option<T>
    where
        T: Deserialize,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Nil => Ok(None),
                other => Ok(Some(T::deserialize(other)?)),
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

    impl<T> Deserialize for Vec<T>
    where
        T: Deserialize,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Array(values) => values.into_iter().map(T::deserialize).collect(),
                other => Err(DeserializeError::InvalidType {
                    expected: "array",
                    got: value_kind(&other),
                }),
            }
        }
    }

    impl<A, B> Serialize for (A, B)
    where
        A: Serialize,
        B: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            rmp::encode::write_array_len(writer, 2)?;
            self.0.serialize(writer)?;
            self.1.serialize(writer)?;
            Ok(())
        }
    }

    impl<A, B> Deserialize for (A, B)
    where
        A: Deserialize,
        B: Deserialize,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Array(values) => {
                    if values.len() != 2 {
                        return Err(DeserializeError::InvalidValue(
                            "expected tuple of size 2".to_owned(),
                        ));
                    }
                    Ok((
                        A::deserialize(values[0].clone())?,
                        B::deserialize(values[1].clone())?,
                    ))
                }
                other => Err(DeserializeError::InvalidType {
                    expected: "array",
                    got: value_kind(&other),
                }),
            }
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

    impl<T> Deserialize for HashSet<T>
    where
        T: Deserialize + Eq + Hash,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Array(values) => values.into_iter().map(T::deserialize).collect(),
                other => Err(DeserializeError::InvalidType {
                    expected: "array",
                    got: value_kind(&other),
                }),
            }
        }
    }

    impl<K, T> Serialize for HashMap<K, T>
    where
        K: Serialize + Eq + Hash,
        T: Serialize,
    {
        fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
            let len = usize_to_u32("map", self.len())?;
            rmp::encode::write_map_len(writer, len)?;
            for (key, value) in self {
                key.serialize(writer)?;
                value.serialize(writer)?;
            }
            Ok(())
        }
    }

    impl<K, T> Deserialize for HashMap<K, T>
    where
        K: Deserialize + Eq + Hash,
        T: Deserialize,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            match value {
                ValueRef::Map(entries) => {
                    let mut out = HashMap::with_capacity(entries.len());
                    for (raw_key, raw_value) in entries {
                        let key = K::deserialize(raw_key)?;
                        let value = T::deserialize(raw_value)?;
                        out.insert(key, value);
                    }
                    Ok(out)
                }
                other => Err(DeserializeError::InvalidType {
                    expected: "map",
                    got: value_kind(&other),
                }),
            }
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

    impl<T> Deserialize for super::Maybe<T>
    where
        T: Deserialize,
    {
        fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
            Ok(super::Maybe::Present(T::deserialize(value)?))
        }
    }
}

impl rmp_serialize::Serialize for DeviceID {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), rmp_serialize::SerializeError> {
        rmp_serialize::Serialize::serialize(self.0.as_str(), writer)
    }
}

impl rmp_serialize::Deserialize for DeviceID {
    fn deserialize(
        value: rmp_serialize::ValueRef<'_>,
    ) -> Result<Self, rmp_serialize::DeserializeError> {
        Ok(Self(rmp_serialize::Deserialize::deserialize(value)?))
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

    fn api_dump(&self) -> Result<Vec<u8>, rmp_serialize::SerializeError>;

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
