// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]

use std::num::NonZeroU64;

use crate::fixtures::{Device, alice, timestamp};
use crate::prelude::*;
use libparsec_tests_lite::prelude::*;

#[rstest]
fn serde_local_file_manifest_ok(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_file_manifest'
    //   base: {
    //     type: 'file_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     size: 700,
    //     blocksize: 512,
    //     blocks: [
    //       {
    //         id: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //         offset: 0,
    //         size: 512,
    //         digest: 0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560,
    //       },
    //       {
    //         id: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b),
    //         offset: 512,
    //         size: 188,
    //         digest: 0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6,
    //       },
    //     ],
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   size: 500
    //   blocksize: 512
    //   blocks: [
    //     [
    //       {
    //         id: ext(2, 0xad67b6b5b9ad4653bf8e2b405bb6115f),
    //         start: 0,
    //         stop: 250,
    //         raw_offset: 0,
    //         raw_size: 512,
    //         access: { id: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //           offset: 0,
    //           size: 512,
    //           digest: 0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560,
    //         },
    //       },
    //       {
    //         id: ext(2, 0x2f99258022a94555b3109e81d34bdf97),
    //         start: 250,
    //         stop: 500,
    //         raw_offset: 250,
    //         raw_size: 250,
    //         access: None,
    //       },
    //     ],
    //   ]
    let data = &hex!(
    "9c71be661347b29f96b31b32976c4bce3997dbc1f6033999dc2ffd14c73c5c2e81db1f"
    "59735067b4c7e1afc74bfa35b5395b2371e30c3b3945265ea2df504ecc82e0cfb2395e"
    "c93cf04e92516e3c543b612dd8e42a2466fdb98505f78baeba6d5dad78ee97f25e5513"
    "bff5bf471822e5210adb9caa4e01acdbd8b7e959d678d1815a437e33c57c431c433375"
    "ea491c086d095beca33d74d8ba5b9ee70cd1e91259e431b486eb6b9fe8644e5d323542"
    "f157ca168ba2809e368a1299a6f24b0e58d5704e9d5838312c0b41d012c79d7da356cb"
    "e1ffc4b0eab584dcf48600fd0a40ef622c96fab9983040db9b17e4e14cf1a8709d1990"
    "91624259e21e78e813f34b999a15fbc5defc68caac902a4497eec7a6acee6b1904cba2"
    "4b845aab519deabdaa57f8b353e43ada8c32dcc1f86c2c31e64f8db4f1ed79cf0e752a"
    "605833a2069f28e37d1a9a32ac97bbfae1b4740ee93097d4a5fb2cec89b1e84eca3380"
    "2e04bc5a53af9b44a3ac921166aa474d6baf9b81dc9538833f25e77263553cbbc0cd24"
    "86e0a19e70fb7effb13640e4e9e0f09c3cf568ef74f74f7ead10c8a61e566f841d9362"
    "1ae29b363411bac33da28aa120d942ab96a12d92399e6a593e39a37056271e8355eba2"
    "8e384f90787165215c4e5ebef6492f4af1aa7a6bdb78cc40e061a1f78e6f3e96db4cad"
    "48f5291e3c23fd929876f1a5d064"
    )[..];
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let expected = LocalFileManifest {
        parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
        updated: now,
        base: FileManifest {
            author: alice.device_id,
            timestamp: now,
            id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
            version: 42,
            created: now,
            updated: now,
            blocks: vec![
                BlockAccess {
                    id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    digest: HashDigest::from(hex!(
                        "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                    )),
                    offset: 0,
                    size: NonZeroU64::try_from(512).unwrap(),
                },
                BlockAccess {
                    id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    digest: HashDigest::from(hex!(
                        "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                    )),
                    offset: 512,
                    size: NonZeroU64::try_from(188).unwrap(),
                },
            ],
            blocksize: Blocksize::try_from(512).unwrap(),
            parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
            size: 700,
        },
        blocks: vec![vec![
            ChunkView {
                id: ChunkID::from_hex("ad67b6b5b9ad4653bf8e2b405bb6115f").unwrap(),
                access: Some(BlockAccess {
                    id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    digest: HashDigest::from(hex!(
                        "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f3"
                        "6560"
                    )),
                    offset: 0,
                    size: NonZeroU64::try_from(512).unwrap(),
                }),
                raw_offset: 0,
                raw_size: NonZeroU64::new(512).unwrap(),
                start: 0,
                stop: NonZeroU64::new(250).unwrap(),
            },
            ChunkView {
                id: ChunkID::from_hex("2f99258022a94555b3109e81d34bdf97").unwrap(),
                access: None,
                raw_offset: 250,
                raw_size: NonZeroU64::new(250).unwrap(),
                start: 250,
                stop: NonZeroU64::new(500).unwrap(),
            },
        ]],
        blocksize: Blocksize::try_from(512).unwrap(),
        need_sync: true,
        size: 500,
    };
    let manifest = LocalChildManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, LocalChildManifest::File(expected.clone()));

    // Also test serialization round trip
    let file_manifest: LocalFileManifest = manifest.try_into().unwrap();
    let data2 = file_manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalChildManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, LocalChildManifest::File(expected));
}

#[rstest]
fn serde_local_file_manifest_invalid_blocksize() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_file_manifest'
    //   base: {
    //     type: 'file_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     size: 0,
    //     blocksize: 512,
    //     blocks: [ ],
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   size: 500
    //   blocksize: 2
    //   blocks: [ ]
    let data = &hex!(
    "80a5fde9bf5bdda170b7492482dec446f38d549bf33447f5af08e4015f38f3c3500111"
    "e76b1a629fc385e3687a2fdbaadca9b513540a49dca401caf7b8ad09337b49789c321c"
    "7edec8924ed65bc58eb792bf6eb173de45755b4b70b3bedfa4bbe888e10d44bbf6be48"
    "13745e17a8a749121a0433348acce6741e94ce58c122a2e6f423e91df5174e6670b1e5"
    "6b3266c5d3d6206dd9b8bb3099294ed601426a8a342a040118a1a84ec4b5910b098791"
    "ea593e958d2abe054b0d564289e4567de31f46e9c3272da6ff83f96c158d9937a6e9f8"
    "eb3656d6e58a462355244c5da1f56e1e4b2e59e641acc10ae83b9d49a984843e21f9f9"
    "4ac720c561d3ad10e73fdf38f4b7791127d861ba1972f28e57da7bf1c6d8"
    )[..];

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    // How to regenerate this test payload ???
    // 1) Disable checks in `Blocksize::try_from` to accept any value
    // 2) uncomment the following code:
    //
    //     let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    //     let expected = LocalFileManifest {
    //         parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
    //         updated: now,
    //         base: FileManifest {
    //             author: "alice@dev1".parse().unwrap(),
    //             timestamp: now,
    //             id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
    //             version: 42,
    //             created: now,
    //             updated: now,
    //             blocks: vec![],
    //             blocksize: Blocksize::try_from(512).unwrap(),
    //             parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
    //             size: 0,
    //         },
    //         blocks: vec![],
    //         blocksize: Blocksize::try_from(2).unwrap(),
    //         need_sync: true,
    //         size: 500,
    //     };
    //
    // 3) Uses `misc/test_expected_payload_cooker.py`

    let outcome = LocalChildManifest::decrypt_and_load(data, &key);
    assert_eq!(
        outcome,
        Err(DataError::BadSerialization {
            format: Some(0),
            step: "msgpack+validation"
        })
    );
}

#[rstest]
fn chunk_new() {
    let chunk_view = ChunkView::new(1, NonZeroU64::try_from(5).unwrap());

    p_assert_eq!(chunk_view.start, 1);
    p_assert_eq!(chunk_view.stop, NonZeroU64::try_from(5).unwrap());
    p_assert_eq!(chunk_view.raw_offset, 1);
    p_assert_eq!(chunk_view.raw_size, NonZeroU64::try_from(4).unwrap());
    p_assert_eq!(chunk_view.access, None);

    p_assert_eq!(chunk_view, 1);
    assert!(chunk_view < 2);
    assert!(chunk_view > 0);
    p_assert_ne!(
        chunk_view,
        ChunkView::new(1, NonZeroU64::try_from(5).unwrap())
    );
}

#[rstest]
fn chunk_promote_as_block() {
    let chunk_view = ChunkView::new(1, NonZeroU64::try_from(5).unwrap());
    let id = chunk_view.id;
    let block = {
        let mut block = chunk_view.clone();
        block.promote_as_block(b"<data>").unwrap();
        block
    };

    p_assert_eq!(block.id, id);
    p_assert_eq!(block.start, 1);
    p_assert_eq!(block.stop, NonZeroU64::try_from(5).unwrap());
    p_assert_eq!(block.raw_offset, 1);
    p_assert_eq!(block.raw_size, NonZeroU64::try_from(4).unwrap());
    p_assert_eq!(*block.access.as_ref().unwrap().id, *id);
    p_assert_eq!(block.access.as_ref().unwrap().offset, 1);
    p_assert_eq!(
        block.access.as_ref().unwrap().size,
        NonZeroU64::try_from(4).unwrap()
    );
    p_assert_eq!(
        block.access.as_ref().unwrap().digest,
        HashDigest::from_data(b"<data>")
    );

    let block_access = BlockAccess {
        id: BlockID::default(),
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(b"<data>"),
    };

    let mut block = ChunkView::from_block_access(block_access);
    let err = block.promote_as_block(b"<data>").unwrap_err();
    p_assert_eq!(err, ChunkViewPromoteAsBlockError::AlreadyPromotedAsBlock);

    let mut chunk_view = ChunkView {
        id,
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 1,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    };

    let err = chunk_view.promote_as_block(b"<data>").unwrap_err();
    p_assert_eq!(err, ChunkViewPromoteAsBlockError::NotAligned);
}

#[rstest]
fn chunk_is_block() {
    let chunk_view = ChunkView {
        id: ChunkID::default(),
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    };

    assert!(chunk_view.is_aligned_with_raw_data());
    assert!(!chunk_view.is_block());

    let mut block = {
        let mut block = chunk_view.clone();
        block.promote_as_block(b"<data>").unwrap();
        block
    };

    assert!(block.is_aligned_with_raw_data());
    assert!(block.is_block());

    block.start = 1;

    assert!(!block.is_aligned_with_raw_data());
    assert!(!block.is_block());

    block.access.as_mut().unwrap().offset = 1;

    assert!(!block.is_aligned_with_raw_data());
    assert!(!block.is_block());

    block.raw_offset = 1;

    assert!(!block.is_aligned_with_raw_data());
    assert!(!block.is_block());

    block.stop = NonZeroU64::try_from(2).unwrap();

    assert!(block.is_aligned_with_raw_data());
    assert!(block.is_block());

    block.stop = NonZeroU64::try_from(5).unwrap();

    assert!(!block.is_aligned_with_raw_data());
    assert!(!block.is_block());

    block.raw_size = NonZeroU64::try_from(4).unwrap();

    assert!(block.is_aligned_with_raw_data());
    assert!(!block.is_block());

    block.access.as_mut().unwrap().size = NonZeroU64::try_from(4).unwrap();

    assert!(block.is_aligned_with_raw_data());
    assert!(block.is_block());
}

#[rstest]
fn local_file_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = VlobID::default();
    let lfm = LocalFileManifest::new(author, parent, timestamp);

    p_assert_eq!(lfm.base.author, author);
    p_assert_eq!(lfm.base.timestamp, timestamp);
    p_assert_eq!(lfm.base.parent, parent);
    p_assert_eq!(lfm.base.version, 0);
    p_assert_eq!(lfm.base.created, timestamp);
    p_assert_eq!(lfm.base.updated, timestamp);
    p_assert_eq!(lfm.base.blocksize, Blocksize::try_from(512 * 1024).unwrap());
    p_assert_eq!(lfm.base.size, 0);
    p_assert_eq!(lfm.base.blocks.len(), 0);
    assert!(lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.blocksize, Blocksize::try_from(512 * 1024).unwrap());
    p_assert_eq!(lfm.size, 0);
    p_assert_eq!(lfm.blocks.len(), 0);
}

