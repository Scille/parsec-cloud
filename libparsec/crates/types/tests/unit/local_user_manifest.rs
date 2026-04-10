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
        // Generated from Parsec 3.8.2-a.0+dev
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
        //       archiving_configuration_origin: {
        //         type: 'CERTIFICATE',
        //         timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
        //       },
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
        //       archiving_configuration_origin: {
        //         type: 'CERTIFICATE',
        //         timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
        //       },
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
        //         deletion_date: ext(1, 1735689600000000) i.e. 2025-01-01T01:00:00Z,
        //       },
        //       archiving_configuration_origin: {
        //         type: 'CERTIFICATE',
        //         timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
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
        //       archiving_configuration_origin: { type: 'PLACEHOLDER', },
        //       id: ext(2, 0x14e08a65e3514e33b95c9de2d52a31a6),
        //       name: 'wksp4',
        //       name_origin: { type: 'PLACEHOLDER', },
        //       role: 'CONTRIBUTOR',
        //       role_origin: { type: 'PLACEHOLDER', },
        //     },
        //   ]
        //   speculative: False
        &hex!(
            "d4a770fb955e91003d8914389211c0e9a23fdddb74b074a2917d1bc2ba4ea7674b873f"
            "7294b2126d7f629a241ecf5963af98e808645cc03fa8d17bcddb032c76f8244d31c11e"
            "ea1e88b41c899c5be28a17203becd34e74bed1819017171524879bb05d097f417bb3f7"
            "3e83276c21750b8ba376381af38c035358a7f4c310a49095c44c477fcb591bf84b0d49"
            "f453f5a7e47690a5f32fce4a2a9e9865e433727db032a00ce8c8f0a4bc42cfe1fa2b9c"
            "6122d5bb2869a3bd5d41101fedb8f2aeb937aced1cba29080ba566314e8c6abf65733e"
            "d1c936ed11986cae7b27bad34c6b489258a5d49a59e363194ec9d7a60ffe0cb4a16fac"
            "d2ab577bd427d531f938c2aad9f9c31e607c04536176d972026943642d211b306d004e"
            "862d30368c76c976b567715c6e595cfe2613101d1c74cbc0871cb85092ab896f4d6a99"
            "701cd5fd8da023fc1576ecabb1ace5cfe26efce7093b9f6a7a65ddd35c11f5b86584c7"
            "f8275401f0b561911614cc82f3dcbcdc35a4de0fbe65fbbbc5b93d8ef8643c93740b09"
            "b4d18e9b5d2f1a6239f7cfddc7381913b4e32390f4b37f0554f4fa47049b1194e3a0b4"
            "1ced0684227a2f98e80f16a7b39adb05387c8bbe8f4e866bccf01f04b9026edce70dff"
            "a32f61ab853cee35ab5686fc2a61643d4613891382e7fb198bc6c38b7c79d5907f2310"
            "44b3f24ec024aeb078e413072954db16e2eaca71"
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
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now }.into(),
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now },
                    archiving_configuration: RealmArchivingConfiguration::Archived.into(),
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now }.into(),
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
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Certificate { timestamp: now }.into(),
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp4".parse().unwrap(),
                    id: VlobID::from_hex("14e08a65e3514e33b95c9de2d52a31a6").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    archiving_configuration: RealmArchivingConfiguration::Available.into(),
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
                },
            ],
        }
    )
}))]
#[case::synced(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
        // Generated from Parsec 3.8.2-a.0+dev
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
        //       archiving_configuration_origin: { type: 'PLACEHOLDER', },
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
            "717bc4b82e12aeaf0ffb00f95e900a5f314e3bb26f7761fda9d13aaddb3a43a00292b4"
            "a8aad8b5f7c317e84e8dea8ed8f6f7b6815c6978179f53a460a92ea3ef4c53091362bc"
            "e8df494ee69c5ba639c14685882d391417711255edf24d0a31482db3a38bfaa4bfc38b"
            "651c15aa3e4311c27702169fd6e22b6881afe3fdf6acfbf9b12227a27a6a722ed3e966"
            "932a2724807e233d7f60e5159f4e1d407b9d227425c11aa5f08dfa981c1a090615d54d"
            "d5d1d045f253a35bec2b11b48079d15298f3534b1e36452fe9d00f6fa4460d03bcf454"
            "bffe82ae900bbc9376e000b6c22888201b95acccc2f069eb4c63531b89d9802c824e83"
            "2a1dfe26fd91fa14b50bae13253337f144c659bc7acaefa2a6504c851b69e0d1513648"
            "4bfa60af95b37a5bd8f99ab7089baef714e1fc8d2124e9c05ca243b94c4082999da08c"
            "de40f74331362837a8aee9ee436753e016c4d73460a88eaefab91cb12d5e69b8f6473a"
            "31c511f17dc90a851d78d4b7"
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
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
                },
            ],
        }
    )
}))]
// Legacy format from Parsec < 3.9, missing the `archiving_configuration`&`archiving_configuration_origin` fields
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
                    archiving_configuration_origin: Maybe::Absent,
                },
                LocalUserManifestWorkspaceEntry {
                    name: "wksp2".parse().unwrap(),
                    id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Contributor,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    archiving_configuration: Maybe::Absent,
                    archiving_configuration_origin: Maybe::Absent,
                },
            ],
        }
    )
}))]
// Legacy format from Parsec < 3.9, missing the `archiving_configuration`&`archiving_configuration_origin` fields
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
                    archiving_configuration_origin: Maybe::Absent,
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
    let raw_expected = expected.dump_and_encrypt(&key);
    println!("***expected: {:?}", raw_expected);

    let manifest = LocalUserManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    // Note we cannot just compare `raw_expected` with `data` due to encryption and keys order
    let manifest2 = LocalUserManifest::decrypt_and_load(&raw_expected, &key).unwrap();

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
                archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
            },
            LocalUserManifestWorkspaceEntry {
                name: "wksp2".parse().unwrap(),
                id: wksp2_id,
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Contributor,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
                archiving_configuration: RealmArchivingConfiguration::Available.into(),
                archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
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
        name: "wksp".parse().unwrap(),
        name_origin: CertificateBasedInfoOrigin::Placeholder,
        role: RealmRole::Owner,
        role_origin: CertificateBasedInfoOrigin::Placeholder,
        archiving_configuration: RealmArchivingConfiguration::Available.into(),
        archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
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
