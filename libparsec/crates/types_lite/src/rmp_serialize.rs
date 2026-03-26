// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "decode error: {err}"),
            Self::InvalidType { expected, got } => {
                write!(f, "invalid type: expected {expected}, got {got}")
            }
            Self::InvalidValue(msg) => write!(f, "invalid value: {msg}"),
            Self::MissingField(name) => write!(f, "missing field `{name}`"),
        }
    }
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
                DeserializeError::InvalidValue("integer cannot be represented as f32".to_owned())
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
                DeserializeError::InvalidValue("integer cannot be represented as f64".to_owned())
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
            ValueRef::String(s) => s
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| DeserializeError::InvalidValue("invalid UTF-8 string".to_owned())),
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
            // Also accept msgpack strings since some implementations encode
            // byte payloads as strings (e.g. Python's msgpack)
            ValueRef::String(s) => Ok(bytes::Bytes::copy_from_slice(s.as_bytes())),
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

// NonZero types

impl Serialize for std::num::NonZeroU8 {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        self.get().serialize(writer)
    }
}

impl Deserialize for std::num::NonZeroU8 {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let v = u8::deserialize(value)?;
        std::num::NonZeroU8::new(v)
            .ok_or_else(|| DeserializeError::InvalidValue("expected non-zero u8".to_owned()))
    }
}

impl Serialize for std::num::NonZeroU64 {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        self.get().serialize(writer)
    }
}

impl Deserialize for std::num::NonZeroU64 {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let v = u64::deserialize(value)?;
        std::num::NonZeroU64::new(v)
            .ok_or_else(|| DeserializeError::InvalidValue("expected non-zero u64".to_owned()))
    }
}

// DateTime (ext type 1, i64 big-endian microseconds)

impl Serialize for super::DateTime {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let buf = self.as_timestamp_micros().to_be_bytes();
        encode::write_ext_meta(writer, buf.len() as u32, super::ext_types::DATETIME_EXT_ID)?;
        writer.extend_from_slice(&buf);
        Ok(())
    }
}

impl Deserialize for super::DateTime {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Ext(tag, data) if tag == super::ext_types::DATETIME_EXT_ID => {
                let bytes: [u8; 8] = data.try_into().map_err(|_| {
                    DeserializeError::InvalidValue("invalid datetime ext data size".to_owned())
                })?;
                let ts = i64::from_be_bytes(bytes);
                super::DateTime::from_timestamp_micros(ts).map_err(|_| {
                    DeserializeError::InvalidValue("out-of-range datetime".to_owned())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "ext(datetime)",
                got: value_kind(&other),
            }),
        }
    }
}

// AccessToken (binary, 16 bytes)

impl Serialize for super::AccessToken {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_bin(writer, self.as_ref())?;
        Ok(())
    }
}

impl Deserialize for super::AccessToken {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Binary(data) => {
                super::AccessToken::try_from(data).map_err(|e| {
                    DeserializeError::InvalidValue(e.to_string())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "binary",
                got: value_kind(&other),
            }),
        }
    }
}

// EntryName (string)

impl Serialize for super::EntryName {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        Serialize::serialize(self.as_ref(), writer)
    }
}

impl Deserialize for super::EntryName {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        s.as_str().try_into().map_err(|e: super::EntryNameError| {
            DeserializeError::InvalidValue(e.to_string())
        })
    }
}

// HumanHandle (serialized as tuple (email, label))

impl Serialize for super::HumanHandle {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_array_len(writer, 2)?;
        Serialize::serialize(&self.email().to_string(), writer)?;
        Serialize::serialize(self.label(), writer)?;
        Ok(())
    }
}

impl Deserialize for super::HumanHandle {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Array(values) if values.len() == 2 => {
                let email: String = Deserialize::deserialize(values[0].clone())?;
                let label: String = Deserialize::deserialize(values[1].clone())?;
                super::HumanHandle::from_raw(&email, &label).map_err(|e| {
                    DeserializeError::InvalidValue(e.to_string())
                })
            }
            ValueRef::Array(_) => Err(DeserializeError::InvalidValue(
                "expected tuple of size 2 for HumanHandle".to_owned(),
            )),
            other => Err(DeserializeError::InvalidType {
                expected: "array",
                got: value_kind(&other),
            }),
        }
    }
}

// ParsecAddr (string, HTTP URL)

impl Serialize for super::ParsecAddr {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let url = self.to_http_url(None);
        Serialize::serialize(url.as_str(), writer)
    }
}

