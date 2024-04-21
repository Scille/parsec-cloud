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
    //         offset: 0
    //         size: 512
    //       }
    //       {
    //         id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //         digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
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
    let data = hex!(
        "a4c84fa2b11cbdf2f60a8da8c04c17fb2a23f854d6018f07d19ed6aff2e8d22c15f3c8402a"
        "318c53b21a9a12b9f3d7ede7080ed07bc93237ce18358962a3f10f4733e050294883baaa9a"
        "4074696adfc8377f9590f0e94bc9f81d785a1d9d4f1ae631d43ae9e730fe2f90705ce3337c"
        "5bf993133504a507f9abeb428a70c4d26e86afd3b0b581067d8690b144e94e82c45ae08f79"
        "a9a7a91aa0d29eec3be9c3760deda111797428755a76950fd9d801931df08f815aa58afc35"
        "5ef119e4e0efbebd66794327ff6823d84bbd2324f28fc84d269ce07a5786395bf239ee45d0"
        "ca50c023f8e06f440521595b667507a1610cb42a4c61d2778134a0c86a399a8a18e688d814"
        "e9a416ba8160550ab517627bca81e40466e1c1c318468d20e9b0f6ba7ac58a3072e123ee80"
        "7e66c75f770a78d85088cac2b0a4d2fc6103dc38518c08f9774053e9843576e0359e29b1d2"
        "ab2f01f9306c22841f2343c6cfa2e74378867ac7158459c30886bcfdb28111799e36838867"
        "76e6c9ca87e79bf1b77b86b75aac5199ca3523a5b5a28cc029e9ab2adfaa67352956efe7b7"
        "27f7b6ad741c067679a07a59e3a80dfdf35f21329e0622d509ebb751cbb8ed07cbcbe305ff"
        "dd0257cfe7754d2c64716e0c2d0094b347efe79838c33b3675aac3a2388839c02af48ee0ec"
        "925fa22e8a261491d289e1eb8acae69d2933ffe13ac9ab327ee4f34b2cb679d2c6e8fcf0e6"
        "96e1b783b0152bd311ebca0f98f66e28ad4f65df7d8e28a1c82ebac3d7e1c4b9838b7ddbe7"
        "85f72cf0412268b76d2a28b65df23bbc6480785ce1d083d5050af337efd352c43bdf062c4e"
        "0836247a72368396c0776afdd96a3dcbcd52ab63a84762a55c84386abe455ba4859414f185"
        "6b1ac07e419a279094e24dac07cd374b1a08706f296a6b00eb0bd95898ae073705206ae6da"
        "7e4495f6bf06ed49c56d77c2721e5014fabef7a3db98cceda96bf5615c2e"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let expected = LocalFileManifest {
        updated: now,
        base: FileManifest {
            author: alice.device_id.to_owned(),
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
                    key: None,
                    offset: 0,
                    size: NonZeroU64::try_from(512).unwrap(),
                },
                BlockAccess {
                    id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    digest: HashDigest::from(hex!(
                        "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                    )),
                    key: None,
                    offset: 512,
                    size: NonZeroU64::try_from(188).unwrap(),
                },
            ],
            blocksize: Blocksize::try_from(512).unwrap(),
            parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
            size: 700,
        },
        blocks: vec![vec![
            Chunk {
                id: ChunkID::from_hex("ad67b6b5b9ad4653bf8e2b405bb6115f").unwrap(),
                access: Some(BlockAccess {
                    id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    digest: HashDigest::from(hex!(
                        "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f3"
                        "6560"
                    )),
                    key: None,
                    offset: 0,
                    size: NonZeroU64::try_from(512).unwrap(),
                }),
                raw_offset: 0,
                raw_size: NonZeroU64::new(512).unwrap(),
                start: 0,
                stop: NonZeroU64::new(250).unwrap(),
            },
            Chunk {
                id: ChunkID::from_hex("2f99258022a94555b3109e81d34bdf97").unwrap(),
                access: None,
                raw_offset: 250,
                raw_size: NonZeroU64::new(250).unwrap(),
                start: 0,
                stop: NonZeroU64::new(250).unwrap(),
            },
        ]],
        blocksize: Blocksize::try_from(512).unwrap(),
        need_sync: true,
        size: 500,
    };

    let manifest = LocalFileManifest::decrypt_and_load(&data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalFileManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, expected);
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
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_folder_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "folder_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {wksp1:ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))}
        //   }
        //   children: {wksp2:ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))}
        //   local_confinement_points: [ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))]
        //   remote_confinement_points: [ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))]
        //   need_sync: true
        //   speculative: false
        &hex!(
            "199d17c0df1c4158f7cf9de4ddac04fc469512e34c9d711d3bce7f0892df3864ef9827"
            "dd1af08207bfc6625619896bb5c8c0a7d2d85580f0e812aa295d0c6cb7f38925c22994"
            "a44b2ed2a2d7b6c993b9d5f16d4ebec4ddcd39f94853b025d7a2115a37c7be2b25550f"
            "d3c5a473ca393cf092b29e76d439f1cb0eed0e1c81aca267065b5f639befdc2f3fe02a"
            "e2008cc185937c7cd491d733135e8ae9d2409587f4cc053e39a9c203e64fd422e3f8b7"
            "a10c68505b378904524cfcb922817d9d37968607e9940aa488ace790d1d1f10e11b359"
            "61832935db003ed513fe3b9d139d355228d401a318ce1fa332088fdaaf1d8d8e39a729"
            "8cec0b5f7ca84530d906878458cfa7aefe538b96b2e8e0e7478cd7e55a9b75642f6def"
            "90d26a0acd21d0803c1895f89fc510bb451bb40f1c4a39cd8200066535e931530d3372"
            "faa9111e08daa13f51ccf6d055c74db7eb31acb1186b70f454adb11333cea3cb57c5fe"
            "166fba7dc830b8951a7c7a87065020d05637be6bb35bea78c89a6f9f8314234b221123"
            "61d6e0a72a8c37acd350d84a769a3cf1b5a11dc5a4b40271ce15aac2f0c234fe4e6516"
            "55ea09"
        )[..],
        LocalFolderManifest {
            updated: now,
            base: FolderManifest {
                author: alice.device_id.to_owned(),
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
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_folder_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   base: {
        //     type: "folder_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
        //     version: 0
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //     children: {}
        //   }
        //   children: {}
        //   local_confinement_points: []
        //   remote_confinement_points: []
        //   need_sync: true
        //   speculative: true
        &hex!(
            "7dce0d23f47e8a33eae42311870f5f030aa2dafea63e695d4eec371105e4a5182d29a5"
            "0f0d667d187e760a36bd429f0df8440dddcfa983987d66e0d2ceeac1238a2d4c47d9a2"
            "e267ee70f6ea84bf48912d92bda0ca6a5a1030d6b9f6b8ad431fd6026e82f4ebb38102"
            "3744cc059cda4c4ee749868e2abd0d920e04c29edc2f2f596f6ba16f0c50a308028dd4"
            "69c4b1265745bc5f2c0355c41f2471d99fe82b081d0da2efff6c4c87ca938159393a10"
            "c2d0250772570caee3d0e63d9a0be05f6533a91740189038dba9b4fa80c867e068bd91"
            "da97c9c4f4d06aff2c74d94e19da4a605631993b71adde221a3c521a26e49ce1928f7e"
            "c38b86f9e6d2b22984eefeab64e42a0f6a83428aaf064a3f72ea8e50709323d36fcffe"
            "56e1f331b7a0eb5cadf4201b072241f9828ea74b01c3a805060e48198537f88d9bb9c5"
            "624260214381e8796b0dea5dda4d43054f250c9f43943953"
        )[..],
        LocalFolderManifest {
            updated: now,
            base: FolderManifest {
                author: alice.device_id.to_owned(),
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

    let manifest = LocalFolderManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalFolderManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, expected);
}

#[rstest]
#[case::need_sync(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: false
        //   local_workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       name_origin: {
        //         type: "CERTIFICATE",
        //         timestamp: ext(1, 1638618643.208821)
        //       }
        //       role: "CONTRIBUTOR"
        //       role_origin: {
        //         type: "CERTIFICATE",
        //         timestamp: ext(1, 1638618643.208821)
        //       }
        //     },
        //     {
        //       name: "wksp2"
        //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
        //       name_origin: {
        //         type: "PLACEHOLDER"
        //       }
        //       role: "CONTRIBUTOR"
        //       role_origin: {
        //         type: "PLACEHOLDER"
        //       }
        //     }
        //   ]
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     created: ext(1, 1638618643.208821)}
        //     updated: ext(1, 1638618643.208821)
        //     version: 42
        //   }
        &hex!(
            "be56b2c031af53f2f006834cb117acbfa1814633a0738b1f33dfddff3a63a3b64deec4"
            "031f9739365ca4971cb40cc209ab167e9cfb69db0922429ae4f0e595d9f4f8eed7846d"
            "7b553ac83256c69a5b46603b43ebe0ee4c70349806a461e180f7d7d12bc483c744195f"
            "fd7a8cb3fd77674371bc8574433ee46e1c18f8a652ff37c435cc5f4d1e380c5902e831"
            "f4e48c8e4a9798c13f5072eaa2730e5b5b06c1ab565ca81eea197e830817a5f1aacd87"
            "f054eeaeb7786a1b94caab10e984f6caa926ec6998478fc27da61a2e853ebf10ca65b0"
            "6610ebf508093dbb38dfccd60d48ae191ae547ad09be84780ff28129b67c509a2bc93f"
            "e6def7c76b8773f34bc7a3ada6c9754ce24dc46f3df055f234bc46345e958cbd82a48b"
            "86766f90973b1f854ad19f693e8d203367b9e7dcb7dc43c62849bbc2c3a8d027471206"
            "3de8398cca20d52543d71a276304fbbe2e7820430f74e39bc0ab6637c4da8e300467ea"
            "4ce5c418d06348ec06ae293e9331e89c6cb429be6ab8ecd0125a1ff516276a3f096601"
            "e8d94614c5368f07221067f8b5b0475ad464f7594a5daa53c4fb4310168701fa4df404"
            "1b7004bef61551d27a444dba384a66c2622d38e1a73fc5b5db1433bb4b731d170db881"
            "56d4cb30723c72f19621b7302fc75f9b3eba2592b5723f67b28d8ff16e76c38418eb93"
            "9af38a2dc18a102be2ec06d979b87f333f7a9d92a92257c149"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: false,
            base: UserManifest {
                author: alice.device_id.to_owned(),
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
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: false
        //   speculative: false
        //   local_workspaces: [
        //     {
        //       name: "wksp1"
        //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //       name_origin: {
        //         type: "PLACEHOLDER"
        //       }
        //       role: "CONTRIBUTOR"
        //       role_origin: {
        //         type: "PLACEHOLDER"
        //       }
        //     },
        //   ]
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 42
        //     created: ext(1, 1638618643.208821)}
        //     updated: ext(1, 1638618643.208821)
        //   }
        &hex!(
            "6645e8c8e9415a69cc47a284dbd6ac1bdc9c66347374b118ba361f9e63cf868edd651d"
            "2f110e8fa3b466a003d7eb0416bf0bc15e6e1cc3699d799b765b919c8f19fc3d3dad8f"
            "a4a1faa0f31f0de7ecc81c120d8b49641c9b21d466db4c204ab5b943e9e91efba84099"
            "9d128a85968103af838ceccb2a102d5ac14216c86df6cf22e1a59b5504dcac1d05f940"
            "d00bd9d5d3ff403be15c7422eb62392c6892fe47eeb453f04300f0306405ef463d2794"
            "8df84b63d5d47e5e78f693c9ec6cd1ffd77e7df72b8c3144c3fb8bc42bf53173ed32ff"
            "edac9e779b09732211400b64407c81c7c5d64fe5465af0cf8f37278ad7c4def9a6d824"
            "a2cb2d4e9b2acbba52ad0c33815ecf01b260e94ad5e4f53eee295f43454d01efe5335f"
            "d02b666d759b7ad25edf1c53a640c008d2af20ddf21e3793b36f6ad0fc44f46caaac25"
            "1438520389da2ef61210288e52e1a1b31d6e7565c9491e3b7873416ec5cd26c74e4c63"
            "6102749d37e9b3424b2ee9a61461c0"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: false,
            speculative: false,
            base: UserManifest {
                author: alice.device_id.to_owned(),
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
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: true
        //   local_workspaces: []
        //   base: {
        //     type: "user_manifest"
        //     author: "alice@dev1"
        //     timestamp: ext(1, 1638618643.208821)
        //     id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
        //     version: 0
        //     created: ext(1, 1638618643.208821)
        //     updated: ext(1, 1638618643.208821)
        //   }
        &hex!(
            "6cf0e1fd8dc90c25e5d27a4179b9953ff5dc4b8f0dc29c005e1c9fe442a7ad83836815"
            "064ceaa35d4dca7ca13095a7ff2ce70b9203f6697e99509bc76e7efee269ab46ca486b"
            "9fb954d28dc2a1bb48e49bafa0f88dd3ab2da9883d69263da18898382c640610ab9748"
            "65036f903a63a64a3ef964ceea4006210a6f1cfffeb33c7d1c575eb35d93fdb1df0cde"
            "d890c39241495805472c1f9ee43aa5f6abb7d13ccfb6540c44d1e2dffc6ec0c6887907"
            "ad810f88dd27cbc95a178680f3e2f00e4f51775b3bd73228dd48eee64b8edf50464606"
            "f0335c3152db0b1bb57235c6e8d9c1ddd269c81a2873426267cccb72869517f97cb134"
            "2e3f2c22c7f7513f6181"
        )[..],
        LocalUserManifest {
            updated: now,
            need_sync: true,
            speculative: true,
            base: UserManifest {
                author: alice.device_id.to_owned(),
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

    println!("{:?}", expected.dump_and_encrypt(&key));
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
    let chunk = Chunk::new(1, NonZeroU64::try_from(5).unwrap());

    p_assert_eq!(chunk.start, 1);
    p_assert_eq!(chunk.stop, NonZeroU64::try_from(5).unwrap());
    p_assert_eq!(chunk.raw_offset, 1);
    p_assert_eq!(chunk.raw_size, NonZeroU64::try_from(4).unwrap());
    p_assert_eq!(chunk.access, None);

    p_assert_eq!(chunk, 1);
    assert!(chunk < 2);
    assert!(chunk > 0);
    p_assert_ne!(chunk, Chunk::new(1, NonZeroU64::try_from(5).unwrap()));
}

#[rstest]
fn chunk_promote_as_block() {
    let chunk = Chunk::new(1, NonZeroU64::try_from(5).unwrap());
    let id = chunk.id;
    let block = {
        let mut block = chunk.clone();
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
    p_assert_eq!(block.access.as_ref().unwrap().key, None);
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
        key: None,
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(b"<data>"),
    };

    let mut block = Chunk::from_block_access(block_access);
    let err = block.promote_as_block(b"<data>").unwrap_err();
    p_assert_eq!(err, "already a block");

    let mut chunk = Chunk {
        id,
        start: 0,
        stop: NonZeroU64::try_from(1).unwrap(),
        raw_offset: 1,
        raw_size: NonZeroU64::try_from(1).unwrap(),
        access: None,
    };

    let err = chunk.promote_as_block(b"<data>").unwrap_err();
    p_assert_eq!(err, "not aligned");
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

    let mut block = {
        let mut block = chunk.clone();
        block.promote_as_block(b"<data>").unwrap();
        block
    };

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
    let parent = VlobID::default();
    let lfm = LocalFileManifest::new(author.clone(), parent, timestamp);

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
        let mut block = Chunk {
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
        key: None,
        offset: 1,
        size: NonZeroU64::try_from(4).unwrap(),
        digest: HashDigest::from_data(&[]),
    },
    BlockAccess {
        id: BlockID::default(),
        key: None,
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
            .map(|block| vec![Chunk::from_block_access(block)])
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
        let mut block = Chunk {
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
    let fm = lfm.to_remote(author.clone(), t3).unwrap();

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
fn local_file_manifest_match_remote(timestamp: DateTime) {
    let fm = FileManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        size: 1,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![BlockAccess {
            id: BlockID::default(),
            key: None,
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
    let parent = VlobID::default();
    let lfm = LocalFolderManifest::new(author.clone(), parent, timestamp);

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
    let lfm = LocalFolderManifest::new_root(author.clone(), realm, timestamp, true);

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
    let fm = lfm.to_remote(author.clone(), timestamp);

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
fn local_folder_manifest_match_remote(timestamp: DateTime) {
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
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    assert!(lfm.match_remote(&fm));
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
    let lum = LocalUserManifest::new(author.clone(), timestamp, Some(id), speculative);

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
    let um = lum.to_remote(author.clone(), timestamp);

    p_assert_eq!(um.author, author);
    p_assert_eq!(um.timestamp, timestamp);
    p_assert_eq!(um.id, lum.base.id);
    p_assert_eq!(um.version, lum.base.version + 1);
    p_assert_eq!(um.created, lum.base.created);
    p_assert_eq!(um.updated, lum.updated);
}

#[rstest]
fn local_user_manifest_match_remote(timestamp: DateTime) {
    let um = UserManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
    };

    let lum = LocalUserManifest {
        base: um.clone(),
        need_sync: false,
        updated: timestamp,
        local_workspaces: vec![],
        speculative: false,
    };

    assert!(lum.match_remote(&um));
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
