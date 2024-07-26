// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
};

use crate::fixtures::{alice, timestamp, Device};
use crate::prelude::*;
use libparsec_tests_lite::prelude::*;

type AliceLocalFolderManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalFolderManifest)>;
type AliceLocalUserManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalUserManifest)>;

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
#[case::folder_manifest(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_folder_manifest'
    //   base:
    //     type: 'folder_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     children: {
    //       wksp1: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //     },
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   children: { wksp2: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b), }
    //   local_confinement_points: [ ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b), ]
    //   remote_confinement_points: [ ext(2, 0xb82954f1138b4d719b7f5bd78915d20f), ]
    //   speculative: False
    &hex!(
    "7335c86779af273389ee30d90a70c9f95e2bc395a97ef804bb84abe0f08b6411abe5b7"
    "cb9fda6d14a16375c488254473a60ca456fa1d735bedf2d8f73ab5cdafcbc1121a6def"
    "62c2b7b8e02b35fffc12fc41ee9cfb2b6cbdcf2f08b5ed869179d2287b74810714a570"
    "fe81f558dba5a7c86f69008e6b34caef79c3ed8cafa55ba5d5fdc63f3a076f583dae5b"
    "da729b02f0e3481b929182c4a459b95b5e4fd45d6ebc07d0619fc1cecdb01df1a819ff"
    "c532a33768f2971421672cdd3582279350b739735d7027cbe6bba65d60f39d74a63a0d"
    "25acff95df7c87a7022ec6aebd7ff758407ffc82bb203725be5ed6cdd2d90a842cf043"
    "181c322dde2e3ccdcbe59921fe1011b09451fd905dc2c4b6f35fd8e69d003f29752e60"
    "68cff472236aa2a92edfe8d207bd71469207f9b3816c21c88e59e1f16c65197124877a"
    "af1e0b4e5354f483946219287a03b4396bac37549f52b30145067750"
    )[..],
        LocalFolderManifest {
            parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
            updated: now,
            base: FolderManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp1".parse().unwrap(), VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap())
                ]),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap())
            ]),
            local_confinement_points: HashSet::from([VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap()]),
            remote_confinement_points: HashSet::from([VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap()]),
            need_sync: true,
            speculative: false,
        }
    )
}))]
#[case::folder_manifest_speculative(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_folder_manifest'
    //   base: { type: 'folder_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 0,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     children: { },
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   children: { }
    //   local_confinement_points: [ ]
    //   remote_confinement_points: [ ]
    //   speculative: True
    &hex!(
    "9f3fa66cd290bd883d465ffd4d69a0022de3821265ff8a9d73ffe6cec71648b13fd8a1"
    "f12828d8fa7f546e6e3e522db59de612d99eb78e916c27c352e702710f16d0284f2ad7"
    "8a0d637c404c3813cca6192df094581c9536f8c2739a2eac516ad0a46a6e61f5fe06c1"
    "54dc26b88334fbfaaecc1278a9938298275acd178df7320bf07e0d55162213dfce122d"
    "5aee5c0cd336ddb7e19d512bed90c22d7ad4901b80f3929b1668eab56867009c981fa3"
    "2637e55bb4527769882d618fa05158aae94307ee875b3f752cca7acdc6fc92652ab3f9"
    "5fea223fe98aef3c5180e50030c24d8175dd44dc3eeb70713a14073fb0ad390732c440"
    "0c43f1ab71e646212ae8a454221fcf3afa90b1ea7babdb04399319f4ec100308edcf7f"
    "f54217977fcd2784e05e5ae9b5d9"
    )[..],
        LocalFolderManifest {
            parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
            updated: now,
            base: FolderManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
                version: 0,
                created: now,
                updated: now,
                children: HashMap::new(),
            },
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            need_sync: true,
            speculative: true,
        }
    )
}))]
fn serde_local_folder_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: AliceLocalFolderManifest,
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalChildManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, LocalChildManifest::Folder(expected.clone()));

    // Also test serialization round trip
    let folder_manifest: LocalFolderManifest = manifest.try_into().unwrap();
    let data2 = folder_manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalChildManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, LocalChildManifest::Folder(expected));
}

