// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
};

use libparsec_client_types::*;
use libparsec_crypto::{prelude::*, *};
use libparsec_types::*;

use tests_fixtures::{alice, timestamp, Device};

type AliceLocalFileManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalFileManifest)>;
type AliceLocalFolderManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalFolderManifest)>;
type AliceLocalWorkspaceManifest =
    Box<dyn FnOnce(&Device) -> (&'static [u8], LocalWorkspaceManifest)>;
type AliceLocalUserManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalUserManifest)>;

#[rstest]
#[case::file_manifest(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_file_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "file_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     blocks: [
        //       {
        //         id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //         digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
        //         key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //         offset: 0
        //         size: 512
        //       }
        //       {
        //         id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
        //         digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
        //         key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
        //         offset: 512
        //         size: 188
        //       }
        //     ]
        //     blocksize: 512
        //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
        //     size: 700
        //   }
        //   blocks: [
        //     [
        //       {
        //         id: ext(2, hex!("ad67b6b5b9ad4653bf8e2b405bb6115f"))
        //         access: {
        //           id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //           digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
        //           key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //           offset: 0
        //           size: 512
        //         }
        //         raw_offset: 0
        //         raw_size: 512
        //         start: 0
        //         stop: 250
        //       }
        //       {
        //         id: ext(2, hex!("2f99258022a94555b3109e81d34bdf97"))
        //         access: None
        //         raw_offset: 250
        //         raw_size: 250
        //         start: 0
        //         stop: 250
        //       }
        //     ]
        //   ]
        //   blocksize: 512
        //   need_sync: true
        //   size: 500
        &hex!(
            "c450757c3d73e4286e1552494251bba10b8cab17c36960c544fad501577b580fe7da7f6159"
            "b5592db42601f13bcf268557de21f99fcf80b97dfe6b180834f791e84f7ce4334751c855ec"
            "c6881e14896f8fd0632fea01976009f913b78641dfc6b6c440fa9e49d2ddc3e1e0302b543a"
            "1c574cbac9c635721aa7ddf427fe9516894db53e9dfc62aeb1aff20bb06c775ca6bf95310c"
            "546ba68680bd532dd8a00b923e675e16fd484d96d08e830fd1f217a8ffe919946b523d3623"
            "75af13648b46abd2a48f6bf7175c899bfaa15653344689189c4eba626092f904d2604605ff"
            "994f45c90e36de0c78597fca533f38c1e8f66e09310922708345cc8fe4225860d45ec3a4ce"
            "11a0fb24953d25aedab9cffdb07e675a02cc0e41df25ee50fb6edcd2dddb58be6f65c6af62"
            "8a46b1bcb079a8ea1c9399c4aaae0f665f7ac842ececf91d0a739401d0635e3ebed48959e4"
            "0498b4e3c32d963b6202a1e1d8e0c99fa6adfcf22626ba5de7d91326d88932a7f4df9c0610"
            "99e69b212296b959e4d6a3cd58a4a6cc4bfc2b2a5b0f490f46fc36f7932eb585cd9ac765ce"
            "1e36730c72498c918514ccf0910f73bda2fea78bfb9cf90a5fe6b099efe95677ea253a6efb"
            "fc0d709d0badf6fe90043433f4a3e8699b747ed079cdff37358ececb0e84b5a597a3edd2c9"
            "26d79bb17607d0fc41c35a5511f84d4b6cb168e5d384f96b86341440a9abe0f6682b148a1e"
            "4926d07f9e408883788a21cf2ffd7dfb930fe26f8a46a4fee80bcbe425dded489dca9f3e7d"
            "45eec851543f35bb1763e3f3a68a54d279b033617339ee9ab6da2db3fb79e5c62c90006ff3"
            "db702f4dc373debfafa3326fe3099768bae557c02837115249a21240af86d227d19c896672"
            "fb6fb729d8131837bfa086682a4299fc173a6b05ce025d8020488625df5931bb1696f06828"
            "d80c868394004b8de95423456a6cef1a8b1a2c9f5836d1a377757d2636c382cc9c6d86e0d7"
            "e4df3752f17e43f5c7e6697cdcf40263f21b9a741aa3dc04bde55e3ea4047f7b8137ab27fc"
            "5706b74e9691788ce3b3d9cc4230bbb5aea343fa0bd13706d002211cc16e29d598cef41085"
            "1777d69068df1250f19518e735cd34f03af9"
        )[..],
        LocalFileManifest {
            updated: now,
            base: FileManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                blocks: vec![
                    BlockAccess {
                        id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                        digest: HashDigest::from(hex!(
                            "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                        )),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )),
                        offset: 0,
                        size: NonZeroU64::try_from(512).unwrap(),
                    },
                    BlockAccess {
                        id: "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap(),
                        digest: HashDigest::from(hex!(
                            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                        )),
                        key: SecretKey::from(hex!(
                            "c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc"
                        )),
                        offset: 512,
                        size: NonZeroU64::try_from(188).unwrap(),
                    }
                ],
                blocksize: Blocksize::try_from(512).unwrap(),
                parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
                size: 700,
            },
            blocks: vec![vec![
                Chunk {
                    id: "ad67b6b5b9ad4653bf8e2b405bb6115f".parse().unwrap(),
                    access: Some(BlockAccess {
                        id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                        digest: HashDigest::from(hex!(
                            "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f3"
                            "6560"
                        )),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5"
                            "ac57"
                        )),
                        offset: 0,
                        size: NonZeroU64::try_from(512).unwrap(),
                    }),
                    raw_offset: 0,
                    raw_size: NonZeroU64::new(512).unwrap(),
                    start: 0,
                    stop: NonZeroU64::new(250).unwrap(),
                },
                Chunk {
                    id: "2f99258022a94555b3109e81d34bdf97".parse().unwrap(),
                    access: None,
                    raw_offset: 250,
                    raw_size: NonZeroU64::new(250).unwrap(),
                    start: 0,
                    stop: NonZeroU64::new(250).unwrap(),
                }
            ]],
            blocksize: Blocksize::try_from(512).unwrap(),
            need_sync: true,
            size: 500,
        }
    )
}))]
fn serde_local_file_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: AliceLocalFileManifest,
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalFileManifest::decrypt_and_load(data, &key).unwrap();

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalFileManifest::decrypt_and_load(&data2, &key).unwrap();

    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_local_file_manifest_invalid_blocksize() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "local_file_manifest"
    //   updated: ext(1, 1638618643.208821)
    //   base: {
    //     type: "file_manifest"
    //     author: "alice@dev1"
    //     timestamp: ext(1, 1638618643.208821)
    //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //     version: 42
    //     created: ext(1, 1638618643.208821)
    //     updated: ext(1, 1638618643.208821)
    //     blocks: []
    //     blocksize: 512
    //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //     size: 700
    //   }
    //   blocks: []
    //   blocksize: 2
    //   need_sync: true
    //   size: 500
    let data = hex!(
        "9642b002d69dca6df364c5828c7308477aba91645175df36e2451798cce2a32b1b68f9138c"
        "24779a4cf009ec1ef21363db76b779e50aaade081b476c9423d033f615a159455c3f79f9ba"
        "eac79059b3aeafda5bb25cf6394b50d1c487fc193740cf207bba4a98803d81c672f05ec2a0"
        "7a88701972bb92cd17400ae25c28344619d5b504b13ff4898831485653b7b260e9622899ac"
        "e85d4897c86b013e2eac1443f1dbd0d3792af7db872c7c4ddece260fa55e24caeeb63b404a"
        "4a775d4ddcfc461b48d4b7773cdf83e0861020c754f51a972bad16c73d5030b7a4f7c6136a"
        "0a8e8eb9664ac5c9b697fd4b693776a6dc6f5eb3b32373d5339c4848ed27cb62f622caffaa"
        "2a67c7a879e877030de49352b649a164902a9c2d74dd84fb84d45a47e811855d369c259bfa"
        "eb7ff946f60d66d48a"
    );

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalFileManifest::decrypt_and_load(&data, &key);

    assert!(manifest.is_err());
}

