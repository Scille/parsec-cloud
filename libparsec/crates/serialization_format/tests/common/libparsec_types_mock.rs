// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// We don't want to have this crate depend on `libparsec_types` for it test
// given `libparsec_types` itself uses this crate... hence in case of bug in
// this crate we wouldn't be able to use the tests !
// So here we simulate `libparsec_types` by implementing the bare minimum that
// our macros need.

pub type Integer = i64;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DeviceID(pub String);

pub trait ProtocolRequest<const V: u32> {
    const API_MAJOR_VERSION: u32 = V;
    type Response: for<'de> Deserialize<'de>;

    fn api_dump(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, rmp_serde::decode::Error>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
