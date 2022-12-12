// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::NonZeroU64;

use libparsec_client_types::{Chunk, LocalFileManifest};
use libparsec_types::{ChunkID, DateTime};

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

#[cfg(test)]
mod tests {
    use crate::file_operations::{prepare_read, prepare_reshape, prepare_resize, prepare_write};
    use libparsec_client_types::{Chunk, LocalFileManifest};
    use libparsec_types::{Blocksize, ChunkID, DateTime, DeviceID, EntryID};
    use std::{collections::HashMap, str::FromStr};

    fn padded_data(data: &[u8], start: i64, stop: i64) -> Vec<u8> {
        assert!(start <= stop);
        if start <= stop && stop <= 0 {
            vec![0; (stop - start) as usize]
        } else if 0 <= start && start <= stop {
            data[start as usize..stop as usize].to_vec()
        } else {
            let mut result: Vec<u8> = vec![0; (-start) as usize];
            result.extend_from_slice(&data[0..stop as usize]);
            result
        }
    }

    #[derive(Default)]
    struct Storage(HashMap<ChunkID, Vec<u8>>);

    impl Storage {
        fn read_chunk_data(&self, chunk_id: ChunkID) -> &[u8] {
            self.0.get(&chunk_id).unwrap()
        }

        fn write_chunk_data(&mut self, chunk_id: ChunkID, data: Vec<u8>) {
            let result = self.0.insert(chunk_id, data);
            assert!(result.is_none());
        }

        fn clear_chunk_data(&mut self, chunk_id: ChunkID) {
            self.0.remove(&chunk_id);
        }

        fn read_chunk(&self, chunk: &Chunk) -> &[u8] {
            let data = self.read_chunk_data(chunk.id);
            let start = (chunk.start - chunk.raw_offset) as usize;
            let stop = (chunk.stop.get() - chunk.raw_offset) as usize;
            data.get(start..stop).unwrap()
        }

        fn write_chunk(&mut self, chunk: &Chunk, content: &[u8], offset: i64) {
            let data = padded_data(
                content,
                offset,
                offset + chunk.stop.get() as i64 - chunk.start as i64,
            );
            self.write_chunk_data(chunk.id, data)
        }

        fn build_data(&self, chunks: &[Chunk]) -> Vec<u8> {
            if chunks.is_empty() {
                return vec![];
            }
            let start = chunks.first().unwrap().start as usize;
            let stop = chunks.last().unwrap().stop.get() as usize;
            let mut result: Vec<u8> = vec![0; stop - start];
            for chunk in chunks {
                let data = self.read_chunk(chunk);
                result[chunk.start as usize - start..chunk.stop.get() as usize - start]
                    .copy_from_slice(data);
            }
            result
        }

        // File operations

        fn read(&self, manifest: &LocalFileManifest, size: u64, offset: u64) -> Vec<u8> {
            let chunks = prepare_read(manifest, size, offset);
            self.build_data(&chunks)
        }

        fn write(
            &mut self,
            manifest: &mut LocalFileManifest,
            content: &[u8],
            offset: u64,
            timestamp: DateTime,
        ) {
            // No-op
            if content.is_empty() {
                return;
            }
            // Write
            let (write_operations, removed_ids) =
                prepare_write(manifest, content.len() as u64, offset, timestamp);
            for (chunk, offset) in write_operations {
                self.write_chunk(&chunk, content, offset);
            }
            for removed_id in removed_ids {
                self.clear_chunk_data(removed_id);
            }
        }

        fn resize(&mut self, manifest: &mut LocalFileManifest, size: u64, timestamp: DateTime) {
            // No-op
            if size == manifest.size {
                return;
            }
            // Resize
            let empty = vec![];
            let (write_operations, removed_ids) = prepare_resize(manifest, size, timestamp);
            for (chunk, offset) in write_operations {
                self.write_chunk(&chunk, &empty, offset);
            }
            for removed_id in removed_ids {
                self.clear_chunk_data(removed_id);
            }
        }

        fn reshape(&mut self, manifest: &mut LocalFileManifest) {
            let collected: Vec<_> = prepare_reshape(manifest).collect();
            for (block, source, destination, write_back, removed_ids) in collected {
                let data = self.build_data(&source);
                let new_chunk = destination.evolve_as_block(&data).unwrap();
                if write_back {
                    self.write_chunk(&new_chunk, &data, 0);
                }
                manifest
                    .set_single_block(block, new_chunk)
                    .expect("block should be valid");
                for removed_id in removed_ids {
                    self.clear_chunk_data(removed_id);
                }
            }
        }
    }

