// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64, str::FromStr};

use libparsec_crypto::{SecretKey, SigningKey};
use libparsec_tests_lite::prelude::*;

use crate::{
    fixtures::{alice, Device},
    BlockAccess, BlockID, Blocksize, ChildManifest, DataError, DateTime, DeviceID, FileManifest,
    FolderManifest, HashDigest, LegacyUserManifestWorkspaceEntry, UserManifest, VlobID,
    WorkspaceManifest,
};

#[rstest]
#[case::blocksize(
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
    //   blocksize: 2  <-- Invalid value, must be >= 8 !
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    &hex!(
        "789ceb5e5e965a549c999fa7b5ac20b12835afe406137b49fffef4656e4e2dea6df1d70df6"
        "db2d4f2e4a4d2c494db9cee87823eb6acbd9d8e04599293798da8f6dfd6bedea3365d756d9"
        "ac8fc726732f2f2d484155b6a4a4b220756d5a664e6a7c6e625e665a6a71c9b2a49cfce4ec"
        "e2094b8a33ab52cf32ed5996585a92915fb42a31273339d52125b5cc706549662e5061626e"
        "01c2a095105d402d4c0066eb5178"
    ),
    DataError::Serialization
)]
#[case::dummy(b"dummy", DataError::Compression)]
#[case::dummy_compressed(
    &hex!("789c4b29cdcdad04000667022d"),
    DataError::Serialization
)]
fn invalid_deserialize_data(#[case] data: &[u8], #[case] error: DataError) {
    let manifest = ChildManifest::deserialize_data(data);

    p_assert_eq!(manifest, Err(error));
}

#[rstest]
fn dump_load(alice: &Device) {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 0
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   blocks: []
    //   blocksize: 8
    //   parent: ext(2, hex!("09e679aa6e1147fbaa5580073202fc7f"))
    //   size: 0
    // see: [docs/development/generate_blob.md]
    let signed = hex!(
        "86f116d8cad06b67df476f0738750a937330f1cb20475f3c1197ea16d069ab6ce1f867fd55"
        "5fd7b4861058c128fa9c68872efed5284135a041e451d4ded93403789c558cb10ec1501440"
        "ab0b62f2013e82d5c220169b18459ebedbb8d5bebebcbe36a9854162b048f80269dab489b1"
        "8b0fe86614fd02bb19096210dbc939c95907d2e770d0d184914518eae0c890b872628b8498"
        "a8418b82578f255aef402c7e29b473e3bc3c0dfb7ba4b9bacad267b3d3db1dd39a71cbb695"
        "9013014ce66af9ea27acdabd278345b1a13ee69107c2419b2991268048a0bf51e472fa6f02"
        "0767a0c463d3d6a61f2c855fdcbc0073734fdf"
    );
    let signed_encrypted = hex!(
        "be4be31375b99ebf3e76c54b1e91d05f5739e7e11b631b0c51cf2a91c3ca92c3a074e4d044"
        "9dfd02c55444d9bc500402680daa04bcbf13d8d38f8a296597cb31cc2bc277c7d1c8bb5bb4"
        "2321b134055351d9a81ce423d42c85ede0cf8bc8722662ce1227e4bef0645c7ae934df708b"
        "397cdc582d6eabe8155cf3a00bde4cc2f2ff58e9bd415829fb4e985bc5c170b94bfe05378c"
        "03bd86a5c074eacef07dfb5760fe6b072960c70d7ca95c9a1ec6ab5769e4bd700d7f31258a"
        "e24e5a8f1525950fea50d447082c90e4599d30833d2776b268b0b59804ead23967e5ef3e2c"
        "f86ad132cc806f23bfcc2be8a2daae49139f9a3fb0a6cf53a3affa839e2b3ee8fae63fc991"
        "d4b644954559695f0c61219e0551edec7123466a20e9"
    );

    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let parent = VlobID::from_hex("09e679aa6e1147fbaa5580073202fc7f").unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();

    let expected_file_manifest = FileManifest {
        author: alice.device_id.clone(),
        timestamp: now,
        id,
        parent,
        version: 0,
        created: now,
        updated: now,
        size: 0,
        blocksize: Blocksize::try_from(8).unwrap(),
        blocks: vec![],
    };

    assert!(expected_file_manifest
        .verify(&alice.device_id, now, Some(id), Some(0))
        .is_ok());

    p_assert_eq!(
        FileManifest::verify_and_load(
            &signed,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap(),
        expected_file_manifest
    );
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &signed_encrypted,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap(),
        expected_file_manifest
    );

    // Also test ChildManifest
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            &signed_encrypted,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap(),
        ChildManifest::File(expected_file_manifest.clone())
    );

    // Also test round trip
    p_assert_eq!(
        FileManifest::verify_and_load(
            &expected_file_manifest.dump_and_sign(&alice.signing_key),
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap(),
        expected_file_manifest
    );
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &expected_file_manifest.dump_sign_and_encrypt(&alice.signing_key, &alice.local_symkey),
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap(),
        expected_file_manifest
    );
}