#[rstest]
#[case::folder_manifest(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_folder_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "folder_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {wksp1:ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))}
        //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
        //   }
        //   children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   local_confinement_points: [ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))]
        //   need_sync: true
        //   remote_confinement_points: [ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))]
        &hex!(
            "282246c0ae88cca5c5b9eed98c0e5dc669c736f798fe61c8c23a399a48588242dd6f8007d2"
            "72aba891e51fcac7f42569067c70fd9a003ebbf5bd93d993cc3250b323472a0941cbcf3209"
            "9a4571e637c85bfb1430954c03490e1e22e481080526f54fb49aab2f454b41b96fe95cdcf8"
            "f1da7eed5813f75f75d67dddee77886c4ed1bcc0b92856c47003b292e5e291b150f226579d"
            "fe54bd5eeb334d83fe4255d83f516d297c20d01931c5d1b80ef07318fa0eea6d0f79d34aac"
            "a63528e519b5305a7e546ddcf2030f2f524cdf20b99398712be017044d899eb84f911d586e"
            "cbfac6223b2d34d8af7b72d92d1aee4c483f9913aee9447848a93fd15c9cfed019f2417b5a"
            "8a1a7c3b3069750fe634c8a5fef093b31763a215db54749d11ac7439d8183dc84e0af45b50"
            "c12746066643a1a664b4ca757447631c7f8262ec461acf0f40bfc529a248881fcaa8afa827"
            "fdeeee2fb5b8f40daf5839936850d086e94c2b441180a029b5b2cf6bef4ca985f94e6da9e3"
            "6786a311afa168c78119c139d5157596d6aae077c4de05b5696874c67acc99726f78d23ff9"
            "a5ad66"
        )[..],
        LocalFolderManifest {
            updated: now,
            base: FolderManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp1".parse().unwrap(), "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap())
                ]),
                parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
            ]),
            local_confinement_points: HashSet::from(["d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap()]),
            remote_confinement_points: HashSet::from(["b82954f1138b4d719b7f5bd78915d20f".parse().unwrap()]),
            need_sync: true,
        }
    )
}))]
#[case::folder_manifest_legacy_no_confinement_fields(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_folder_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "folder_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
        //   }
        //   children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   need_sync: false
        &hex!(
            "9ff2aaf585d985bf38d333b6ac48e345a52c28c4d9645ec7ad8258d4c52c7e36acefd6cbb3"
            "cc5fee4e3a3319f39cc91d9dbd0bcd108d417cf5a7f813cfc4d859585d56bbe7a89aeb9860"
            "3d40226b25a02bafd6a1d3b1a93ee1cc75662e363a14b93df07790cb598da337bd86aa1e1b"
            "078871b89e4254c25f80bff6b3dec3d7838d60d7eb021c7f7cbe7b2554df9f454e1a33be8e"
            "515a4e9fe11c0f034939c41cc2ed3ca0299f0fbb02cbd229a65ecdb401fe733f7d2a1703e4"
            "3f7bbef050707857c57886f5402a1b325c4660000e9c6171ac7e745699477ff271c641bed4"
            "8a3f52cdaa58ab0234977e3eb5f9627dcad489c67a6fdbc46bc2648b213ad243d6b4141e2b"
            "9f67491aa96d9ba9501b8b9fe2788ad687b8adaebce66ca05a89462182e4ba97628b129735"
            "b693adfb83f25cb8d0a87a80787a9dda7a3d0f16c965340f77"
        )[..],
        LocalFolderManifest {
            updated: now,
            base: FolderManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
                ]),
                parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
            ]),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            need_sync: false,
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

    let manifest = LocalFolderManifest::decrypt_and_load(data, &key).unwrap();

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalFolderManifest::decrypt_and_load(&data2, &key).unwrap();

    assert_eq!(manifest2, expected);
}

