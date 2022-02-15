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
        let mut e =
            ::flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        use std::io::Write;
        e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
        let compressed = e.finish().unwrap_or_else(|_| unreachable!());
        key.encrypt(&compressed)
    }
}

impl LocalUserManifest {
    pub fn decrypt_and_load(
        encrypted: &[u8],
        key: &::parsec_api_crypto::SecretKey,
        expected_author: &DeviceID,
        expected_timestamp: &DateTime<Utc>,
    ) -> Result<LocalUserManifest, &'static str> {
        let compressed = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        let mut serialized = vec![];
        use std::io::Read;
        ::flate2::read::ZlibDecoder::new(&compressed[..])
            .read_to_end(&mut serialized)
            .map_err(|_| "Invalid compression")?;
        let obj: LocalUserManifest =
            rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?;
        if &obj.author != expected_author {
            return Err("Unexpected author");
        } else if &obj.timestamp != expected_timestamp {
            Err("Unexpected timestamp")
        } else {
            Ok(obj)
        }
    }
}

/*
 * LocalUserManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalUserManifestData", from = "LocalUserManifestData")]
pub struct LocalUserManifest {
    pub author: DeviceID,
    pub timestamp: DateTime<Utc>,

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
    #[serde(default = "generate_local_author_legacy_placeholder")]
    author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    timestamp: DateTime<Utc>,
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
    author,
    timestamp,
    base,
    need_sync,
    updated,
    last_processed_message,
    workspaces,
    speculative,
);