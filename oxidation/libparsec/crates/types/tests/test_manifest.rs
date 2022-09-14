// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::{collections::HashMap, num::NonZeroU64, str::FromStr};

use libparsec_crypto::*;
use libparsec_types::*;

use tests_fixtures::{alice, Device};

#[rstest]
fn serde_file_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   blocks: [
    //     {
    //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //       digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
    //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //       offset: 0
    //       size: 512
    //     }
    //     {
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
    //       offset: 512
    //       size: 188
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    let data = hex!(
        "300e6b0018dd047a166988cf92d570fb9ad4ca2155237d273f4adc73c8d908de55de11cd85"
        "7b3eee0f662778b0c3430de34ffce444bd43f6449a16f87042fc3a2d72437b099ff25d9123"
        "4a54cb010487566516f0e1b4f21c7eb4585d31f14567e8fb9e202e7eda3c63cb720a1246c7"
        "88d2429b13be6057aa825a0f3b23a9d674cff2b726930102a83895aac3b183bd228ab5c304"
        "89e90dc191239c7b14186145fb39719390a64d6b93fa78b79773e1d3ada951063ef28d7378"
        "314e0c2f6cb4a131f13ec72f4d4b01fdf6d4d7f482c6033fd4ae104339ae7b75b123bc947d"
        "7ecd6f4722b9df93ccb381c22bc0f586f0e39889c6d2d87d449bc5dafbe35fe5f29e87f69f"
        "d5adf300c9c66211fcfd9bbf37f45ac41926f58103b6a39ebc93e0613d96861131c5e7ddc8"
        "1e86a5dde990dacab4cf78b5f47e4985fe26f4a9181a419f0a94b76aff6db85e31934a5940"
        "fd0887cd91f6cc81e94885e77e2dc3d057a0800365bfeac1451c0b0321bb7e4decc7804a74"
        "362cfb3abbe0771f355ba297129926c3dd899740fae3195d57cd93a0004df0b5787967d5c5"
        "a9e1c6274391f5f79f1cd1d38c0b6fb1f5b2f13500e706d607c5c5dcd7e6bd37bf70190626"
        "d7e43c85de28a72c8a55ffa405cb4c89ab32182b1959c175495058b965db96f44c"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
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
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
                key: SecretKey::from(hex!(
                    "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                )),
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
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
                size: NonZeroU64::try_from(188).unwrap(),
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
        now,
        None,
        None,
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
        now,
        None,
        None,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_file_manifest_invalid_blocksize(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   blocks: []
    //   blocksize: 2
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    let data = hex!(
        "3d24370f85424221a83a0dd4bc5d27b0d5816fc62ef3a6c3186a9211f0b2ec0955ec20204a"
        "9b351224b789537d478b2a5d8a047c3fb9b3a039d90ff8a00cb53ee1ae1f2a24bafd14dbf6"
        "74e1c7bcf7a782dd9aa695ff93263e83cb1611d6d18b8348d2f2f184fdbdcb8979640fe7fc"
        "c0c1d39c930dda0ccb243d298e357b0830774fbb630bbe3009982cc8806c89e709bd4d4e33"
        "b43966b3d0cf55440a57bb84bd3c78da48f04512c386afa17a1bfa2e14fcc0a6210661086b"
        "7427eb7ca77b28078b496c984bf5d51da7023e13485a6c34b2d80d1380cc94b2546f06aa99"
        "3dbbfe3c6d6aa46820ea7bed353d14892048cfb622144d99659e6ea429f54cc0e53840c86c"
        "9ac81147860b3e"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = FileManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        now,
        None,
        None,
    );

    assert!(manifest.is_err());
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
        "afc13ea5d28a0da5f17662d5110dd75d983f22efc1c09ddce90fb3ff8fef001aad83179578"
        "74a52cf75882123b1030891e5b46052d5bddfd1885de9fc784e07afa0d415fe7e0ffde9416"
        "d1285a8738a0e634fc1185e441886db61aa8fce5b588f502604fe26d28e29523f424b65465"
        "dcde840192a68b1668376ac86e1949c7613f65e1f760cc031df2306ae573d0178d7a388515"
        "ffe241562e167af7eb7484a3b0608438a08ff3449759d2622847efc05698d772b0fff9f529"
        "b1c3582b1137f7121bc2f3dcdb4de32dc3f8daac75602273de7b7d8b87451c517c1d147642"
        "2c51d72375520808e85eb64c0f6a6f2de5ca564df7cca286530c527b4bad7594f9e3dad1f4"
        "0845af2dbe8fbb97201f3c617d5d63824ff36ce61fccd31ac8d5cb3c2f1609236a266b46ae"
        "9d01b508"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
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
                "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
            ),
            (
                "wksp2".parse().unwrap(),
                "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap(),
            ),
        ]),
    };

    let manifest = FolderManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        now,
        None,
        None,
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
        now,
        None,
        None,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_workspace_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "workspace_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   children: {
    //     wksp1: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //     wksp2: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //   }
    let data = hex!(
        "81de3866c170f0fe2de103cbdcd518952c9d821e2b751b1ca43b2502333efa33142b52d0aa"
        "c474f24e71e71caa858f2786dfbecada89d92f577e970f1ac92b8e385b3f278172d787da92"
        "d8b5e98d772893268652c0590644acb1344f0f3c13b5a1c49a96f58b9959f06ebcca5c8cdf"
        "f4b921bf83d61515d039ede22766cae7fe657b9f21746717a06995a910c350c5082d5c6db8"
        "a6762d55e7b0ed26bd29f8f1d21e543022380c886dae8bc5dd1a8eb6254cd90b47eead401f"
        "9e9a91668cf757ab15f7c0f799b02b5a3d36e1c5df5d62b3fe07b1f45071b655f197e47b3c"
        "41f31a58d4eb6e7c3666220fe0c8a4fb21c9e21334299cc7397f94ea0829a6613383ce83ec"
        "2561f5c8758f71dc00df7b82bc6bdd9831de"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
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
                "b82954f1138b4d719b7f5bd78915d20f".parse().unwrap(),
            ),
            (
                "wksp2".parse().unwrap(),
                "d7e3af6a03e1414db0f4682901e9aa4b".parse().unwrap(),
            ),
        ]),
    };

    let manifest = WorkspaceManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        now,
        None,
        None,
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
        now,
        None,
        None,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_user_manifest(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0+dev)
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
    //       encrypted_on: ext(1, 1638618643.208821)
    //       encryption_revision: 2
    //       key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //       role: "OWNER"
    //       role_cached_on: ext(1, 1638618643.208821)
    //     }
    //     {
    //       name: "wksp2"
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       encrypted_on: ext(1, 1638618643.208821)
    //       encryption_revision: 1
    //       key: hex!("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
    //       role: None
    //       role_cached_on: ext(1, 1638618643.208821)
    //     }
    //   ]
    let data = hex!(
        "f4b0c4cdd53b423399100942eb063f47f802071291be37b2a8badd60f0d0065893f3af42cc"
        "649e62d2e907e96e55df4b7fe27fc5460def6b457c85e350a9727bf4b69b181e7c29aa3a82"
        "44d50b6d6f872e325fd2ceb7466c66e5e9a7fffca8a5b4c00302c398ef2f1b5ee399de4b3b"
        "0cb867746df3d678804765de87f2fb1eb0d10547c6634e2a30808efee84dfd1232cd0799d4"
        "41f8731689c86404088c9e8b8347b36aa7d3857a5f559f9ecf5a6938542ba8650cdc083119"
        "6ec8bcde7eb533175007637379ce8404e5d0aea5ab33727f99f74cb1ac4973703a978260e9"
        "920107c543c95f1115f0ade65e4ab15cf85426d1f3d83bb5a1d33fff60c4e99c8a702cebcc"
        "bae4b6e9aad8e820863310d0d93997e0915ea335200b35867ee09e9bc0243779973f84d6c3"
        "aa08cd8ffde260bca89e90b5ece3f98af0bd4df3812d7a31f044773741217084fbf4b7bcd0"
        "a16dfb47509c19a683840c56e81ed78cbed7b44298b068dae5e0ebb9f9fb4815bd2ffee6da"
        "432c3cdc2750d5c27afd2a246146661115e941b2dd42ede908d766af56d2a766a08b3b517a"
        "297a9cca1a3d80e7d9454abbe28bf91e8012ebcfe367f290d46c22a8"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
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
        now,
        None,
        None,
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
        now,
        None,
        None,
    )
    .unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
