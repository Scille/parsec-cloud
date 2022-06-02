// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::NonZeroU64;

use parsec_api_types::{ChunkID, DateTime};
use parsec_client_types::{Chunk, LocalFileManifest};

type WriteOperation = (Chunk, i64);

// Prepare read

fn block_read(chunks: &[Chunk], size: u64, start: u64) -> impl Iterator<Item = Chunk> + '_ {
    let stop = start + size;

    // Bisect
    let start_index = match chunks.binary_search_by_key(&start, |x| x.start) {
        Ok(x) => x,
        Err(x) => x
            .checked_sub(1)
            .expect("First chunk should always start at 0"),
    };
    let stop_index = match chunks.binary_search_by_key(&stop, |x| x.start) {
        Ok(x) => x,
        Err(x) => x,
    };

    // Loop over chunks
    chunks
        .get(start_index..stop_index)
        .expect("Indexes are found using binary search and hence always valid")
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
            // Get substart, substop and subsize
            let blockstart = block * blocksize;
            let substart = max(offset, blockstart);
            let substop = min(offset + size, blockstart + blocksize);
            let subsize = substop
                .checked_sub(substart)
                .expect("Substop is always greater than substart");
            // Get the corresponding chunks
            let block_chunks = manifest
                .get_chunks(block as usize)
                .expect("A valid manifest must have enough blocks to cover its full range.");
            block_read(block_chunks, subsize, substart)
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
    let start_index = match chunks.binary_search_by_key(&start, |x| x.start) {
        Ok(x) => x,
        Err(x) => x
            .checked_sub(1)
            .expect("First chunk should always start at 0"),
    };
    let stop_index = match chunks.binary_search_by_key(&stop, |x| x.start) {
        Ok(x) => x,
        Err(x) => x,
    };

    // Removed ids
    let mut removed_ids: HashSet<ChunkID> = chunks
        .get(start_index..stop_index)
        .unwrap()
        .iter()
        .map(|x| x.id)
        .collect();

    // New chunks
    let mut new_chunks: Vec<Chunk> = chunks.get(0..start_index).unwrap().to_vec();

    // Test start chunk
    let start_chunk = chunks.get(start_index).unwrap();
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
    let stop_chunk = chunks.get(stop_index - 1).unwrap();
    if stop_chunk.stop.get() > stop {
        let mut new_stop_chunk = stop_chunk.clone();
        new_stop_chunk.start = stop;
        new_chunks.push(new_stop_chunk);
        removed_ids.remove(&stop_chunk.id);
    }

    // Fill up
    new_chunks.extend(chunks.get(stop_index..).unwrap().iter().cloned());

    // IDs might appear multiple times
    for chunk in &new_chunks {
        removed_ids.remove(&chunk.id);
        if removed_ids.is_empty() {
            break;
        }
    }

    (new_chunks, removed_ids)
}

pub fn prepare_write(
    manifest: &LocalFileManifest,
    mut size: u64,
    mut offset: u64,
    timestamp: DateTime,
) -> (LocalFileManifest, Vec<WriteOperation>, HashSet<ChunkID>) {
    let mut padding = 0;
    let mut removed_ids = HashSet::new();
    let mut write_operations = vec![];

    // Padding
    if offset > manifest.size {
        padding = offset - manifest.size;
        size += padding;
        offset = manifest.size;
    }

    // Copy buffers
    let mut blocks = manifest.blocks.clone();

    // Find proper block indexes
    let blocksize = u64::from(manifest.blocksize);
    let start_block = offset / blocksize;
    let stop_block = (offset + size + blocksize - 1) / blocksize;

    // Loop over blocks
    for block in start_block..stop_block {
        // Get substart, substop and subsize
        let blockstart = block * blocksize;
        let substart = max(offset, blockstart);
        let substop = min(offset + size, blockstart + blocksize);
        let subsize = substop
            .checked_sub(substart)
            .expect("Substop is always greater than substart");
        let content_offset = substart - offset;

        // Prepare new chunk
        let new_chunk = Chunk::new(substart, NonZeroU64::new(substart + subsize).unwrap());
        write_operations.push((new_chunk.clone(), content_offset as i64 - padding as i64));

        // Get the corresponding chunks
        let new_chunks = match manifest.get_chunks(block as usize) {
            Some(block_chunks) => {
                let (new_chunks, more_removed_ids) =
                    block_write(block_chunks, subsize, substart, new_chunk);
                removed_ids.extend(more_removed_ids);
                new_chunks
            }
            None => vec![new_chunk],
        };

        // Update data structures
        if blocks.len() == block as usize {
            blocks.push(new_chunks);
        } else {
            blocks[block as usize] = new_chunks;
        }
    }

    // Evolve manifest
    let new_size = max(manifest.size, offset + size);
    let new_manifest = manifest
        .clone()
        .evolve_and_mark_updated(new_size, blocks, timestamp);

    (new_manifest, write_operations, removed_ids)
}

// Prepare truncate

pub fn prepare_truncate(
    manifest: &LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> (LocalFileManifest, HashSet<ChunkID>) {
    // Find limit block
    let blocksize = u64::from(manifest.blocksize);
    let block = size / blocksize;
    let remainder = size % blocksize;

    // Prepare removed ids and new blocks
    let mut removed_ids: HashSet<ChunkID> = manifest
        .blocks
        .get(block as usize..)
        .unwrap()
        .iter()
        .flatten()
        .map(|x| x.id)
        .collect();
    let mut new_blocks = manifest.blocks.get(0..block as usize).unwrap().to_vec();

    // Last block needs to be split
    if remainder != 0 {
        let chunks = manifest.get_chunks(block as usize).unwrap();

        // Find the index of the last chunk to include
        let chunk_index = match chunks.binary_search_by_key(&size, |x| x.start) {
            Ok(x) => x - 1,
            Err(x) => x - 1,
        };

        // Create the new last chunk
        let last_chunk = chunks.get(chunk_index).unwrap();
        let mut new_chunk = last_chunk.clone();
        new_chunk.stop =
            NonZeroU64::new(size).expect("Cannot be zero since the remainder is not zero");

        // Create the new chunks for the last block
        let mut new_chunks = chunks.get(..chunk_index).unwrap().to_vec();
        new_chunks.push(new_chunk);

        // Those new chunks should not be removed
        for chunk in &new_chunks {
            removed_ids.remove(&chunk.id);
        }

        // Add to the new blocks
        new_blocks.push(new_chunks);
    }

    // Create the new manifest
    let new_manifest = manifest
        .clone()
        .evolve_and_mark_updated(size, new_blocks, timestamp);
    (new_manifest, removed_ids)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!()
    }
}
