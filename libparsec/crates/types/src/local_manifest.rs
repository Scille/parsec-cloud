// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Types used to store the content of the workspace on local storage.
//! These types are never sent to the server.
//!
//! The local manifests are based on remote manifests but contain additional information.
//! Unlike remote manifests that represent an immutable version of a data
//! that have been uploaded to the server, the local manifests represent the
//! data as it is currently known by the client:
//! - It is based on a remote manifest (see the `base` field).
//! - If the local manifest has just been created locally, then its base remote
//!   manifest is a placeholder with version = 0.
//! - It gets modified by the client (e.g. when a file is created, the parent folder
//!   manifest is updated to add the new file entry).
//! - It gets converted into a remote manifest when data is synchronized with the server.
//! - When synchronizing, the remote manifest (in which the local manifest is based on),
//!   is used to merge the local changes with the remote changes.

use std::{
    cmp::Ordering,
    collections::{hash_map::RandomState, HashMap, HashSet},
    num::NonZeroU64,
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use libparsec_crypto::{HashDigest, SecretKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types, impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    BlockAccess, BlockID, Blocksize, ChunkID, DataError, DataResult, DateTime, DeviceID, EntryName,
    FileManifest, FolderManifest, Regex, UserManifest, VlobID, DEFAULT_BLOCK_SIZE,
};

macro_rules! impl_local_manifest_dump {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
                self.check_data_integrity().expect("Manifest integrity");
                let serialized = format_v0_dump(&self);
                key.encrypt(&serialized)
            }
        }
    };
}

macro_rules! impl_local_manifest_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &SecretKey,
            ) -> libparsec_types::DataResult<Self> {
                let serialized = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;
                let result = format_vx_load(&serialized);
                result.and_then(|manifest: Self| manifest.check_data_integrity().map(|_| manifest))
            }
        }
    };
}

