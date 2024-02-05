// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::NonZeroU64;

use libparsec_types::prelude::*;

pub(crate) struct WriteOperation {
    pub chunk: Chunk,
    pub offset: i64,
}

// Prepare read

fn block_read(chunks: &[Chunk], size: u64, start: u64) -> impl Iterator<Item = Chunk> + '_ {
    let stop = start + size;

    // Bisect
    let start_index = match chunks.binary_search_by_key(&start, |chunk| chunk.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index
            .checked_sub(1)
            .expect("First chunk always exists and start at 0"),
    };
    let stop_index = match chunks.binary_search_by_key(&stop, |chunk| chunk.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index,
    };

    // Loop over chunks
    chunks
        .get(start_index..stop_index)
        .unwrap_or_default()
        .iter()
        .map(move |chunk| {
            let mut new_chunk = chunk.clone();
            new_chunk.start = max(chunk.start, start);
            new_chunk.stop = min(
                chunk.stop,
                NonZeroU64::new(stop)
                    .expect("The stop offset can only be 0 if the index range is empty"),
            );
            new_chunk
        })
}

// TODO: replace Chunk in the return by a dedicated type

/// Prepare a read operation on a provided manifest.
///
/// Return a contiguous list of chunks that must be read and concatenated.
pub(crate) fn prepare_read(
    manifest: &LocalFileManifest,
    size: u64,
    offset: u64,
) -> impl Iterator<Item = Chunk> + '_ {
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
    let stop_block = (offset + size + blocksize - 1) / blocksize;

    // Loop over blocks
    (start_block..stop_block).flat_map(move |block| {
        // Get sub_start, sub_stop and sub_size
        let block_start = block * blocksize;
        let sub_start = max(offset, block_start);
        let sub_stop = min(offset + size, block_start + blocksize);
        let sub_size = sub_stop
            .checked_sub(sub_start)
            .expect("Sub-stop is always greater than sub-start");
        // Get the corresponding chunks
        let block_chunks = manifest
            .get_chunks(block as usize)
            // TODO: handle invalid manifest !
            .expect("A valid manifest must have enough blocks to cover its full range.");
        block_read(block_chunks, sub_size, sub_start)
    })
}

// Prepare write

// TODO: replace the HashSet by a simpler Vec
fn block_write(
    chunks: &[Chunk],
    size: u64,
    start: u64,
    new_chunk: Chunk,
) -> (Vec<Chunk>, HashSet<ChunkID>) {
    let stop = start + size;

    // Edge case
    if chunks.is_empty() {
        return (vec![new_chunk], HashSet::new());
    }

    // Bisect
    let start_index = match chunks.binary_search_by_key(&start, |chunk| chunk.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index
            .checked_sub(1)
            .expect("First chunk always exists and start at 0"),
    };
    let stop_index = match chunks.binary_search_by_key(&stop, |chunk| chunk.start) {
        Ok(found_index) => found_index,
        Err(insert_index) => insert_index,
    };

    // Removed ids
    let mut removed_ids: HashSet<ChunkID> = chunks
        .get(start_index..stop_index)
        .unwrap_or_default()
        .iter()
        .map(|x| x.id)
        .collect();

    // Chunks before start chunk
    let mut new_chunks: Vec<Chunk> = chunks
        .get(0..start_index)
        .unwrap_or_default()
        .iter()
        .map(|chunk| {
            // The same ID might appear in multiple chunks,
            // so it's crucial that we make sure to not remove an ID
            // that ends up being part of the new manifest
            removed_ids.remove(&chunk.id);
            chunk
        })
        .cloned()
        .collect();

    // Test start chunk
    let start_chunk = chunks
        .get(start_index)
        .expect("Indexes are found using binary search and hence always valid");
    if start_chunk.start < start {
        let mut new_start_chunk = start_chunk.clone();
        new_start_chunk.stop = NonZeroU64::new(start)
            .expect("Cannot be zero since it's strictly greater than start_chunk.start");
        new_chunks.push(new_start_chunk);
        removed_ids.remove(&start_chunk.id);
    }

    // Add new buffer
    new_chunks.push(new_chunk);

    // Test stop_chunk
    let stop_chunk = chunks
        .get(stop_index - 1)
        .expect("Indexes are found using binary search and hence always valid");
    if stop_chunk.stop.get() > stop {
        let mut new_stop_chunk = stop_chunk.clone();
        new_stop_chunk.start = stop;
        new_chunks.push(new_stop_chunk);
        removed_ids.remove(&stop_chunk.id);
    }

    // Chunks after start chunk
    new_chunks.extend(
        chunks
            .get(stop_index..)
            .unwrap_or_default()
            .iter()
            .map(|chunk| {
                // The same ID might appear in multiple chunks,
                // so it's crucial that we make sure to not remove an ID
                // that ends up being part of the new manifest
                removed_ids.remove(&chunk.id);
                chunk
            })
            .cloned(),
    );

    (new_chunks, removed_ids)
}

