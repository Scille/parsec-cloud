// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_types::regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU64;

use libparsec_crypto::{HashDigest, SecretKey};
use libparsec_types::{
    impl_transparent_data_format_conversion, BlockAccess, BlockID, Blocksize, ChunkID, DateTime,
    DeviceID, EntryID, EntryName, FileManifest, FolderManifest, UserManifest, WorkspaceEntry,
    WorkspaceManifest,
};
use serialization_format::parsec_data;

use crate as libparsec_client_types;

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
                ::rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")
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

impl PartialEq<u64> for Chunk {
    fn eq(&self, other: &u64) -> bool {
        self.start == *other
    }
}

impl PartialOrd<u64> for Chunk {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        Some(self.start.cmp(other))
    }
}

impl Chunk {
    pub fn new(start: u64, stop: NonZeroU64) -> Self {
        Self {
            id: ChunkID::default(),
            start,
            stop,
            raw_offset: start,
            // TODO: what to do with overflow
            raw_size: NonZeroU64::try_from(stop.get() - start).unwrap_or_else(|_| unreachable!()),
            access: None,
        }
    }
    pub fn from_block_access(block_access: BlockAccess) -> Result<Self, &'static str> {
        Ok(Self {
            id: ChunkID::from(*block_access.id),
            raw_offset: block_access.offset,
            raw_size: block_access.size,
            start: block_access.offset,
            // TODO: what to do with overflow
            stop: (block_access.offset + block_access.size.get())
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
            access: Some(block_access),
        })
    }

    pub fn evolve_as_block(mut self, data: &[u8]) -> Result<Self, &'static str> {
        // No-op
        if self.is_block() {
            return Ok(self);
        }

        // Check alignement
        if self.raw_offset != self.start {
            return Err("This chunk is not aligned");
        }

        // Craft access
        self.access = Some(BlockAccess {
            id: BlockID::from(*self.id),
            key: SecretKey::generate(),
            offset: self.start,
            size: (self.stop.get() - self.start)
                .try_into()
                .map_err(|_| "Stop - Start must be > 0")?,
            digest: HashDigest::from_data(data),
        });

        Ok(self)
    }

    pub fn is_block(&self) -> bool {
        // Requires an access
        if let Some(access) = &self.access {
            // Pseudo block
            self.is_pseudo_block()
                // Offset inconsistent
                && self.raw_offset == access.offset
                // Size inconsistent
                && self.raw_size == access.size
        } else {
            false
        }
    }

    pub fn is_pseudo_block(&self) -> bool {
        // Not left aligned
        if self.start != self.raw_offset {
            return false;
        }
        // Not right aligned
        if self.stop.get() != self.raw_offset + self.raw_size.get() {
            return false;
        }
        true
    }

    pub fn get_block_access(&self) -> Result<&BlockAccess, &'static str> {
        if !self.is_block() {
            return Err("This chunk does not correspond to a block");
        }
        Ok(self.access.as_ref().unwrap())
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

