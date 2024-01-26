// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::io::{Read, Write};

use bytes::Bytes;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use paste::paste;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{SecretKey, SigningKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, data_macros::impl_transparent_data_format_conversion, DataError,
    DataResult, DateTime, DeviceID, IndexInt, UnsecureSkipValidationReason, VlobID,
};

fn load<T: DeserializeOwned>(raw: &[u8]) -> DataResult<T> {
    let mut decompressed = vec![];

    ZlibDecoder::new(raw)
        .read_to_end(&mut decompressed)
        .map_err(|_| DataError::Compression)?;

    rmp_serde::from_slice(&decompressed).map_err(|_| DataError::Serialization)
}

fn dump<T: Serialize>(data: &T) -> Vec<u8> {
    let serialized = rmp_serde::to_vec_named(data).expect("Unreachable");

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&serialized).expect("Unreachable");

    e.finish().expect("Unreachable")
}

/*
 * RealmKeysBundle
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "RealmKeysBundleData", try_from = "RealmKeysBundleData")]
pub struct RealmKeysBundle {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: VlobID,
    /// The keys used to encrypt the realm, ordered from the oldest to the newest.
    /// Guaranteed to be > 1
    keys: Vec<SecretKey>,
}

impl RealmKeysBundle {
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        keys: Vec<SecretKey>,
    ) -> Self {
        assert!(!keys.is_empty(), "keys must not be empty");
        Self {
            author,
            timestamp,
            realm_id,
            keys,
        }
    }

    pub fn key_index(&self) -> IndexInt {
        self.keys.len() as IndexInt
    }

    pub fn keys(&self) -> &[SecretKey] {
        &self.keys
    }

    pub fn last_key(&self) -> &SecretKey {
        self.keys
            .last()
            .expect("Realm keys bundle should not be empty")
    }
}

parsec_data!("schema/realm/realm_keys_bundle.json5");

impl TryFrom<RealmKeysBundleData> for RealmKeysBundle {
    type Error = &'static str;

    fn try_from(data: RealmKeysBundleData) -> Result<Self, Self::Error> {
        if data.keys.is_empty() {
            return Err("RealmKeysBundle must have at least one key");
        }
        Ok(Self {
            author: data.author,
            timestamp: data.timestamp,
            realm_id: data.realm_id,
            keys: data.keys,
        })
    }
}

impl From<RealmKeysBundle> for RealmKeysBundleData {
    fn from(obj: RealmKeysBundle) -> Self {
        Self {
            ty: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            realm_id: obj.realm_id,
            keys: obj.keys,
        }
    }
}

super::certif::impl_unsecure_load!(RealmKeysBundle -> DeviceID);
super::certif::impl_unsecure_dump!(RealmKeysBundle);
super::certif::impl_dump_and_sign!(RealmKeysBundle);

/*
 * RealmKeysBundleAccess
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "RealmKeysBundleAccessData", from = "RealmKeysBundleAccessData")]
pub struct RealmKeysBundleAccess {
    pub keys_bundle_key: SecretKey,
}

parsec_data!("schema/realm/realm_keys_bundle_access.json5");

impl_transparent_data_format_conversion!(
    RealmKeysBundleAccess,
    RealmKeysBundleAccessData,
    keys_bundle_key,
);

impl RealmKeysBundleAccess {
    pub fn load(raw: &[u8]) -> DataResult<Self> {
        load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        dump(&self)
    }
}