#[rstest]
#[case::need_sync(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_user_manifest'
    //   base: {
    //     type: 'user_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //   }
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   local_workspaces: [
    //     {
    //       id: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //       name: 'wksp1',
    //       name_origin: {
    //         type: 'CERTIFICATE',
    //         timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //       },
    //       role: 'CONTRIBUTOR',
    //       role_origin: {
    //         type: 'CERTIFICATE',
    //         timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //       },
    //     },
    //     {
    //       id: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b),
    //       name: 'wksp2',
    //       name_origin: { type: 'PLACEHOLDER' },
    //       role: 'CONTRIBUTOR',
    //       role_origin: { type: 'PLACEHOLDER' },
    //     },
    //   ]
    //   speculative: False
    &hex!(
    "0f753820f7e25cf7af67272fa482b0d7d9c473f958864fa39568da1da3bfaf7b7b1498"
    "3e0e07cfa3fb9dc9cd16c1fd128a80a344e5546cd93ebb58fec559665d61fe81734748"
    "17fe97e88736b518abd2f75528cd6f08b8aa4ae54bbb87b1e30c1e2a2b3d5be569eb41"
    "7454916eb6360ebd7680f423eef60909bc7cb9e570c20bcb3af6ce2f525c79ffc7e090"
    "ab18993db144f7e1e6a83bfc0602a43443ca423a7f9ea8a8773927c2859f50ef97eea3"
    "96fe3589349f04b384c88b7d983dc27b98adfe042fdbd04f92fa6b4172402f9a4d761f"
    "8ffd72a939ae63f81e355d170f9fcb2fb518b65cb99667d61ecf97886180edfa734f74"
    "53816f16e09b90c10d051acdf2e4b436712d9cff46f07729b3faeaf506f7046df071e7"
    "26635bb62866131a6d5413eb3666498dcdc76e0718f152a55fb12720201e6202ad8213"
    "4ab79f12aaa74ed69002c90c4727863dcc44768d8e02838251c9168466688af704583f"
    "f6"
    )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: false,
            base: UserManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 42,
                created: now,
                updated: now,
            },
            local_workspaces: vec![
                LocalUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap() },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap() },
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                },
            ],
        }
    )
}))]
#[case::synced(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_user_manifest'
    //   base: {
    //     type: 'user_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //   }
    //   need_sync: False
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   local_workspaces: [
    //     {
    //       id: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //       name: 'wksp1',
    //       name_origin: { type: 'PLACEHOLDER' },
    //       role: 'CONTRIBUTOR',
    //       role_origin: { type: 'PLACEHOLDER' },
    //     },
    //   ]
    //   speculative: False
    &hex!(
    "c2fbefd3eb566c936946ff3dba967816e71309af7c08629b388afe0f13c8d576e6fedb"
    "6c222276d37cb75f22f839bfbd988a44bb2bb30d400451992c739a39f5d2257beefce3"
    "66893af3153c02c2f7a07473c1ee9b42158800165a74151139fbadb424315cb20183a1"
    "678204a3ee48d7053d261c7c88d1b00095e1d4c93b857a4873daf8b1f5da9d7d870406"
    "6c53ff3f00d7480fba95faa42b6d453fbd0451c325617212c1e795cca0db31f24bec99"
    "5bccab4975f46cf5fc80c1aea6afeda5f715a97abc7eee902157353033e0c56c72226a"
    "8037e96f4580b9a381e19f924238fd27c40e6fa6f5ab34e05b8cfae95a9b51b459fa48"
    "a1c6f951a362da351a2d4e65fb75268bed8451b4adcd28cb504d286d59caf4b802bf4a"
    "133252a22eb7b511e43acf944890c41a0ded5ce78e356a"
    )[..],
        LocalUserManifest {
            updated: now,
            need_sync: false,
            speculative: false,
            base: UserManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 42,
                created: now,
                updated: now,
            },
            local_workspaces: vec![
                LocalUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                }
            ],
        }
    )
}))]
#[case::speculative(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_user_manifest'
    //   base: {
    //     type: 'user_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     version: 0,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //   }
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   local_workspaces: [ ]
    //   speculative: True
    &hex!(
    "d579acddc4dc982ce72bc15753a3e5743b791813be9790ac984af8e8cd99688ffb1dfe"
    "8b4e6fdbfba6acec7f29d098469186160e9b1d90501a63fea75c90ff2f325a13665e63"
    "bd5c5c9b603470084e6a9767b31d3da2ef27c80f24ae16dc44374d734388caebf02ba9"
    "e069312011883ecde33977890510fbc3cfaf94f6585f3e1314c769d76d0723d8bcc040"
    "14356e64c41360c9e93d6ee62ee666cabc9dc930b691e1d0f2a3cadb5fdbfc3b25541e"
    "a971427811fa39acc4609c938fec7ca457901f1e74b1da623bf63b69f77a84e876936d"
    "7fcead51737ebc8afd6b3d"
    )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: true,
            base: UserManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 0,
                created: now,
                updated: now,
            },
            local_workspaces: vec![],
        }
    )
}))]
fn serde_local_user_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: AliceLocalUserManifest,
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalUserManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalUserManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, expected);
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

