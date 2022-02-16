// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use parsec_api_types::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::{HashMap, HashSet};

/*
 * LocalFileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFileManifestData", from = "LocalFileManifestData")]
pub struct LocalFileManifest {
    pub base: FileManifest,
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub size: i64,
    // Is it ok if blocksize < 8 ? because FileManifest doesn't restrict
    pub blocksize: i64,
    pub blocks: Vec<BlockAccess>,
}

new_data_struct_type!(
    LocalFileManifestData,
    type: "local_file_manifest",
    base: FileManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    size: i64,
    blocksize: i64,
    blocks: Vec<BlockAccess>,
);

impl_transparent_data_format_conversion!(
    LocalFileManifest,
    LocalFileManifestData,
    base,
    need_sync,
    updated,
    size,
    blocksize,
    blocks,
);

/*
 * LocalFolderManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFolderManifestData", from = "LocalFolderManifestData")]
pub struct LocalFolderManifest {
    pub base: FolderManifest,
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub children: HashMap<EntryName, ManifestEntry>,
    pub local_confinement_points: HashSet<ManifestEntry>,
    pub remote_confinement_points: HashSet<ManifestEntry>,
}

new_data_struct_type!(
    LocalFolderManifestData,
    type: "local_folder_manifest",
    base: FolderManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    children: HashMap<EntryName, ManifestEntry>,
    local_confinement_points: HashSet<ManifestEntry>,
    remote_confinement_points: HashSet<ManifestEntry>,
);

impl_transparent_data_format_conversion!(
    LocalFolderManifest,
    LocalFolderManifestData,
    base,
    need_sync,
    updated,
    children,
    local_confinement_points,
    remote_confinement_points,
);

/*
 * LocalWorkspaceManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "LocalWorkspaceManifestData",
    from = "LocalWorkspaceManifestData"
)]
pub struct LocalWorkspaceManifest {
    pub base: WorkspaceManifest,
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub children: HashMap<EntryName, ManifestEntry>,
    pub local_confinement_points: HashSet<ManifestEntry>,
    pub remote_confinement_points: HashSet<ManifestEntry>,
    pub speculative: bool,
}

new_data_struct_type!(
    LocalWorkspaceManifestData,
    type: "local_workspace_manifest",
    base: WorkspaceManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    children: HashMap<EntryName, ManifestEntry>,
    local_confinement_points: HashSet<ManifestEntry>,
    remote_confinement_points: HashSet<ManifestEntry>,
    speculative: Option<bool>,
);

impl From<LocalWorkspaceManifestData> for LocalWorkspaceManifest {
    fn from(data: LocalWorkspaceManifestData) -> Self {
        Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            children: data.children,
            local_confinement_points: data.local_confinement_points,
            remote_confinement_points: data.remote_confinement_points,
            speculative: data.speculative.unwrap_or(false),
        }
    }
}

impl From<LocalWorkspaceManifest> for LocalWorkspaceManifestData {
    fn from(obj: LocalWorkspaceManifest) -> Self {
        Self {
            type_: Default::default(),
            base: obj.base,
            need_sync: obj.need_sync,
            updated: obj.updated,
            children: obj.children,
            local_confinement_points: obj.local_confinement_points,
            remote_confinement_points: obj.remote_confinement_points,
            speculative: Some(obj.speculative),
        }
    }
}

/*
 * LocalUserManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalUserManifestData", from = "LocalUserManifestData")]
pub struct LocalUserManifest {
    pub base: UserManifest,
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub last_processed_message: u32,
    pub workspaces: Vec<WorkspaceEntry>,
    pub speculative: bool,
}

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

new_data_struct_type!(
    LocalUserManifestData,
    type: "local_user_manifest",
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
