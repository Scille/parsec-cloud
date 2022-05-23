// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::cmp::{max, min};
use std::iter::empty;
use std::num::NonZeroU64;

use parsec_client_types::{Chunk, LocalFileManifest};

fn split_read(size: u64, offset: u64, blocksize: u64) -> Box<dyn Iterator<Item = (u64, u64, u64)>> {
    // Nothing to read
    if size == 0 {
        return Box::new(empty());
    }

    // Loop over blocks
    let start_block = offset / blocksize;
    let stop_block = (offset + size).checked_sub(1).unwrap() / blocksize;
    Box::new((start_block..stop_block + 1).map(move |block| {
        // Get substart / substop
        let blockstart = block * blocksize;
        let substart = max(offset, blockstart);
        let substop = min(offset + size, blockstart + blocksize);
        (block, substop - substart, substart)
    }))
}

fn block_read(chunks: &[Chunk], size: u64, start: u64) -> Box<dyn Iterator<Item = Chunk> + '_> {
    let stop = start + size;

    // Bisect
    let start_index = match chunks.binary_search_by_key(&start, |x| x.start) {
        Ok(x) => x,
        Err(x) => x - 1,
    };
    let stop_index = match chunks.binary_search_by_key(&stop, |x| x.start) {
        Ok(x) => x,
        Err(x) => x,
    };

    // Loop over chunks
    Box::new(chunks[start_index..stop_index].iter().map(move |chunk| {
        let mut new_chunk = chunk.clone();
        new_chunk.start = max(chunk.start, start);
        new_chunk.stop = min(chunk.stop, NonZeroU64::new(stop).unwrap());
        new_chunk
    }))
}

pub fn prepare_read(manifest: LocalFileManifest, size: u64, offset: u64) -> Vec<Chunk> {
    let offset = min(offset, manifest.size);
    let size = min(size, manifest.size - offset);
    let blocksize = u64::from(manifest.blocksize);

    split_read(size, offset, blocksize)
        .flat_map(|(block, length, start)| {
            let block_chunks = manifest.get_chunks(block as usize).unwrap();
            block_read(block_chunks, length, start)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
