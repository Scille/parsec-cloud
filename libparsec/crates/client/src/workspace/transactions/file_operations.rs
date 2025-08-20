// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::NonZeroU64;

use libparsec_types::prelude::*;

pub(crate) struct WriteOperation {
    pub chunk_view: ChunkView,
    pub offset: i64,
}

// Prepare read

fn block_read(
    chunk_views: &[ChunkView],
    size: u64,
    start: u64,
) -> impl Iterator<Item = ChunkView> + '_ {
    let stop = start + size;

    // Bisect
    let start_index = match chunk_views.binary_search_by_key(&start, |chunk_view| chunk_view.start)
    {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index.saturating_sub(1),
    };
    let stop_index = match chunk_views.binary_search_by_key(&stop, |chunk_view| chunk_view.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index,
    };

    // Loop over chunks
    chunk_views
        .get(start_index..stop_index)
        .unwrap_or_default()
        .iter()
        .filter(move |chunk_view| chunk_view.start < stop && chunk_view.stop.get() > start)
        .map(move |chunk_view| {
            let mut new_chunk_view = chunk_view.clone();
            new_chunk_view.start = max(chunk_view.start, start);
            new_chunk_view.stop = min(
                chunk_view.stop,
                NonZeroU64::new(stop)
                    .expect("The stop offset can only be 0 if the index range is empty"),
            );
            new_chunk_view
        })
}

/// Prepare a read operation on a provided manifest.
///
/// Return a contiguous iterator of chunks that must be read and concatenated.
pub fn prepare_read(
    manifest: &LocalFileManifest,
    size: u64,
    offset: u64,
) -> (u64, impl Iterator<Item = ChunkView> + '_) {
    // Sanitize size and offset to fit the manifest
    let offset = min(offset, manifest.size);
    let size = min(
        size,
        manifest
            .size
            .checked_sub(offset)
            .expect("The offset computed above cannot be greater than the manifest size"),
    );

    // Find proper block indexes
    let blocksize = u64::from(manifest.blocksize);
    let start_block = offset / blocksize;
    let stop_block = (offset + size).div_ceil(blocksize);

    // Loop over blocks
    let chunks = (start_block..stop_block)
        .filter_map(move |block| {
            // Get sub_start, sub_stop and sub_size
            let block_start = block * blocksize;
            let sub_start = max(offset, block_start);
            let sub_stop = min(offset + size, block_start + blocksize);
            let sub_size = sub_stop
                .checked_sub(sub_start)
                .expect("Sub-stop is always greater than sub-start");
            // Get the corresponding chunks
            manifest
                .get_chunks(block as usize)
                .filter(|block_chunks| !block_chunks.is_empty())
                .map(|block_chunks| block_read(block_chunks, sub_size, sub_start))
        })
        .flatten();

    (size, chunks)
}

// Prepare write

fn block_write(
    chunk_views: &[ChunkView],
    size: u64,
    start: u64,
    new_chunk_view: ChunkView,
) -> (Vec<ChunkView>, HashSet<ChunkID>) {
    let stop = start + size;

    // Edge case
    if chunk_views.is_empty() {
        return (vec![new_chunk_view], HashSet::new());
    }

    // Bisect
    let start_index = match chunk_views.binary_search_by_key(&start, |chunk_view| chunk_view.start)
    {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index.saturating_sub(1),
    };
    let stop_index = match chunk_views.binary_search_by_key(&stop, |chunk_view| chunk_view.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index,
    };

    // Removed ids
    let mut removed_ids: HashSet<ChunkID> = chunk_views
        .get(start_index..stop_index)
        .unwrap_or_default()
        .iter()
        .map(|x| x.id)
        .collect();

    // Chunks before start chunk
    let mut new_chunk_views: Vec<ChunkView> = chunk_views
        .get(0..start_index)
        .unwrap_or_default()
        .iter()
        .inspect(|chunk_view| {
            // The same ID might appear in multiple chunks,
            // so it's crucial that we make sure to not remove an ID
            // that ends up being part of the new manifest
            removed_ids.remove(&chunk_view.id);
        })
        .cloned()
        .collect();

    // Test start chunk
    let start_chunk_view = chunk_views
        .get(start_index)
        .expect("Indexes are found using binary search and hence always valid");
    if start_chunk_view.start < start {
        let mut new_start_chunk_view = start_chunk_view.clone();
        new_start_chunk_view.stop = start_chunk_view.stop.min(
            NonZeroU64::new(start)
                .expect("Cannot be zero since it's strictly greater than start_chunk_view.start"),
        );
        new_chunk_views.push(new_start_chunk_view);
        removed_ids.remove(&start_chunk_view.id);
    }

    // Add new buffer
    new_chunk_views.push(new_chunk_view);

    // Test stop_chunk
    if stop_index > 0 {
        let stop_chunk_view = chunk_views
            .get(stop_index - 1)
            .expect("Indexes are found using binary search and hence always valid");
        if stop_chunk_view.stop.get() > stop {
            let mut new_stop_chunk_view = stop_chunk_view.clone();
            new_stop_chunk_view.start = stop;
            new_chunk_views.push(new_stop_chunk_view);
            removed_ids.remove(&stop_chunk_view.id);
        }
    }

    // Chunks after start chunk
    new_chunk_views.extend(
        chunk_views
            .get(stop_index..)
            .unwrap_or_default()
            .iter()
            .inspect(|chunk_view| {
                // The same ID might appear in multiple chunks,
                // so it's crucial that we make sure to not remove an ID
                // that ends up being part of the new manifest
                removed_ids.remove(&chunk_view.id);
            })
            .cloned(),
    );

    (new_chunk_views, removed_ids)
}

