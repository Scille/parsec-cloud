// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::*;
use std::collections::HashMap;

use parsec_api_crypto::*;
use parsec_api_types::*;

use tests_fixtures::{alice, Device};

#[rstest]
fn serde_file_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   size: 700
    //   blocksize: 512
    //   blocks: [
    //     {
    //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //       offset: 0
    //       size: 512
    //       digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
    //     }
    //     {
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b")
    //       key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
    //       offset: 512
    //       size: 188
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //     }
    //   ]
    let data = hex!(
        "78cb19052e01b4744a19b853a21b5301dad871a9fdecbc3cbc2337fe5df46a487656500aedd53b"
        "73c0d91bd08dadaff857ed7ec0ca153a3cd16da1ab820c7dfe03d5293acb2f94582c54cf2b801f"
        "bbbfb36a9a8ae8af9493d1b29fbea760584a05b1b2649ec67b0b2917df6e19159bd321baa86716"
        "a4a572f549100db7b2e28082e854ee76e8dc6f2989610d26638b426fef8452204d5b74f205a36e"
        "73e5d0eda45a933409248f1d262465751c4e18a161154cbb84139ae4c34a8c74ddcfda266eb1bc"
        "65a553d7ce8d458d6ae64c472a09ba4b9cf927953f11c44dacc10f0680618346e32b4e1e6cbf6d"
        "dae6050562541b63b02caef3226f388a7f7ff1b32cb066d2f5b021fd55379b49da2d270a508d22"
        "23a8bee46fbc5274336dfb9ca0cb3edd9ec910cfef9e0203456461aeddb068f821e3eb0d8ef376"
        "5c8cb276b750058459ad24a041088b7c4c3e07f4cbbc601976ea9a4c8d67d00c6cdf906d16cd45"
        "5988adfbd1ab7209f666635adfd5ffac800c0ffe89cfde7b752266ccaaf12b050a602558cced0a"
        "96ca03ede769ea46fabd15804e2b1a459ee5fc6fb4db3ab3d690e8e32b83ccda21ec1c7f5585c4"
        "de7ec43d62345482ee52a14c17870530a2bd89a68af7748e885b5c444b86e09deda9b4576dcb7b"
        "d4f898ec9fb28a74730285"
    );
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = FileManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
        parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 700,
        blocksize: 512,
        blocks: vec![
            BlockAccess {
                id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                key: SecretKey::from(hex!(
                    "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                )),
                offset: 0,
                size: 512,
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap(),
                key: SecretKey::from(hex!(
                    "c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc"
                )),
                offset: 512,
                size: 188,
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
        ],
    };

    let manifest = FileManifest::decrypt_verify_and_load(
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
    let manifest2 = FileManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_folder_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "folder_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   children: {
    //     "wksp1": hex!("b82954f1138b4d719b7f5bd78915d20f")
    //     "wksp2": hex!("d7e3af6a03e1414db0f4682901e9aa4b")
    //   }
    let data = hex!(
        "14adb64a9dd335cafefa683b2f2087573aa8b5ca37f8c4d393e4eb8cc54b4444960c972094872f"
        "3ab9c7d246fab90034c9da028ace0c230077f97294430d207a516508cb72e655cd8eadf4557ef8"
        "551a0e3c24019b8264469cb76ab0f85aa8e2e73cdb4b9b95c18236d8853968fbcc762a446f8c57"
        "9eba294ba6068be5b86c3fa9683a3f0bc649e2a1306f09279c52edcbe41d213838037c7c5974a4"
        "ac6c26da8c9200efc88eec7cc31dea89270c4842088250bdc77b460d42f25961259b79b90932cd"
        "8651105572d8c5dbe98b03531612bad7b03318395d1110232c7509f9ecff4e3e25627f5cc22f08"
        "f3ee2f3d4790cd943a4829a9d987332be878987aeb25192715c50b9a2c43121aec2d8a6edb08c0"
        "a1a3414e98812e0ce69e17daee7839d3d79eb3da0f8d4e089e901e6dfa"
    );
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = FolderManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
        parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
        version: 42,
        created: now,
        updated: now,
        children: HashMap::from([
            (
                "wksp1".parse().unwrap(),
                ManifestEntry("b82954f1138b4d719b7f5bd78915d20f".parse().unwrap()),
            ),
            (
                "wksp2".parse().unwrap(),
                ManifestEntry("d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap()),
            ),
        ]),
    };

    let manifest = FolderManifest::decrypt_verify_and_load(
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
    let manifest2 = FolderManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_workspace_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "workspace_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   children: {
    //     "wksp1": hex!("b82954f1138b4d719b7f5bd78915d20f")
    //     "wksp2": hex!("d7e3af6a03e1414db0f4682901e9aa4b")
    //   }
    let data = hex!(
        "48c24986ac45100cd38684be507f7c65a67df2cb36f61bb9595bb3e96f311b9183d03a4fe70721"
        "629c2769d6225994295127a96e7773209af60438d8db12cd26a311a8b0397aebc81090fd0523d8"
        "9c11b6e849e14897c40060cf1a15815be11e80f56ac1848c18ff6406934e0410d9d73ec1088385"
        "60edd27fbde269faedb063add9a1241864e7170da2bfdd7214697b0914a92cef4ad116b802d1f8"
        "9a7e7b9d0396630de175a213c27b3e0496777da0beb316c7b543bb79c13eb56de13706abf5ab48"
        "218e22a786288305173c86ecfa63907380183fd507d7b36e7e8f3ef50ef4be094c2fd26ab5f83d"
        "1f11c3617cde53c49ef2200352f02ca73b675a99b0832234f077e3aee861a6ffab149fcea40a17"
        "4388ecd472f2"
    );
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = WorkspaceManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: "87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap(),
        version: 42,
        created: now,
        updated: now,
        children: HashMap::from([
            (
                "wksp1".parse().unwrap(),
                ManifestEntry("b82954f1138b4d719b7f5bd78915d20f".parse().unwrap()),
            ),
            (
                "wksp2".parse().unwrap(),
                ManifestEntry("d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap()),
            ),
        ]),
    };

    let manifest = WorkspaceManifest::decrypt_verify_and_load(
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
    let manifest2 = WorkspaceManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

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
        "1804b39770a58dfed1a2896459e5364fc7004d0e67c39dc5bbc747569abe5cb7a1ea5c437d8752"
        "f792670fbd91dcd2279b2c281b1d7fceaa553a73d1c2b98aa5b637aa63e0e11ab0efad50b55e8c"
        "d88a0d208fa593e9a5038e7f6a5046a07816f3918dda19292a8034ebb4b9b9d0abec63af0219be"
        "fd949103bf4d96c2e614c3b09cbd96559890d419f5c2cb74a5d87ee5cd1aaed317d1c719c83e01"
        "a42a987ef3bbcab5f3d4607fbc1fee71998f41ff0ff81fc86d3b99fe0df0c9da1b06be40e0f4b4"
        "b2b49ca5700fc2b31642808ff0458d91199eed4f7dc2a2a0ac2080fc15e04ebc2470a9014991e2"
        "6363eec7235f6369276f30b8cd4d77eeb5ddeeb875ed327bf99543d022add79fcf6f87b05e92ba"
        "4cee5b9cbabd41a15aff15d17210c6421b877a748036163c70e5804e0f461043ce2614595bcc41"
        "c1237daa6372726b1d28fcca89a76e35144d4ec8becb6ab33f6b039810253a3460b489099c6f79"
        "acfd3b86b50e19692cd41362283f09f4a8f1d6dabfef2103d98abf735d788a98705d03910fd65d"
        "49294d47c7158103ebac8909da538380081088cc07051db6a93ab44c0424ee20e060dd7eb2bc38"
        "fb66757960aa"
    );
    let now = "2021-12-04T11:50:43.208820992Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = UserManifest {
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
    };

    let manifest = UserManifest::decrypt_verify_and_load(
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
    let manifest2 = UserManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        &now,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}