#[rstest]
#[case::workspace_manifest(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_workspace_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "workspace_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {wksp1:ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))}
        //   }
        //   children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   local_confinement_points: [ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))]
        //   need_sync: true
        //   remote_confinement_points: [ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))]
        //   speculative: false
        &hex!(
            "7d8fd9ebeac56689cf29c1c16370e88d84673121bbc8cc4278ddbed40b612851464f01ccac"
            "53b10aca3c59720968d652c86f1f2ae29387294d4a3f467d86cdb67ee40ac023a72f26c9c8"
            "f785b2b31c20bb279a7c2824fe739b4029819ac59cad0b97cda4abc558916015fb3ea3fdd4"
            "65f9e02168ae6c5c9e6fd4c2b9c2c394b6f355f764300e22e1f49fee02936a45bf494d7819"
            "69c14d843d017269aeea5f69fbf08b9c0d7d7ec12168cbf8eed727cd3708d9752145bef4c5"
            "ca20ae7da5eca90f4e60be5e3375092d4df96dc793469251f6188e6fef1d4128d78c134e9b"
            "eb2134fa7e624528dfaa1c340167982dd7308211933044bbe7380e8f3862adf3a59307a171"
            "8bf50a41095c39ab004db8eafaab0c3c51c3a48fa310488338bd6829532a75864fc437cc64"
            "66992bfbc1f7ef8ea49ac0bef02d2046e6b475e6ef88d54156e18109ed88ba8bad25541be9"
            "ef7d006d6bf79b6d467243aad8df333414a927d5bd35a78358292a6f4bbf63f6901cfcc216"
            "bcd1a8244b875aee764e3265216a212244634171e7954a9895e8ba3972caaf786811"
        )[..],
        LocalWorkspaceManifest {
            updated: now,
            base: WorkspaceManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp1".parse().unwrap(), "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap())
                ]),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
            ]),
            local_confinement_points: HashSet::from(["d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap()]),
            remote_confinement_points: HashSet::from(["b82954f1138b4d719b7f5bd78915d20f".parse().unwrap()]),
            need_sync: true,
            speculative: false,
        }
    )
}))]
#[case::workspace_manifest_speculative(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_workspace_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "workspace_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 0
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {}
        //   }
        //   children: {}
        //   local_confinement_points: []
        //   need_sync: true
        //   remote_confinement_points: []
        //   speculative: false
        &hex!(
            "c33e89c2d046596ce6f30a66ea3f52488d7fa1dd02164cfba435445dc55ffb408da44094be"
            "f5f8ee26e498072f80bc841e77dd98fe42b1499cd97b8883c76589a9613770f1c9f9b7a044"
            "f798a7c73944dd5b4fe1675782049027b32791c5f06d687c589a0ca444080ff26ff912b53f"
            "4878faf3a587b8f9e1924957da366bd59d7defa8954a52bbc2c2af4f3de3f7ed777a76b17b"
            "78b9e0a12604107fb1d0dbd3f9f05e64cf1dffe1f527b6b0d08133d999bed5c9345f7af184"
            "1951554f84037fa9aa28782bfba596ac00b59b38f5ac75753397abee6270800545a413a994"
            "4f408fb51d76b0ce60fccbd49a10ef0736c50e98a2a4c44ccd4917f1bd8aee9923d23e732e"
            "d91fcf90852a7c9567a20c289a874edc5343f8a2c77a070defe074f4564853f5ccedb3238a"
            "5177f593f3dbf6ee8eda2d9721c8c29aea5d57fb2376f5c7"
        )[..],
        LocalWorkspaceManifest {
            updated: now,
            base: WorkspaceManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
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
#[case::workspace_manifest_legacy_no_confinement_and_speculative_fields(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "local_workspace_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "workspace_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   }
        //   children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   need_sync: false
        &hex!(
            "edd81ceda3e081ad6b3329679e55e608943416fecf095ee2613ba33a9d35087a03a728080e"
            "b63d6c1e038cc6a959f90fb24b7d9d224f68b21e8253ff607c2203983b372e6a84113231e3"
            "e8752766d42df2e232c9b154046184bd81f6f2eb1aaad84f0aa098c651a47579f5320ea551"
            "9d700eb6044827e96911ec695aa91a4f686a66ba361df539686a9d13cced69768a78a88e24"
            "6a4a92f86168f2633203919b31d61902cd234b5462c0ca2b84f89428d42e9f1da2e012e940"
            "67dbed57b641e28688f4fe46cec83831dc8d3101fa21541050eea7db4cd3da1c5319baf792"
            "e21bc234b9a681d91ee2a818a7dd67511fd04d0fad40538e89e1981c2d2e3bf364b9641ae7"
            "1872f93eb0a7f160c4e0347166a1026698d9adaaf2299afb96dc6476021d940f8b3957d34d"
            "50dbdb08d9df"
        )[..],
        LocalWorkspaceManifest {
            updated: now,
            base: WorkspaceManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
                ]),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap())
            ]),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            need_sync: false,
            speculative: false,
        }
    )
}))]
fn serde_local_workspace_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: AliceLocalWorkspaceManifest,
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalWorkspaceManifest::decrypt_and_load(data, &key).unwrap();

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalWorkspaceManifest::decrypt_and_load(&data2, &key).unwrap();

    assert_eq!(manifest2, expected);
}