/*
 * Method specific errors
 */

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkViewPromoteAsBlockError {
    #[error("This chunk view has already been promoted as a block")]
    AlreadyPromotedAsBlock,

    #[error("This chunk view is not aligned and can't be promoted as a block")]
    NotAligned,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkViewGetBlockAccessError {
    #[error("This chunk view hasn't been promoted as a block")]
    NotPromotedAsBlock,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LocalFileManifestToRemoteError {
    #[error("Local file manifest needs reshape before being converted to remote")]
    NeedReshape,
}

/*
 * ChunkView
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Represents the content of a file manifest over a specific range of addresses.
///
/// It is called a `ChunkView` because it is implemented as a view over an
/// underlying chunk of data. Example:
///
/// File addressing:            raw offset            raw offset + raw size
/// Underlying chunk data: |--------|abcdefghijklmnopqrstuvwxyz|-------------|
/// Specific chunk view:   |--------------|ghijklmnopqrstu|------------------|
/// File addressing:                    start           stop
///
/// The raw data is identified by its `id` attribute and is aligned using the
/// `raw_offset` attribute with respect to the file addressing. The raw data
/// size is stored as `raw_size`.
///
/// The `start` and `stop` attributes then describes the span of the actual data
/// still with respect to the file addressing.
///
/// This means the following rule applies:
///     raw_offset <= start < stop <= raw_offset + raw_size
///
/// Access is an optional block access that can be used to produce a remote manifest
/// when the chunk corresponds to an actual block within the context of this manifest.
pub struct ChunkView {
    // This ID identifies the raw data the chunk view is based on
    // Two chunks can have the same id if they point to the same underlying data
    pub id: ChunkID,
    // TODO: don't directly expose those fields, this way we can guarantee
    //       invariances on their order (e.g. start < stop, raw_offset <= start, etc.)
    pub start: u64,
    pub stop: NonZeroU64,
    pub raw_offset: u64,
    pub raw_size: NonZeroU64,
    pub access: Option<BlockAccess>,
}

impl PartialEq<u64> for ChunkView {
    fn eq(&self, other: &u64) -> bool {
        self.start == *other
    }
}

impl PartialOrd<u64> for ChunkView {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        Some(self.start.cmp(other))
    }
}

impl ChunkView {
    pub fn new(start: u64, stop: NonZeroU64) -> Self {
        let chunk_view = Self {
            id: ChunkID::default(),
            start,
            stop,
            raw_offset: start,
            // TODO: what to do with overflow
            raw_size: NonZeroU64::try_from(stop.get() - start)
                .expect("Chunk view raw_size should be NonZeroU64"),
            access: None,
        };
        // Sanity check
        chunk_view
            .check_data_integrity()
            .expect("Chunk view integrity");
        chunk_view
    }

    pub fn size(&self) -> u64 {
        self.stop.get() - self.start
    }

    pub fn copy_between_start_and_stop(
        &self,
        chunk_data: &[u8],
        offset: u64,
        dst: &mut impl std::io::Write,
        dst_size: &mut usize,
    ) -> std::io::Result<()> {
        let start = (self.start - self.raw_offset) as usize;
        let stop = (self.stop.get() - self.raw_offset) as usize;
        let data_slice = &chunk_data[start..stop];
        let dst_index = (self.start - offset) as usize;
        // Fill-up with zeroes to reach dst_index
        if dst_index > *dst_size {
            dst.write_all(&vec![0; dst_index - *dst_size])?;
            *dst_size = dst_index;
        }
        // Write data and update destination size
        dst.write_all(data_slice)?;
        *dst_size += data_slice.len();
        Ok(())
    }

    pub fn from_block_access(block_access: BlockAccess) -> Self {
        Self {
            id: ChunkID::from(*block_access.id),
            raw_offset: block_access.offset,
            raw_size: block_access.size,
            start: block_access.offset,
            // TODO: what to do with overflow
            stop: (block_access.offset + block_access.size.get())
                .try_into()
                .expect(
                "Chunk view stop should be NonZeroU64 since bloc_access.size is already NonZeroU64",
            ),
            access: Some(block_access),
        }
    }

    pub fn promote_as_block(&mut self, data: &[u8]) -> Result<(), ChunkViewPromoteAsBlockError> {
        // No-op
        if self.is_block() {
            return Err(ChunkViewPromoteAsBlockError::AlreadyPromotedAsBlock);
        }

        // Check alignement
        if !self.is_aligned_with_raw_data() {
            return Err(ChunkViewPromoteAsBlockError::NotAligned);
        }

        // Craft access
        self.access = Some(BlockAccess {
            id: BlockID::from(*self.id),
            offset: self.start,
            size: self.size().try_into().expect("size must be > 0"),
            digest: HashDigest::from_data(data),
        });

        Ok(())
    }

    fn block(&self) -> Option<&BlockAccess> {
        // Requires an access
        if let Some(access) = &self.access {
            // Correctly aligned
            if self.is_aligned_with_raw_data()
            // Offset inconsistent
            && self.raw_offset == access.offset
            // Size inconsistent
                && self.raw_size == access.size
            {
                return Some(access);
            }
        }
        None
    }

    pub fn is_block(&self) -> bool {
        self.block().is_some()
    }

    /// A chunk view needs to be aligned with its raw data in order to form a valid
    /// block that can be uploaded.
    ///
    /// For instance let's consider a file being modified by the user:
    ///
    /// 1. The user creates a new file with a blocksize of 512 bytes.
    /// 2. The user writes 400 bytes of data.
    /// 3. The user writes 200 bytes at the offset 100.
    ///
    /// Up until step 3, the local file manifest contains a single chunk view of 400
    /// bytes that is properly aligned with its raw data (i.e. if we sync the file
    /// right now we will end up with a single block taken verbatim from this chunk).
    ///
    /// Then after step 3, the local file manifest contains 3 chunks:
    /// a. A chunk view of 100 bytes at offset 0
    /// b. A chunk view of 200 bytes at offset 100
    /// c. A chunk view of 100 bytes at offset 300
    ///
    ///         0       100      200      300      400       512
    /// Step 2  |xxxxxxxx|xxxxxxxx|xxxxxxxx|xxxxxxxx|---------|
    ///         |-------single chunk view-----------|---------|
    ///
    /// Step 3  |xxxxxxxx|xxxxxxxx|xxxxxxxx|xxxxxxxx|---------|
    ///         |chunk a-|                 |-chunk c|---------|
    ///                  |oooooooo|oooooooo|
    ///                  |--chunk view b---|
    ///
    /// Chunk views a and c have the same chunk ID as they are two windows over different
    /// parts of the same piece of data that was locally saved at step 2.
    /// Hence those two chunks are not properly aligned: a reshape is required to
    /// flatten the data written at step 2 and 3 to obtain a piece of data
    /// that can be uploaded as a proper block, i.e:
    /// d. A chunk view of 400 bytes at offset 0
    ///
    /// Also note that blocksize doesn't play a role here: chunk view d is a considered
    /// aligned with its raw data even if it is not aligned on blocksize (being smaller
    /// than the blocksize). This is because:
    /// - The last block in a file is allowed to be smaller than the blocksize.
    /// - The alignment on the blocksize is enforced at the `LocalFileManifest`
    ///   level (chunks are stored grouped to form a block slot).
    pub fn is_aligned_with_raw_data(&self) -> bool {
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

    pub fn get_block_access(&self) -> Result<&BlockAccess, ChunkViewGetBlockAccessError> {
        self.block()
            .ok_or(ChunkViewGetBlockAccessError::NotPromotedAsBlock)
    }

    #[allow(clippy::nonminimal_bool)]
    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        // As explained above, the following rule applies:
        //   raw_offset <= start < stop <= raw_offset + raw_size
        if !(self.raw_offset <= self.start) {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "raw_offset <= start",
            });
        }
        if !(self.start < self.stop.get()) {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "start < stop",
            });
        }
        if !(self.stop.get() <= self.raw_offset + self.raw_size.get()) {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "stop <= raw_offset + raw_size",
            });
        }
        Ok(())
    }
}

/*
 * LocalFileManifest
 */

/// The `LocalFileManifest` represents a file in the client.
///
/// Unlike `FileManifest`, it is designed to be modified as changes
/// occur locally. It is also stored serialized on the local storage.
///
/// It can be merged with a `FileManifest` if only the parent field differs (i.e.
/// the file has been moved to another folder). However if the file contents differ,
/// then we get a merge conflict that leads to the creation of a copy of the
/// file containing the local modifications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFileManifestData", try_from = "LocalFileManifestData")]
pub struct LocalFileManifest {
    pub base: FileManifest,
    pub parent: VlobID,
    pub need_sync: bool,
    pub updated: DateTime,
    pub size: u64,
    pub blocksize: Blocksize,
    /// This field is named after the `FileManifest::blocks` field, however it
    /// works quiet differently !
    ///
    /// You should see this field as a list of *block slots*, i.e. a list of
    /// arenas of `blocksize` bytes (except the very last that can be smaller).
    ///
    /// Each *block slot* itself contains a list of chunks that represent the
    /// actual data present.
    ///
    /// To be able to be synchronized, each block slot must be reshaped. This
    /// process flatten the chunks into a single one (hence a `LocalFileManifest`
    /// created from a `FileManifest` should only contains block slots made of
    /// a single chunk).
    pub blocks: Vec<Vec<ChunkView>>,
}

parsec_data!("schema/local_manifest/local_file_manifest.json5");

impl TryFrom<LocalFileManifestData> for LocalFileManifest {
    type Error = &'static str;
    fn try_from(data: LocalFileManifestData) -> Result<Self, Self::Error> {
        Ok(Self {
            base: data.base,
            parent: data.parent,
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
            parent: obj.parent,
            base: obj.base,
            need_sync: obj.need_sync,
            updated: obj.updated,
            size: obj.size,
            blocksize: obj.blocksize.into(),
            blocks: obj.blocks,
        }
    }
}

impl_local_manifest_dump!(LocalFileManifest);

impl LocalFileManifest {
    pub fn new(author: DeviceID, parent: VlobID, timestamp: DateTime) -> Self {
        Self {
            base: FileManifest {
                author,
                timestamp,
                id: VlobID::default(),
                parent,
                version: 0,
                created: timestamp,
                updated: timestamp,
                blocksize: DEFAULT_BLOCK_SIZE,
                size: 0,
                blocks: vec![],
            },
            parent,
            need_sync: true,
            updated: timestamp,
            blocksize: DEFAULT_BLOCK_SIZE,
            size: 0,
            blocks: vec![],
        }
    }

    pub fn get_chunks(&self, block: usize) -> Option<&Vec<ChunkView>> {
        self.blocks.get(block)
    }

    pub fn is_reshaped(&self) -> bool {
        for chunks in self.blocks.iter() {
            if chunks.is_empty() {
                continue;
            }
            if chunks.len() > 1 || !chunks[0].is_block() {
                return false;
            }
        }
        true
    }

    /// The chunks in a local file manifest should:
    /// - belong to their corresponding block span
    /// - not overlap
    /// - not go passed the file size
    /// - not share the same block span
    /// - not span over multiple block spans
    /// - be internally consistent
    ///
    /// Also the last block span should not be empty.
    /// Note that they do not have to be contiguous.
    /// Those checks have to remain compatible with `FileManifest::check_data_integrity`.
    /// Also, the id and parent id should be different so the manifest does not point to itself.
    ///
    /// A note about this method being public:
    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        // Check that id and parent are different
        if self.base.id == self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are different",
            });
        }

        let mut current = 0;

        // Loop over block spans
        for (i, chunks) in self.blocks.iter().enumerate() {
            let block_span_start = i as u64 * *self.blocksize;
            let block_span_stop = block_span_start + *self.blocksize;

            for chunk_view in chunks {
                // Check that the chunk view is internally consistent
                chunk_view.check_data_integrity()?;

                // Check that the chunk view belong to the block span
                if chunk_view.start < block_span_start || chunk_view.stop.get() > block_span_stop {
                    return Err(DataError::DataIntegrity {
                        data_type: std::any::type_name::<Self>(),
                        invariant: "Chunk view belong to the block span",
                    });
                }

                // Check that the chunks are ordered and do not overlap
                if current > chunk_view.start {
                    return Err(DataError::DataIntegrity {
                        data_type: std::any::type_name::<Self>(),
                        invariant: "Chunk views are ordered and do not overlap",
                    });
                }
                current = chunk_view.stop.get();
            }
        }

        // Check that the last block span is not empty
        if let Some(chunks) = self.blocks.last() {
            assert!(!chunks.is_empty())
        }

        // Check that the file size is consistent with the last chunk
        if current > self.size {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "File size is consistent with the last chunk view",
            });
        }

        Ok(())
    }

    pub fn from_remote(remote: FileManifest) -> Self {
        let chunk_views: Vec<ChunkView> = remote
            .blocks
            .iter()
            .map(|access| ChunkView::from_block_access(access.to_owned()))
            .collect();

        let mut blocks = vec![];
        for chunk_view in chunk_views {
            let block = (chunk_view.start / *remote.blocksize) as usize;
            while blocks.len() <= block {
                blocks.push(vec![]);
            }
            blocks[block].push(chunk_view);
        }

        let manifest = Self {
            parent: remote.parent,
            need_sync: false,
            updated: remote.updated,
            size: remote.size,
            blocksize: remote.blocksize,
            blocks,
            base: remote,
        };

        // Remote manifests comes from the certificate ops, so they are expected
        // to be validated using `CertifOps::validate_child_manifest`.
        // However, we still check the content integrity of the local manifest just in case.
        manifest.check_data_integrity().expect("Manifest integrity");
        manifest
    }

    pub fn to_remote(
        &self,
        author: DeviceID,
        timestamp: DateTime,
    ) -> Result<FileManifest, LocalFileManifestToRemoteError> {
        // Sanity check: make sure we don't upload an invalid manifest
        self.check_data_integrity()
            .expect("Local file manifest content integrity");

        let blocks = self
            .blocks
            .iter()
            // In a local manifest, each blocksize area is represented by a list of chunks.
            // That list might be empty if it doesn't contain any data (e.g when the file has been resized)
            // Since remote manifests is composed of a flat list of ordered and reshaped blocks,
            // empty blocks (i.e lists containing no chunks) are simply filtered out.
            .filter(|chunks| !chunks.is_empty())
            // Each blocksize area is expected to contain a single chunk view, reshaped as an uploadable block
            // (i.e a chunk view with an access). If not, the `NotReshaped` error is returned.
            .map(|chunks| match chunks.len() {
                0 => unreachable!(),
                1 => chunks[0]
                    .get_block_access()
                    .map_err(|_| LocalFileManifestToRemoteError::NeedReshape),
                _ => Err(LocalFileManifestToRemoteError::NeedReshape),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .cloned()
            .collect();

        let manifest = FileManifest::new(
            author,
            timestamp,
            self.base.id,
            self.parent,
            self.base.version + 1,
            self.base.created,
            self.updated,
            self.size,
            self.blocksize,
            blocks,
        );

        Ok(manifest)
    }

    pub fn set_single_block(
        &mut self,
        block: u64,
        new_chunk_view: ChunkView,
    ) -> Result<Vec<ChunkView>, u64> {
        let slice = self.blocks.get_mut(block as usize).ok_or(block)?;
        Ok(std::mem::replace(slice, vec![new_chunk_view]))
    }
}

/*
 * LocalFolderManifest
 */

/// The `LocalFolderManifest` represents a folder in the client.
///
/// Unlike `FolderManifest`, it is designed to be modified as changes
/// occur locally. It is also stored serialized on the local storage.
///
/// It can always be merged with a `FolderManifest` without conflict (CRDT ftw!).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "LocalFolderManifestData", from = "LocalFolderManifestData")]
pub struct LocalFolderManifest {
    pub base: FolderManifest,
    pub parent: VlobID,
    pub need_sync: bool,
    pub updated: DateTime,
    pub children: HashMap<EntryName, VlobID>,
    // Confined entries are entries that are meant to stay locally and not be added
    // to the uploaded remote manifest when synchronizing. The criteria for being
    // confined is to have a filename that matched the "prevent sync" pattern at the time of
    // the last change (or when a new filter was successfully applied)
    pub local_confinement_points: HashSet<VlobID>,
    // Filtered entries are entries present in the base manifest that are not exposed
    // locally. We keep track of them to remember that those entries have not been
    // deleted locally and hence should be restored when crafting the remote manifest
    // to upload.
    pub remote_confinement_points: HashSet<VlobID>,
    // Speculative placeholders are created when we want to access a workspace
    // but didn't retrieve manifest data from server yet. This implies:
    // - only the root folder can be speculative
    // - non-placeholders cannot be speculative
    // - the only non-speculative placeholder is the placeholder initialized
    //   during the initial workspace creation
    // This speculative information is useful during merge to understand if
    // a data is not present in the placeholder compared with a remote because:
    // a) the data is not locally known (speculative is True)
    // b) the data is known, but has been locally removed (speculative is False)
    pub speculative: bool,
}

parsec_data!("schema/local_manifest/local_folder_manifest.json5");

impl_transparent_data_format_conversion!(
    LocalFolderManifest,
    LocalFolderManifestData,
    base,
    parent,
    need_sync,
    updated,
    children,
    local_confinement_points,
    remote_confinement_points,
    speculative,
);

impl_local_manifest_dump!(LocalFolderManifest);
impl_local_manifest_load!(LocalFolderManifest);

impl LocalFolderManifest {
    pub fn new(author: DeviceID, parent: VlobID, timestamp: DateTime) -> Self {
        Self {
            base: FolderManifest {
                author,
                timestamp,
                id: VlobID::default(),
                parent,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            parent,
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative: false,
        }
    }

    /// A note about this method being public:
    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    ///
    /// Note that this method does not perform data integrity check related to the manifest being a
    /// child or root manifest.
    pub fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    fn check_data_integrity_as_child(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are different
        if self.base.id == self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are different for child manifest",
            });
        }
        Ok(())
    }

    fn check_data_integrity_as_root(&self) -> DataResult<()> {
        self.check_data_integrity()?;

        // Check that id and parent are the same
        if self.base.id != self.parent {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "id and parent are the same for root manifest",
            });
        }
        Ok(())
    }

    /// Root folder manifest (aka "workspace manifest" for historical reasons) is a special
    /// folder manifest which ID the same as the realm ID.
    /// It is the only folder manifest that can be speculative.
    pub fn new_root(
        author: DeviceID,
        realm: VlobID,
        timestamp: DateTime,
        speculative: bool,
    ) -> Self {
        Self {
            base: FolderManifest {
                author,
                timestamp,
                id: realm,
                parent: realm,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            parent: realm,
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative,
        }
    }

    /// Note the manifest will be marked as updated (i.e. `need_sync` set to `true`
    /// and `updated` field set to `timestamp`) only if the changes concern non
    /// confined entries.
    pub fn evolve_children_and_mark_updated(
        mut self,
        data: HashMap<EntryName, Option<VlobID>>,
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

    /// Clear the local confinement points, and remove from the local children any
    /// entry that was previously confined.
    ///
    /// For example, considering a manifest with:
    /// - `local_confinement_points`: [1, 3]
    /// - `children`: `{"a.tmp": 1, "b.txt": 2, "c.tmp": 3, "d.txt": 4}`
    ///
    /// Then the resulting manifest would have an empty `local_confinement points` and
    /// `{"b.txt": 2, "d.txt": 4}` as `children`.
    fn filter_local_confinement_points(mut self) -> Self {
        if self.local_confinement_points.is_empty() {
            return self;
        }

        self.children
            .retain(|_, entry_id| !self.local_confinement_points.contains(entry_id));

        self.local_confinement_points.clear();
        self
    }

    /// Restore in the current manifest all the entries that:
    /// - Were confined in `other` (according to `other`'s `local_confinement_points`).
    /// - Were in `other` but are now part of our remote confinement points.
    ///
    /// On top of that this method also apply `prevent_sync_pattern` on the restored
    /// entries.
    ///
    /// This method can be seen as the opposite of `filter_local_confinement_points`.
    ///
    /// Notes:
    /// - This method must be called only after `filter_remote_entries`, otherwise
    ///   our `remote_confinement_points` field may be out-of-sync with
    ///   `prevent_sync_pattern` and lead to inconsistent results.
    /// - `other` doesn't need to be up to date with `prevent_sync_pattern` (i.e.
    ///   `apply_prevent_sync_pattern()` may have been called on it with a different
    ///   prevent sync pattern).
    /// - `timestamp` is only used to update the `updated` field if a previously
    ///   confined entry is now not confined (and hence should now be synchronized !).
    ///
    ///
    /// For example, considering `.tmp` as prevent sync pattern and a manifest with:
    /// - `remote_confinement_points`: [1, 2]
    /// - `children`: `{"3.txt": 3}`
    /// - `base.children`: `{"1.tmp": 1, "2.tmp": 2, "3.txt": 3}`
    ///
    /// And an `other` manifest with:
    /// - `local_confinement_points`: [4]
    /// - `children`: `{"3.txt": 3, "2-renamed.txt": 2, "a.tmp": 4, "b.txt": 5}`
    ///
    /// Then the resulting manifest would have:
    /// - `base.children`: `{"1.tmp": 1, "2.tmp": 2, "3.txt": 3}` (no changes)
    /// - `remote_confinement_points`: [1, 2] (no changes)
    /// - `local_confinement_points`: []
    /// - `children`: `{"3.txt": 3, "2-renamed.txt": 2, "a.tmp": 4}`
    ///
    /// Note that `"b.txt": 5` is not restored as it is a local change that has nothing
    /// to do with confinement. Hence the output of this method is not to be considered
    /// as a merge with `other` (for that see `libparsec_client`'s `merge_local_folder_manifest`).
    fn restore_local_confinement_points(
        self,
        other: &Self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        // Using `self.remote_confinement_points` is useful to restore entries that were present locally
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

    /// Apply the prevent sync pattern *on the local children* (i.e. not on
    /// `base.children` as one could expect) to remove from it all the confined
    /// entries and collect them into `remote_confinement_points`.
    ///
    /// This method is expected to be run on a local manifest where it's local children
    /// are currently a mere copy of it remote children (i.e. `base.children`).
    /// This occurs in two places:
    /// - When creating a new local manifest from a remote manifest.
    /// - When applying a new prevent sync pattern, in which case `filter_local_confinement_points`
    ///   and `restore_remote_confinement_points` has just been previously called to revert
    ///   the local children according to the local&remote confinement points.
    ///
    /// Once in this state, it is the method's goal to filter out the entries that
    /// should be remotely confined (i.e. entries that should be kept in `base.children`,
    /// but not appear in local children).
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

    /// Clear the `remote_confinement_points` and restore from `base.children` any
    /// entry that was previously confined in the local children.
    ///
    /// This method can be seen as the opposite of `filter_remote_entries`.
    ///
    /// For example, considering a manifest with:
    /// - `remote_confinement_points`: [1]
    /// - `children`: `{"b.txt": 2}`
    /// - `base.children`: `{"a.tmp": 1, "c.tmp": 3}`
    ///
    /// Then the resulting manifest would have an empty `remote_confinement points` and
    /// `{"a.tmp": 1, "b.txt": 2}` as `children`.
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
            .filter_local_confinement_points()
            // At this point, `result.children` no longer contains previous local confined entries
            .restore_remote_confinement_points()
            // At this point, `result.children` contains previous remote confined entries
            .filter_remote_entries(prevent_sync_pattern)
            // At this point, `result.children` no longer contains new remote confined entries
            // and `remote_confinement_points` has reached it final value.
            .restore_local_confinement_points(self, prevent_sync_pattern, timestamp)
        // At this point, `result.children` contains the local confined entries,
        // and `local_confinement_points` has reached it final value.
    }

    pub fn from_remote(remote: FolderManifest, prevent_sync_pattern: &Regex) -> Self {
        Self {
            parent: remote.parent,
            need_sync: false,
            updated: remote.updated,
            children: remote.children.clone(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative: false,
            base: remote,
        }
        .filter_remote_entries(prevent_sync_pattern)
    }

    /// Create a `LocalFolderManifest` from the provided `FolderManifest`, then
    /// apply to the new local folder manifest the entry that were locally confined
    /// in the provided `local_manifest`.
    ///
    /// The result shouldn't be considered a merge of remote and local, but an intermediate
    /// representation required to do a proper merge.
    ///
    /// Also note `local_manifest` doesn't need to have it confinement points up-to-date
    /// with the provided `prevent_sync_pattern`. As a matter of fact, the `timestamp`
    /// parameter is only used to update
    pub fn from_remote_with_restored_local_confinement_points(
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
            .filter_local_confinement_points()
            .restore_remote_confinement_points();
        // Create remote manifest
        FolderManifest {
            author,
            timestamp,
            id: result.base.id,
            version: result.base.version + 1,
            created: result.base.created,
            parent: result.parent,
            updated: result.updated,
            children: result.children,
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
    pub updated: DateTime,
    /// This field is used to store the name of the realm:
    /// - When the realm got created, its name is stored here until the initial
    ///   `RealmNameCertificate` is uploaded (which can take time, e.g. if the
    ///   client is offline).
    /// - After that, to access the workspace name even when the client is offline (given
    ///   `RealmNameCertificate` contains the name encrypted, but the decryption key
    ///   must be fetched by `realm_get_keys_bundle` (which cannot be done while offline).
    pub local_workspaces: Vec<LocalUserManifestWorkspaceEntry>,
    /// Speculative placeholders are created when we want to access the
    /// user manifest but didn't retrieve it from server yet. This implies:
    /// - non-placeholders cannot be speculative
    /// - the only non-speculative placeholder is the placeholder initialized
    ///   during the initial user claim (by opposition of subsequent device
    ///   claims on the same user)
    ///
    /// This speculative information is useful during merge to understand if
    /// a data is not present in the placeholder compared with a remote because:
    /// a) the data is not locally known (speculative is True)
    /// b) the data is known, but has been locally removed (speculative is False)
    pub speculative: bool,
}

impl_local_manifest_dump!(LocalUserManifest);
impl_local_manifest_load!(LocalUserManifest);

parsec_data!("schema/local_manifest/local_user_manifest.json5");

impl_transparent_data_format_conversion!(
    LocalUserManifest,
    LocalUserManifestData,
    base,
    need_sync,
    updated,
    local_workspaces,
    speculative,
);

impl LocalUserManifest {
    pub fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: Option<VlobID>,
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
            },
            need_sync: true,
            updated: timestamp,
            local_workspaces: vec![],
            speculative,
        }
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn get_local_workspace_entry(
        &self,
        realm_id: VlobID,
    ) -> Option<&LocalUserManifestWorkspaceEntry> {
        self.local_workspaces.iter().find(|w| w.id == realm_id)
    }

    pub fn from_remote(remote: UserManifest) -> Self {
        Self {
            need_sync: false,
            updated: remote.updated,
            local_workspaces: vec![],
            speculative: false,
            base: remote,
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
        }
    }
}

/*
 * LocalChildManifest
 */

#[derive(Debug, Clone)]
pub enum ArcLocalChildManifest {
    File(Arc<LocalFileManifest>),
    Folder(Arc<LocalFolderManifest>),
}

impl ArcLocalChildManifest {
    pub fn id(&self) -> VlobID {
        match self {
            ArcLocalChildManifest::File(m) => m.base.id,
            ArcLocalChildManifest::Folder(m) => m.base.id,
        }
    }

    pub fn parent(&self) -> VlobID {
        match self {
            ArcLocalChildManifest::File(m) => m.parent,
            ArcLocalChildManifest::Folder(m) => m.parent,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum LocalChildManifest {
    File(LocalFileManifest),
    Folder(LocalFolderManifest),
}

impl_local_manifest_load!(LocalChildManifest);

impl LocalChildManifest {
    pub fn id(&self) -> VlobID {
        match self {
            Self::File(manifest) => manifest.base.id,
            Self::Folder(manifest) => manifest.base.id,
        }
    }

    pub fn need_sync(&self) -> bool {
        match self {
            Self::File(manifest) => manifest.need_sync,
            Self::Folder(manifest) => manifest.need_sync,
        }
    }

    pub fn base_version(&self) -> u32 {
        match self {
            Self::File(manifest) => manifest.base.version,
            Self::Folder(manifest) => manifest.base.version,
        }
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    pub fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            Self::File(manifest) => manifest.check_data_integrity()?,
            Self::Folder(manifest) => manifest.check_data_integrity_as_child()?,
        }
        Ok(())
    }
}

impl From<LocalFileManifest> for LocalChildManifest {
    fn from(value: LocalFileManifest) -> Self {
        Self::File(value)
    }
}

impl From<LocalFolderManifest> for LocalChildManifest {
    fn from(value: LocalFolderManifest) -> Self {
        Self::Folder(value)
    }
}

impl TryFrom<LocalChildManifest> for LocalFileManifest {
    type Error = ();

    fn try_from(value: LocalChildManifest) -> Result<Self, Self::Error> {
        match value {
            LocalChildManifest::File(manifest) => Ok(manifest),
            _ => Err(()),
        }
    }
}

impl TryFrom<LocalChildManifest> for LocalFolderManifest {
    type Error = ();

    fn try_from(value: LocalChildManifest) -> Result<Self, Self::Error> {
        match value {
            LocalChildManifest::Folder(manifest) => Ok(manifest),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocalWorkspaceManifest(pub LocalFolderManifest);

impl_local_manifest_load!(LocalWorkspaceManifest);

impl LocalWorkspaceManifest {
    pub fn new(author: DeviceID, realm: VlobID, timestamp: DateTime, speculative: bool) -> Self {
        Self(LocalFolderManifest::new_root(
            author,
            realm,
            timestamp,
            speculative,
        ))
    }

    /// This structure represents mutable data (it gets loaded from disk, updated, then stored back modified)
    /// Hence this `check_data_integrity`'s main goal is during deserialization.
    /// However it is also useful as sanity check:
    /// - Right before serialization
    /// - After any modification (hence the need for this method to be public)
    fn check_data_integrity(&self) -> DataResult<()> {
        self.0.check_data_integrity_as_root()
    }
}

impl From<LocalFolderManifest> for LocalWorkspaceManifest {
    fn from(value: LocalFolderManifest) -> Self {
        Self(value)
    }
}

impl From<LocalWorkspaceManifest> for LocalFolderManifest {
    fn from(value: LocalWorkspaceManifest) -> Self {
        value.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[path = "../tests/unit/local_manifest.rs"]
mod tests;
