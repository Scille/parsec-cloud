// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use parsec_api_types::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

impl LocalUserManifest {
    pub fn dump_and_encrypt(&self, key: &::parsec_api_crypto::SecretKey) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        key.encrypt(&serialized)
    }
}

impl LocalUserManifest {
    pub fn decrypt_and_load(
        encrypted: &[u8],
        key: &::parsec_api_crypto::SecretKey,
    ) -> Result<LocalUserManifest, &'static str> {
        let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        let obj: LocalUserManifest =
            rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?;
        Ok(obj)
    }
}

/*
 * LocalUserManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalUserManifestData", from = "LocalUserManifestData")]
pub struct LocalUserManifest {
    pub base: UserManifest,
    // Version 0 means the data is not synchronized
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub last_processed_message: u32,
    pub workspaces: Vec<WorkspaceEntry>,
    pub speculative: bool,
}

new_data_struct_type!(
    LocalUserManifestData,
    type: "local_user_manifest",
    // Compatibility with versions <= 1.14
    base: UserManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    last_processed_message: u32,
    workspaces: Vec<WorkspaceEntry>,
    speculative: Option<bool>,
);

impl From<LocalUserManifestData> for LocalUserManifest {
    fn from(data: LocalUserManifestData) -> Self {
        Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            last_processed_message: data.last_processed_message,
            workspaces: data.workspaces,
            speculative: data.speculative.unwrap_or(false),
        }
    }
}

impl From<LocalUserManifest> for LocalUserManifestData {
    fn from(obj: LocalUserManifest) -> Self {
        Self {
            type_: Default::default(),
            base: obj.base,
            need_sync: obj.need_sync,
            updated: obj.updated,
            last_processed_message: obj.last_processed_message,
            workspaces: obj.workspaces,
            speculative: Some(obj.speculative),
        }
    }
}
