// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU64;

use parsec_api_crypto::SecretKey;
use parsec_api_types::*;

macro_rules! impl_local_manifest_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
                let serialized =
                    ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                key.encrypt(&serialized)
            }

            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &SecretKey,
            ) -> Result<Self, &'static str> {
                let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                ::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")
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
    //
    // The `start` and `stop` attributes then describes the span of the actual data
    // still with respect to the file addressing.
    //
    // This means the following rule applies:
    //     raw_offset <= start < stop <= raw_start + raw_size
    //
    // Access is an optional block access that can be used to produce a remote manifest
    // when the chunk corresponds to an actual block within the context of this manifest.
    pub id: ChunkID,
    pub start: u64,
    pub stop: NonZeroU64,
    pub raw_offset: u64,
    pub raw_size: NonZeroU64,
    pub access: Option<BlockAccess>,
}

impl Chunk {
    pub fn from_block_access(block_access: BlockAccess) -> Result<Self, &'static str> {
        let raw_size = NonZeroU64::try_from(block_access.size).map_err(|_| "invalid raw size")?;
        let stop = NonZeroU64::try_from(block_access.offset + block_access.size)
            .map_err(|_| "invalid stop")?;
        Ok(Self {
            id: parsec_api_types::ChunkID::from(*block_access.id),
            raw_offset: block_access.offset,
            raw_size,
            start: block_access.offset,
            stop,
            access: Some(block_access),
        })
    }

    pub fn evolve_as_block(&self, data: &[u8]) -> Result<Self, &'static str> {
        let mut result = self.clone();
        // No-op
        if result.is_block() {
            return Ok(result);
        }

        // Check alignement
        if result.raw_offset != result.start {
            return Err("This chunk is not aligned");
        }

        // Craft access
        result.access = Some(parsec_api_types::BlockAccess {
            id: parsec_api_types::BlockID::from(*result.id),
            key: parsec_api_crypto::SecretKey::generate(),
            offset: result.start,
            size: u64::from(result.stop) - result.start,
            digest: parsec_api_crypto::HashDigest::from_data(data),
        });

        Ok(result)
    }

    pub fn is_block(&self) -> bool {
        // Requires an access
        if let Some(access) = &self.access {
            // Pseudo block
            !(!self.is_pseudo_block()
                // Offset inconsistent
                || self.raw_offset != access.offset
                // Size inconsistent
                || u64::from(self.raw_size) != access.size)
        } else {
            false
        }
    }

    pub fn is_pseudo_block(&self) -> bool {
        if self.start != self.raw_offset {
            return false;
        }
        if u64::from(self.stop) != self.raw_offset + u64::from(self.raw_size) {
            return false;
        }
        true
    }

    pub fn get_block_access(&self) -> Result<Option<BlockAccess>, &'static str> {
        if !self.is_block() {
            return Err("This chunk does not coresspond to a block");
        }
        Ok(self.access.clone())
    }
}

