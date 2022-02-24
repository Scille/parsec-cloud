// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU64;

use parsec_api_crypto::SecretKey;
use parsec_api_types::*;

use crate::maybe_field;

macro_rules! impl_local_manifest_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
                let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                key.encrypt(&serialized)
            }

            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &SecretKey,
            ) -> Result<Self, &'static str> {
                let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                rmp_serde::from_read_ref::<_, Self>(&serialized)
                    .map_err(|_| "Invalid serialization")
            }
        }
    };
}

/*
 * Chunk
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chunk {
    // Represents a chunk of a data in file manifest.
    // The raw data is identified by its `id` attribute and is aligned using the
    // `raw_offset` attribute with respect to the file addressing. The raw data
    // size is stored as `raw_size`.

    // The `start` and `stop` attributes then describes the span of the actual data
    // still with respect to the file addressing.

    // This means the following rule applies:
    //     raw_offset <= start < stop <= raw_start + raw_size

    // Access is an optional block access that can be used to produce a remote manifest
    // when the chunk corresponds to an actual block within the context of this manifest.
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
    pub blocksize: Blocksize,
    pub blocks: Vec<Vec<Chunk>>,
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
        Ok(Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            size: data.size,
            blocksize: data.blocksize.try_into()?,
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
            blocksize: obj.blocksize.into(),
            blocks: obj.blocks,
        }
    }
}

impl_local_manifest_dump_load!(LocalFileManifest);

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
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<ManifestEntry>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
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
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    local_confinement_points: Option<HashSet<ManifestEntry>>,
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    remote_confinement_points: Option<HashSet<ManifestEntry>>,
);

impl From<LocalFolderManifestData> for LocalFolderManifest {
    fn from(data: LocalFolderManifestData) -> Self {
        Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            children: data.children,
            local_confinement_points: data.local_confinement_points.unwrap_or_default(),
            remote_confinement_points: data.remote_confinement_points.unwrap_or_default(),
        }
    }
}

impl From<LocalFolderManifest> for LocalFolderManifestData {
    fn from(obj: LocalFolderManifest) -> Self {
        Self {
            type_: Default::default(),
            base: obj.base,
            need_sync: obj.need_sync,
            updated: obj.updated,
            children: obj.children,
            local_confinement_points: Some(obj.local_confinement_points),
            remote_confinement_points: Some(obj.remote_confinement_points),
        }
    }
}

impl_local_manifest_dump_load!(LocalFolderManifest);

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
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<ManifestEntry>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
    pub remote_confinement_points: HashSet<ManifestEntry>,
    // Speculative placeholders are created when we want to access a workspace
    // but didn't retrieve manifest data from backend yet. This implies:
    // - non-placeholders cannot be speculative
    // - the only non-speculative placeholder is the placeholder initialized
    //   during the initial workspace creation
    // This speculative information is useful during merge to understand if
    // a data is not present in the placeholder compared with a remote because:
    // a) the data is not locally known (speculative is True)
    // b) the data is known, but has been locally removed (speculative is False)
    // Prevented to be `required=True` by backward compatibility
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
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    local_confinement_points: Option<HashSet<ManifestEntry>>,
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    remote_confinement_points: Option<HashSet<ManifestEntry>>,
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    speculative: Option<bool>,
);

impl_local_manifest_dump_load!(LocalWorkspaceManifest);

impl From<LocalWorkspaceManifestData> for LocalWorkspaceManifest {
    fn from(data: LocalWorkspaceManifestData) -> Self {
        Self {
            base: data.base,
            need_sync: data.need_sync,
            updated: data.updated,
            children: data.children,
            local_confinement_points: data.local_confinement_points.unwrap_or_default(),
            remote_confinement_points: data.remote_confinement_points.unwrap_or_default(),
            speculative: data.speculative.unwrap_or_default(),
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
            local_confinement_points: Some(obj.local_confinement_points),
            remote_confinement_points: Some(obj.remote_confinement_points),
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
    // Speculative placeholders are created when we want to access the
    // user manifest but didn't retrieve it from backend yet. This implies:
    // - non-placeholders cannot be speculative
    // - the only non-speculative placeholder is the placeholder initialized
    //   during the initial user claim (by opposition of subsequent device
    //   claims on the same user)
    // This speculative information is useful during merge to understand if
    // a data is not present in the placeholder compared with a remote because:
    // a) the data is not locally known (speculative is True)
    // b) the data is known, but has been locally removed (speculative is False)
    // Prevented to be `required=True` by backward compatibility
    pub speculative: bool,
}

impl_local_manifest_dump_load!(LocalUserManifest);

new_data_struct_type!(
    LocalUserManifestData,
    type: "local_user_manifest",
    base: UserManifest,
    need_sync: bool,
    #[serde_as(as = "DateTimeExtFormat")]
    updated: DateTime<Utc>,
    last_processed_message: u32,
    workspaces: Vec<WorkspaceEntry>,
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
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
            speculative: data.speculative.unwrap_or_default(),
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