#[rstest]
fn invalid_load(alice: &Device) {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 0
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   blocks: []
    //   blocksize: 8
    //   parent: ext(2, hex!("09e679aa6e1147fbaa5580073202fc7f"))
    //   size: 0
    let data = hex!(
        "be4be31375b99ebf3e76c54b1e91d05f5739e7e11b631b0c51cf2a91c3ca92c3a074e4d044"
        "9dfd02c55444d9bc500402680daa04bcbf13d8d38f8a296597cb31cc2bc277c7d1c8bb5bb4"
        "2321b134055351d9a81ce423d42c85ede0cf8bc8722662ce1227e4bef0645c7ae934df708b"
        "397cdc582d6eabe8155cf3a00bde4cc2f2ff58e9bd415829fb4e985bc5c170b94bfe05378c"
        "03bd86a5c074eacef07dfb5760fe6b072960c70d7ca95c9a1ec6ab5769e4bd700d7f31258a"
        "e24e5a8f1525950fea50d447082c90e4599d30833d2776b268b0b59804ead23967e5ef3e2c"
        "f86ad132cc806f23bfcc2be8a2daae49139f9a3fb0a6cf53a3affa839e2b3ee8fae63fc991"
        "d4b644954559695f0c61219e0551edec7123466a20e9"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();

    let dummy_without_compression = hex!(
        "d6e76bcf3b749f3b5e70f2e17ccb4a397c382921bea368c57cca7000ada6bcfad4622fdce6"
        "637a6b7290d8c764b3ca5c1ca6504b873773db6d61546bd4e50f7e3cba3b7f25036d35ed0a"
        "267b5c4b531e2131884c287ff803994b9d4b2fe5ee1b91fff5963aae8d6c7e479f042f"
    );
    let dummy = hex!(
        "e2a8070aa052bfda83732d9ab5fb7177b5d9c6c4986f0563dfc18fd136d250b0283632548e"
        "26987d4902e6448133ea1288eed9f0d8a3d4cb78b4c09a469896f326f839fff79e185ff40f"
        "5a42ed59880c6aa1c0576e332b4428bac7b9da4a188198a055e237ea6e0c3df4864877f57a"
        "f6cfb243b0b7"
    );
    let expected_author = "bob@dev1".parse().unwrap();
    let expected_timestamp = "2021-12-04T11:50:43.000000Z".parse().unwrap();
    let expected_id = VlobID::from_hex("09e679aa6e1147fbaa5580073202fc7f").unwrap();
    let expected_version = 1;

    // Check that the compression is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &dummy_without_compression,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::Compression)
    );

    // Check that the serialization is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &dummy,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::Serialization)
    );

    // Check that the encryption is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &SecretKey::generate(),
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::Decryption)
    );

    // Check that the signature is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &SigningKey::generate().verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::Signature)
    );

    // Check that the author is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &alice.verify_key(),
            &expected_author,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::UnexpectedAuthor {
            expected: Box::new(expected_author),
            got: Some(Box::new(alice.device_id.clone())),
        })
    );

    // Check that the timestamp is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            expected_timestamp,
            Some(id),
            Some(0)
        ),
        Err(DataError::UnexpectedTimestamp {
            expected: expected_timestamp,
            got: now,
        })
    );

    // Check that the id is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(expected_id),
            Some(0)
        ),
        Err(DataError::UnexpectedId {
            expected: expected_id,
            got: id,
        })
    );

    // Check that the version is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &alice.verify_key(),
            &alice.device_id,
            now,
            Some(id),
            Some(expected_version)
        ),
        Err(DataError::UnexpectedVersion {
            expected: expected_version,
            got: 0,
        })
    );
}

