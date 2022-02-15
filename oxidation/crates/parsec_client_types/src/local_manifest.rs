use chrono::prelude::*;
use parsec_api_types::*;
use serde::{Deserialize, Serialize};
use serde_with::*;

impl LocalUserManifest {
    pub fn dump_and_encrypt(
        &self,
        key: &::parsec_api_crypto::SecretKey,
    ) -> Vec<u8> {
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
            rmp_serde::from_read_ref(&serialized).map_err(|e| {println!("{}", e);"Invalid serialization"})?;
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
    pub speculative: Option<bool>,
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

impl_transparent_data_format_convertion!(
    LocalUserManifest,
    LocalUserManifestData,
    base,
    need_sync,
    updated,
    last_processed_message,
    workspaces,
    speculative,
);