impl Deserialize for super::ParsecAddr {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        super::ParsecAddr::from_http_url(&s).map_err(|e| {
            DeserializeError::InvalidValue(e.to_string())
        })
    }
}

// PKIEncryptionAlgorithm (string via Display/FromStr)

impl Serialize for super::PKIEncryptionAlgorithm {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let s: &str = (*self).into();
        Serialize::serialize(s, writer)
    }
}

impl Deserialize for super::PKIEncryptionAlgorithm {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        s.parse().map_err(|e: &str| {
            DeserializeError::InvalidValue(e.to_owned())
        })
    }
}

// X509CertificateHash (string via Display/FromStr)

impl Serialize for super::X509CertificateHash {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        Serialize::serialize(&self.to_string(), writer)
    }
}

impl Deserialize for super::X509CertificateHash {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        s.parse().map_err(|e: super::FormatError| {
            DeserializeError::InvalidValue(e.to_string())
        })
    }
}

// X509WindowsCngURI (map with issuer and serial_number as bytes)

impl Serialize for super::X509WindowsCngURI {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_map_len(writer, 2)?;
        Serialize::serialize("issuer", writer)?;
        rmp::encode::write_bin(writer, &self.issuer)?;
        Serialize::serialize("serial_number", writer)?;
        rmp::encode::write_bin(writer, &self.serial_number)?;
        Ok(())
    }
}

impl Deserialize for super::X509WindowsCngURI {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(entries) => {
                let mut obj = HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key: String = Deserialize::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }
                let issuer = obj
                    .remove("issuer")
                    .ok_or(DeserializeError::MissingField("issuer"))?;
                let issuer = match issuer {
                    ValueRef::Binary(data) => data.to_vec(),
                    other => return Err(DeserializeError::InvalidType { expected: "binary", got: value_kind(&other) }),
                };
                let serial_number = obj
                    .remove("serial_number")
                    .ok_or(DeserializeError::MissingField("serial_number"))?;
                let serial_number = match serial_number {
                    ValueRef::Binary(data) => data.to_vec(),
                    other => return Err(DeserializeError::InvalidType { expected: "binary", got: value_kind(&other) }),
                };
                Ok(super::X509WindowsCngURI { issuer, serial_number })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}

// X509Pkcs11URI (unit struct, serialized as empty map)

impl Serialize for super::X509Pkcs11URI {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_map_len(writer, 0)?;
        Ok(())
    }
}

impl Deserialize for super::X509Pkcs11URI {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(_) => Ok(super::X509Pkcs11URI),
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}

// X509URIFlavorValue (tagged enum: windowscng or pkcs11)

impl Serialize for super::X509URIFlavorValue {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        match self {
            super::X509URIFlavorValue::WindowsCNG(uri) => {
                rmp::encode::write_map_len(writer, 1)?;
                Serialize::serialize("windowscng", writer)?;
                uri.serialize(writer)
            }
            super::X509URIFlavorValue::PKCS11(uri) => {
                rmp::encode::write_map_len(writer, 1)?;
                Serialize::serialize("pkcs11", writer)?;
                uri.serialize(writer)
            }
        }
    }
}

impl Deserialize for super::X509URIFlavorValue {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(entries) => {
                for (raw_key, raw_value) in entries {
                    let key: String = Deserialize::deserialize(raw_key)?;
                    match key.as_str() {
                        "windowscng" => return Ok(super::X509URIFlavorValue::WindowsCNG(
                            Deserialize::deserialize(raw_value)?,
                        )),
                        "pkcs11" => return Ok(super::X509URIFlavorValue::PKCS11(
                            Deserialize::deserialize(raw_value)?,
                        )),
                        _ => continue,
                    }
                }
                Err(DeserializeError::InvalidValue(
                    "unknown X509URIFlavorValue variant".to_owned(),
                ))
            }
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}

// X509CertificateReference (map with uris and hash)

impl Serialize for super::X509CertificateReference {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_map_len(writer, 2)?;
        Serialize::serialize("hash", writer)?;
        self.hash.serialize(writer)?;
        Serialize::serialize("uris", writer)?;
        let uris: Vec<&super::X509URIFlavorValue> = self.uris().collect();
        Serialize::serialize(&uris, writer)?;
        Ok(())
    }
}