/// Prepare a write operation by updating the provided manifest.
///
/// Return a `Vec` of write operations that must be performed in order for the updated
/// manifest to become valid.
/// Each write operation consists of a new chunk to store, along with an offset to apply
/// to the corresponding raw data.
/// Note that the raw data also needs to be sliced to the chunk view size and padded with
/// null bytes if necessary.
/// Also return a `HashSet` of chunk IDs that must cleaned up from the storage, after
/// the updated manifest has been successfully stored.
pub fn prepare_write(
    manifest: &mut LocalFileManifest,
    size: u64,
    offset: u64,
    timestamp: DateTime,
) -> (Vec<WriteOperation>, HashSet<ChunkID>) {
    let mut removed_ids = HashSet::new();
    let mut write_operations = vec![];

    // Writing zero bytes is a no-op (it does not extend the file if the offset is past the end of the file)
    if size == 0 {
        return (write_operations, removed_ids);
    }

    // Find proper block indexes
    let blocksize = u64::from(manifest.blocksize);
    let start_block = offset / blocksize;
    let stop_block = (offset + size).div_ceil(blocksize);

    // Loop over blocks
    for block in start_block..stop_block {
        // Get sub_start, sub_stop and sub_size
        let block_start = block * blocksize;
        let sub_start = max(offset, block_start);
        let sub_stop = min(offset + size, block_start + blocksize);
        let sub_size = sub_stop
            .checked_sub(sub_start)
            .expect("Sub-stop is always greater than sub-start");
        let content_offset = sub_start - offset;

        // Prepare new chunk
        let new_chunk_view = ChunkView::new(
            sub_start,
            NonZeroU64::new(sub_start + sub_size)
                .expect("sub-size is always strictly greater than zero"),
        );
        write_operations.push(WriteOperation {
            chunk_view: new_chunk_view.clone(),
            offset: content_offset as i64,
        });

        // Get the corresponding chunks
        let new_chunks = match manifest.get_chunks(block as usize) {
            Some(block_chunks) => {
                let (new_chunks, more_removed_ids) =
                    block_write(block_chunks, sub_size, sub_start, new_chunk_view);
                removed_ids.extend(more_removed_ids);
                new_chunks
            }
            None => vec![new_chunk_view],
        };

        // Update data structures
        while manifest.blocks.len() as u64 <= block {
            manifest.blocks.push(vec![]);
        }
        manifest.blocks[block as usize] = new_chunks;
    }

    // Evolve manifest
    let new_size = max(manifest.size, offset + size);

    // Update manifest
    manifest.need_sync = true;
    manifest.updated = timestamp;
    manifest.size = new_size;

    (write_operations, removed_ids)
}

// Prepare resize