parsec_data!("schema/local_file_manifest.json");

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
            ty: Default::default(),
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
    pub fn new(
        author: DeviceID,
        parent: EntryID,
        timestamp: DateTime,
        blocksize: Blocksize,
    ) -> Self {
        Self {
            base: FileManifest {
                author,
                timestamp,
                id: EntryID::default(),
                parent,
                version: 0,
                created: timestamp,
                updated: timestamp,
                blocksize,
                size: 0,
                blocks: vec![],
            },
            need_sync: true,
            updated: timestamp,
            blocksize,
            size: 0,
            blocks: vec![],
        }
    }

    pub fn get_chunks(&self, block: usize) -> Option<&Vec<Chunk>> {
        self.blocks.get(block)
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
                assert!(chunk.stop.get() <= chunk.raw_offset + chunk.raw_size.get());
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

        let blocks = self
            .blocks
            .iter()
            .map(|chunks| chunks[0].get_block_access())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Need reshape")?
            .into_iter()
            .cloned()
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

    pub fn match_remote(&self, remote_manifest: &FileManifest) -> bool {
        let mut reference =
            match self.to_remote(remote_manifest.author.clone(), remote_manifest.timestamp) {
                Ok(reference) => reference,
                _ => return false,
            };
        reference.version = remote_manifest.version;
        reference == *remote_manifest
    }

    pub fn set_single_block(&mut self, block: u64, new_chunk: Chunk) -> Result<Vec<Chunk>, u64> {
        let slice = self.blocks.get_mut(block as usize).ok_or(block)?;
        Ok(std::mem::replace(slice, vec![new_chunk]))
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

parsec_data!("schema/local_folder_manifest.json");

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

impl_local_manifest_dump_load!(LocalFolderManifest);

impl LocalFolderManifest {
    pub fn new(author: DeviceID, parent: EntryID, timestamp: DateTime) -> Self {
        Self {
            base: FolderManifest {
                author,
                timestamp,
                id: EntryID::default(),
                parent,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
        }
    }

    pub fn evolve_children_and_mark_updated(
        mut self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let mut actually_updated = false;
        // Deal with removal first
        for (name, entry_id) in data.iter() {
            // Here `entry_id` can be either:
            // - a new entry id that might overwrite the previous one with the same name if it exists
            // - `None` which means the entry for the corresponding name should be removed
            if !self.children.contains_key(name) {
                // Make sure we don't remove a name that does not exist
                assert!(entry_id.is_some());
                continue;
            }
            // Remove old entry
            if let Some(old_entry_id) = self.children.remove(name) {
                if !self.local_confinement_points.remove(&old_entry_id) {
                    actually_updated = true;
                }
            }
        }
        // Make sure no entry_id is duplicated
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(data.values().filter_map(|v| v.as_ref()))
                .intersection(&HashSet::from_iter(self.children.values()))
                .count(),
            0
        );

        // Deal with additions second
        for (name, entry_id) in data.into_iter() {
            if let Some(entry_id) = entry_id {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    self.local_confinement_points.insert(entry_id);
                } else {
                    actually_updated = true;
                }
                // Add new entry
                self.children.insert(name, entry_id);
            }
        }

        if !actually_updated {
            return self;
        }

        self.need_sync = true;
        self.updated = timestamp;
        self
    }

    fn filter_local_confinement_points(mut self) -> Self {
        if self.local_confinement_points.is_empty() {
            return self;
        }

        self.children
            .retain(|_, entry_id| !self.local_confinement_points.contains(entry_id));

        self.local_confinement_points.clear();
        self
    }

    fn restore_local_confinement_points(
        self,
        other: &Self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        // Using self.remote_confinement_points is useful to restore entries that were present locally
        // before applying a new filter that filtered those entries from the remote manifest
        if other.local_confinement_points.is_empty() && self.remote_confinement_points.is_empty() {
            return self;
        }
        // Create a set for fast lookup in order to make sure no entry gets duplicated.
        // This might happen when a synchronized entry is renamed to a confined name locally.
        let self_entry_ids = HashSet::<_, RandomState>::from_iter(self.children.values());
        let previously_local_confinement_points = other
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if !self_entry_ids.contains(entry_id)
                    && (other.local_confinement_points.contains(entry_id)
                        || self.remote_confinement_points.contains(entry_id))
                {
                    Some((name.clone(), Some(*entry_id)))
                } else {
                    None
                }
            })
            .collect();

        self.evolve_children_and_mark_updated(
            previously_local_confinement_points,
            prevent_sync_pattern,
            timestamp,
        )
    }

    fn filter_remote_entries(mut self, prevent_sync_pattern: &Regex) -> Self {
        let remote_confinement_points: HashSet<_> = self
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        if remote_confinement_points.is_empty() {
            return self;
        }

        self.remote_confinement_points = remote_confinement_points;
        self.children
            .retain(|_, entry_id| !self.remote_confinement_points.contains(entry_id));

        self
    }

    fn restore_remote_confinement_points(mut self) -> Self {
        if self.remote_confinement_points.is_empty() {
            return self;
        }

        for (name, entry_id) in self.base.children.iter() {
            if self.remote_confinement_points.contains(entry_id) {
                self.children.insert(name.clone(), *entry_id);
            }
        }
        self.remote_confinement_points.clear();
        self
    }

    pub fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let result = self.clone();
        result
            // Filter local confinement points
            .filter_local_confinement_points()
            // Restore remote confinement points
            .restore_remote_confinement_points()
            // Filter remote confinement_points
            .filter_remote_entries(prevent_sync_pattern)
            // Restore local confinement points
            .restore_local_confinement_points(self, prevent_sync_pattern, timestamp)
    }

    pub fn from_remote(remote: FolderManifest, prevent_sync_pattern: &Regex) -> Self {
        let updated = remote.updated;
        let children = remote.children.clone();

        Self {
            base: remote,
            need_sync: false,
            updated,
            children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
        }
        .filter_remote_entries(prevent_sync_pattern)
    }

    pub fn from_remote_with_local_context(
        remote: FolderManifest,
        prevent_sync_pattern: &Regex,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        Self::from_remote(remote, prevent_sync_pattern).restore_local_confinement_points(
            local_manifest,
            prevent_sync_pattern,
            timestamp,
        )
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        let result = self
            .clone()
            // Filter confined entries
            .filter_local_confinement_points()
            // Restore filtered entries
            .restore_remote_confinement_points();
        // Create remote manifest
        FolderManifest {
            author,
            timestamp,
            id: result.base.id,
            parent: result.base.parent,
            version: result.base.version + 1,
            created: result.base.created,
            updated: result.updated,
            children: result.children,
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

parsec_data!("schema/local_workspace_manifest.json");

impl_local_manifest_dump_load!(LocalWorkspaceManifest);

impl_transparent_data_format_conversion!(
    LocalWorkspaceManifest,
    LocalWorkspaceManifestData,
    base,
    need_sync,
    updated,
    children,
    local_confinement_points,
    remote_confinement_points,
    speculative
);

impl LocalWorkspaceManifest {
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: Option<EntryID>,
        speculative: bool,
    ) -> Self {
        Self {
            base: WorkspaceManifest {
                author,
                timestamp,
                id: id.unwrap_or_default(),
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative,
        }
    }

    pub fn evolve_children_and_mark_updated(
        mut self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let mut actually_updated = false;
        // Deal with removal first
        for (name, entry_id) in data.iter() {
            // Here `entry_id` can be either:
            // - a new entry id that might overwrite the previous one with the same name if it exists
            // - `None` which means the entry for the corresponding name should be removed
            if !self.children.contains_key(name) {
                // Make sure we don't remove a name that does not exist
                assert!(entry_id.is_some());
                continue;
            }
            // Remove old entry
            if let Some(old_entry_id) = self.children.remove(name) {
                if !self.local_confinement_points.remove(&old_entry_id) {
                    actually_updated = true;
                }
            }
        }
        // Make sure no entry_id is duplicated
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(data.values().filter_map(|v| v.as_ref()))
                .intersection(&HashSet::from_iter(self.children.values()))
                .count(),
            0
        );

        // Deal with additions second
        for (name, entry_id) in data.into_iter() {
            if let Some(entry_id) = entry_id {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    self.local_confinement_points.insert(entry_id);
                } else {
                    actually_updated = true;
                }
                // Add new entry
                self.children.insert(name, entry_id);
            }
        }

        if !actually_updated {
            return self;
        }

        self.need_sync = true;
        self.updated = timestamp;
        self
    }

    fn filter_local_confinement_points(mut self) -> Self {
        if self.local_confinement_points.is_empty() {
            return self;
        }

        self.children
            .retain(|_, entry_id| !self.local_confinement_points.contains(entry_id));

        self.local_confinement_points.clear();
        self
    }

    fn restore_local_confinement_points(
        self,
        other: &Self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        // Using self.remote_confinement_points is useful to restore entries that were present locally
        // before applying a new filter that filtered those entries from the remote manifest
        if other.local_confinement_points.is_empty() && self.remote_confinement_points.is_empty() {
            return self;
        }
        // Create a set for fast lookup in order to make sure no entry gets duplicated.
        // This might happen when a synchronized entry is renamed to a confined name locally.
        let self_entry_ids = HashSet::<_, RandomState>::from_iter(self.children.values());
        let previously_local_confinement_points = other
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if !self_entry_ids.contains(entry_id)
                    && (other.local_confinement_points.contains(entry_id)
                        || self.remote_confinement_points.contains(entry_id))
                {
                    Some((name.clone(), Some(*entry_id)))
                } else {
                    None
                }
            })
            .collect();

        self.evolve_children_and_mark_updated(
            previously_local_confinement_points,
            prevent_sync_pattern,
            timestamp,
        )
    }

    fn filter_remote_entries(mut self, prevent_sync_pattern: &Regex) -> Self {
        let remote_confinement_points: HashSet<_> = self
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if prevent_sync_pattern.is_match(name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        if remote_confinement_points.is_empty() {
            return self;
        }

        self.remote_confinement_points = remote_confinement_points;
        self.children
            .retain(|_, entry_id| !self.remote_confinement_points.contains(entry_id));

        self
    }

    fn restore_remote_confinement_points(mut self) -> Self {
        if self.remote_confinement_points.is_empty() {
            return self;
        }

        for (name, entry_id) in self.base.children.iter() {
            if self.remote_confinement_points.contains(entry_id) {
                self.children.insert(name.clone(), *entry_id);
            }
        }
        self.remote_confinement_points.clear();
        self
    }

    pub fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let result = self.clone();
        result
            // Filter local confinement points
            .filter_local_confinement_points()
            // Restore remote confinement points
            .restore_remote_confinement_points()
            // Filter remote confinement_points
            .filter_remote_entries(prevent_sync_pattern)
            // Restore local confinement points
            .restore_local_confinement_points(self, prevent_sync_pattern, timestamp)
    }

    pub fn from_remote(remote: WorkspaceManifest, prevent_sync_pattern: &Regex) -> Self {
        let updated = remote.updated;
        let children = remote.children.clone();

        Self {
            base: remote,
            need_sync: false,
            updated,
            children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative: false,
        }
        .filter_remote_entries(prevent_sync_pattern)
    }

    pub fn from_remote_with_local_context(
        remote: WorkspaceManifest,
        prevent_sync_pattern: &Regex,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        Self::from_remote(remote, prevent_sync_pattern).restore_local_confinement_points(
            local_manifest,
            prevent_sync_pattern,
            timestamp,
        )
    }

    pub fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> WorkspaceManifest {
        let result = self
            .clone()
            // Filter confined entries
            .filter_local_confinement_points()
            // Restore filtered entries
            .restore_remote_confinement_points();
        // Create remote manifest
        WorkspaceManifest {
            author,
            timestamp,
            id: result.base.id,
            version: result.base.version + 1,
            created: result.base.created,
            updated: result.updated,
            children: result.children,
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
    pub last_processed_message: u64,
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

parsec_data!("schema/local_user_manifest.json");

impl_transparent_data_format_conversion!(
    LocalUserManifest,
    LocalUserManifestData,
    base,
    need_sync,
    updated,
    last_processed_message,
    workspaces,
    speculative
);

impl LocalUserManifest {
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: Option<EntryID>,
        speculative: bool,
    ) -> Self {
        Self {
            base: UserManifest {
                author,
                timestamp,
                id: id.unwrap_or_default(),
                version: 0,
                created: timestamp,
                updated: timestamp,
                last_processed_message: 0,
                workspaces: vec![],
            },
            need_sync: true,
            updated: timestamp,
            last_processed_message: 0,
            workspaces: vec![],
            speculative,
        }
    }

    pub fn evolve_workspaces(mut self, workspace: WorkspaceEntry) -> Self {
        let mut workspaces =
            HashMap::<_, _, RandomState>::from_iter(self.workspaces.into_iter().map(|w| (w.id, w)));
        workspaces.insert(workspace.id, workspace);
        self.workspaces = workspaces.into_values().collect();
        self
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum LocalManifest {
    File(LocalFileManifest),
    Folder(LocalFolderManifest),
    Workspace(LocalWorkspaceManifest),
    User(LocalUserManifest),
}

impl LocalManifest {
    pub fn id(&self) -> EntryID {
        match self {
            Self::File(manifest) => manifest.base.id,
            Self::Folder(manifest) => manifest.base.id,
            Self::Workspace(manifest) => manifest.base.id,
            Self::User(manifest) => manifest.base.id,
        }
    }

    pub fn need_sync(&self) -> bool {
        match self {
            Self::File(manifest) => manifest.need_sync,
            Self::Folder(manifest) => manifest.need_sync,
            Self::Workspace(manifest) => manifest.need_sync,
            Self::User(manifest) => manifest.need_sync,
        }
    }

    pub fn base_version(&self) -> u32 {
        match self {
            Self::File(manifest) => manifest.base.version,
            Self::Folder(manifest) => manifest.base.version,
            Self::Workspace(manifest) => manifest.base.version,
            Self::User(manifest) => manifest.base.version,
        }
    }

    pub fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
        match self {
            Self::File(manifest) => manifest.dump_and_encrypt(key),
            Self::Folder(manifest) => manifest.dump_and_encrypt(key),
            Self::Workspace(manifest) => manifest.dump_and_encrypt(key),
            Self::User(manifest) => manifest.dump_and_encrypt(key),
        }
    }

    pub fn decrypt_and_load(encrypted: &[u8], key: &SecretKey) -> Result<Self, &'static str> {
        let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")
    }
}