#[rstest]
fn serde_file_manifest_ok(alice: &Device) {
    // Generated from Python implementation (Parsec v3.0.0-a.0+dev)
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
    //       key_index: 1
    //       offset: 0
    //       size: 512
    //     }
    //     {
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       key_index: 2
    //       offset: 512
    //       size: 188
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    let data = hex!(
        "2985bdaad264bc0df6d3dd5c3850e82cf17a3912bb287cce96b2378ad7a66dacf019a81154"
        "227446b8d962098c1d3d9d8ab038dfef664367828d6ae7f4bdc75deda6ec2d5776c9d8c50d"
        "3a5ce2c3859d86c44e97b65dbd627cc351a5bf01e5a1e224a0f2c995fb143e60d1492804f1"
        "494a506c7b93e2ee22993721d7c04724974d4d46339eb1133abde54bcd2022206e1475d151"
        "b34e3b3dde36010927b71504e00183b81a71d0a3b201ed147102843dd8b722f87f278a8cf4"
        "0de764a017080c6c62b608a865c9b9229e38900537281cc218e593ae9ddb665581ac7db4d9"
        "bd04d4ba27ad0b26322c8967ace5f803248fbfd60e4220fbc2a04a978b28aa823335811a14"
        "48a54d4a2256ce7702caa552398731365a09f4ac40e968be63193a7054db42a914a7be221d"
        "d98399d757141dcdf1d3f9d155b9d6611b8e7a54a7b1618116bf0cca8b345d8c6f5230e1f1"
        "445a58bc92cb040fad79d438622c6c7ff1e099b708a687a0c5d67c80e006ba8d328966ecdc"
        "535adef7fcfd9b00ddc3ea79ba9cd7c225755d7a6929275b9c409b220e2e47147c498953ab"
        "586032fe9917fca377fba4fb210a5623d0ee23b8c2f78be696f639919519977ec68f5964b4"
        "48915928c5"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = FileManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 700,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                key_index: 1,
                key: None,
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                key_index: 2,
                key: None,
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_file_manifest_legacy_pre_parsec_v3_0(alice: &Device) {
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
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 700,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                key_index: 0,
                key: Some(SecretKey::from(hex!(
                    "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                ))),
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                key_index: 0,
                key: Some(SecretKey::from(hex!(
                    "c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc"
                ))),
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
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

    let outcome = FileManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        &alice.device_id,
        now,
        None,
        None,
    );
    assert!(matches!(outcome, Err(DataError::Serialization)));
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
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        children: HashMap::from([
            (
                "wksp1".parse().unwrap(),
                VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
            ),
            (
                "wksp2".parse().unwrap(),
                VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
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
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        version: 42,
        created: now,
        updated: now,
        children: HashMap::from([
            (
                "wksp1".parse().unwrap(),
                VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
            ),
            (
                "wksp2".parse().unwrap(),
                VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_user_manifest_legacy_pre_parsec_v3_0(alice: &Device) {
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
            },
            LegacyUserManifestWorkspaceEntry {
                name: "wksp2".parse().unwrap(),
                id: VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                key: SecretKey::from(hex!(
                    "c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc"
                )),
                encryption_revision: 1,
            },
        ],
        // User manifest's `last_processed_message` field is deprecated (and hence ignored)
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
}

#[rstest]
fn serde_user_manifest(alice: &Device) {
    // Generated from Rust implementation (Parsec v3.0.0-alpha)
    // Content:
    //   type: "user_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    let data = hex!(
        "a7f9f9e4d8e30951e7977a4d142f7f6bd623f7590dff7da3059fcf851cb172f23f4eacf275"
        "d78de986ee2cd64049c54cff626ad777f7928ab5a47f751db9f7896bf64c8b10e245e40e6f"
        "847f52f503d4d0c9ad812642d5bbf71e2c7d80bc5159c191279b2f9e47a83bba7f85d8431d"
        "77570a93de678d5a3268c76aaede7790945c653d487c958f689172c25c1f160c872dc30bb1"
        "c4e63157b981d2698ddb5b8253c140e54e3028017d8bd461fcc6624a26e7daa6be4ef396cc"
        "587f368a523bc585e3b606ddf72be6c7f8eaf12f6b2410d78359782a9b508148cd30fcada6"
        "bd0f3f1cec49bd30afc6e1fa2a1fb3a0b8"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = UserManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        version: 42,
        created: now,
        updated: now,
        workspaces_legacy_initial_info: vec![],
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

    p_assert_eq!(manifest, expected);

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
    p_assert_eq!(manifest2, expected);
}

#[rstest]
#[case::valid(None, None, None, None, None)]
#[case::valid_id(None, None, Some(VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap()), None, None)]
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
    Some("Invalid timestamp: expected `2021-10-24T11:50:43.208821Z`, got `2021-12-04T11:50:43.208821Z`".to_string())
)]
#[case::invalid_id(
    None,
    None,
    Some(VlobID::from_hex("6b398b3dc6804bb784bb07b0d7038c63").unwrap()),
    None,
    Some("Invalid entry ID: expected `6b398b3d-c680-4bb7-84bb-07b0d7038c63`, got `87c6b5fd-3b45-4c94-bab5-1d6af1c6930b`".to_string())
)]
#[case::invalid_version(None, None, None, Some(0x1337), Some("Invalid version: expected `4919`, got `42`".to_string()))]
fn file_manifest_verify(
    alice: &Device,
    #[case] expected_author: Option<&str>,
    #[case] expected_timestamp: Option<DateTime>,
    #[case] expected_id: Option<VlobID>,
    #[case] expected_version: Option<u32>,
    #[case] expected_result: Option<String>,
) {
    let now = "2021-12-04T11:50:43.208821Z".parse::<DateTime>().unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let version = 42;

    let expected_author = expected_author
        .map(|author| DeviceID::from_str(author).expect("Invalid raw DeviceID"))
        .unwrap_or_else(|| alice.device_id.to_owned());
    let expected_timestamp = expected_timestamp.unwrap_or(now);

    let manifest = FileManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id,
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version,
        created: now,
        updated: now,
        size: 700,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![],
    };

    p_assert_eq!(
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
