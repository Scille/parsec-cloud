// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{KeyDerivation, SecretKey, SigningKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DataResult, DateTime, DeviceID, IndexInt, UnsecureSkipValidationReason, VlobID,
};

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
    keys: Vec<KeyDerivation>,
}

impl RealmKeysBundle {
    pub fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        keys: Vec<KeyDerivation>,
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

    pub fn keys(&self) -> &[KeyDerivation] {
        &self.keys
    }

    pub fn last_key(&self) -> &KeyDerivation {
        self.keys
            .last()
            .expect("Realm keys bundle should not be empty")
    }
}

parsec_data!("schema/realm/realm_keys_bundle.json5");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyRealmKeysBundle;

impl std::error::Error for EmptyRealmKeysBundle {}

impl std::fmt::Display for EmptyRealmKeysBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RealmKeysBundle must have at least one key")
    }
}

impl TryFrom<RealmKeysBundleData> for RealmKeysBundle {
    type Error = EmptyRealmKeysBundle;

    fn try_from(data: RealmKeysBundleData) -> Result<Self, Self::Error> {
        if data.keys.is_empty() {
            return Err(EmptyRealmKeysBundle);
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

super::certif::impl_base_load!(RealmKeysBundle);
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
        format_vx_load(raw)
    }
    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(&self)
    }
}

#[cfg(test)]
#[path = "../tests/unit/realm.rs"]
mod tests;