#[rstest]
#[case::need_sync(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: false
        //   last_processed_message: 4
        //   workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       role: "OWNER"
        //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //       encryption_revision: 2
        //       encrypted_on: ext(1, 1638618643.208821)
        //       role_cached_on: ext(1, 1638618643.208821)
        //     },
        //     {
        //       name: "wksp2"
        //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
        //       role: None
        //       key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
        //       encryption_revision: 1
        //       encrypted_on: ext(1, 1638618643.208821)
        //       role_cached_on: ext(1, 1638618643.208821)
        //     }
        //   ]
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     created: ext(1, 1638618643.208821)}
        //     updated: ext(1, 1638618643.208821)
        //     last_processed_message: 3
        //     version: 42
        //     workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       role: "OWNER"
        //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //       encryption_revision: 2
        //       encrypted_on: ext(1, 1638618643.208821)
        //       role_cached_on: ext(1, 1638618643.208821)
        //     }
        //   ]
        &hex!(
            "ed02cad442320f04035cca2c094081b9995189a1645bc55568339a3a9233d91f8c31dee2c7"
            "9890f0c474fb7a799deb0ad4b7ecd102453c33268354e79b612934517d599f8d53a61a9372"
            "3dacae87922feb1a05ab987eb9922fcf751109e6ce279e38d09f6febbf3068b1bfd5390a13"
            "f90b90f8349e8e02e4714689317b96d1778e60735b14978f5e7e2663a21a1e7b31018c1f0e"
            "b3a945b226e0aad02fdca5327b649faa04ad064cb34aa86d5464536878227a504d3a0ffa32"
            "17d364db018d7ea1cf3a24251582ca7f6de30e4feefbe40c09a6dbec96caca55c274043a68"
            "5012c8272e981a2fa06fa4cdde2b5b0884a0ce4598886905b99e0148c5282e6021a57fcd4c"
            "043cd5744abe7ae0a504e616c7853db9df70c48579d084b88b36d485992d181578b467e7e6"
            "bdbb7417df43340548ac1a5b4a4e9f47461be954de7d5e7fb3c9ede4a2abae3a0c4d130a34"
            "d213ad1efcd90747eb5280765c4a1e4a9fc8803bb889b5fa29e748b920c5b0ce1d88506e79"
            "5a65a297682973735e83a59c345607a1066bf64ba08a2364750b6a8a8801b2e9f0a7309d08"
            "d66d8ef1fb69462845e63a51f9c9035cbf0443d60d3114a7390f8a9d58e69b2b94d8a7fdfb"
            "5437e3c516b33a3d07f81bb4d14d61522d437abd0e9979d63564555a6bfc8856d97233f5e2"
            "8baf607b8944fe137e53eb956d78ca800721a6cd34e4f56f3d855e16be29580739b6fe03ac"
            "2ec806f27e8d87d83386a1fc3076526e62bf298b3a2d0ec6e441e08301e4b5a5f8aa72a5ca"
            "51aa3a683064e20a735b531e40c3a3d3e26be61841cfa53b385aee36d899890b42d62a3710"
            "89752739effec2316199a358236bb12af40e0bf9111c8a1667002a1c1a1201df044a4f405b"
            "fa662e4a230b31bb12c44176489dafd34ad40ce121effe130eef81175b37f0eb8bff752025"
            "b73a08470e5869e6c7b90e8a0c061cd66880ed90d6e57f09ff49ff4286ac82c33c7ecb75cd"
            "42abf37aa5abd4a7ef8bc406c022097f9271476503de00b476ccf3e750d9d242c49d7f6e3e"
            "940ef3704fdef14e33f7927786bc501f40"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: false,
            last_processed_message: 4,
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                last_processed_message: 3,
                workspaces: vec![
                    WorkspaceEntry {
                        name: "wksp1".parse().unwrap(),
                        id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )),
                        encryption_revision: 2,
                        encrypted_on: now,
                        role_cached_on: now,
                        role: Some(RealmRole::Owner),
                    },
                ],
            },
            workspaces: vec![
                WorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                    key: SecretKey::from(hex!(
                        "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                    )),
                    encryption_revision: 2,
                    encrypted_on: now,
                    role_cached_on: now,
                    role: Some(RealmRole::Owner),
                },
                WorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap(),
                    key: SecretKey::from(hex!(
                        "c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc"
                    )),
                    encryption_revision: 1,
                    encrypted_on: now,
                    role_cached_on: now,
                    role: None,
                },
            ],
        }
    )
}))]
#[case::synced(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: false
        //   speculative: false
        //   last_processed_message: 3
        //   workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       role: "OWNER"
        //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //       encryption_revision: 2
        //       encrypted_on: ext(1, 1638618643.208821)
        //       role_cached_on: ext(1, 1638618643.208821)
        //     },
        //   ]
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     version: 42
        //     created: ext(1, 1638618643.208821)}
        //     updated: ext(1, 1638618643.208821)
        //     last_processed_message: 3
        //     workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       role: "OWNER"
        //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //       encryption_revision: 2
        //       encrypted_on: ext(1, 1638618643.208821)
        //       role_cached_on: ext(1, 1638618643.208821)
        //     }
        //   ]
        &hex!(
            "07d50897a2530d094f195290dda1e120b6ca52e050aedb0d04dddc5f8d3886751cbdd62d86"
            "928c1052f8317ff32667efdf380fa12f4da793d874d15f0ace61d48df2ab981aaffb7041d7"
            "04fb61f0c540e1e3f7056809a31f499e32a6078be256b40cf0713984bd4040cbbe8f15aad5"
            "94a2f9e0dc9de8b1a354166331532e25b579ee4c84aceee9508877c0be5b6a74178e74c1b5"
            "4033793b6699540acea85d2bddaf6abf5f8ca8609670b3780994cbb408e33daf665f1c872c"
            "33fecc799e64688ba9601c61c6fc90d5645a9fbbd0cbf26d653b1c7ca67c3e9cf43a89597d"
            "ac825bffd8ae7556d8409442e88ee01a92bd7c4bf24a0a6b7756f30b2e5a1d5f26925c7c87"
            "98bd9e48866c2b55de12a1fd4a3f1b89168c90b74350ae3c13a4660c999d835b75b6f043ed"
            "bd961110b85babfc3bf442c1f780d0c8927376681a36c8e96763429e22f280a59aecf611f0"
            "08c96a4fc2d8dd56699217b768d7dcd42262672f73a16b70946748b4b7b18d2976571014f5"
            "a652a1c12749e0255e7052535b4bf838052ab0546cc77796d9ad45db19bc1eed127642316f"
            "0d60c926b93d97f865959dcc297ba924c9b18fe0c091b4178183b0ec9af70cdfbf26625fef"
            "13e8924cf54db0f625973cb1933db92fdc1afb52ffce1261e6b66d2f383f88b877f73d40f6"
            "606b3a8d36a5323793d149c3d731fe3f86c0e1760966a4f4b5f5f2033178a6f9168999e6b3"
            "58fa7879891e323aba8076e0a978cba6d5a9d6edc235184065f9a385a967547b4cfacf5efc"
            "28acfb004e86194a9f9402a423bf21469821ee00ff0a28ab90bfc150682e0ca430987c9dc9"
            "00aac57a581b7420073bef892a066f35c4f5f0"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: false,
            speculative: false,
            last_processed_message: 3,
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 42,
                created: now,
                updated: now,
                last_processed_message: 3,
                workspaces: vec![
                    WorkspaceEntry {
                        name: "wksp1".parse().unwrap(),
                        id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )),
                        encryption_revision: 2,
                        encrypted_on: now,
                        role_cached_on: now,
                        role: Some(RealmRole::Owner),
                    },
                ],
            },
            workspaces: vec![
                WorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                    key: SecretKey::from(hex!(
                        "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                    )),
                    encryption_revision: 2,
                    encrypted_on: now,
                    role_cached_on: now,
                    role: Some(RealmRole::Owner),
                },
            ],
        }
    )
}))]
#[case::speculative(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: true
        //   last_processed_message: 0
        //   workspaces: []
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 0
        //     created: ext(1, 1638618643.208821)}
        //     updated: ext(1, 1638618643.208821)
        //     last_processed_message: 0
        //     workspaces: []
        &hex!(
            "ce0274b3890ec74dff002d7b587e591ba4876b7f08fa227eb1ba737f6eae2490c79c1be9c6"
            "211179274ce62439caeea5829c9265e46371fb6e198f3d13ef5e366e11653a64f049941a1b"
            "6c70ba424019f7e09bcab7f53872cc3d65a9f23b71ec85b2f5b4524f58a03528ccf335af19"
            "8e87fc1f1675da3473bd6a43804b4177267c9ba55c1ec955f5068de1676f9f8731ea4932a0"
            "c55c7eb3741893a5ce7e2c4f20406485821e964b7cf46442338c63771883da9d90098c1fea"
            "73339fa97aba3145977114b5f4c05d3b21733d3a9f476e7abf90602bc7b538ac7794e96e76"
            "c279e236e80564790efc67e61d8196cd4ba1806b7636070976bf4306be2f51fa705cbaa423"
            "890ba6a40b5b33ac75f71dd2a8e1bc93532a3bbee659da075245905b5c46583e0479e8c600"
            "60e0e8bfaa94af4c32a201f6b0"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: true,
            last_processed_message: 0,
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 0,
                created: now,
                updated: now,
                last_processed_message: 0,
                workspaces: vec![],
            },
            workspaces: vec![],
        }
    )
}))]
#[case::legacy_missing_speculative_field(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Python implementation (Parsec v2.6.0)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: false
        //   last_processed_message: 0
        //   workspaces: []
        //   base: {
        //       type: "user_manifest"
        //       author: str(alice.device_id)
        //       timestamp: ext(1, 1638618643.208821)
        //       id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //       version: 0
        //       created: ext(1, 1638618643.208821)
        //       updated: ext(1, 1638618643.208821)
        //       last_processed_message: 0
        //       workspaces: []
        //   }
        &hex!(
            "7d38802a10c9b998d3cf460a48792a305113398fda609eb88ed2dbc40ad51098a5847e50a5"
            "8f69bc374fba586d0b60cbc686819e4ae7d1507ed333d5f63caffae84ec5acf40ec7302c0c"
            "62e75407b0820fdcf1f8211143ba0415074033b37b4d6d136a7ba956c4fb0e046499822d37"
            "7d5a82e3155537db73e58c48adbd5ed41abc4d7498c72947219b49ce6de396beb42cef2f03"
            "7e234792cebe3e46726b471ad3d0a1020a1f9d814d359b8763cb86992578c81e4c5f47d523"
            "33694e5df74f303a99c3b744ef0c942074aacf695d4e6eb38952a1ec9b2a414e6a00f2924d"
            "c4af0c349b0fc959a4422b6a92b7f9233403f71b1cb9bd6d367bf336b8acbc4eb82bfdacf9"
            "48272fe53b3ecec4f06be968149c597f1e6afd529c2ece0410ec7cb6042e3eedf966c7248c"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: false,
            speculative: false,
            last_processed_message: 0,
            workspaces: vec![],
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
                version: 0,
                created: now,
                updated: now,
                last_processed_message: 0,
                workspaces: vec![],
            },
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

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalUserManifest::decrypt_and_load(&data2, &key).unwrap();

    assert_eq!(manifest2, expected);
}