/// Prepare a write operation by updating the provided manifest.
///
/// Return a list of write operations that must be performed in order for the updated manifest to become valid.
/// Each write operation consists of a new chunk to store, along with an offset to apply to the corresponding raw data.
/// Note that the raw data also needs to be sliced to the chunk size and padded with null bytes if necessary.
/// Also return a list of chunk IDs that must cleaned up from the storage, after the updated manifest has been successfully stored.
pub(crate) fn prepare_write(
    manifest: &mut LocalFileManifest,
    mut size: u64,
    mut offset: u64,
    timestamp: DateTime,
) -> (Vec<WriteOperation>, Vec<ChunkID>) {
    let mut padding = 0;
    let mut removed_ids = vec![];
    let mut write_operations = vec![];

    // Padding
    if offset > manifest.size {
        padding = offset - manifest.size;
        size += padding;
        offset = manifest.size;
    }

    // Find proper block indexes
    let blocksize = u64::from(manifest.blocksize);
    let start_block = offset / blocksize;
    let stop_block = (offset + size + blocksize - 1) / blocksize;

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
        let new_chunk = Chunk::new(
            sub_start,
            NonZeroU64::new(sub_start + sub_size)
                .expect("sub-size is always strictly greater than zero"),
        );
        write_operations.push(WriteOperation {
            chunk: new_chunk.clone(),
            offset: content_offset as i64 - padding as i64,
        });

        // Get the corresponding chunks
        let new_chunks = match manifest.get_chunks(block as usize) {
            Some(block_chunks) => {
                let (new_chunks, more_removed_ids) =
                    block_write(block_chunks, sub_size, sub_start, new_chunk);
                removed_ids.extend(more_removed_ids);
                new_chunks
            }
            None => vec![new_chunk],
        };

        // Update data structures
        if manifest.blocks.len() as u64 == block {
            manifest.blocks.push(new_chunks);
        } else {
            manifest.blocks[block as usize] = new_chunks;
        }
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
) -> Vec<ChunkID> {
    // Check that there is something to truncate
    if size >= manifest.size {
        return vec![];
    }

    // Find limit block
    let blocksize = u64::from(manifest.blocksize);
    let last_block_index = (size / blocksize) as usize;
    let remainder = size % blocksize;

    let last_block_chunks = manifest
        .get_chunks(last_block_index)
        .expect("Block is expected to be part of the manifest");

    let (blocks_drain_at, chunks_skip_drain_until, maybe_new_last_block_chunks) = if remainder == 0
    {
        // Last block's start corresponds to the new end of the file, hence it is entirely removed !
        (last_block_index, 0, None)
    } else {
        // Only part of last block should be kept

        // Find the index of the last chunk to include
        let last_block_last_chunk_index =
            match last_block_chunks.binary_search_by_key(&size, |chunk| chunk.start) {
                Ok(found_index) => found_index - 1,
                Err(insert_index) => insert_index - 1,
            };

        let last_block_last_chunk = last_block_chunks
            .get(last_block_last_chunk_index)
            .expect("The index is found using binary search and hence always valid");

        // Create the new last chunk of the last block
        let new_last_block_last_chunk = {
            let mut new_last_block_last_chunk = last_block_last_chunk.clone();
            new_last_block_last_chunk.stop =
                NonZeroU64::new(size).expect("Cannot be zero since the remainder is not zero");
            new_last_block_last_chunk
        };

        // Create the new last block
        let new_last_block_chunks = {
            let mut new_last_block_chunks = Vec::with_capacity(last_block_last_chunk_index + 1);
            new_last_block_chunks.extend_from_slice(
                last_block_chunks
                    .get(..last_block_last_chunk_index)
                    .unwrap_or_default(),
            );
            new_last_block_chunks.push(new_last_block_last_chunk);
            new_last_block_chunks
        };

        // Remove all blocks after the new size, and also the last block...
        let blocks_drain_at = last_block_index;
        // ...but in the drain ignore the first chunks of the last block (as there are still in use)...
        let chunks_skip_drain_until = last_block_last_chunk_index + 1;
        // ...and finally we have an updated last block to append (containing the chunks ignored in the drain).
        let maybe_new_last_block_chunks = Some(new_last_block_chunks);
        (
            blocks_drain_at,
            chunks_skip_drain_until,
            maybe_new_last_block_chunks,
        )
    };

    // Update the manifest with the new last block

    manifest.need_sync = true;
    manifest.size = size;
    manifest.updated = timestamp;
    // Only keep the blocks that should still be used as-is...
    let removed_chunks = manifest
        .blocks
        .drain(blocks_drain_at..)
        .flatten()
        .skip(chunks_skip_drain_until)
        .map(|x| x.id)
        .collect();
    // ...then re-insert the last block if only part of it is still used
    if let Some(new_last_block_chunks) = maybe_new_last_block_chunks {
        manifest.blocks.push(new_last_block_chunks);
    }

    removed_chunks
}