#[rstest]
fn local_folder_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = VlobID::default();
    let lfm = LocalFolderManifest::new(author, parent, timestamp);

    p_assert_eq!(lfm.base.author, author);
    p_assert_eq!(lfm.base.timestamp, timestamp);
    p_assert_eq!(lfm.base.parent, parent);
    p_assert_eq!(lfm.base.version, 0);
    p_assert_eq!(lfm.base.created, timestamp);
    p_assert_eq!(lfm.base.updated, timestamp);
    assert!(lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children.len(), 0);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
    assert!(!lfm.speculative);
}

#[rstest]
fn local_folder_manifest_new_root(timestamp: DateTime) {
    let author = DeviceID::default();
    let realm = VlobID::default();
    let lfm = LocalFolderManifest::new_root(author, realm, timestamp, true);

    p_assert_eq!(lfm.base.author, author);
    p_assert_eq!(lfm.base.timestamp, timestamp);
    p_assert_eq!(lfm.base.id, realm);
    p_assert_eq!(lfm.base.parent, realm);
    p_assert_eq!(lfm.base.version, 0);
    p_assert_eq!(lfm.base.created, timestamp);
    p_assert_eq!(lfm.base.updated, timestamp);
    assert!(lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children.len(), 0);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
    assert!(lfm.speculative);
}

#[rstest]
#[case::empty((
    HashMap::new(),
    HashMap::new(),
    0,
    "",
))]
#[case::children_filtered((
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    1,
    ".+",
))]
#[case::children((
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    0,
    ".mp4",
))]
fn local_folder_manifest_from_remote(
    timestamp: DateTime,
    #[case] input: (
        HashMap<EntryName, VlobID>,
        HashMap<EntryName, VlobID>,
        usize,
        &str,
    ),
) {
    let (children, expected_children, filtered, regex) = input;
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm =
        LocalFolderManifest::from_remote(fm.clone(), Some(&Regex::from_regex_str(regex).unwrap()));

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), filtered);
    assert!(!lfm.speculative);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, 0, false, "")]
#[case::children_filtered(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashMap::new(),
    1,
    0,
    false,
    ".+",
)]
#[case::children(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    0,
    0,
    false,
    ".mp4",
)]
#[case::children_merged(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    0,
    1,
    false,
    ".mp4",
)]
#[case::need_sync(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    0,
    0,
    true,
    ".png",
)]
fn local_folder_manifest_from_remote_with_local_context(
    timestamp: DateTime,
    #[case] children: HashMap<EntryName, VlobID>,
    #[case] local_children: HashMap<EntryName, VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] filtered: usize,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: local_children.clone(),
        local_confinement_points: local_children.into_values().collect(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let lfm = LocalFolderManifest::from_remote_with_local_context(
        fm.clone(),
        Some(&Regex::from_regex_str(regex).unwrap()),
        &lfm,
        timestamp,
    );

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), merged);
    p_assert_eq!(lfm.remote_confinement_points.len(), filtered);
    assert!(!lfm.speculative);
}

#[rstest]
fn local_folder_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let parent = VlobID::default();
    let mut lfm = LocalFolderManifest::new(author, parent, t1);

    lfm.children
        .insert("file1.png".parse().unwrap(), VlobID::default());
    lfm.updated = t2;

    let author = DeviceID::default();
    let fm = lfm.to_remote(author, timestamp);

    p_assert_eq!(fm.author, author);
    p_assert_eq!(fm.timestamp, timestamp);
    p_assert_eq!(fm.id, lfm.base.id);
    p_assert_eq!(fm.parent, lfm.base.parent);
    p_assert_eq!(fm.version, lfm.base.version + 1);
    p_assert_eq!(fm.created, lfm.base.created);
    p_assert_eq!(fm.updated, lfm.updated);
    p_assert_eq!(fm.children, lfm.children);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, false, "")]