#[rstest]
fn chunk_new() {
    let chunk = Chunk::new(1, NonZeroU64::try_from(5).unwrap());

    assert_eq!(chunk.start, 1);
    assert_eq!(chunk.stop, NonZeroU64::try_from(5).unwrap());
    assert_eq!(chunk.raw_offset, 1);
    assert_eq!(chunk.raw_size, NonZeroU64::try_from(4).unwrap());
    assert_eq!(chunk.access, None);

    assert_eq!(chunk, 1);
    assert!(chunk < 2);
    assert!(chunk > 0);
    assert_ne!(chunk, Chunk::new(1, NonZeroU64::try_from(5).unwrap()));
}

#[rstest]
fn chunk_evolve_as_block() {
    let chunk = Chunk::new(1, NonZeroU64::try_from(5).unwrap());
    let id = chunk.id;
    let block = chunk.evolve_as_block(&[]).unwrap();

    assert_eq!(block.id, id);
    assert_eq!(block.start, 1);
    assert_eq!(block.stop, NonZeroU64::try_from(5).unwrap());
    assert_eq!(block.raw_offset, 1);
    assert_eq!(block.raw_size, NonZeroU64::try_from(4).unwrap());
    assert_eq!(*block.access.as_ref().unwrap().id, *id);
    assert_eq!(block.access.as_ref().unwrap().offset, 1);
    assert_eq!(
        block.access.as_ref().unwrap().size,
        NonZeroU64::try_from(4).unwrap()
    );
    assert_eq!(
        block.access.as_ref().unwrap().digest,
        HashDigest::from_data(&[])
    );

    let block_access = BlockAccess {
        id: BlockID::default(),
        key: SecretKey::generate(),
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    };

    let chunk = Chunk::from_block_access(block_access).unwrap();
    let block = chunk.clone().evolve_as_block(&[]).unwrap();
    assert_eq!(chunk, block);

    let chunk = Chunk {
        id,
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 1,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    };

    let err = chunk.evolve_as_block(&[]).unwrap_err();
    assert_eq!(err, "This chunk is not aligned");
}

#[rstest]
fn chunk_is_block() {
    let chunk = Chunk {
        id: ChunkID::default(),
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    };

    assert!(chunk.is_pseudo_block());
    assert!(!chunk.is_block());

    let mut block = chunk.evolve_as_block(&[]).unwrap();

    assert!(block.is_pseudo_block());
    assert!(block.is_block());

    block.start = 1;

    assert!(!block.is_pseudo_block());
    assert!(!block.is_block());

    block.access.as_mut().unwrap().offset = 1;

    assert!(!block.is_pseudo_block());
    assert!(!block.is_block());

    block.raw_offset = 1;

    assert!(!block.is_pseudo_block());
    assert!(!block.is_block());

    block.stop = NonZeroU64::try_from(2).unwrap();

    assert!(block.is_pseudo_block());
    assert!(block.is_block());

    block.stop = NonZeroU64::try_from(5).unwrap();

    assert!(!block.is_pseudo_block());
    assert!(!block.is_block());

    block.raw_size = NonZeroU64::try_from(4).unwrap();

    assert!(block.is_pseudo_block());
    assert!(!block.is_block());

    block.access.as_mut().unwrap().size = NonZeroU64::try_from(4).unwrap();

    assert!(block.is_pseudo_block());
    assert!(block.is_block());
}