impl Deserialize for super::X509CertificateReference {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(entries) => {
                let mut obj = HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key: String = Deserialize::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }
                let hash = obj
                    .remove("hash")
                    .ok_or(DeserializeError::MissingField("hash"))?;
                let hash: super::X509CertificateHash = Deserialize::deserialize(hash)?;
                let uris = match obj.remove("uris") {
                    Some(v) => {
                        // Like serde_with::VecSkipError, skip URIs that fail to deserialize
                        match v {
                            ValueRef::Array(values) => {
                                values.into_iter()
                                    .filter_map(|v| super::X509URIFlavorValue::deserialize(v).ok())
                                    .collect()
                            }
                            _ => Vec::new(),
                        }
                    }
                    None => Vec::new(),
                };
                let mut result = super::X509CertificateReference::from(hash);
                for uri in uris {
                    result = result.add_or_replace_uri_wrapped(uri);
                }
                Ok(result)
            }
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}

// Crypto types (re-exported from libparsec_crypto via `pub use libparsec_crypto::*;`)

// Helper macro for types that serialize as bytes with AsRef<[u8]> + TryFrom<&[u8]>
macro_rules! impl_bytes_serialize {
    ($ty:path) => {
        impl Serialize for $ty {
            fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
                rmp::encode::write_bin(writer, self.as_ref())?;
                Ok(())
            }
        }

        impl Deserialize for $ty {
            fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
                match value {
                    ValueRef::Binary(data) => {
                        Self::try_from(data).map_err(|_| {
                            DeserializeError::InvalidValue(
                                concat!("invalid ", stringify!($ty)).to_owned(),
                            )
                        })
                    }
                    other => Err(DeserializeError::InvalidType {
                        expected: "binary",
                        got: value_kind(&other),
                    }),
                }
            }
        }
    };
}

impl_bytes_serialize!(super::PublicKey);
impl_bytes_serialize!(super::SecretKey);
impl_bytes_serialize!(super::VerifyKey);
impl_bytes_serialize!(super::KeyDerivation);
impl_bytes_serialize!(super::HashDigest);

// PrivateKey: uses to_bytes() instead of AsRef<[u8]>

impl Serialize for super::PrivateKey {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_bin(writer, self.to_bytes().as_ref())?;
        Ok(())
    }
}

impl Deserialize for super::PrivateKey {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Binary(data) => {
                Self::try_from(data).map_err(|_| {
                    DeserializeError::InvalidValue("invalid PrivateKey".to_owned())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "binary",
                got: value_kind(&other),
            }),
        }
    }
}

// SigningKey: uses to_bytes() instead of AsRef<[u8]>

impl Serialize for super::SigningKey {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_bin(writer, self.to_bytes().as_ref())?;
        Ok(())
    }
}

impl Deserialize for super::SigningKey {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Binary(data) => {
                Self::try_from(data).map_err(|_| {
                    DeserializeError::InvalidValue("invalid SigningKey".to_owned())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "binary",
                got: value_kind(&other),
            }),
        }
    }
}

// SequesterPublicKeyDer: uses dump() for serialize, TryFrom<&[u8]> for deserialize

impl Serialize for super::SequesterPublicKeyDer {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_bin(writer, &self.dump())?;
        Ok(())
    }
}

impl Deserialize for super::SequesterPublicKeyDer {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Binary(data) => {
                Self::try_from(data).map_err(|_| {
                    DeserializeError::InvalidValue("invalid SequesterPublicKeyDer".to_owned())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "binary",
                got: value_kind(&other),
            }),
        }
    }
}

// SequesterVerifyKeyDer: uses dump() for serialize, TryFrom<&[u8]> for deserialize

impl Serialize for super::SequesterVerifyKeyDer {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_bin(writer, &self.dump())?;
        Ok(())
    }
}

impl Deserialize for super::SequesterVerifyKeyDer {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Binary(data) => {
                Self::try_from(data).map_err(|_| {
                    DeserializeError::InvalidValue("invalid SequesterVerifyKeyDer".to_owned())
                })
            }
            other => Err(DeserializeError::InvalidType {
                expected: "binary",
                got: value_kind(&other),
            }),
        }
    }
}

// TrustedPasswordAlgorithm (tagged enum with "type" discriminant)

