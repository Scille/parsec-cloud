// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, str::FromStr};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::transactions::{
    prepare_read, prepare_reshape, prepare_resize, prepare_write,
};

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
        let (written_size, chunks) = prepare_read(manifest, size, offset);
        let data = self.build_data(&chunks.collect::<Vec<_>>());
        p_assert_eq!(data.len() as u64, written_size);
        data
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
        for wo in write_operations {
            self.write_chunk(&wo.chunk, content, wo.offset);
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
        for wo in write_operations {
            self.write_chunk(&wo.chunk, &empty, wo.offset);
        }
        for removed_id in removed_ids {
            self.clear_chunk_data(removed_id);
        }
    }

    fn reshape(&mut self, manifest: &mut LocalFileManifest) {
        let operations: Vec<_> = prepare_reshape(manifest).collect();
        for operation in operations {
            let data = self.build_data(&operation.source());
            let new_chunk = operation.destination().to_owned();
            for old_chunk in operation.source() {
                self.clear_chunk_data(old_chunk.id);
            }
            operation.commit(&data);
            self.write_chunk(&new_chunk, &data, 0);
        }
    }
}

#[parsec_test]
fn full_scenario() {
    // Initialize storage and manifest
    let mut storage = Storage::default();
    let blocksize = Blocksize::try_from(16).unwrap();
    let t1 = DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap();
    let mut manifest = LocalFileManifest::new(DeviceID::default(), VlobID::default(), t1);
    manifest.blocksize = blocksize;
    manifest.base.blocksize = blocksize;

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

    // Resize noop
    let t8 = DateTime::from_str("2000-01-01 08:00:00 UTC").unwrap();
    storage.resize(&mut manifest, 25, t8);
    assert_eq!(
        storage.read(&manifest, 25, 0),
        b"Hello world !\n More conte"
    );
    assert_eq!(manifest.size, 25);
    assert_eq!(manifest.updated, t7);
    assert_eq!(manifest.blocks.len(), 2);
    assert_eq!(manifest.blocks[0].len(), 1);
    assert_eq!(manifest.blocks[1].len(), 1);

    // Truncate with a size aligned on blocksize
    let t9 = DateTime::from_str("2000-01-01 09:00:00 UTC").unwrap();
    storage.resize(&mut manifest, 16, t9);
    assert_eq!(storage.read(&manifest, 16, 0), b"Hello world !\n M");
    assert_eq!(manifest.size, 16);
    assert_eq!(manifest.updated, t9);
    assert_eq!(manifest.blocks.len(), 1);
    assert_eq!(manifest.blocks, vec![vec![chunk10.clone()]]);

    // Extend size with a size aligned on blocksize
    let t10 = DateTime::from_str("2000-01-01 10:00:00 UTC").unwrap();
    storage.resize(&mut manifest, 32, t10);
    assert_eq!(
        storage.read(&manifest, 32, 0),
        b"Hello world !\n M\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
    );
    assert_eq!(manifest.size, 32);
    assert_eq!(manifest.updated, t10);
    assert_eq!(manifest.blocks.len(), 2);
    assert_eq!(manifest.blocks[0], vec![chunk10.clone()]);
    let chunk12 = manifest.blocks[1][0].clone();
    assert_eq!(chunk12.start, 16);
    assert_eq!(chunk12.stop.get(), 32);
    assert_eq!(chunk12.raw_offset, 16);
    assert_eq!(chunk12.raw_size.get(), 16);
    assert!(!chunk12.access.is_some());
    assert!(!chunk12.is_block());
}

// TODO: split this test in smaller, easier to maintain, parts
// TODO: add more tests about truncate/resize: aligned or not on blocksize,
//       single vs multiple chunks removed/added, first chunk or block removed etc.
