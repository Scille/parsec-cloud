use hex_literal::hex;
use rstest::*;

use parsec_api_crypto::*;
use parsec_api_types::*;
use parsec_client_types::*;

use tests_fixtures::{alice, Device};

#[rstest]
fn serde_user_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "user_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   last_processed_message: 3
    //   workspaces: [
    //     {
    //       name: "wksp1"
    //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //       encryption_revision: 2
    //       encrypted_on: ext(1, 1638618643.208821)
    //       role_cached_on: ext(1, 1638618643.208821)
    //       role: "OWNER"
    //     }
    //     {
    //       name: "wksp2"
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
    //       encryption_revision: 1
    //       encrypted_on: ext(1, 1638618643.208821)
    //       role_cached_on: ext(1, 1638618643.208821)
    //       role: None
    //     },
    //   ]
    let data = hex!(
        "ed02cad442320f04035cca2c094081b9995189a1645bc55568339a3a9233d91f8c31dee2c79890"
        "f0c474fb7a799deb0ad4b7ecd102453c33268354e79b612934517d599f8d53a61a93723dacae87"
        "922feb1a05ab987eb9922fcf751109e6ce279e38d09f6febbf3068b1bfd5390a13f90b90f8349e"
        "8e02e4714689317b96d1778e60735b14978f5e7e2663a21a1e7b31018c1f0eb3a945b226e0aad0"
        "2fdca5327b649faa04ad064cb34aa86d5464536878227a504d3a0ffa3217d364db018d7ea1cf3a"
        "24251582ca7f6de30e4feefbe40c09a6dbec96caca55c274043a685012c8272e981a2fa06fa4cd"
        "de2b5b0884a0ce4598886905b99e0148c5282e6021a57fcd4c043cd5744abe7ae0a504e616c785"
        "3db9df70c48579d084b88b36d485992d181578b467e7e6bdbb7417df43340548ac1a5b4a4e9f47"
        "461be954de7d5e7fb3c9ede4a2abae3a0c4d130a34d213ad1efcd90747eb5280765c4a1e4a9fc8"
        "803bb889b5fa29e748b920c5b0ce1d88506e795a65a297682973735e83a59c345607a1066bf64b"
        "a08a2364750b6a8a8801b2e9f0a7309d08d66d8ef1fb69462845e63a51f9c9035cbf0443d60d31"
        "14a7390f8a9d58e69b2b94d8a7fdfb5437e3c516b33a3d07f81bb4d14d61522d437abd0e9979d6"
        "3564555a6bfc8856d97233f5e28baf607b8944fe137e53eb956d78ca800721a6cd34e4f56f3d85"
        "5e16be29580739b6fe03ac2ec806f27e8d87d83386a1fc3076526e62bf298b3a2d0ec6e441e083"
        "01e4b5a5f8aa72a5ca51aa3a683064e20a735b531e40c3a3d3e26be61841cfa53b385aee36d899"
        "890b42d62a371089752739effec2316199a358236bb12af40e0bf9111c8a1667002a1c1a1201df"
        "044a4f405bfa662e4a230b31bb12c44176489dafd34ad40ce121effe130eef81175b37f0eb8bff"
        "752025b73a08470e5869e6c7b90e8a0c061cd66880ed90d6e57f09ff49ff4286ac82c33c7ecb75"
        "cd42abf37aa5abd4a7ef8bc406c022097f9271476503de00b476ccf3e750d9d242c49d7f6e3e94"
        "0ef3704fdef14e33f7927786bc501f40"
    );
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    
    let expected = LocalUserManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        need_sync: true, 
        updated: now,
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
        speculative: Some(false),
    };

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
    let data2 = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
    // Note we cannot just compare with `data` due to signature and keys order
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