/*
 * LocalFileManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFileManifestData", try_from = "LocalFileManifestData")]
pub struct LocalFileManifest {
    pub base: FileManifest,
    pub need_sync: bool,
    pub updated: DateTime,
    pub size: u64,
    pub blocksize: Blocksize,
    pub blocks: Vec<Vec<Chunk>>,
}

new_data_struct_type!(
    LocalFileManifestData,
    type: "local_file_manifest",
    base: FileManifest,
    need_sync: bool,
    updated: DateTime,
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

impl LocalFileManifest {
    pub fn get_chunks(&self, block: usize) -> Vec<Chunk> {
        self.blocks.get(block).cloned().unwrap_or_default()
    }

    pub fn is_reshaped(&self) -> bool {
        for chunks in self.blocks.iter() {
            if chunks.len() != 1 || !chunks[0].is_block() {
                return false;
            }
        }
        true
    }

    pub fn assert_integrity(&self) {
        let mut current = 0;
        for (i, chunks) in self.blocks.iter().enumerate() {
            assert_eq!(i as u64 * *self.blocksize, current);
            assert!(!chunks.is_empty());
            for chunk in chunks {
                assert_eq!(chunk.start, current);
                assert!(chunk.start < chunk.stop.into());
                assert!(chunk.raw_offset <= chunk.start);
                assert!(u64::from(chunk.stop) <= chunk.raw_offset + u64::from(chunk.raw_size));
                current = chunk.stop.into()
            }
        }
        assert_eq!(current, self.size);
    }

    pub fn from_remote(remote: FileManifest) -> Result<Self, &'static str> {
        let base = remote.clone();
        let blocks = remote
            .blocks
            .into_iter()
            .map(Chunk::from_block_access)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|b| vec![b])
            .collect();

        Ok(Self {
            base,
            need_sync: false,
            updated: remote.updated,
            size: remote.size,
            blocksize: remote.blocksize,
            blocks,
        })
    }

    pub fn to_remote(
        &self,
        author: DeviceID,
        timestamp: DateTime,
    ) -> Result<FileManifest, &'static str> {
        self.assert_integrity();
        assert!(self.is_reshaped());

        let blocks = self
            .blocks
            .iter()
            .map(|chunks| chunks[0].get_block_access())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(FileManifest {
            author,
            timestamp,
            id: self.base.id,
            parent: self.base.parent,
            version: self.base.version + 1,
            created: self.base.created,
            updated: self.updated,
            size: self.size,
            blocksize: self.blocksize,
            blocks,
        })
    }

    pub fn match_remote(&self, remote_manifest: &FileManifest) -> Result<bool, &'static str> {
        if !self.is_reshaped() {
            return Ok(false);
        }
        let mut reference =
            self.to_remote(remote_manifest.author.clone(), remote_manifest.timestamp)?;
        reference.version = remote_manifest.version;
        Ok(&reference == remote_manifest)
    }
}

/*
 * LocalFolderManifest
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFolderManifestData", from = "LocalFolderManifestData")]
pub struct LocalFolderManifest {
    pub base: FolderManifest,
    pub need_sync: bool,
    pub updated: DateTime,
    pub children: HashMap<EntryName, EntryID>,
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<EntryID>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
    pub remote_confinement_points: HashSet<EntryID>,
}

new_data_struct_type!(
    LocalFolderManifestData,
    type: "local_folder_manifest",
    base: FolderManifest,
    need_sync: bool,
    updated: DateTime,
    children: HashMap<EntryName, EntryID>,
    // Added in Parsec v1.15
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    local_confinement_points: Option<HashSet<EntryID>>,
    // Added in Parsec v1.15
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    remote_confinement_points: Option<HashSet<EntryID>>,
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

impl LocalFolderManifest {
    pub fn _filter_local_confinement_points(&self) -> Self {
        let mut result = self.clone();

        if result.local_confinement_points.is_empty() {
            return result;
        }

        result
            .children
            .retain(|_, entry_id| !self.local_confinement_points.contains(entry_id));

        result.local_confinement_points.clear();
        result
    }

    pub fn _restore_remote_confinement_points(&self) -> Self {
        let mut result = self.clone();

        if result.remote_confinement_points.is_empty() {
            return result;
        }

        for (name, entry_id) in self.base.children.iter() {
            if self.remote_confinement_points.contains(entry_id) {
                result.children.insert(name.clone(), *entry_id);
            }
        }
        result.remote_confinement_points.clear();
        result
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        // Filter confined entries
        let mut processed_manifest = self._filter_local_confinement_points();
        // Restore filtered entries
        processed_manifest = processed_manifest._restore_remote_confinement_points();
        // Create remote manifest
        FolderManifest {
            author,
            timestamp,
            id: self.base.id,
            parent: self.base.parent,
            version: self.base.version + 1,
            created: self.base.created,
            updated: self.updated,
            children: processed_manifest.children,
        }
    }

    pub fn match_remote(&self, remote_manifest: &FolderManifest) -> bool {
        let mut reference =
            self.to_remote(remote_manifest.author.clone(), remote_manifest.timestamp);
        reference.version = remote_manifest.version;
        reference == *remote_manifest
    }
}

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
    pub updated: DateTime,
    pub children: HashMap<EntryName, EntryID>,
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<EntryID>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
    pub remote_confinement_points: HashSet<EntryID>,
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
    updated: DateTime,
    children: HashMap<EntryName, EntryID>,
    // Added in Parsec v1.15
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    local_confinement_points: Option<HashSet<EntryID>>,
    // Added in Parsec v1.15
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    remote_confinement_points: Option<HashSet<EntryID>>,
    // Added in Parsec v2.6
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
            local_confinement_points: Some(obj.local_confinement_points),
            remote_confinement_points: Some(obj.remote_confinement_points),
            speculative: Some(obj.speculative),
        }
    }
}

impl LocalWorkspaceManifest {
    pub fn _filter_local_confinement_points(&self) -> Self {
        let mut result = self.clone();

        if result.local_confinement_points.is_empty() {
            return result;
        }

        result
            .children
            .retain(|_, entry_id| !self.local_confinement_points.contains(entry_id));

        result.local_confinement_points.clear();
        result
    }

    pub fn _restore_remote_confinement_points(&self) -> Self {
        let mut result = self.clone();

        if result.remote_confinement_points.is_empty() {
            return result;
        }

        for (name, entry_id) in self.base.children.iter() {
            if self.remote_confinement_points.contains(entry_id) {
                result.children.insert(name.clone(), *entry_id);
            }
        }
        result.remote_confinement_points.clear();
        result
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> WorkspaceManifest {
        let mut processed_manifest = self._filter_local_confinement_points();
        processed_manifest = processed_manifest._restore_remote_confinement_points();

        WorkspaceManifest {
            author,
            timestamp,
            id: self.base.id,
            version: self.base.version + 1,
            created: self.base.created,
            updated: self.updated,
            children: processed_manifest.children,
        }
    }

    pub fn match_remote(&self, remote_manifest: &WorkspaceManifest) -> bool {
        let mut reference =
            self.to_remote(remote_manifest.author.clone(), remote_manifest.timestamp);
        reference.version = remote_manifest.version;
        reference == *remote_manifest
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
    pub updated: DateTime,
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
    pub speculative: bool,
}

impl_local_manifest_dump_load!(LocalUserManifest);

new_data_struct_type!(
    LocalUserManifestData,
    type: "local_user_manifest",
    base: UserManifest,
    need_sync: bool,
    updated: DateTime,
    last_processed_message: u32,
    workspaces: Vec<WorkspaceEntry>,
    // Added in Parsec v2.6
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

impl LocalUserManifest {
    pub fn evolve_workspaces(&self, workspace: WorkspaceEntry) -> Self {
        let mut out = self.clone();
        let mut workspaces = HashMap::<_, _, RandomState>::from_iter(
            self.workspaces.clone().into_iter().map(|w| (w.id, w)),
        );
        workspaces.insert(workspace.id, workspace);
        out.workspaces = workspaces.into_values().collect();
        out
    }

    pub fn get_workspace_entry(&self, workspace_id: EntryID) -> Option<&WorkspaceEntry> {
        self.workspaces.iter().find(|w| w.id == workspace_id)
    }

    pub fn from_remote(remote: UserManifest) -> Self {
        let base = remote.clone();

        Self {
            base,
            need_sync: false,
            updated: remote.updated,
            last_processed_message: remote.last_processed_message,
            workspaces: remote.workspaces,
            speculative: false,
        }
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> UserManifest {
        UserManifest {
            author,
            timestamp,
            id: self.base.id,
            version: self.base.version + 1,
            created: self.base.created,
            updated: self.updated,
            last_processed_message: self.last_processed_message,
            workspaces: self.workspaces.clone(),
        }
    }

    pub fn match_remote(&self, remote_manifest: &UserManifest) -> bool {
        let mut reference =
            self.to_remote(remote_manifest.author.clone(), remote_manifest.timestamp);
        reference.version = remote_manifest.version;
        reference == *remote_manifest
    }
}