#[rstest]
fn local_file_manifest_is_reshaped(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = VlobID::default();
    let mut lfm = LocalFileManifest::new(author, parent, timestamp);

    assert!(lfm.is_reshaped());

    let block = {
        let mut block = ChunkView {
            id: ChunkID::default(),
            start: 0,
            stop: NonZeroU64::try_from(1).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::try_from(1).unwrap(),
            access: None,
        };
        block.promote_as_block(b"<data>").unwrap();
        block
    };

    lfm.blocks.push(vec![block.clone()]);

    assert!(lfm.is_reshaped());

    lfm.blocks[0].push(block);

    assert!(!lfm.is_reshaped());

    lfm.blocks[0].pop();
    lfm.blocks[0][0].access = None;

    assert!(!lfm.is_reshaped());
}

#[rstest]
#[case::empty((0, vec![]))]
#[case::blocks((1024, vec![
    BlockAccess {
        id: BlockID::default(),
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    },
    BlockAccess {
        id: BlockID::default(),
        offset: 513,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    }
]))]
fn local_file_manifest_from_remote(timestamp: DateTime, #[case] input: (u64, Vec<BlockAccess>)) {
    let (size, blocks) = input;
    let fm = FileManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        size,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: blocks.clone(),
    };

    let lfm = LocalFileManifest::from_remote(fm.clone());

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.size, size);
    p_assert_eq!(lfm.blocksize, Blocksize::try_from(512).unwrap());
    p_assert_eq!(
        lfm.blocks,
        blocks
            .into_iter()
            .map(|block| vec![ChunkView::from_block_access(block)])
            .collect::<Vec<_>>()
    );
}

