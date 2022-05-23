// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::cmp::{max, min};
// use std::collections::HashSet;
use std::num::NonZeroU64;

// use parsec_api_types::{BlockID, ChunkID};
use parsec_client_types::{Chunk, LocalFileManifest};

type Chunks = Vec<Chunk>;

// enum ChunkOrBlockID {
//     ChunkID,
//     BlockID,
// }
// type ChunkIDSet = HashSet<ChunkOrBlockID>;
// type WriteOperationList = Vec<(ChunkID, u64)>;

pub fn prepare_read(manifest: LocalFileManifest, size: u64, offset: u64) -> Chunks {
    let mut chunks = vec![];
    let offset = min(offset, manifest.size);
    let size = min(size, manifest.size - offset);

    // Nothing to read
    if size == 0 {
        return chunks;
    }

    // Loop over blocks
    let blocksize = u64::from(manifest.blocksize);
    let start_block = offset / blocksize;
    let stop_block = (offset + size - 1) / blocksize;
    for block in start_block..stop_block + 1 {
        // Get substart / substop
        let blockstart = block * blocksize;
        let substart = max(offset, blockstart);
        let substop = min(offset + size, blockstart + blocksize);

        // Get chunks
        let block_chunks = manifest.get_chunks(block as usize).unwrap();

        // Bisect
        let start_index = match block_chunks.binary_search_by_key(&substart, |x| x.start) {
            Ok(x) => x,
            Err(x) => x - 1,
        };
        let stop_index = match block_chunks.binary_search_by_key(&substop, |x| x.start) {
            Ok(x) => x,
            Err(x) => x,
        };

        // Loop over chunks
        for chunk in &block_chunks[start_index..stop_index] {
            let mut new_chunk = chunk.clone();
            new_chunk.start = max(chunk.start, substart);
            new_chunk.stop = min(chunk.stop, NonZeroU64::new(substop).unwrap());
            chunks.push(new_chunk)
        }
    }
    chunks
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
