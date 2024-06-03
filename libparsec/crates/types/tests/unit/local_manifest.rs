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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_local_file_manifest_ok(alice: &Device) {
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "local_file_manifest"
    //   parent: ext(2, hex!("40c8fe8cd69742479f418f1a6d54ea7a"))
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
        "231b4a42f4d017b8681c4c9704c8ef47e0cedfe10ed934504fce15552418ffdf4f10e1"
        "4c516af4c9c70193b282c37964371d62b2d544d0d7beccd3878cd9a50e8cf83afbb80b"
        "546e7b97e6253ef5fb98a45aa85789d5fa62f6109985dd64d4832118ae855f6dcc43a2"
        "1d8f3cd0256d3dd7235cdb0e999b10b08e6042e80b4df0804a2d31b2579d917959db79"
        "22b757a0686c1a9f145e16ee9db0e424b29b63f7d8c937c39bc6cadc40a05e6d74cdca"
        "f1e3535c88d4461c1494fe377c30224d26733e88024e78ffda037be2ba055025645b25"
        "82ec3527c64128ffeb84407594d89c7d7fb8162b74b44b132c87c44a2dc6932562b768"
        "0ec18f3e5eaa3bd54a14b89525475560307ca33b25cc4879dc3c92cb8233403948b4af"
        "081e37d1c56da35a74bf3e4d7bb8c574b9687d4ee7789848f5eba2ff1048a4e57042b6"
        "ce47da9c808a29b61911693ca62abab710a3e9fe289bfad0292f280980aaefc9fa3e59"
        "788936cd6083c4b2d702cf093828a5b179061122524df95a27669a1a0c6cc0cad560cf"
        "43a54db556b4e04b3de3d0175eee80213649284ee99b4488bdeadcf22e175a03cf469a"
        "7d0daedf7f5e148ef5b76e758e3fc9e778e50470493fc054b6f206c30e15600f682305"
        "a0eb0d3cadcc06224c15845a3cc983181837d39c3c44cb0abd58d76bcbf89c5e9e8359"
        "c49f4c06a17fc3e42f13506ef4e9184401832fc6498fce8534ac1e4f412906ee326942"
        "27a4d5f00414ebb0a6bdab592d70bff9f561b9b413e544d590c9d1f9d73de0abfb63f3"
        "4c509a4e9c546d3b47c46da43a20368e13607ac73688c832415257b46e999b9b0f60b5"
        "cc1f5a59a9235d32d7d0f66733142ae147dca1150510b7ac2dbd42734011794d39fbf5"
        "0090be56ac2562af2b7b94c9935c09aeff0614bfb6db46308e5c5168558d05bb9c6d47"
        "22396f828d62d81332bbaf9df45878b8f89e435ea9b9ec85e174744948d374f3f4cf13"
        "e5ac58fc5c73"
    );
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
            Chunk {
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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_local_file_manifest_invalid_blocksize() {
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "local_file_manifest"
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
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
    //     size: 0
    //   }
    //   blocks: []
    //   blocksize: 2
    //   need_sync: true
    //   size: 0
    let data = hex!(
        "499cd965c263dd61f437d04caec66a0f73a50d2a27ea54aa38a1b45af1bc037911f87c"
        "4487cdb3484f0121d21da7b612bca847ac193b6f0b16a40ebb59fff3d153f9f7ca1582"
        "e4c34450f1b7dcf53a9fa5b2edea8655410a479856865234969fd51cdcfbdde01928df"
        "b88405fd61d3e43b28316680dbbff6a78b97e72e8e3b3f61aa033e6036550e8902ab40"
        "d7393f48a1d1017033af349fff6e718e08b6e9f81a97c18826bb81d22e12a5a7a9bda1"
        "368ec38dd3436dcd7ce8688c7d14560b25117dc44630d7c0431fb0b19b967442fa6697"
        "620be5d2f47eb7e08300b66e7a19902186d516f79f4091e39038f2c19b7430c0623ced"
        "99ba3575ad2f28b4d24d958bd42d20eebb0593b7ccf3e13256974298de75ec793dba99"
        "25942a2a12b9eb91b085a44fdc2ff0f214df58102fe350c139dab6353a18e07a269c78"
        "8eacbf6cc7b9c4698d04b6"
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
        //   parent: ext(2, hex!("40c8fe8cd69742479f418f1a6d54ea7a"))
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
            "9c8a25134ade10150c12d5c9b9f4d47dfd32c887102d9aeecde2bd7f752651e3c5cdf0"
            "ef6bbd9b7c395437af73696b27bfd6fd2e8b0d5dd7269969e1bc3a1e8aa124be6218a1"
            "4e7432abc5d263c641b8b4742d4a3470d415323f967e3825e4aafc02b7aafe7ddd62ff"
            "f0910b66fe1133889a7a583fd1b945eec325f99b0ca5003bf4658355c188625fb1bb13"
            "87b19533eace14045191e35197566ddb1c650bbe6abc4d95ed354242ce55f15e53e354"
            "186eb15c44a0bc78abf94a723613ab3bb58e1186e1c157488c18a615ccb623df06561c"
            "e5919b10c726812366bdac8cda53c8337157848f6ad9dfd531bce00b9c0b19a855a3ee"
            "4ecb1cb6f9b74302674e996d3ddf3530483044754e4767cb6ecfc74ff25519e3bc5593"
            "2bbf94f74bf4664167af157f170d5d5978307018f35edffcab17c1cd617f0d17a3136b"
            "9ed5275ae56349567ce1f7ea5f65a3c4740c259fe50de67b067488548354969ee16f1c"
            "ca6cc763194414a469b875b98e6e82d2a95a0f9d2701844fe8781227a59a607bab8b5d"
            "ce7b8bd2c17590c15718a129a6ee7c6691eff49c4c33e56b5c94d66d005292f68e556e"
            "7d99ce68645d7fda63803581e29f1909e06f67a2783afd77bff4a18a"
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
        // Generated from Parsec v3.0.0-b.6+dev
        // Content:
        //   type: "local_folder_manifest"
        //   parent: ext(2, hex!("40c8fe8cd69742479f418f1a6d54ea7a"))
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
            "c9478625a33eb9516ae53b2b38428ed093ad36091db5305fb70c1cb39810677baed3b0"
            "0ee5b142f1cc3957809ba360af76282b41eb4834262e6afe1b2ec8adcf7d866bfcb4dd"
            "6e2b91bc7316a20250e37a3063d9220dd3b5b93c45dbc7e1a1fbad032c5f6215a10431"
            "ff8ce49dc4692a84794b61a2e2d7c4180b60386eca4789a2fd02e699d889af23a360ad"
            "6a4db25baa9bbcdae8b59542a77770d3488d39d0a1b87f04c3d79dc26397930f7dddcb"
            "1ce7cc31d166150fd7689737235b27a8e1b4136a526523e897f628fb5fbc8df021abbd"
            "7021d02560f048a55e01209ad8c761ef1397ed3360b68fc56911a129c56e9495a62a76"
            "3fef4be06d9c529d1f4a5088532ad92050d3ca2a4023045c81ebf255bc59349c7fc1f9"
            "fa50717fb7a70e4d09544ffd9c89a4ead4cd8eebff83656be1b7ea43bae784e58c940d"
            "63eb7c6c2fa6d95cdc7c35dc1441939196a7d3c2fcca21b6e4a9787da43b7f3e7a732c"
            "5e55cf055f3b954d96a54bfe1c0f"
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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
