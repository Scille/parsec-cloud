// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]

use crate::fixtures::{alice, timestamp, Device};
use crate::prelude::*;
use libparsec_tests_lite::prelude::*;

type AliceLocalUserManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalUserManifest)>;

#[rstest]
#[case::need_sync(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Parsec 3.8.1-a.0+dev
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
        //       archiving_configuration: { type: 'AVAILABLE', },
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
        //       archiving_configuration: { type: 'ARCHIVED', },
        //       id: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b),
        //       name: 'wksp2',
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
        //       archiving_configuration: {
        //         type: 'DELETION_PLANNED',
        //         deletion_date: ext(1, 1735689600000000) i.e. 2025-01-01T00:00:00Z,
        //       },
        //       id: ext(2, 0xa21a40e7408d4aa8b5ffa42bb6a7c832),
        //       name: 'wksp3',
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
        //       archiving_configuration: { type: 'AVAILABLE', },
        //       id: ext(2, 0x14e08a65e3514e33b95c9de2d52a31a6),
        //       name: 'wksp4',
        //       name_origin: { type: 'PLACEHOLDER', },
        //       role: 'CONTRIBUTOR',
        //       role_origin: { type: 'PLACEHOLDER', },
        //     },
        //   ]
        //   speculative: False
        &hex!(
        "19c63d03cf407619dd6e74cc4126a325990bfe3d6090d1bda001af7a08c15b4394955a"
        "e7c46a7150781922ce413ba4b98d74ff9ee79743377d8771ed3c0e1ff6afa599a52344"
        "03deafbdb52f49bc06f47935df8f73c173a0e1fe6bd41bac09d83f66e1f87e89aa4575"
        "9205e98fde81f02c1dffb6afd86d6f274865234fdbfe34e6650fdd5a2c88042077f7c2"
        "b3286cf71eb5d1316385301ff5e50464cd6e79952d96abbef5b742bb4f25a0fe712f2b"
        "26fb7a7aa28735658758a093aaf6e505689c54a2062ae8070b1d3ab0505c76ef7cf779"
        "0e830b157a7ed9013a65ff58f2ac15b55ed14d46acbb463d1498a9c2c8fd62caa6fdfc"
        "786097e250063551e7ef5e92892d4c4d130fe07a6888354e7a3dd80aeeb5354b520bc8"
        "994c1dcce2a621078eb558fe0784ea1307d357009e94c2b58625e4f9f4e47fc4d68e7e"
        "cb15f5cfba330813dcf852f4d21101802c40d71156dbdb27e1d27826e71aa3187a6ad9"
        "63f4fda6562226f791d973b2bcf7d32fa812db10f8d07d4825b31914152c94b7de27c3"
        "c032c92dfcc85a0d0d0ab642454c144a531eb4e3dacb8eddde233ef74d86d8fb70cb60"
        "69a1d50b0b7774b3e3b0eba670ad46d851fc3c267dcfc4087d2fdc16e08c96387d414f"
        "184d9568f5e1e7cfb84c7cf245fbbb71eab7de4778d4dc110cded3359a89d3ea8f057e"
        "42e8e1b85bb3ba"
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
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    archiving_configuration: RealmArchivingConfiguration::Available.into(),
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    archiving_configuration: RealmArchivingConfiguration::Archived.into(),
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp3".parse().unwrap(),
                    id: VlobID::from_hex("a21a40e7408d4aa8b5ffa42bb6a7c832").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    archiving_configuration: RealmArchivingConfiguration::DeletionPlanned {
                        deletion_date: "2025-01-01T00:00:00Z".parse().unwrap(),
                    }.into(),
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp4".parse().unwrap(),
                    id: VlobID::from_hex("14e08a65e3514e33b95c9de2d52a31a6").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    archiving_configuration: RealmArchivingConfiguration::Available.into(),
                },
            ],
        }
    )
}))]
#[case::synced(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Parsec 3.8.1-a.0+dev
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
        //       archiving_configuration: { type: 'AVAILABLE', },
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
        //   ]
        //   speculative: False
        &hex!(
        "8b54efce275052a4db3386c12ea5b4da507a7c4f19d7f0de7077fceb8c1c3d2b89edee"
        "4b059fec9d8fb25d23b85bbdf53eab764ad04b5daa2f1569fa0beaa4c4c4af9d97b2a3"
        "1536ed4b242876fbe918a5855ac574b8030cbd58bf1ffe101dd8dea897ee78cc52ce31"
        "e2113517407d4c31b57a711efedcac97235f0689805d04abe15f90046c59fcf4812962"
        "273b2e9827ba9fee1319c8cc78734b74e8b1b7e970875020d2c19c5cb1fa35225e183b"
        "d7301235b479954086d3b6a4c9c14d85b9eb54033d46f0ff227667af1e343e3cd90995"
        "20152b321333eca2b55d5f272771b3964651162028dd987c11efc52cc79484cf4900a5"
        "9bb30b9014d4f4be464c33bd741487ee1174053b9f2fa797e94373ecb4eca8140bd29e"
        "c6dab9cc22fe4cc03bfde1db37733163dccbfeef56a623bdb46b14fa5aa175fdd87c42"
        "f4c76febaee2f2f3362bf5ac52a8c892622cf7500bbe845246c3849acf"
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
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    archiving_configuration: RealmArchivingConfiguration::Available.into(),
                },
            ],
        }
    )
}))]
// Legacy format from Parsec < 3.9, missing the `archiving_configuration` field
#[case::legacy_need_sync(Box::new(|alice: &Device| {
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
                    archiving_configuration: Maybe::Absent,
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    archiving_configuration: Maybe::Absent,
                },
            ],
        }
    )
}))]
// Legacy format from Parsec < 3.9, missing the `archiving_configuration` field
#[case::legacy_synced(Box::new(|alice: &Device| {
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
                    archiving_configuration: Maybe::Absent,
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
fn local_user_manifest_new(
    #[values("non_speculative", "speculative")] kind: &str,
    timestamp: DateTime,
) {
    let expected_speculative = match kind {
        "non_speculative" => false,
        "speculative" => true,
        unknown => panic!("Unknown kind: {unknown}"),
    };
    let expected_author = DeviceID::default();
    let expected_id = VlobID::default();
    let lum = LocalUserManifest::new(
        expected_author,
        timestamp,
        Some(expected_id),
        expected_speculative,
    );

    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let LocalUserManifest {
        base:
            UserManifest {
                author: base_author,
                timestamp: base_timestamp,
                id: base_id,
                version: base_version,
                created: base_created,
                updated: base_updated,
            },
        need_sync,
        updated,
        local_workspaces,
        speculative,
    } = lum;

    p_assert_eq!(base_author, expected_author);
    p_assert_eq!(base_timestamp, timestamp);
    p_assert_eq!(base_id, expected_id);
    p_assert_eq!(base_version, 0);
    p_assert_eq!(base_created, timestamp);
    p_assert_eq!(base_updated, timestamp);
    p_assert_eq!(need_sync, true);
    p_assert_eq!(updated, timestamp);
    p_assert_eq!(local_workspaces, vec![]);
    p_assert_eq!(speculative, expected_speculative);
}