/// Prepare a resize operation by updating the provided manifest.
///
/// Return a `Vec` of write operations that must be performed in order for the updated manifest to become valid.
/// Each write operation consists of a new chunk to store, along with an offset to apply to the corresponding raw data.
/// Note that the raw data also needs to be sliced to the chunk size and padded with null bytes if necessary.
/// Also return a list of chunk IDs that must cleaned up from the storage, after the updated manifest has been successfully stored.
pub(crate) fn prepare_resize(
    manifest: &mut LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> (Vec<WriteOperation>, Vec<ChunkID>) {
    if size >= manifest.size {
        // Extend
        prepare_write(manifest, 0, size, timestamp)
    } else {
        // Truncate
        let removed_ids = prepare_truncate(manifest, size, timestamp);
        (vec![], removed_ids)
    }
}

// Prepare reshape

pub(crate) struct ReshapeBlockOperation<'a> {
    manifest_chunks: &'a mut Vec<Chunk>,
    reshaped_chunk: Chunk,
}

impl ReshapeBlockOperation<'_> {
    pub fn destination(&self) -> &Chunk {
        &self.reshaped_chunk
    }

    pub fn source(&self) -> &[Chunk] {
        self.manifest_chunks
    }

    pub fn commit(self, chunk_data: &[u8]) {
        let ReshapeBlockOperation {
            manifest_chunks: chunks,
            mut reshaped_chunk,
        } = self;
        chunks.clear();
        reshaped_chunk
            .promote_as_block(chunk_data)
            .expect("chunk is block-compatible");
        chunks.push(reshaped_chunk);
    }
}

pub(crate) fn prepare_reshape(
    manifest: &mut LocalFileManifest,
) -> impl Iterator<Item = ReshapeBlockOperation<'_>> + '_ {
    manifest.blocks.iter_mut().filter_map(|manifest_chunks| {
        let reshaped_chunk = {
            let first_chunk = match manifest_chunks.first() {
                Some(first_chunk) => first_chunk,
                // TODO: chunks should never be empty, we should enforce that
                //       by using a dedicated type to represent the chunks in a block
                None => return None,
            };
            if manifest_chunks.len() == 1 {
                if first_chunk.is_block() {
                    // Already a valid block
                    return None;
                }
                if first_chunk.is_pseudo_block() {
                    // Pseudo-block is what the reshape is aiming at, so nothing to do here
                    return None;
                }
            }

            // Reshape is needed to turn those multiple chunks into a single one forming a pseudo-block
            let start = first_chunk.start;
            let stop = manifest_chunks
                .last()
                .expect("already checked chunks is not empty")
                .stop;
            Chunk::new(start, stop)
        };
        Some(ReshapeBlockOperation {
            manifest_chunks,
            reshaped_chunk,
        })
    })
}