#[rstest]
fn local_file_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = EntryID::default();
    let blocksize = Blocksize::try_from(512).unwrap();
    let lfm = LocalFileManifest::new(author.clone(), parent, timestamp, blocksize);

    assert_eq!(lfm.base.author, author);
    assert_eq!(lfm.base.timestamp, timestamp);
    assert_eq!(lfm.base.parent, parent);
    assert_eq!(lfm.base.version, 0);
    assert_eq!(lfm.base.created, timestamp);
    assert_eq!(lfm.base.updated, timestamp);
    assert_eq!(lfm.base.blocksize, blocksize);
    assert_eq!(lfm.base.size, 0);
    assert_eq!(lfm.base.blocks.len(), 0);
    assert!(lfm.need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.blocksize, blocksize);
    assert_eq!(lfm.size, 0);
    assert_eq!(lfm.blocks.len(), 0);
}

#[rstest]
fn local_file_manifest_is_reshaped(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = EntryID::default();
    let blocksize = Blocksize::try_from(512).unwrap();
    let mut lfm = LocalFileManifest::new(author, parent, timestamp, blocksize);

    assert!(lfm.is_reshaped());

    let block = Chunk {
        id: ChunkID::default(),
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    }
    .evolve_as_block(&[])
    .unwrap();

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
#[case::blocks((2, vec![
    BlockAccess {
        id: BlockID::default(),
        key: SecretKey::generate(),
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    },
    BlockAccess {
        id: BlockID::default(),
        key: SecretKey::generate(),
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    }
]))]
fn local_file_manifest_from_remote(timestamp: DateTime, #[case] input: (u64, Vec<BlockAccess>)) {
    let (size, blocks) = input;
    let fm = FileManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        size,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: blocks.clone(),
    };

    let lfm = LocalFileManifest::from_remote(fm.clone()).unwrap();

    assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.size, size);
    assert_eq!(lfm.blocksize, Blocksize::try_from(512).unwrap());
    assert_eq!(
        lfm.blocks,
        blocks
            .into_iter()
            .map(|block| vec![Chunk::from_block_access(block).unwrap()])
            .collect::<Vec<_>>()
    );
}

#[rstest]
fn local_file_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let t3 = t2.add_us(1);
    let author = DeviceID::default();
    let parent = EntryID::default();
    let blocksize = Blocksize::try_from(512).unwrap();
    let mut lfm = LocalFileManifest::new(author, parent, t1, blocksize);

    let block = Chunk {
        id: ChunkID::default(),
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 0,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    }
    .evolve_as_block(&[])
    .unwrap();

    let block_access = block.access.clone().unwrap();
    lfm.blocks.push(vec![block]);
    lfm.size = 1;
    lfm.updated = t2;

    let author = DeviceID::default();
    let fm = lfm.to_remote(author.clone(), t3).unwrap();

    assert_eq!(fm.author, author);
    assert_eq!(fm.timestamp, t3);
    assert_eq!(fm.id, lfm.base.id);
    assert_eq!(fm.parent, lfm.base.parent);
    assert_eq!(fm.version, lfm.base.version + 1);
    assert_eq!(fm.created, lfm.base.created);
    assert_eq!(fm.updated, lfm.updated);
    assert_eq!(fm.size, lfm.size);
    assert_eq!(fm.blocksize, lfm.blocksize);
    assert_eq!(fm.blocks, vec![block_access]);
}

#[rstest]
fn local_file_manifest_match_remote(timestamp: DateTime) {
    let fm = FileManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        size: 1,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![BlockAccess {
            id: BlockID::default(),
            key: SecretKey::generate(),
            offset: 0,
            size: NonZeroU64::try_from(1).unwrap(),
            digest: HashDigest::from_data(&[]),
        }],
    };

    let lfm = LocalFileManifest {
        base: fm.clone(),
        need_sync: false,
        updated: timestamp,
        size: fm.size,
        blocksize: fm.blocksize,
        blocks: vec![vec![Chunk {
            id: ChunkID::default(),
            start: 0,
            stop: NonZeroU64::try_from(1).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::try_from(1).unwrap(),
            access: Some(fm.blocks[0].clone()),
        }]],
    };

    assert!(lfm.match_remote(&fm));
}

#[rstest]
fn local_folder_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = EntryID::default();
    let lfm = LocalFolderManifest::new(author.clone(), parent, timestamp);

    assert_eq!(lfm.base.author, author);
    assert_eq!(lfm.base.timestamp, timestamp);
    assert_eq!(lfm.base.parent, parent);
    assert_eq!(lfm.base.version, 0);
    assert_eq!(lfm.base.created, timestamp);
    assert_eq!(lfm.base.updated, timestamp);
    assert!(lfm.need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.children.len(), 0);
    assert_eq!(lfm.local_confinement_points.len(), 0);
    assert_eq!(lfm.remote_confinement_points.len(), 0);
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
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::new(),
    1,
    ".+",
))]
#[case::children((
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    0,
    ".mp4",
))]
fn local_folder_manifest_from_remote(
    timestamp: DateTime,
    #[case] input: (
        HashMap<EntryName, EntryID>,
        HashMap<EntryName, EntryID>,
        usize,
        &str,
    ),
) {
    let (children, expected_children, filtered, regex) = input;
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm = LocalFolderManifest::from_remote(fm.clone(), &Regex::from_regex_str(regex).unwrap());

    assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.children, expected_children);
    assert_eq!(lfm.local_confinement_points.len(), 0);
    assert_eq!(lfm.remote_confinement_points.len(), filtered);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, 0, false, "")]