#[case::valid(None, None, None, None, None)]
#[case::valid_id(None, None, Some("87c6b5fd3b454c94bab51d6af1c6930b".parse().unwrap()), None, None)]
#[case::valid_version(None, None, None, Some(42), None)]
#[case::invalid_dev_id(
    Some("maurice@pc1"),
    None,
    None,
    None,
    Some("Invalid author: expected `maurice@pc1`, got `alice@dev1`".to_string())
)]
#[case::invalid_timestamp(
    None,
    Some("2021-10-24T11:50:43.208821Z".parse().unwrap()),
    None,
    None,
    Some("Invalid timestamp: expected `2021-10-24T11:50:43.208821+00:00`, got `2021-12-04T11:50:43.208821+00:00`".to_string())
)]
#[case::invalid_id(
    None,
    None,
    Some("6b398b3dc6804bb784bb07b0d7038c63".parse().unwrap()),
    None,
    Some("Invalid entry ID: expected `6b398b3dc6804bb784bb07b0d7038c63`, got `87c6b5fd3b454c94bab51d6af1c6930b`".to_string())
)]
#[case::invalid_version(None, None, None, Some(0x1337), Some("Invalid version: expected `4919`, got `42`".to_string()))]
fn test_file_manifest_verify(
    alice: &Device,
    #[case] expected_author: Option<&str>,
    #[case] expected_timestamp: Option<DateTime>,
    #[case] expected_id: Option<EntryID>,
    #[case] expected_version: Option<u32>,
    #[case] expected_result: Option<String>,
) {
    let now = "2021-12-04T11:50:43.208821Z".parse::<DateTime>().unwrap();
    let id = "87c6b5fd3b454c94bab51d6af1c6930b"
        .parse::<EntryID>()
        .unwrap();
    let version = 42;

    let expected_author = expected_author
        .map(|author| DeviceID::from_str(author).expect("Invalid raw DeviceID"))
        .unwrap_or_else(|| alice.device_id.to_owned());
    let expected_timestamp = expected_timestamp.unwrap_or(now);
    let expected_id = expected_id;
    let expected_version = expected_version;

    let manifest = FileManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id,
        parent: "07748fbf67a646428427865fd730bf3e".parse().unwrap(),
        version,
        created: now,
        updated: now,
        size: 700,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![],
    };

    assert_eq!(
        manifest
            .verify(
                &expected_author,
                expected_timestamp,
                expected_id,
                expected_version,
            )
            .map_err(|e| e.to_string())
            .err(),
        expected_result
    );
}