impl Serialize for super::TrustedPasswordAlgorithm {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        match self {
            super::TrustedPasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
                salt,
            } => {
                encode::write_map_len(writer, 5)?;
                Serialize::serialize("memlimit_kb", writer)?;
                Serialize::serialize(memlimit_kb, writer)?;
                Serialize::serialize("opslimit", writer)?;
                Serialize::serialize(opslimit, writer)?;
                Serialize::serialize("parallelism", writer)?;
                Serialize::serialize(parallelism, writer)?;
                Serialize::serialize("salt", writer)?;
                rmp::encode::write_bin(writer, salt.as_ref())?;
                Serialize::serialize("type", writer)?;
                Serialize::serialize("ARGON2ID", writer)?;
                Ok(())
            }
        }
    }
}

impl Deserialize for super::TrustedPasswordAlgorithm {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(entries) => {
                let mut obj = HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key: String = Deserialize::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }
                let ty = obj
                    .remove("type")
                    .ok_or(DeserializeError::MissingField("type"))?;
                let ty: String = Deserialize::deserialize(ty)?;
                match ty.as_str() {
                    "ARGON2ID" => {
                        let memlimit_kb = obj
                            .remove("memlimit_kb")
                            .ok_or(DeserializeError::MissingField("memlimit_kb"))?;
                        let memlimit_kb: u32 = Deserialize::deserialize(memlimit_kb)?;
                        let opslimit = obj
                            .remove("opslimit")
                            .ok_or(DeserializeError::MissingField("opslimit"))?;
                        let opslimit: u32 = Deserialize::deserialize(opslimit)?;
                        let parallelism = obj
                            .remove("parallelism")
                            .ok_or(DeserializeError::MissingField("parallelism"))?;
                        let parallelism: u32 = Deserialize::deserialize(parallelism)?;
                        let salt = obj
                            .remove("salt")
                            .ok_or(DeserializeError::MissingField("salt"))?;
                        let salt = match salt {
                            ValueRef::Binary(data) => {
                                <[u8; super::ARGON2ID_SALTBYTES]>::try_from(data).map_err(|_| {
                                    DeserializeError::InvalidValue(
                                        "invalid salt size".to_owned(),
                                    )
                                })?
                            }
                            other => {
                                return Err(DeserializeError::InvalidType {
                                    expected: "binary",
                                    got: value_kind(&other),
                                })
                            }
                        };
                        Ok(super::TrustedPasswordAlgorithm::Argon2id {
                            memlimit_kb,
                            opslimit,
                            parallelism,
                            salt,
                        })
                    }
                    _ => Err(DeserializeError::InvalidValue(format!(
                        "unknown TrustedPasswordAlgorithm type: {ty}"
                    ))),
                }
            }
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}

// 3-tuple

impl<A, B, C> Serialize for (A, B, C)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_array_len(writer, 3)?;
        self.0.serialize(writer)?;
        self.1.serialize(writer)?;
        self.2.serialize(writer)?;
        Ok(())
    }
}

impl<A, B, C> Deserialize for (A, B, C)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
{
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Array(values) => {
                if values.len() != 3 {
                    return Err(DeserializeError::InvalidValue(
                        "expected tuple of size 3".to_owned(),
                    ));
                }
                Ok((
                    A::deserialize(values[0].clone())?,
                    B::deserialize(values[1].clone())?,
                    C::deserialize(values[2].clone())?,
                ))
            }
            other => Err(DeserializeError::InvalidType {
                expected: "array",
                got: value_kind(&other),
            }),
        }
    }
}

// 6-tuple

impl<A, B, C, D, E, F> Serialize for (A, B, C, D, E, F)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
{
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        rmp::encode::write_array_len(writer, 6)?;
        self.0.serialize(writer)?;
        self.1.serialize(writer)?;
        self.2.serialize(writer)?;
        self.3.serialize(writer)?;
        self.4.serialize(writer)?;
        self.5.serialize(writer)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F> Deserialize for (A, B, C, D, E, F)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
{
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Array(values) => {
                if values.len() != 6 {
                    return Err(DeserializeError::InvalidValue(
                        "expected tuple of size 6".to_owned(),
                    ));
                }
                Ok((
                    A::deserialize(values[0].clone())?,
                    B::deserialize(values[1].clone())?,
                    C::deserialize(values[2].clone())?,
                    D::deserialize(values[3].clone())?,
                    E::deserialize(values[4].clone())?,
                    F::deserialize(values[5].clone())?,
                ))
            }
            other => Err(DeserializeError::InvalidType {
                expected: "array",
                got: value_kind(&other),
            }),
        }
    }
}

// ActiveUsersLimit (serialized as Option<u64>: None = NoLimit, Some(n) = LimitedTo(n))