#[case::children_filtered(
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
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
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::new(),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    0,
    0,
    false,
    ".mp4",
)]
#[case::children_merged(
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap()),
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    0,
    1,
    false,
    ".mp4",
)]
#[case::need_sync(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    0,
    0,
    true,
    ".png",
)]
fn local_folder_manifest_from_remote_with_local_context(
    timestamp: DateTime,
    #[case] children: HashMap<EntryName, EntryID>,
    #[case] local_children: HashMap<EntryName, EntryID>,
    #[case] expected_children: HashMap<EntryName, EntryID>,
    #[case] filtered: usize,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        need_sync: false,
        updated: timestamp,
        children: local_children.clone(),
        local_confinement_points: local_children.into_values().collect(),
        remote_confinement_points: HashSet::new(),
    };

    let lfm = LocalFolderManifest::from_remote_with_local_context(
        fm.clone(),
        &Regex::from_regex_str(regex).unwrap(),
        &lfm,
        timestamp,
    );

    assert_eq!(lfm.base, fm);
    assert_eq!(lfm.need_sync, need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.children, expected_children);
    assert_eq!(lfm.local_confinement_points.len(), merged);
    assert_eq!(lfm.remote_confinement_points.len(), filtered);
}

#[rstest]
fn local_folder_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let parent = EntryID::default();
    let mut lfm = LocalFolderManifest::new(author, parent, t1);

    lfm.children
        .insert("file1.png".parse().unwrap(), EntryID::default());
    lfm.updated = t2;

    let author = DeviceID::default();
    let fm = lfm.to_remote(author.clone(), timestamp);

    assert_eq!(fm.author, author);
    assert_eq!(fm.timestamp, timestamp);
    assert_eq!(fm.id, lfm.base.id);
    assert_eq!(fm.parent, lfm.base.parent);
    assert_eq!(fm.version, lfm.base.version + 1);
    assert_eq!(fm.created, lfm.base.created);
    assert_eq!(fm.updated, lfm.updated);
    assert_eq!(fm.children, lfm.children);
}

#[rstest]
fn local_folder_manifest_match_remote(timestamp: DateTime) {
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
    };

    assert!(lfm.match_remote(&fm));
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, false, "")]
#[case::no_data(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    0,
    false,
    ".mp4",
)]
#[case::data(
    HashMap::from([
        ("file1.png".parse().unwrap(), Some("936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())),
        ("file2.mp4".parse().unwrap(), Some("3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap()),
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    1,
    true,
    ".png",
)]
fn local_folder_manifest_evolve_children_and_mark_updated(
    timestamp: DateTime,
    #[case] data: HashMap<EntryName, Option<EntryID>>,
    #[case] children: HashMap<EntryName, EntryID>,
    #[case] expected_children: HashMap<EntryName, EntryID>,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let prevent_sync_pattern = Regex::from_regex_str(regex).unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        need_sync: false,
        updated: timestamp,
        children,
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
    }
    .evolve_children_and_mark_updated(data, &prevent_sync_pattern, timestamp);

    assert_eq!(lfm.base, fm);
    assert_eq!(lfm.need_sync, need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.children, expected_children);
    assert_eq!(lfm.local_confinement_points.len(), merged);
    assert_eq!(lfm.remote_confinement_points.len(), 0);
}

// TODO
#[rstest]
fn local_folder_manifest_apply_prevent_sync_pattern(timestamp: DateTime) {
    let prevent_sync_pattern = Regex::from_regex_str("").unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        parent: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
    }
    .apply_prevent_sync_pattern(&prevent_sync_pattern, timestamp);

    assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    assert_eq!(lfm.updated, timestamp);
    assert_eq!(lfm.children, HashMap::new());
    assert_eq!(lfm.local_confinement_points, HashSet::new());
    assert_eq!(lfm.remote_confinement_points, HashSet::new());
}

#[rstest]
fn local_workspace_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let id = EntryID::default();
    let speculative = false;
    let lwm = LocalWorkspaceManifest::new(author.clone(), timestamp, Some(id), speculative);

    assert_eq!(lwm.base.id, id);
    assert_eq!(lwm.base.author, author);
    assert_eq!(lwm.base.timestamp, timestamp);
    assert_eq!(lwm.base.version, 0);
    assert_eq!(lwm.base.created, timestamp);
    assert_eq!(lwm.base.updated, timestamp);
    assert!(lwm.need_sync);
    assert_eq!(lwm.updated, timestamp);
    assert_eq!(lwm.children.len(), 0);
    assert_eq!(lwm.local_confinement_points.len(), 0);
    assert_eq!(lwm.remote_confinement_points.len(), 0);
    assert_eq!(lwm.speculative, speculative);
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
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::new(),
    1,
    ".+",
))]
#[case::children((
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    0,
    ".mp4",
))]
fn local_workspace_manifest_from_remote(
    timestamp: DateTime,
    #[case] input: (
        HashMap<EntryName, EntryID>,
        HashMap<EntryName, EntryID>,
        usize,
        &str,
    ),
) {
    let (children, expected_children, filtered, regex) = input;
    let wm = WorkspaceManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lwm =
        LocalWorkspaceManifest::from_remote(wm.clone(), &Regex::from_regex_str(regex).unwrap());

    assert_eq!(lwm.base, wm);
    assert!(!lwm.need_sync);
    assert_eq!(lwm.updated, timestamp);
    assert_eq!(lwm.children, expected_children);
    assert_eq!(lwm.local_confinement_points.len(), 0);
    assert_eq!(lwm.remote_confinement_points.len(), filtered);
    assert!(!lwm.speculative);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, 0, false, "")]
#[case::children_filtered(
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
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
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::new(),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    0,
    0,
    false,
    ".mp4",
)]
#[case::children_merged(
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap()),
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    0,
    1,
    false,
    ".mp4",
)]
#[case::need_sync(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    0,
    0,
    true,
    ".png",
)]
#[allow(clippy::too_many_arguments)]
fn local_workspace_manifest_from_remote_with_local_context(
    timestamp: DateTime,
    #[case] children: HashMap<EntryName, EntryID>,
    #[case] local_children: HashMap<EntryName, EntryID>,
    #[case] expected_children: HashMap<EntryName, EntryID>,
    #[case] filtered: usize,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let wm = WorkspaceManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lwm = LocalWorkspaceManifest {
        base: wm.clone(),
        need_sync: false,
        updated: timestamp,
        children: local_children.clone(),
        local_confinement_points: local_children.into_values().collect(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let lwm = LocalWorkspaceManifest::from_remote_with_local_context(
        wm.clone(),
        &Regex::from_regex_str(regex).unwrap(),
        &lwm,
        timestamp,
    );

    assert_eq!(lwm.base, wm);
    assert_eq!(lwm.need_sync, need_sync);
    assert_eq!(lwm.updated, timestamp);
    assert_eq!(lwm.children, expected_children);
    assert_eq!(lwm.local_confinement_points.len(), merged);
    assert_eq!(lwm.remote_confinement_points.len(), filtered);
    assert!(!lwm.speculative);
}

#[rstest]
fn local_workspace_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let id = EntryID::default();
    let speculative = false;
    let mut lwm = LocalWorkspaceManifest::new(author, t1, Some(id), speculative);

    lwm.children
        .insert("file1.png".parse().unwrap(), EntryID::default());
    lwm.updated = t2;

    let author = DeviceID::default();
    let wm = lwm.to_remote(author.clone(), timestamp);

    assert_eq!(wm.author, author);
    assert_eq!(wm.timestamp, timestamp);
    assert_eq!(wm.id, lwm.base.id);
    assert_eq!(wm.version, lwm.base.version + 1);
    assert_eq!(wm.created, lwm.base.created);
    assert_eq!(wm.updated, lwm.updated);
    assert_eq!(wm.children, lwm.children);
}

