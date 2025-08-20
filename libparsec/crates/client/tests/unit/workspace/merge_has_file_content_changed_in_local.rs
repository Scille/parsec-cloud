// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroU64;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::has_file_content_changed_in_local;

#[test]
fn empty_file_no_changes() {
    let base_size = 0;
    let base_blocksize = Blocksize::try_from(512 * 1024).unwrap();
    let base_blocks = vec![];

    let local_size = base_size;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        false,
    );
}

#[test]
fn non_empty_file_no_changes() {
    let base_size = 25;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![
        BlockAccess {
            id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
            offset: 0,
            size: NonZeroU64::new(10).unwrap(),
            digest: HashDigest::from(hex!(
                "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
            )),
        },
        // No block between offset 10 and 20: this blocksize area contains zero-filled data
        BlockAccess {
            id: BlockID::from_hex("35723b4f3f8145bba7910c3c50ab965c").unwrap(),
            offset: 20,
            size: NonZeroU64::new(5).unwrap(),
            digest: HashDigest::from(hex!(
                "64178bc1274c44cc96e7cbdca341f73c6d4c473ecffe4116af72a13246e36532"
            )),
        },
    ];

    let local_size = base_size;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![
        // Blocksize area 0-10
        vec![ChunkView {
            id: base_blocks[0].id.into(),
            start: 0,
            stop: NonZeroU64::new(10).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::new(10).unwrap(),
            access: Some(base_blocks[0].clone()),
        }],
        // Blocksize area 10-20
        vec![],
        // Blocksize area 20-30
        vec![ChunkView {
            id: base_blocks[1].id.into(),
            start: 20,
            stop: NonZeroU64::new(25).unwrap(),
            raw_offset: 20,
            raw_size: NonZeroU64::new(5).unwrap(),
            access: Some(base_blocks[1].clone()),
        }],
    ];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        false,
    );
}

#[test]
fn blocksize_changed() {
    let base_size = 0;
    let base_blocksize = Blocksize::try_from(512 * 1024).unwrap();
    let base_blocks = vec![];

    let local_size = base_size;
    let local_blocksize = Blocksize::try_from(1024 * 1024).unwrap();
    let local_blocks = vec![];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[test]
fn empty_base_then_local_write() {
    let base_size = 0;
    let base_blocksize = Blocksize::try_from(512 * 1024).unwrap();
    let base_blocks = vec![];

    let local_size = 10;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![vec![ChunkView {
        id: BlockID::from_hex("41aefe7cecd04916bf4ffbb8ebc6a7cb")
            .unwrap()
            .into(),
        start: 0,
        stop: NonZeroU64::new(10).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::new(10).unwrap(),
        access: None,
    }]];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[test]
fn non_empty_base_then_local_full_truncate() {
    let base_size = 5;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![BlockAccess {
        id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
        offset: 0,
        size: NonZeroU64::new(5).unwrap(),
        digest: HashDigest::from(hex!(
            "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
        )),
    }];

    let local_size = 0;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[test]
fn non_empty_base_then_local_partial_truncate() {
    let base_size = 5;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![BlockAccess {
        id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
        offset: 0,
        size: NonZeroU64::new(5).unwrap(),
        digest: HashDigest::from(hex!(
            "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
        )),
    }];

    let local_size = 2;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![vec![ChunkView {
        id: base_blocks[0].id.into(),
        start: 0,
        // Only the first 2 bytes are kept
        stop: NonZeroU64::new(2).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::new(5).unwrap(),
        access: Some(base_blocks[0].clone()),
    }]];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[test]
fn non_empty_base_then_local_partial_overwriting() {
    let base_size = 10;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![BlockAccess {
        id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
        offset: 0,
        size: NonZeroU64::new(10).unwrap(),
        digest: HashDigest::from(hex!(
            "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
        )),
    }];

    let local_size = base_size;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![vec![
        ChunkView {
            id: base_blocks[0].id.into(),
            start: 0,
            // Only the first 5 bytes are kept from base data...
            stop: NonZeroU64::new(5).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::new(10).unwrap(),
            access: Some(base_blocks[0].clone()),
        },
        ChunkView {
            id: BlockID::from_hex("aba192fe3dc74ae699157986327052c9")
                .unwrap()
                .into(),
            start: 5,
            // ...then 5 more bytes are written
            stop: NonZeroU64::new(10).unwrap(),
            raw_offset: 5,
            raw_size: NonZeroU64::new(5).unwrap(),
            access: None,
        },
    ]];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[test]
fn non_empty_base_then_local_total_overwriting() {
    let base_size = 10;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![BlockAccess {
        id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
        offset: 0,
        size: NonZeroU64::new(10).unwrap(),
        digest: HashDigest::from(hex!(
            "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
        )),
    }];

    let local_size = base_size;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![vec![ChunkView {
        id: BlockID::from_hex("aba192fe3dc74ae699157986327052c9")
            .unwrap()
            .into(),
        start: 0,
        stop: NonZeroU64::new(10).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::new(10).unwrap(),
        access: None,
    }]];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}

#[parsec_test]
fn non_empty_base_local_block_access_fields_mismatch(
    #[values("offset", "size", "digest")] kind: &str,
) {
    let base_size = 10;
    let base_blocksize = Blocksize::try_from(10).unwrap();
    let base_blocks = vec![BlockAccess {
        id: BlockID::from_hex("4a621015b4974a64b2e3028c9b3c8178").unwrap(),
        offset: 0,
        size: NonZeroU64::new(10).unwrap(),
        digest: HashDigest::from(hex!(
            "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
        )),
    }];

    let mut local_chunk_view_block_access = base_blocks[0].clone();
    match kind {
        "offset" => {
            local_chunk_view_block_access.offset = 42;
        }
        "size" => {
            local_chunk_view_block_access.size = NonZeroU64::new(42).unwrap();
        }
        "digest" => {
            local_chunk_view_block_access.digest = HashDigest::from(hex!(
                "b3a656e061934ca59d2f34ed5d1bae80e45c3b6f7d6d4ef1be5d5667fd7dbc0d"
            ));
        }
        unknown => panic!("Unknown kind: {unknown}"),
    }

    let local_size = base_size;
    let local_blocksize = base_blocksize;
    let local_blocks = vec![vec![ChunkView {
        id: base_blocks[0].id.into(),
        start: 0,
        stop: NonZeroU64::new(10).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::new(10).unwrap(),
        access: Some(local_chunk_view_block_access),
    }]];

    p_assert_eq!(
        has_file_content_changed_in_local(
            base_size,
            base_blocksize,
            &base_blocks,
            local_size,
            local_blocksize,
            &local_blocks,
        ),
        true,
    );
}