#[rstest]
fn local_file_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let t3 = t2.add_us(1);
    let author = DeviceID::default();
    let parent = VlobID::default();
    let mut lfm = LocalFileManifest::new(author, parent, t1);

    let block = {
        let mut block = ChunkView {
            id: ChunkID::default(),
            start: 0,
            stop: NonZeroU64::try_from(1).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::try_from(1).unwrap(),
            access: None,
        };
        block.promote_as_block(b"<data>").unwrap();
        block
    };

    let block_access = block.access.clone().unwrap();
    lfm.blocks.push(vec![block]);
    lfm.size = 1;
    lfm.updated = t2;

    let author = DeviceID::default();
    let fm = lfm.to_remote(author, t3).unwrap();

    p_assert_eq!(fm.author, author);
    p_assert_eq!(fm.timestamp, t3);
    p_assert_eq!(fm.id, lfm.base.id);
    p_assert_eq!(fm.parent, lfm.base.parent);
    p_assert_eq!(fm.version, lfm.base.version + 1);
    p_assert_eq!(fm.created, lfm.base.created);
    p_assert_eq!(fm.updated, lfm.updated);
    p_assert_eq!(fm.size, lfm.size);
    p_assert_eq!(fm.blocksize, lfm.blocksize);
    p_assert_eq!(fm.blocks, vec![block_access]);
}

// TODO: Add integrity tests for:
// - `LocalFileManifest` with the following failing invariants:
//   * blocks belong to their corresponding block span
//   * blocks do not overlap
//   * blocks do not go passed the file size
//   * blocks do not share the same block span
//   * blocks not span over multiple block spans
//   * blocks are  internally consistent
//   * the manifest ID is different from the parent ID