#[rstest]
fn local_workspace_manifest_match_remote(timestamp: DateTime) {
    let wm = WorkspaceManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lwm = LocalWorkspaceManifest {
        base: wm.clone(),
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    assert!(lwm.match_remote(&wm));
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, false, "")]
#[case::no_data(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    0,
    false,
    ".mp4",
)]
#[case::data(
    HashMap::from([
        ("file1.png".parse().unwrap(), Some("936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap())),
        ("file2.mp4".parse().unwrap(), Some("3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), "936DA01F9ABD4d9d80C702AF85C822A8".parse().unwrap()),
        ("file2.mp4".parse().unwrap(), "3DF3AC53967C43D889860AE2F459F42B".parse().unwrap()),
    ]),
    1,
    true,
    ".png",
)]
fn local_workspace_manifest_evolve_children_and_mark_updated(
    timestamp: DateTime,
    #[case] data: HashMap<EntryName, Option<EntryID>>,
    #[case] children: HashMap<EntryName, EntryID>,
    #[case] expected_children: HashMap<EntryName, EntryID>,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let prevent_sync_pattern = Regex::from_regex_str(regex).unwrap();
    let wm = WorkspaceManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lwm = LocalWorkspaceManifest {
        base: wm.clone(),
        need_sync: false,
        updated: timestamp,
        children,
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .evolve_children_and_mark_updated(data, &prevent_sync_pattern, timestamp);

    assert_eq!(lwm.base, wm);
    assert_eq!(lwm.need_sync, need_sync);
    assert_eq!(lwm.updated, timestamp);
    assert_eq!(lwm.children, expected_children);
    assert_eq!(lwm.local_confinement_points.len(), merged);
    assert_eq!(lwm.remote_confinement_points.len(), 0);
}

// TODO
#[rstest]
fn local_workspace_manifest_apply_prevent_sync_pattern(timestamp: DateTime) {
    let prevent_sync_pattern = Regex::from_regex_str("").unwrap();

    let wm = WorkspaceManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lwm = LocalWorkspaceManifest {
        base: wm.clone(),
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .apply_prevent_sync_pattern(&prevent_sync_pattern, timestamp);

    assert_eq!(lwm.base, wm);
    assert!(!lwm.need_sync);
    assert_eq!(lwm.updated, timestamp);
    assert_eq!(lwm.children, HashMap::new());
    assert_eq!(lwm.local_confinement_points, HashSet::new());
    assert_eq!(lwm.remote_confinement_points, HashSet::new());
}

#[rstest]
fn local_user_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let id = EntryID::default();
    let speculative = false;
    let lum = LocalUserManifest::new(author.clone(), timestamp, Some(id), speculative);

    assert_eq!(lum.base.id, id);
    assert_eq!(lum.base.author, author);
    assert_eq!(lum.base.timestamp, timestamp);
    assert_eq!(lum.base.version, 0);
    assert_eq!(lum.base.created, timestamp);
    assert_eq!(lum.base.updated, timestamp);
    assert!(lum.need_sync);
    assert_eq!(lum.updated, timestamp);
    assert_eq!(lum.workspaces.len(), 0);
    assert_eq!(lum.speculative, speculative);
}

#[rstest]
#[case::empty((0, vec![]))]
#[case::last_processed_message((10, vec![]))]
#[case::workspaces((0, vec![WorkspaceEntry::generate("alice".parse().unwrap(), "2000-01-01T00:00:00Z".parse().unwrap())]))]
fn local_user_manifest_from_remote(timestamp: DateTime, #[case] input: (u64, Vec<WorkspaceEntry>)) {
    let (last_processed_message, workspaces) = input;
    let um = UserManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        last_processed_message,
        workspaces,
    };

    let lum = LocalUserManifest::from_remote(um.clone());

    assert_eq!(lum.base, um);
    assert!(!lum.need_sync);
    assert_eq!(lum.updated, timestamp);
    assert_eq!(lum.workspaces, um.workspaces);
}

#[rstest]
fn local_user_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let t3 = t2.add_us(1);
    let author = DeviceID::default();
    let id = EntryID::default();
    let speculative = false;
    let mut lum = LocalUserManifest::new(author, t1, Some(id), speculative);

    lum.workspaces
        .push(WorkspaceEntry::generate("alice".parse().unwrap(), t2));
    lum.updated = t3;

    let author = DeviceID::default();
    let um = lum.to_remote(author.clone(), timestamp);

    assert_eq!(um.author, author);
    assert_eq!(um.timestamp, timestamp);
    assert_eq!(um.id, lum.base.id);
    assert_eq!(um.version, lum.base.version + 1);
    assert_eq!(um.created, lum.base.created);
    assert_eq!(um.updated, lum.updated);
    assert_eq!(um.workspaces, lum.workspaces);
}

#[rstest]
fn local_user_manifest_match_remote(timestamp: DateTime) {
    let um = UserManifest {
        author: DeviceID::default(),
        timestamp,
        id: EntryID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        last_processed_message: 0,
        workspaces: vec![],
    };

    let lum = LocalUserManifest {
        base: um.clone(),
        need_sync: false,
        updated: timestamp,
        last_processed_message: 0,
        workspaces: vec![],
        speculative: false,
    };

    assert!(lum.match_remote(&um));
}