fn prepare_truncate(
    manifest: &mut LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> HashSet<ChunkID> {
    // Check that there is something to truncate
    if size >= manifest.size {
        return HashSet::new();
    }

    // Find limit block
    let blocksize = u64::from(manifest.blocksize);
    let block = (size / blocksize) as usize;
    let remainder = size % blocksize;

    // Prepare removed ids
    let mut removed_ids: HashSet<ChunkID> = manifest
        .blocks
        .get(block..)
        .unwrap_or_default()
        .iter()
        .flatten()
        .map(|x| x.id)
        .collect();

    let empty = vec![];
    let chunks = manifest.get_chunks(block).unwrap_or(&empty);

    // Find the index of the first chunk view to exclude
    let chunk_index = match chunks.binary_search_by_key(&size, |chunk_view| chunk_view.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index,
    };

    if chunk_index == 0 {
        // No need to split the last block
        manifest.blocks.truncate(block);
        while manifest.blocks.last().is_some_and(|x| x.is_empty()) {
            manifest.blocks.pop();
        }
    } else {
        assert!(remainder != 0, "The remainder cannot be zero");

        // Find the index of the last chunk view to include
        let chunk_index = chunk_index - 1;

        // Create the new last chunk
        let last_chunk_view = chunks
            .get(chunk_index)
            .expect("The index is found using binary search and hence always valid");
        let mut new_chunk_view = last_chunk_view.clone();
        new_chunk_view.stop = new_chunk_view
            .stop
            .min(NonZeroU64::new(size).expect("Cannot be zero since the remainder is not zero"));

        // Create the new chunks for the last block
        let mut new_chunk_views = chunks.get(..chunk_index).unwrap_or_default().to_vec();
        new_chunk_views.push(new_chunk_view);

        // Those new chunks should not be removed
        for chunk_view in &new_chunk_views {
            removed_ids.remove(&chunk_view.id);
        }

        // Truncate and add the new chunks
        manifest.blocks.truncate(block);
        manifest.blocks.push(new_chunk_views);
    }

    // Update the manifest
    manifest.need_sync = true;
    manifest.size = size;
    manifest.updated = timestamp;

    removed_ids
}

/// Prepare a resize operation by updating the provided manifest.
///
/// Return a `HashSet` of chunk IDs that must cleaned up from the storage, after the
/// updated manifest has been successfully stored.
pub fn prepare_resize(
    manifest: &mut LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> HashSet<ChunkID> {
    if size >= manifest.size {
        // Extend
        manifest.need_sync = true;
        manifest.size = size;
        manifest.updated = timestamp;
        HashSet::new()
    } else {
        // Truncate
        prepare_truncate(manifest, size, timestamp)
    }
}

pub(crate) enum ReshapeBlockOperation<'a> {
    ToReshape {
        manifest_chunk_views: &'a mut Vec<ChunkView>,
        reshaped_chunk_view: ChunkView,
    },
    ToPromote {
        chunk_view: &'a mut ChunkView,
    },
}

impl ReshapeBlockOperation<'_> {
    pub fn try_reshape(chunk_views: &mut Vec<ChunkView>) -> Option<ReshapeBlockOperation<'_>> {
        // All zeroes or already a valid block
        if chunk_views.is_empty() || chunk_views.len() == 1 && chunk_views[0].is_block() {
            None
        // Already ready for block promotion, we can keep the chunk view as it is
        } else if chunk_views.len() == 1 && chunk_views[0].is_aligned_with_raw_data() {
            Some(ReshapeBlockOperation::ToPromote {
                chunk_view: &mut chunk_views[0],
            })
        // Reshape those chunks as a single block
        } else {
            let start = chunk_views[0].start;
            let stop = chunk_views.last().expect("A block cannot be empty").stop;
            let reshaped_chunk_view = ChunkView::new(start, stop);
            Some(ReshapeBlockOperation::ToReshape {
                manifest_chunk_views: chunk_views,
                reshaped_chunk_view,
            })
        }
    }
    pub fn destination(&self) -> &ChunkView {
        match self {
            ReshapeBlockOperation::ToReshape {
                reshaped_chunk_view,
                ..
            } => reshaped_chunk_view,
            ReshapeBlockOperation::ToPromote { chunk_view } => chunk_view,
        }
    }

    pub fn source(&self) -> &[ChunkView] {
        match self {
            ReshapeBlockOperation::ToReshape {
                manifest_chunk_views: manifest_chunks,
                ..
            } => manifest_chunks,
            ReshapeBlockOperation::ToPromote { chunk_view } => std::slice::from_ref(chunk_view),
        }
    }

    pub fn write_back(&self) -> bool {
        matches!(self, ReshapeBlockOperation::ToReshape { .. })
    }

    pub fn cleanup_ids(&self) -> HashSet<ChunkID> {
        match self {
            ReshapeBlockOperation::ToReshape {
                manifest_chunk_views,
                ..
            } => {
                // Remove duplicate IDs by returning a HashSet
                manifest_chunk_views
                    .iter()
                    .map(|chunk_view| chunk_view.id)
                    .collect()
            }
            ReshapeBlockOperation::ToPromote { .. } => HashSet::new(),
        }
    }

    pub fn commit(self, chunk_data: &[u8]) {
        match self {
            ReshapeBlockOperation::ToPromote { chunk_view } => {
                chunk_view
                    .promote_as_block(chunk_data)
                    .expect("chunk is block-compatible");
            }
            ReshapeBlockOperation::ToReshape {
                manifest_chunk_views,
                mut reshaped_chunk_view,
            } => {
                manifest_chunk_views.clear();
                reshaped_chunk_view
                    .promote_as_block(chunk_data)
                    .expect("chunk is block-compatible");
                manifest_chunk_views.push(reshaped_chunk_view);
            }
        }
    }
}

// Prepare reshape

/// Prepare a reshape operation without updating the provided manifest.
/// The reason why the manifest is not updated is because the hash of the
/// corresponding data is required to turn a chunk view into a block.
/// Instead, it's up to the caller to call `chunk_view.evolve_as_block` and
/// `manifest.set_single_block` to update the manifest.
///
/// Return an iterator where each item corresponds to a block to reshape.
/// Each item consists of:
/// - the index of the block that is being reshaped
/// - a source block, represented as a `Vec` of chunks
/// - a destination block, represented as a single chunk
/// - a write back boolean indicating the new chunk of data must be written
/// - a `HashSet` of chunk IDs that must cleaned up from the storage
pub fn prepare_reshape(
    manifest: &mut LocalFileManifest,
) -> impl Iterator<Item = ReshapeBlockOperation<'_>> + '_ {
    // Loop over blocks
    manifest
        .blocks
        .iter_mut()
        .filter_map(ReshapeBlockOperation::try_reshape)
}