impl Serialize for super::ActiveUsersLimit {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let as_option: Option<u64> = (*self).into();
        Serialize::serialize(&as_option, writer)
    }
}

impl Deserialize for super::ActiveUsersLimit {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let opt: Option<u64> = Deserialize::deserialize(value)?;
        Ok(opt.into())
    }
}

// InvitationType (string, SCREAMING_SNAKE_CASE)

impl Serialize for super::InvitationType {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let s = match self {
            super::InvitationType::User => "USER",
            super::InvitationType::Device => "DEVICE",
            super::InvitationType::ShamirRecovery => "SHAMIR_RECOVERY",
        };
        Serialize::serialize(s, writer)
    }
}

impl Deserialize for super::InvitationType {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        match s.as_str() {
            "USER" => Ok(super::InvitationType::User),
            "DEVICE" => Ok(super::InvitationType::Device),
            "SHAMIR_RECOVERY" => Ok(super::InvitationType::ShamirRecovery),
            _ => Err(DeserializeError::InvalidValue(format!(
                "unknown InvitationType: {s}"
            ))),
        }
    }
}

// EmailAddress (string via Display/FromStr)

impl Serialize for super::EmailAddress {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        Serialize::serialize(&self.to_string(), writer)
    }
}

impl Deserialize for super::EmailAddress {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        s.parse().map_err(|e: super::EmailAddressParseError| {
            DeserializeError::InvalidValue(e.to_string())
        })
    }
}

// PkiSignatureAlgorithm (string via Display/FromStr)

impl Serialize for super::PkiSignatureAlgorithm {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        let s: &str = (*self).into();
        Serialize::serialize(s, writer)
    }
}

impl Deserialize for super::PkiSignatureAlgorithm {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        let s: String = Deserialize::deserialize(value)?;
        s.parse().map_err(|e: &str| {
            DeserializeError::InvalidValue(e.to_owned())
        })
    }
}

// UntrustedPasswordAlgorithm (tagged map with "type" discriminant)

impl Serialize for super::UntrustedPasswordAlgorithm {
    fn serialize(&self, writer: &mut Vec<u8>) -> Result<(), SerializeError> {
        match self {
            super::UntrustedPasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
            } => {
                encode::write_map_len(writer, 4)?;
                Serialize::serialize("memlimit_kb", writer)?;
                Serialize::serialize(memlimit_kb, writer)?;
                Serialize::serialize("opslimit", writer)?;
                Serialize::serialize(opslimit, writer)?;
                Serialize::serialize("parallelism", writer)?;
                Serialize::serialize(parallelism, writer)?;
                Serialize::serialize("type", writer)?;
                Serialize::serialize("ARGON2ID", writer)?;
                Ok(())
            }
        }
    }
}

impl Deserialize for super::UntrustedPasswordAlgorithm {
    fn deserialize(value: ValueRef<'_>) -> Result<Self, DeserializeError> {
        match value {
            ValueRef::Map(entries) => {
                let mut obj = HashMap::with_capacity(entries.len());
                for (raw_key, raw_value) in entries {
                    let key: String = Deserialize::deserialize(raw_key)?;
                    obj.insert(key, raw_value);
                }
                let ty = obj
                    .remove("type")
                    .ok_or(DeserializeError::MissingField("type"))?;
                let ty: String = Deserialize::deserialize(ty)?;
                match ty.as_str() {
                    "ARGON2ID" => {
                        let memlimit_kb = obj
                            .remove("memlimit_kb")
                            .ok_or(DeserializeError::MissingField("memlimit_kb"))?;
                        let memlimit_kb: u32 = Deserialize::deserialize(memlimit_kb)?;
                        let opslimit = obj
                            .remove("opslimit")
                            .ok_or(DeserializeError::MissingField("opslimit"))?;
                        let opslimit: u32 = Deserialize::deserialize(opslimit)?;
                        let parallelism = obj
                            .remove("parallelism")
                            .ok_or(DeserializeError::MissingField("parallelism"))?;
                        let parallelism: u32 = Deserialize::deserialize(parallelism)?;
                        Ok(super::UntrustedPasswordAlgorithm::Argon2id {
                            memlimit_kb,
                            opslimit,
                            parallelism,
                        })
                    }
                    _ => Err(DeserializeError::InvalidValue(format!(
                        "unknown UntrustedPasswordAlgorithm type: {ty}"
                    ))),
                }
            }
            other => Err(DeserializeError::InvalidType {
                expected: "map",
                got: value_kind(&other),
            }),
        }
    }
}