    #[test]
    fn full_scenario() {
        // Initialize storage and manifest
        let mut storage = Storage::default();
        let blocksize = Blocksize::try_from(16).unwrap();
        let t1 = DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap();
        let mut manifest =
            LocalFileManifest::new(DeviceID::default(), EntryID::default(), t1, blocksize);

        // Write some data and read it back
        let t2 = DateTime::from_str("2000-01-01 02:00:00 UTC").unwrap();
        storage.write(&mut manifest, b"Hello ", 0, t2);
        assert_eq!(storage.read(&manifest, 6, 0), b"Hello ");

        // Check manifest
        assert_eq!(manifest.size, 6);
        assert_eq!(manifest.updated, t2);
        assert_eq!(manifest.blocks.len(), 1);
        assert_eq!(manifest.blocks[0].len(), 1);
        let chunk0 = manifest.blocks[0][0].clone();

        // Check chunks
        assert_eq!(chunk0.start, 0);
        assert_eq!(chunk0.stop.get(), 6);
        assert_eq!(chunk0.raw_offset, 0);
        assert_eq!(chunk0.raw_size.get(), 6);
        assert_eq!(chunk0.access, None);
        assert_eq!(storage.read_chunk_data(chunk0.id), b"Hello ");

        // Append more data to the file and read everything back
        let t3 = DateTime::from_str("2000-01-01 03:00:00 UTC").unwrap();
        storage.write(&mut manifest, b"world !", 6, t3);
        assert_eq!(storage.read(&manifest, 13, 0), b"Hello world !");

        // Check manifest
        assert_eq!(manifest.size, 13);
        assert_eq!(manifest.updated, t3);
        assert_eq!(manifest.blocks.len(), 1);
        assert_eq!(manifest.blocks[0].len(), 2);
        let chunk1 = manifest.blocks[0][1].clone();
        assert_eq!(manifest.blocks, vec![vec![chunk0.clone(), chunk1.clone()]]);

        // Check chunks
        assert_eq!(chunk1.start, 6);
        assert_eq!(chunk1.stop.get(), 13);
        assert_eq!(chunk1.raw_offset, 6);
        assert_eq!(chunk1.raw_size.get(), 7);
        assert_eq!(chunk1.access, None);
        assert_eq!(storage.read_chunk_data(chunk1.id), b"world !");

        // Append even more data to reach the second block and read everything back
        let t4 = DateTime::from_str("2000-01-01 04:00:00 UTC").unwrap();
        storage.write(&mut manifest, b"\n More content", 13, t4);
        assert_eq!(
            storage.read(&manifest, 27, 0),
            b"Hello world !\n More content"
        );

        // Check manifest
        assert_eq!(manifest.size, 27);
        assert_eq!(manifest.updated, t4);
        assert_eq!(manifest.blocks.len(), 2);
        assert_eq!(manifest.blocks[0].len(), 3);
        assert_eq!(manifest.blocks[1].len(), 1);
        let chunk2 = manifest.blocks[0][2].clone();
        let chunk3 = manifest.blocks[1][0].clone();
        assert_eq!(
            manifest.blocks,
            vec![
                vec![chunk0.clone(), chunk1.clone(), chunk2.clone()],
                vec![chunk3.clone()]
            ]
        );

        // Check chunks
        assert_eq!(chunk2.start, 13);
        assert_eq!(chunk2.stop.get(), 16);
        assert_eq!(chunk2.raw_offset, 13);
        assert_eq!(chunk2.raw_size.get(), 3);
        assert_eq!(chunk2.access, None);
        assert_eq!(storage.read_chunk_data(chunk2.id), b"\n M");
        assert_eq!(chunk3.start, 16);
        assert_eq!(chunk3.stop.get(), 27);
        assert_eq!(chunk3.raw_offset, 16);
        assert_eq!(chunk3.raw_size.get(), 11);
        assert_eq!(chunk3.access, None);
        assert_eq!(storage.read_chunk_data(chunk3.id), b"ore content");

        // Fix the typo and read everything back
        let t5 = DateTime::from_str("2000-01-01 05:00:00 UTC").unwrap();
        storage.write(&mut manifest, b"c", 20, t5);
        assert_eq!(
            storage.read(&manifest, 27, 0),
            b"Hello world !\n More content"
        );

        // Check manifest
        assert_eq!(manifest.size, 27);
        assert_eq!(manifest.updated, t5);
        assert_eq!(manifest.blocks.len(), 2);
        assert_eq!(manifest.blocks[0].len(), 3);
        assert_eq!(manifest.blocks[1].len(), 3);
        let chunk4 = manifest.blocks[1][0].clone();
        let chunk5 = manifest.blocks[1][1].clone();
        let chunk6 = manifest.blocks[1][2].clone();
        assert_eq!(
            manifest.blocks[0],
            vec![chunk0.clone(), chunk1.clone(), chunk2.clone()]
        );

        // Check chunks
        assert_eq!(chunk4.id, chunk3.id);
        assert_eq!(chunk6.id, chunk3.id);
        assert_eq!(storage.read_chunk_data(chunk5.id), b"c");

        // Extend file and read everything back
        let t6 = DateTime::from_str("2000-01-01 06:00:00 UTC").unwrap();
        storage.resize(&mut manifest, 40, t6);
        let mut expected = b"Hello world !\n More content".to_vec();
        expected.extend([0; 13]);
        assert_eq!(storage.read(&manifest, 40, 0), expected);

        // Check manifest
        assert_eq!(manifest.size, 40);
        assert_eq!(manifest.updated, t6);
        assert_eq!(manifest.blocks.len(), 3);
        assert_eq!(manifest.blocks[0].len(), 3);
        assert_eq!(manifest.blocks[1].len(), 4);
        assert_eq!(manifest.blocks[2].len(), 1);
        let chunk7 = manifest.blocks[1][3].clone();
        let chunk8 = manifest.blocks[2][0].clone();
        assert_eq!(
            manifest.blocks,
            vec![
                vec![chunk0.clone(), chunk1.clone(), chunk2.clone()],
                vec![
                    chunk4.clone(),
                    chunk5.clone(),
                    chunk6.clone(),
                    chunk7.clone()
                ],
                vec![chunk8.clone()],
            ]
        );

        // Check chunks
        assert_eq!(storage.read_chunk_data(chunk7.id), [0; 5]);
        assert_eq!(storage.read_chunk_data(chunk8.id), [0; 8]);

        // Extend file and read everything back
        let t7 = DateTime::from_str("2000-01-01 07:00:00 UTC").unwrap();
        storage.resize(&mut manifest, 25, t7);
        assert_eq!(
            storage.read(&manifest, 25, 0),
            b"Hello world !\n More conte"
        );

        // Check manifest
        assert_eq!(manifest.size, 25);
        assert_eq!(manifest.updated, t7);
        assert_eq!(manifest.blocks.len(), 2);
        assert_eq!(manifest.blocks[1].len(), 3);
        let chunk9 = manifest.blocks[1][2].clone();
        assert_eq!(
            manifest.blocks,
            vec![
                vec![chunk0, chunk1, chunk2],
                vec![chunk4, chunk5, chunk9.clone()],
            ]
        );

        // Check chunks
        assert_eq!(chunk9.id, chunk6.id);
        assert_eq!(chunk9.start, 21);
        assert_eq!(chunk9.stop.get(), 25);
        assert_eq!(chunk9.raw_offset, 16);
        assert_eq!(chunk9.raw_size.get(), 11);
        assert_eq!(chunk9.access, None);

        // Reshape manifest and read everything back
        assert!(!manifest.is_reshaped());
        storage.reshape(&mut manifest);
        assert_eq!(
            storage.read(&manifest, 25, 0),
            b"Hello world !\n More conte"
        );
        assert!(manifest.is_reshaped());

        // Check manifest
        assert_eq!(manifest.size, 25);
        assert_eq!(manifest.updated, t7);
        assert_eq!(manifest.blocks.len(), 2);
        assert_eq!(manifest.blocks[0].len(), 1);
        assert_eq!(manifest.blocks[1].len(), 1);
        let chunk10 = manifest.blocks[0][0].clone();
        let chunk11 = manifest.blocks[1][0].clone();

        // Check chunks
        assert_eq!(chunk10.start, 0);
        assert_eq!(chunk10.stop.get(), 16);
        assert_eq!(chunk10.raw_offset, 0);
        assert_eq!(chunk10.raw_size.get(), 16);
        assert!(chunk10.access.is_some());
        assert!(chunk10.is_block());
        assert_eq!(storage.read_chunk_data(chunk10.id), b"Hello world !\n M");
        assert_eq!(chunk11.start, 16);
        assert_eq!(chunk11.stop.get(), 25);
        assert_eq!(chunk11.raw_offset, 16);
        assert_eq!(chunk11.raw_size.get(), 9);
        assert!(chunk11.access.is_some());
        assert!(chunk11.is_block());
        assert_eq!(storage.read_chunk_data(chunk11.id), b"ore conte");
    }
}
