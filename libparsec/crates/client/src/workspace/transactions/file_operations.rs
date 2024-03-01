// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::NonZeroU64;

use libparsec_types::prelude::*;

type WriteOperation = (Chunk, i64);

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

/// Prepare a read operation on a provided manifest.
///
/// Return a contiguous `Vec` of chunks that must be read and concatenated.
pub fn prepare_read(manifest: &LocalFileManifest, size: u64, offset: u64) -> Vec<Chunk> {
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
    (start_block..stop_block)
        .flat_map(move |block| {
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
                .expect("A valid manifest must have enough blocks to cover its full range.");
            block_read(block_chunks, sub_size, sub_start)
        })
        // Collect as a flatten vec of Chunks
        .collect()
}

// Prepare write

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
/// Return a `Vec` of write operations that must be performed in order for the updated manifest to become valid.
/// Each write operation consists of a new chunk to store, along with an offset to apply to the corresponding raw data.
/// Note that the raw data also needs to be sliced to the chunk size and padded with null bytes if necessary.
/// Also return a `HashSet` of chunk IDs that must cleaned up from the storage, after the updated manifest has been successfully stored.
pub fn prepare_write(
    manifest: &mut LocalFileManifest,
    mut size: u64,
    mut offset: u64,
    timestamp: DateTime,
) -> (Vec<WriteOperation>, HashSet<ChunkID>) {
    let mut padding = 0;
    let mut removed_ids = HashSet::new();
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
        write_operations.push((new_chunk.clone(), content_offset as i64 - padding as i64));

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

    // No block to split
    if remainder == 0 {
        // Simply truncate to the correct amount of blocks
        manifest.blocks.truncate(block);

    // Last block needs to be split
    } else {
        let chunks = manifest
            .get_chunks(block)
            .expect("Block is expected to be part of the manifest");

        // Find the index of the last chunk to include
        let chunk_index = match chunks.binary_search_by_key(&size, |chunk| chunk.start) {
            Ok(found_index) => found_index - 1,
            Err(insert_index) => insert_index - 1,
        };

        // Create the new last chunk
        let last_chunk = chunks
            .get(chunk_index)
            .expect("The index is found using binary search and hence always valid");
        let mut new_chunk = last_chunk.clone();
        new_chunk.stop =
            NonZeroU64::new(size).expect("Cannot be zero since the remainder is not zero");

        // Create the new chunks for the last block
        let mut new_chunks = chunks.get(..chunk_index).unwrap_or_default().to_vec();
        new_chunks.push(new_chunk);

        // Those new chunks should not be removed
        for chunk in &new_chunks {
            removed_ids.remove(&chunk.id);
        }

        // Truncate and add the new chunks
        manifest.blocks.truncate(block);
        manifest.blocks.push(new_chunks);
    }

    // Update the manifest
    manifest.need_sync = true;
    manifest.size = size;
    manifest.updated = timestamp;

    removed_ids
}

/// Prepare a resize operation by updating the provided manifest.
///
/// Return a `Vec` of write operations that must be performed in order for the updated manifest to become valid.
/// Each write operation consists of a new chunk to store, along with an offset to apply to the corresponding raw data.
/// Note that the raw data also needs to be sliced to the chunk size and padded with null bytes if necessary.
/// Also return a `HashSet` of chunk IDs that must cleaned up from the storage, after the updated manifest has been successfully stored.
pub fn prepare_resize(
    manifest: &mut LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> (Vec<WriteOperation>, HashSet<ChunkID>) {
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

/// Prepare a reshape operation without updating the provided manifest.
/// The reason why the manifest is not updated is because the hash of the corresponding data is required to turn a chunk into a block.
/// Instead, it's up to the caller to call `chunk.evolve_as_block` and `manifest.set_single_block` to update the manifest.
///
/// Return an iterator where each item corresponds to a block to reshape.
/// Each item consists of:
/// - the index of the block that is being reshaped
/// - a source block, represented as a `Vec` of chunks
/// - a destination block, represented as a single chunk
/// - a write back boolean indicating the new chunk must be written
/// - a `HashSet` of chunk IDs that must cleaned up from the storage
pub fn prepare_reshape(
    manifest: &LocalFileManifest,
) -> impl Iterator<Item = (u64, Vec<Chunk>, Chunk, bool, HashSet<ChunkID>)> + '_ {
    // Loop over blocks
    manifest
        .blocks
        .iter()
        .enumerate()
        .filter_map(|(block, chunks)| {
            let block = block as u64;
            // Already a valid block
            if chunks.len() == 1 && chunks[0].is_block() {
                None
            // Already a pseudo-block, we can keep the chunk as it is
            } else if chunks.len() == 1 && chunks[0].is_pseudo_block() {
                let new_chunk = chunks[0].clone();
                let to_remove = HashSet::new();
                let write_back = false;
                Some((block, chunks.to_vec(), new_chunk, write_back, to_remove))
            // Reshape those chunks as a single block
            } else {
                // Start and stop should be 0 and blocksize respectively
                let start = chunks[0].start;
                let stop = chunks.last().expect("A block cannot be empty").stop;
                let new_chunk = Chunk::new(start, stop);
                let to_remove = chunks.iter().map(|x| x.id).collect();
                let write_back = true;
                Some((block, chunks.to_vec(), new_chunk, write_back, to_remove))
            }
        })
}