#[rstest]
fn file_manifest_unverified_load() {
    // Test the unverified_load() function on a file manifest.
    // File manifest has been generated with Parsec 1.12, it is provided raw as it is a bit hard
    // to generate (would need to create keys and all that). We're only interested in the
    // deserialization of raw unciphered data to a FileManifest.

    let serialized_manifest = hex!(
        "af080459a4924e3934f2cfcbe90ce658ef42954f2abe9dab2524a417bfd833959a9c4de3c9abe34
        c1447152be9398944b6a1c4dca7609ce47e0782f881a1c209789c015d01a2fe8ba474797065ad666
        96c655f6d616e6966657374a6617574686f72d941623365306562343134623063343466316239363
        33337373232633135366563364030613239373331356163366634633766613538333962303031633
        93530623233a974696d657374616d70d70141d8c7d36e7af26fa26964d80269f15a0d48c040bf884
        a5d41eb8b528fa6706172656e74d80224a5fac2b57a4be2b88550e077ee524ca776657273696f6e0
        1a763726561746564d70141d8c7bcaada06eaa775706461746564d70141d8c7bcaadb9e3da473697
        a6505a9626c6f636b73697a65ce00080000a6626c6f636b739185a26964d802e53a7750ed014b67b
        c7683abb0801d95a36b6579c4205c0ab829952db2b6f9d2a8dfcbaafa356290e6b0555a0ca8804b4
        a665f86cb00a66f666673657400a473697a6505a6646967657374c420f2ca1bb6c7e907d06dafe46
        87e579fce76b37e4e93b7605022da52e6ccc26fd2e5299be8"
    );

    let manifest = Manifest::unverified_load(&serialized_manifest).unwrap();

    let file_manifest = match manifest {
        Manifest::File(fm) => fm,
        _ => panic!("Should be a file manifest"),
    };

    assert_eq!(
        file_manifest.author,
        DeviceID {
            user_id: UserID::from_str("b3e0eb414b0c44f1b96337722c156ec6").unwrap(),
            device_name: DeviceName::from_str("0a297315ac6f4c7fa5839b001c950b23").unwrap()
        }
    );
    assert_eq!(
        file_manifest.id,
        EntryID::from_str("69f15a0d48c040bf884a5d41eb8b528f").unwrap()
    );
    assert_eq!(
        file_manifest.parent,
        EntryID::from_str("24a5fac2b57a4be2b88550e077ee524c").unwrap()
    );
    assert_eq!(file_manifest.version, 1);
    assert_eq!(file_manifest.size, 5);
    assert_eq!(file_manifest.blocks.len(), 1);
}
