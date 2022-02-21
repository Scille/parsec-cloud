// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use parsec_api_types::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU64;

pub trait Encrypt
where
    Self: Sized + Serialize + DeserializeOwned,
{
    fn dump_and_encrypt(&self, key: &::parsec_api_crypto::SecretKey) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        key.encrypt(&serialized)
    }
    fn decrypt_and_load(
        encrypted: &[u8],
        key: &parsec_api_crypto::SecretKey,
    ) -> Result<Self, &'static str> {
        let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        rmp_serde::from_read_ref::<_, Self>(&serialized).map_err(|_| "Invalid serialization")
    }
}

/*
 * Chunk
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chunk {
    pub id: ChunkID,
    pub start: u64,
    pub stop: NonZeroU64,
    pub raw_offset: u64,
    pub raw_size: NonZeroU64,
    pub access: Option<BlockAccess>,
}

/*
 * LocalFileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFileManifestData", try_from = "LocalFileManifestData")]
pub struct LocalFileManifest {
    pub base: FileManifest,
    pub need_sync: bool,
    pub updated: DateTime<Utc>,
    pub size: u64,
    pub blocksize: u64,
    pub blocks: Vec<Vec<Chunk>>,
}

impl LocalFileManifest {
    pub fn new(
        base: FileManifest,
        need_sync: bool,
        blocksize: u64,
        blocks: Vec<Vec<Chunk>>,
    ) -> Result<Self, &'static str> {
        if blocksize < 8 {
            return Err("Invalid blocksize");
        }

        Ok(Self {
            base,
            need_sync,
            updated: Utc::now(),
            size: blocks.len() as u64,
            blocksize,
            blocks,
        })
    }
}

new_data_struct_type!(
    LocalFileManifestData,
    type: "local_file_manifest",
    base: FileManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    size: u64,
    blocksize: u64,
    blocks: Vec<Vec<Chunk>>,
);

impl TryFrom<LocalFileManifestData> for LocalFileManifest {
    type Error = &'static str;
    fn try_from(data: LocalFileManifestData) -> Result<Self, Self::Error> {
        if data.blocksize < 8 {
            return Err("Invalid blocksize");
        }

        Ok(Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            size: data.size,
            blocksize: data.blocksize,
            blocks: data.blocks,
        })
    }
}

impl From<LocalFileManifest> for LocalFileManifestData {
    fn from(obj: LocalFileManifest) -> Self {
        Self {
            type_: LocalFileManifestDataDataType,
            base: obj.base,
            need_sync: obj.need_sync,
            updated: obj.updated,
            size: obj.size,
            blocksize: obj.blocksize,
            blocks: obj.blocks,
        }
    }
}

impl Encrypt for LocalFileManifest {}

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
    pub local_confinement_points: Option<HashSet<ManifestEntry>>,
    pub remote_confinement_points: Option<HashSet<ManifestEntry>>,
}

new_data_struct_type!(
    LocalFolderManifestData,
    type: "local_folder_manifest",
    base: FolderManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    children: HashMap<EntryName, ManifestEntry>,
    local_confinement_points: Option<HashSet<ManifestEntry>>,
    remote_confinement_points: Option<HashSet<ManifestEntry>>,
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

impl Encrypt for LocalFolderManifest {}

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
    pub local_confinement_points: Option<HashSet<ManifestEntry>>,
    pub remote_confinement_points: Option<HashSet<ManifestEntry>>,
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
    local_confinement_points: Option<HashSet<ManifestEntry>>,
    remote_confinement_points: Option<HashSet<ManifestEntry>>,
    speculative: Option<bool>,
);

impl Encrypt for LocalWorkspaceManifest {}

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

impl Encrypt for LocalUserManifest {}

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
