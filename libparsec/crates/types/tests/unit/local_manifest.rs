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
        // Generated from Rust implementation (Parsec v3.0.0-alpha)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: false
        //   workspaces: [
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
            "dda242af1e4b65c1e2e3b858588faa1f7132b820d1cab62de4cb066956476551c57c5c"
            "9144183e4720ea536edad40c4878be651a74afa4b2d7153b1fe44fb3f882388a5df429"
            "95b7087b97d781d9cc8ae162d6187cdb6cc7e8942c1301e080e2cce588dd5a3e097f55"
            "1c77ac132119b6597d58f3ad499e51db4f312e8e65877188ad716987a073707f25d7a2"
            "bd58c6267f67b5316227b0ba18a261583de145a93708ac97dfe0d0e5dcb8715a11cded"
            "a022e76c20f2a4e7e5c190d7de5d2c7a3e08e8a15d65cdb1d3ef7a02ae1b55b35bdc6a"
            "178dd5a7819ea76f8002c73a2480f6c71de2aa163b5d8711111212d09b62468b464b92"
            "505ea6778eede30579a3d0158a9c2220f93ea6faec4472b89f91f5f676604c56e68d1f"
            "ee743b8dc7a691c7cd6d404132e4c79ba001f8c6baa3225e75ac1510471cdae5510dc6"
            "36ab526ba7b620c2a047836dfba8ab3ba6c99cab2ceb8c859fff86fc158028e95b87f4"
            "fe95669ddecd8d9c81be86e9f468aa6c3c936cfd8cb68e2914f51e0fd3c37e62ee6d66"
            "58e1f1e96bd6e4574588f30ef2c4b0b0736f3c0c760f387fcb9778b3468ddda3a14b59"
            "daf83c24c9005974ecb846f6515a347685db64fd18a263e92bafa9169d10b6b5937675"
            "0c725087b795a60a675c21da05acf14a3360a75db8a66c390d130f39972388a444292f"
            "295eb85b6d6c2bc610d44d6f0c0191b5cd1d0ef2acca3658b4dc27b820ba06"
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
                workspaces_legacy_initial_info: vec![],
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
        // Generated from Rust implementation (Parsec v3.0.0-alpha)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: false
        //   speculative: false
        //   workspaces: [
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
            "50010cba4177fab8462d492f50904a25ca9b4dab9ca438b3ceee4b60cc041d89778396"
            "8da007b2246b71f092f442763ad2caae31e2c15faedd534d30df64d4668fa966ce368d"
            "05bf6aff136dd4a245f17ebe0421518e2ae15b232b9bea8c2fc5d82247d928db9403f6"
            "adabc4a4a5e03b9bc041110ec02e1b3c527d1ffab2162d3b6320210b9bf79dff082603"
            "043732bb0b12cbb61b2d90935c073816877625759d5f9f489f6b26a485b60cc8cfff78"
            "a491e8a3c5d5ff2dd3b746e2afa58aa37fe85e52e15d045020ae24439a458c5a7aba70"
            "dccbd544fac9153cd7a6a40d9aff2492ec24dfe13f7afda8b63d4d7f31a20d94bfc03c"
            "cf4278d2a49f5a811f293b0e732b1b5e3ee79baf9c29c79e8dd75f1d9485ea8dfa73f4"
            "f5454b79baa4fb556f931ca8da9280b9624c46f1ab78f0f8c597d42483e362e8eb1121"
            "eacfdec9384f743b0da370bb4c147496c5206aa3d7fa5483e9d8a7ed3202fb9822814d"
            "1f6b6a05e2b56816fb8d29e386fd34a37b083db13c"
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
                workspaces_legacy_initial_info: vec![],
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
        // Generated from Rust implementation (Parsec v3.0.0-alpha)
        // Content:
        //   type: "local_user_manifest"
        //   updated: ext(1, 1638618643.208821)
        //   need_sync: true
        //   speculative: true
        //   workspaces: []
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
            "9fed0982212a0cdeeb931d76ccc1e3cc45e70d6a0166ca366fd694031487f2f5187e45"
            "d99a55130b413f8953728ddcffca051b48b3d21487fa15c29977c5a4cdda91c34e13c6"
            "854eb122403b60d2cbd8799680178fbfcd990463fe11791a5c62b964d9b4f699bac938"
            "54494e06e1a848807f727039188c40246519076d626719f21e80bffeb61acdd60bd835"
            "d7941d3e9f18ea1af94e78e5380f086ee84f5be94e400d54cba65314fb9a0e02194d77"
            "43037bb0321e051e33653d3544d468d7cd2adb92aebb978170fc0e268d4bac3c5a960a"
            "22fb6015bc79e6338dc36b786f4e7d49b2d5a41b466a6afe4fca858b90c8e97407e85a"
            "acc575ec"
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
                workspaces_legacy_initial_info: vec![],
            },
            local_workspaces: vec![],
        }
    )
}))]
#[case::legacy_pre_parsec_v3_0_need_sync(Box::new(|alice: &Device| {
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
        //       {
        //         name: "wksp1"
        //         id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //         role: "OWNER"
        //         key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //         encryption_revision: 2
        //         encrypted_on: ext(1, 1638618643.208821)
        //         role_cached_on: ext(1, 1638618643.208821)
        //       }
        //     ]
        //   }
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
            // Local user manifest's `last_processed_message` field is deprecated (and hence ignored)
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 42,
                created: now,
                updated: now,
                workspaces_legacy_initial_info: vec![
                    LegacyUserManifestWorkspaceEntry {
                        name: "wksp1".parse().unwrap(),
                        id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )),
                        encryption_revision: 2,
                        // Fields `encrypted_on/role_cache_timestamp/role_cache_value` are deprecated
                    }
                ],
                // User manifest's `last_processed_message` field is deprecated (and hence ignored)
            },
            local_workspaces: vec![
                LocalUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Owner,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    // Fields `key/encryption_revision/encrypted_on/role_cache_timestamp/role_cache_value` are deprecated
                }
            ],
        }
    )
}))]
#[case::legacy_pre_parsec_v3_0_synced(Box::new(|alice: &Device| {
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
        //       {
        //         name: "wksp1"
        //         id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
        //         role: "OWNER"
        //         key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //         encryption_revision: 2
        //         encrypted_on: ext(1, 1638618643.208821)
        //         role_cached_on: ext(1, 1638618643.208821)
        //       }
        //     ]
        //   }
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
            // Local user manifest's `last_processed_message` field is deprecated (and hence ignored)
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 42,
                created: now,
                updated: now,
                workspaces_legacy_initial_info: vec![
                    LegacyUserManifestWorkspaceEntry {
                        name: "wksp1".parse().unwrap(),
                        id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                        key: SecretKey::from(hex!(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )),
                        encryption_revision: 2,
                    }
                ],
                // User manifest's `last_processed_message` field is deprecated (and hence ignored)
            },
            local_workspaces: vec![
                LocalUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Owner,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    // Fields `key/encryption_revision/encrypted_on/role_cache_timestamp/role_cache_value` are deprecated
                }
            ],
        }
    )
}))]
#[case::legacy_pre_parsec_v3_0_speculative(Box::new(|alice: &Device| {
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
        //   }
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
            // Local user manifest's `last_processed_message` field is deprecated (and hence ignored)
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 0,
                created: now,
                updated: now,
                workspaces_legacy_initial_info: vec![],
                // User manifest's `last_processed_message` field is deprecated (and hence ignored)
            },
            local_workspaces: vec![],
        }
    )
}))]
#[case::legacy_pre_parsec_v1_15_missing_speculative_field(Box::new(|alice: &Device| {
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
            // Local user manifest's `last_processed_message` field is deprecated (and hence ignored)
            local_workspaces: vec![],
            base: UserManifest {
                author: alice.device_id.to_owned(),
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                version: 0,
                created: now,
                updated: now,
                workspaces_legacy_initial_info: vec![],
                // User manifest's `last_processed_message` field is deprecated (and hence ignored)
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
            offset: 0,
            size: NonZeroU64::try_from(1).unwrap(),
            digest: HashDigest::from_data(&[]),
        }],
    };

    let lfm = LocalFileManifest {
        base: fm.clone(),
        parent: fm.parent,
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

    {
        let mut lfm = lfm.clone();
        lfm.parent = VlobID::default();
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.updated = fm.updated.add_us(1);
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.size = fm.size + 1;
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.blocksize = (*fm.blocksize + 1).try_into().unwrap();
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.blocks[0][0].raw_size = lfm.blocks[0][0].raw_size.checked_add(1).unwrap();
        assert!(!lfm.match_remote(&fm));
    }
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
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    assert!(lfm.match_remote(&fm));

    {
        let mut lfm = lfm.clone();
        lfm.parent = VlobID::default();
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.updated = fm.updated.add_us(1);
        assert!(!lfm.match_remote(&fm));
    }
    {
        let mut lfm = lfm.clone();
        lfm.children.insert("foo".parse().unwrap(), VlobID::default());
        assert!(!lfm.match_remote(&fm));
    }
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
    let lum = LocalUserManifest::new(author.clone(), timestamp, Some(id), speculative);

    p_assert_eq!(lum.base.id, id);
    p_assert_eq!(lum.base.author, author);
    p_assert_eq!(lum.base.timestamp, timestamp);
    p_assert_eq!(lum.base.version, 0);
    p_assert_eq!(lum.base.created, timestamp);
    p_assert_eq!(lum.base.updated, timestamp);
    assert!(lum.need_sync);
    p_assert_eq!(lum.updated, timestamp);
    p_assert_eq!(lum.local_workspaces.len(), 0);
    p_assert_eq!(lum.speculative, speculative);
}