#[rstest]
fn local_user_manifest_from_remote() {
    let um = UserManifest {
        author: DeviceID::default(),
        timestamp: "2000-01-30T00:00:00Z".parse().unwrap(),
        id: VlobID::default(),
        version: 0,
        created: "2000-01-01T00:00:00Z".parse().unwrap(),
        updated: "2000-01-02T00:00:00Z".parse().unwrap(),
    };

    let lum = LocalUserManifest::from_remote(um.clone());

    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let LocalUserManifest {
        base,
        need_sync,
        updated,
        local_workspaces,
        speculative,
    } = lum;

    p_assert_eq!(base, um);
    p_assert_eq!(need_sync, false);
    p_assert_eq!(updated, um.updated);
    p_assert_eq!(speculative, false);
    p_assert_eq!(local_workspaces, vec![]);
}

#[rstest]
fn local_user_manifest_to_remote(#[values("with_v1_base", "placeholder")] kind: &str) {
    let lum_created = "2000-01-01T00:00:00Z".parse().unwrap();
    let lum_updated = "2000-01-10T00:00:00Z".parse().unwrap();
    let lum_author = DeviceID::default();
    let lum = {
        let id = VlobID::default();
        let speculative = false;
        let mut lum = LocalUserManifest::new(lum_author, lum_created, Some(id), speculative);
        lum.updated = lum_updated;

        match kind {
            "placeholder" => {
                // Sanity check
                p_assert_eq!(lum.base.version, 0);
            }
            "with_v1_base" => {
                lum.base.version = 1;
                lum.base.timestamp = "2000-01-30T00:00:00Z".parse().unwrap();
                lum.base.updated = "2000-01-05T00:00:00Z".parse().unwrap();
            }
            unknown => panic!("Unknown kind: {unknown}"),
        }

        lum
    };

    let expected_um_author = DeviceID::default();
    let expected_um_timestamp = "2000-02-28T00:00:00Z".parse().unwrap();
    let um = lum.to_remote(expected_um_author, expected_um_timestamp);

    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let UserManifest {
        author,
        timestamp,
        id,
        version,
        created,
        updated,
    } = um;

    p_assert_eq!(author, expected_um_author);
    p_assert_eq!(timestamp, expected_um_timestamp);
    p_assert_eq!(id, lum.base.id);
    p_assert_eq!(version, lum.base.version + 1);
    p_assert_eq!(created, lum.base.created);
    p_assert_eq!(updated, lum.updated);
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
                archiving_configuration: RealmArchivingConfiguration::Available.into(),
            },
            LocalUserManifestWorkspaceEntry {
                name: "wksp2".parse().unwrap(),
                id: wksp2_id,
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Contributor,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
                archiving_configuration: RealmArchivingConfiguration::Available.into(),
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

#[rstest]
fn local_user_manifest_workspace_entry_is_read_only() {
    let mut entry = LocalUserManifestWorkspaceEntry {
        id: VlobID::default(),
        archiving_configuration: RealmArchivingConfiguration::Available.into(),
        name: "wksp".parse().unwrap(),
        name_origin: CertificateBasedInfoOrigin::Placeholder,
        role: RealmRole::Owner,
        role_origin: CertificateBasedInfoOrigin::Placeholder,
    };

    let far_in_the_future = "2999-01-01T00:00:00Z".parse().unwrap();

    for (is_read_only, role, archiving_configuration) in [
        (
            false,
            RealmRole::Owner,
            RealmArchivingConfiguration::Available,
        ),
        (
            false,
            RealmRole::Contributor,
            RealmArchivingConfiguration::Available,
        ),
        (
            false,
            RealmRole::Manager,
            RealmArchivingConfiguration::Available,
        ),
        (
            true,
            RealmRole::Reader,
            RealmArchivingConfiguration::Available,
        ),
        (
            true,
            RealmRole::Owner,
            RealmArchivingConfiguration::Archived,
        ),
        (
            true,
            RealmRole::Contributor,
            RealmArchivingConfiguration::Archived,
        ),
        (
            true,
            RealmRole::Manager,
            RealmArchivingConfiguration::Archived,
        ),
        (
            true,
            RealmRole::Reader,
            RealmArchivingConfiguration::Archived,
        ),
        (
            true,
            RealmRole::Owner,
            RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: far_in_the_future,
            },
        ),
        (
            true,
            RealmRole::Contributor,
            RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: far_in_the_future,
            },
        ),
        (
            true,
            RealmRole::Manager,
            RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: far_in_the_future,
            },
        ),
        (
            true,
            RealmRole::Reader,
            RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: far_in_the_future,
            },
        ),
    ] {
        entry.role = role;
        entry.archiving_configuration = archiving_configuration.into();
        p_assert_eq!(entry.is_read_only(), is_read_only);
    }
}