#[case::no_data(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    0,
    false,
    ".mp4",
)]
#[case::data(
    HashMap::from([
        ("file1.png".parse().unwrap(), Some(VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
        ("file2.mp4".parse().unwrap(), Some(VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    1,
    true,
    ".png",
)]
fn local_folder_manifest_evolve_children_and_mark_updated(
    timestamp: DateTime,
    #[case] data: HashMap<EntryName, Option<VlobID>>,
    #[case] children: HashMap<EntryName, VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let prevent_sync_pattern = Regex::from_regex_str(regex).unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children,
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .evolve_children_and_mark_updated(data, Some(&prevent_sync_pattern), timestamp);

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), merged);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
}

// TODO
#[rstest]
fn local_folder_manifest_apply_prevent_sync_pattern(timestamp: DateTime) {
    let prevent_sync_pattern = Regex::from_regex_str("").unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .apply_prevent_sync_pattern(Some(&prevent_sync_pattern), timestamp);

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, HashMap::new());
    p_assert_eq!(lfm.local_confinement_points, HashSet::new());
    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
}

#[rstest]
fn local_user_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let id = VlobID::default();
    let speculative = false;
    let lum = LocalUserManifest::new(author, timestamp, Some(id), speculative);

    p_assert_eq!(lum.base.id, id);
    p_assert_eq!(lum.base.author, author);
    p_assert_eq!(lum.base.timestamp, timestamp);
    p_assert_eq!(lum.base.version, 0);
    p_assert_eq!(lum.base.created, timestamp);
    p_assert_eq!(lum.base.updated, timestamp);
    assert!(lum.need_sync);
    p_assert_eq!(lum.updated, timestamp);
    p_assert_eq!(lum.speculative, speculative);
}

#[rstest]
fn local_user_manifest_from_remote(timestamp: DateTime) {
    let um = UserManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
    };

    let lum = LocalUserManifest::from_remote(um.clone());

    p_assert_eq!(lum.base, um);
    assert!(!lum.need_sync);
    p_assert_eq!(lum.updated, timestamp);
}

#[rstest]
fn local_user_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let id = VlobID::default();
    let speculative = false;
    let lum = {
        let mut lum = LocalUserManifest::new(author, t1, Some(id), speculative);
        lum.updated = t2;
        lum
    };

    let author = DeviceID::default();
    let um = lum.to_remote(author, timestamp);

    p_assert_eq!(um.author, author);
    p_assert_eq!(um.timestamp, timestamp);
    p_assert_eq!(um.id, lum.base.id);
    p_assert_eq!(um.version, lum.base.version + 1);
    p_assert_eq!(um.created, lum.base.created);
    p_assert_eq!(um.updated, lum.updated);
}

#[rstest]
fn local_user_manifest_get_local_workspace_entry(timestamp: DateTime) {
    let wksp1_id = VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap();
    let wksp2_id = VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap();

    let lum = LocalUserManifest {
        base: UserManifest {
            author: DeviceID::default(),
            timestamp,
            id: VlobID::default(),
            version: 0,
            created: timestamp,
            updated: timestamp,
        },
        need_sync: false,
        updated: timestamp,
        local_workspaces: vec![
            LocalUserManifestWorkspaceEntry {
                name: "wksp1".parse().unwrap(),
                id: wksp1_id,
                name_origin: CertificateBasedInfoOrigin::Certificate {
                    timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
                },
                role: RealmRole::Contributor,
                role_origin: CertificateBasedInfoOrigin::Certificate {
                    timestamp: "2021-12-04T11:50:43.208821Z".parse().unwrap(),
                },
            },
            LocalUserManifestWorkspaceEntry {
                name: "wksp2".parse().unwrap(),
                id: wksp2_id,
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Contributor,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
            },
        ],
        speculative: false,
    };

    p_assert_eq!(lum.get_local_workspace_entry(VlobID::default()), None);
    p_assert_eq!(
        lum.get_local_workspace_entry(wksp2_id),
        Some(&lum.local_workspaces[1])
    );
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
// - `LocalFolderManifest` with the following failing invariants:
//   * the manifest ID is different from the parent ID (when loaded as a child manifest)