#[rstest]
fn local_user_manifest_from_remote(
    #[values(false, true)] with_workspaces_legacy_initial_info: bool,
    timestamp: DateTime,
) {
    let workspaces_legacy_initial_info = {
        let mut workspaces_legacy_initial_info = vec![];
        if with_workspaces_legacy_initial_info {
            workspaces_legacy_initial_info.push(LegacyUserManifestWorkspaceEntry {
                name: "wksp1".parse().unwrap(),
                id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                key: SecretKey::from(hex!(
                    "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                )),
                encryption_revision: 1,
            })
        }
        workspaces_legacy_initial_info
    };

    let um = UserManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        workspaces_legacy_initial_info,
    };

    let lum = LocalUserManifest::from_remote(um.clone());

    p_assert_eq!(lum.base, um);
    assert!(!lum.need_sync);
    p_assert_eq!(lum.updated, timestamp);
    p_assert_eq!(lum.local_workspaces, vec![]);
}

#[rstest]
fn local_user_manifest_to_remote(
    #[values(false, true)] with_workspaces_legacy_initial_info: bool,
    timestamp: DateTime,
) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let id = VlobID::default();
    let speculative = false;
    let lum = {
        let mut lum = LocalUserManifest::new(author, t1, Some(id), speculative);
        lum.local_workspaces.push(LocalUserManifestWorkspaceEntry {
            id: VlobID::default(),
            name: "wksp1".parse().unwrap(),
            name_origin: CertificateBasedInfoOrigin::Placeholder,
            role: RealmRole::Contributor,
            role_origin: CertificateBasedInfoOrigin::Placeholder,
        });
        lum.updated = t2;
        if with_workspaces_legacy_initial_info {
            lum.base
                .workspaces_legacy_initial_info
                .push(LegacyUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    key: SecretKey::from(hex!(
                        "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                    )),
                    encryption_revision: 1,
                });
        }
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
    p_assert_eq!(
        um.workspaces_legacy_initial_info,
        lum.base.workspaces_legacy_initial_info
    );
}

#[rstest]
fn local_user_manifest_match_remote(
    #[values(false, true)] with_workspaces_legacy_initial_info: bool,
    timestamp: DateTime,
) {
    let um = {
        let mut um = UserManifest {
            author: DeviceID::default(),
            timestamp,
            id: VlobID::default(),
            version: 0,
            created: timestamp,
            updated: timestamp,
            workspaces_legacy_initial_info: vec![],
        };
        if with_workspaces_legacy_initial_info {
            um.workspaces_legacy_initial_info
                .push(LegacyUserManifestWorkspaceEntry {
                    name: "wksp1".parse().unwrap(),
                    id: VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                    key: SecretKey::from(hex!(
                        "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                    )),
                    encryption_revision: 1,
                });
        }
        um
    };

    let lum = LocalUserManifest {
        base: um.clone(),
        need_sync: false,
        updated: timestamp,
        local_workspaces: vec![],
        speculative: false,
    };

    assert!(lum.match_remote(&um));

    {
        let mut lum = lum.clone();
        lum.updated = um.updated.add_us(1);
        assert!(!lum.match_remote(&um));
    }
}
