// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::*;

use parsec_api_crypto::*;
use parsec_api_types::*;
use parsec_client_types::*;

use tests_fixtures::{alice, Device};

#[rstest]
#[case::need_sync(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
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
        hex!(
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
        ),
        LocalUserManifest {
            author: alice.device_id.to_owned(),
            timestamp: now,
            updated: now,
            need_sync: true,
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
})]
#[case::synced(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
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
        hex!(
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
        ),
        LocalUserManifest {
            author: alice.device_id.to_owned(),
            timestamp: now,
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
})]
#[case::speculative(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
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
        hex!(
            "ce0274b3890ec74dff002d7b587e591ba4876b7f08fa227eb1ba737f6eae2490c79c1be9c6"
            "211179274ce62439caeea5829c9265e46371fb6e198f3d13ef5e366e11653a64f049941a1b"
            "6c70ba424019f7e09bcab7f53872cc3d65a9f23b71ec85b2f5b4524f58a03528ccf335af19"
            "8e87fc1f1675da3473bd6a43804b4177267c9ba55c1ec955f5068de1676f9f8731ea4932a0"
            "c55c7eb3741893a5ce7e2c4f20406485821e964b7cf46442338c63771883da9d90098c1fea"
            "73339fa97aba3145977114b5f4c05d3b21733d3a9f476e7abf90602bc7b538ac7794e96e76"
            "c279e236e80564790efc67e61d8196cd4ba1806b7636070976bf4306be2f51fa705cbaa423"
            "890ba6a40b5b33ac75f71dd2a8e1bc93532a3bbee659da075245905b5c46583e0479e8c600"
            "60e0e8bfaa94af4c32a201f6b0"
        ),
        LocalUserManifest {
            author: alice.device_id.to_owned(),
            timestamp: now,
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
})]
fn serde_local_user_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: &dyn Fn(&Device) -> (&'static [u8], LocalManifest),
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalUserManifest::decrypt_and_load(
        &data,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&alice.signing_key, &key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalUserManifest::decrypt_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}
