// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{cmp::Ordering, num::NonZeroU64};

use libparsec_crypto::HashDigest;
use libparsec_serialization_format::parsec_data;
use serde::{Deserialize, Serialize};

use crate::{
    self as libparsec_types, BlockAccess, BlockID, Blocksize, ChunkID, DataError, DataResult,
    DateTime, DeviceID, FileManifest, InvalidBlockSize, VlobID, DEFAULT_BLOCK_SIZE,
};

use super::impl_local_manifest_dump;

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
    type Error = InvalidBlockSize;
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

    /// Note `chunk_data` is expected to have a size compatible with the chunk view
    /// (typically when chunk view correspond to a remote block, it has been created from
    /// a block access that has been also used to validate the block data size).
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

        // Check alignment
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
