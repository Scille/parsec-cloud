// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64};

use libparsec_crypto::{SecretKey, SigningKey};
use libparsec_tests_lite::prelude::*;

use crate::{
    BlockAccess, BlockID, Blocksize, ChildManifest, DataError, DateTime, DeviceID, FileManifest,
    FolderManifest, HashDigest, UserManifest, VlobID,
    fixtures::{Device, alice},
};

#[rstest]
#[case::cannot_detect_format(b"dummy", DataError::BadSerialization { format: None, step: "format detection" })]
#[case::format_0x00_cannot_decompress(
    &hex!("00789c4b29cdcdad04000667022d"),
    DataError::BadSerialization { format: Some(0), step: "zstd" }
)]
#[case::format_0x00_cannot_deserialize(
    &hex!("0028b52ffd005829000064756d6d79"),
    DataError::BadSerialization { format: Some(0), step: "msgpack+validation" }
)]
fn invalid_deserialize_data(#[case] data: &[u8], #[case] error: DataError) {
    let outcome = UserManifest::deserialize_data(data);
    p_assert_eq!(outcome, Err(error.clone()));

    let outcome = ChildManifest::deserialize_data(data);
    p_assert_eq!(outcome, Err(error));
}

#[rstest]
fn serde_file_manifest_ok_and_invalid_checks(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'file_manifest'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   parent: ext(2, 0x07748fbf67a646428427865fd730bf3e)
    //   version: 42
    //   created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   size: 700
    //   blocksize: 512
    //   blocks: [
    //     {
    //       id: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //       offset: 0,
    //       size: 512,
    //       digest: 0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560,
    //     },
    //     {
    //       id: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b),
    //       offset: 512,
    //       size: 188,
    //       digest: 0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6,
    //     },
    //   ]
    let data = &hex!(
    "0018aab6ae0ea6116fbfd96c0375f9fa23a74fbb778f1e7c4f5e419f28b34cc52a7c85"
    "c74ed81f01c6e8313b8bcdaa37afd72467c941974760bb7d8131b1c14be3c98ebe4670"
    "43eab7723b90c5f35f17ea136ce40f8dc7633a36b0b6949554a895122c1dce1ac7c589"
    "f303ee01371fcbaacf8023c3f5e0677b53302e326478423651e6faa47f8eb290f6a364"
    "f68519e4070e4d99a503d533fbc9425b614608b25c13c500e69ef1ab0f6414a74fc394"
    "377a10a840afd0f76b90288774de0213850120cc481f42ab4609b4b386394f039d01e3"
    "ee7d707a251b87136c56f0759c66b4983870171b170360d984802bad52ed33e26c3020"
    "ef94002a8c0690692ad10ee710fdaab6d46463978b58e7a8814602a904ecda74e7ea40"
    "2b250591834d74dbfa8d8ba2efd0b7552f5b9d089a5f4f5713f63e5a5ec2f02520ae96"
    "c506ba82d01a1109197983672f7e4787cf8e3c077806c9de775c4851cba40c41abf512"
    "b05f19f983727338128c8e707cc8ee9022cec5bb8129b53aeaefc515a53cd643c6b79f"
    "5841c283f267b82e9c4a01e0082f11a15d8888cc"
    )[..];
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = FileManifest {
        author: alice.device_id,
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
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                offset: 512,
                size: NonZeroU64::try_from(188).unwrap(),
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
        ],
    };

    let manifest = ChildManifest::decrypt_verify_and_load(
        data,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    )
    .unwrap()
    .into_file_manifest()
    .unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = ChildManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    )
    .unwrap()
    .into_file_manifest()
    .unwrap();
    p_assert_eq!(manifest2, expected);

    // Now that we know our payload is valid, let's make
    // sure invalid checks are detected...

    // Invalid decryption key
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &SecretKey::generate(),
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Decryption)
    );

    // Invalid signature verification key
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &SigningKey::generate().verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Signature)
    );

    // Invalid expected author
    let bad_author = DeviceID::default();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            bad_author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedAuthor {
            expected: bad_author,
            got: Some(expected.author),
        })
    );

    // Invalid expected timestamp
    let bad_timestamp = "2000-01-01T00:00:00Z".parse().unwrap();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            bad_timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedTimestamp {
            expected: bad_timestamp,
            got: expected.timestamp,
        })
    );

    // Invalid expected ID
    let bad_id = VlobID::default();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(bad_id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedId {
            expected: bad_id,
            got: expected.id,
        })
    );

    // Invalid version
    let bad_version = 1000;
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(bad_version)
        ),
        Err(DataError::UnexpectedVersion {
            expected: bad_version,
            got: expected.version,
        })
    );
}

#[rstest]
fn serde_file_manifest_invalid_blocksize(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'file_manifest'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   parent: ext(2, 0x07748fbf67a646428427865fd730bf3e)
    //   version: 42
    //   created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   size: 700
    //   blocksize: 2
    //   blocks: [ ]
    let data = &hex!(
    "bab54461c0af35956cfb8060f7af4534e6b7bc17ddef217d6d3f4ae258ae3f075a8ae4"
    "329c2c763115cd28699a0561c62bdeb4cf210ab7c5ff66eca96d00511ec3eae4c65353"
    "73a4c17063a71f920ed3fd8bf504ae0aa501b91140b923002f6a020a733b46476d1f1c"
    "6a1a39089f442f9d1bfc71be927b7707d8aeccf599bddfa2e9890fec811708ff1bd2ef"
    "6a1d35087dafee5f9e002b02eb9b90b9e49aa55b497c847e616cb448464bab2215eac0"
    "43c0128f676a5b9c70ee39bf324fbd38a87e496f4d99c48783c1a77b121684ad163ebf"
    "7dd37d17dc232a6cdc031f6ff496e4848a5cd8be20023e9e4c8c7a426a7a2efb620fb8"
    "53ef9b45fa4f3b004674e47667e5da048d77fec282dd34e386b3d6e501b3e8925a"
    )[..];
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    // How to regenerate this test payload ???
    // 1) Disable checks in `Blocksize::try_from` to accept any value
    // 2) uncomment the following code:
    //
    //     let expected = FileManifest {
    //         author: alice.device_id,
    //         timestamp: now,
    //         id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
    //         parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
    //         version: 42,
    //         created: now,
    //         updated: now,
    //         size: 700,
    //         blocksize: Blocksize::try_from(2).unwrap(),
    //         blocks: vec![
    //         ],
    //     };
    //
    // 3) Uses `misc/test_expected_payload_cooker.py`

    let outcome = ChildManifest::decrypt_verify_and_load(
        data,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    );
    assert_eq!(
        outcome,
        Err(DataError::BadSerialization {
            format: Some(0),
            step: "msgpack+validation"
        })
    );
}

#[rstest]
fn serde_folder_manifest_ok_and_invalid_checks(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'folder_manifest'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   parent: ext(2, 0x07748fbf67a646428427865fd730bf3e)
    //   version: 42
    //   created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   children: {
    //     wksp1: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //     wksp2: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b),
    //   }
    let data = &hex!(
    "7c34b29d49ec4128ee27d06da5dd1f434e94f10ff3b6fb569b5a6cb3e431373bc4337f"
    "8ef20bfa3306d6918e9b15d65e4cdccbd355fed125ff733b4f021d745b4492cf063c13"
    "76264682125be1481440c41b4ad6159098c33ef5c3547ae216b99f7ee398a976f2809e"
    "95c0a1ca3d086f16eaf688c1605922b3e4803bcca1752e00189685940108ddba4ca421"
    "8a8ac98a061ee165293e850a0e1c56ae0d4380c2cd9a397625b027b50cb7bc729915b7"
    "97e5183afb880cc022f41c5273c608e46b58a41e79443955ab4b699f025eaaaded1369"
    "313c3bd3836a1ab688eab9bb0570a435e51cb9b8da01389eed472667801277d4faa182"
    "bd567e75eca7e2915ad3f4fe526664ae1c5140c588f94e5a8575d4b6c3cc40fcfde1af"
    "a453f30703c082d2c93ba6bc57d699ae5189dc4cf8b2e4f8ddc9dc7d"
    )[..];
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = FolderManifest {
        author: alice.device_id,
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

    let manifest = ChildManifest::decrypt_verify_and_load(
        data,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    )
    .unwrap()
    .into_folder_manifest()
    .unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = ChildManifest::decrypt_verify_and_load(
        &data2,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    )
    .unwrap()
    .into_folder_manifest()
    .unwrap();
    p_assert_eq!(manifest2, expected);

    // Now that we know our payload is valid, let's make
    // sure invalid checks are detected...

    // Invalid decryption key
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &SecretKey::generate(),
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Decryption)
    );

    // Invalid signature verification key
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &SigningKey::generate().verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Signature)
    );

    // Invalid expected author
    let bad_author = DeviceID::default();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            bad_author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedAuthor {
            expected: bad_author,
            got: Some(expected.author),
        })
    );

    // Invalid expected timestamp
    let bad_timestamp = "2000-01-01T00:00:00Z".parse().unwrap();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            bad_timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedTimestamp {
            expected: bad_timestamp,
            got: expected.timestamp,
        })
    );

    // Invalid expected ID
    let bad_id = VlobID::default();
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(bad_id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedId {
            expected: bad_id,
            got: expected.id,
        })
    );

    // Invalid version
    let bad_version = 1000;
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(bad_version)
        ),
        Err(DataError::UnexpectedVersion {
            expected: bad_version,
            got: expected.version,
        })
    );
}

#[rstest]
fn serde_user_manifest_ok_and_invalid_checks(alice: &Device) {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'user_manifest'
    //   author: ext(2, 0xde10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
    //   version: 42
    //   created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    let data = &hex!(
    "ae48d1ded09d700e227723599ea232a266822aecc50e96f0d647f5afc31c35c8355646"
    "7827d317284b2c8ccb39c8d46a4ce4ec40f067b21e8625381e21d94c3657c1c304f1ee"
    "c4b54d3b610393c83624d6cb67612d7fc750472836434e724c6c22619a29fbbad99570"
    "557adb0c0fc794e6cf053ca1aae2f4e4c3656118970dbd12421f6b80c2f5f337ab81b4"
    "57a18f06a7883e01d3fb9321c987f5c0d1379b90393ae76b99d1f1359b1ba3afc63672"
    "09f82371c4d1d0b84ed4fcdeb89d14c77e978f49f83a745955560d966450aaf6a249c5"
    "cd69b85acc8eaec2aeb1946046e94d"
    )[..];
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = UserManifest {
        author: alice.device_id,
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        version: 42,
        created: now,
        updated: now,
    };

    let manifest = UserManifest::decrypt_verify_and_load(
        data,
        &key,
        &alice.verify_key(),
        alice.device_id,
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
        alice.device_id,
        now,
        None,
        None,
    )
    .unwrap();
    p_assert_eq!(manifest2, expected);

    // Now that we know our payload is valid, let's make
    // sure invalid checks are detected...

    // Invalid decryption key
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &SecretKey::generate(),
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Decryption)
    );

    // Invalid signature verification key
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &key,
            &SigningKey::generate().verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::Signature)
    );

    // Invalid expected author
    let bad_author = DeviceID::default();
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            bad_author,
            expected.timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedAuthor {
            expected: bad_author,
            got: Some(expected.author),
        })
    );

    // Invalid expected timestamp
    let bad_timestamp = "2000-01-01T00:00:00Z".parse().unwrap();
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            bad_timestamp,
            Some(expected.id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedTimestamp {
            expected: bad_timestamp,
            got: expected.timestamp,
        })
    );

    // Invalid expected ID
    let bad_id = VlobID::default();
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(bad_id),
            Some(expected.version)
        ),
        Err(DataError::UnexpectedId {
            expected: bad_id,
            got: expected.id,
        })
    );

    // Invalid version
    let bad_version = 1000;
    p_assert_eq!(
        UserManifest::decrypt_verify_and_load(
            data,
            &key,
            &alice.verify_key(),
            expected.author,
            expected.timestamp,
            Some(expected.id),
            Some(bad_version)
        ),
        Err(DataError::UnexpectedVersion {
            expected: bad_version,
            got: expected.version,
        })
    );
}

fn encode_and_format_with_70_width<T: AsRef<[u8]>>(data: T) -> String {
    format!(
        "\"{}\"",
        hex::encode(data)
            .as_bytes()
            .chunks(70)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("\"\n\"")
    )
}

fn generate_file_manifest_blocks_not_sorted(
    alice: &Device,
) -> (DateTime, Vec<u8>, FileManifest, &'static str) {
    // Generated from Parsec v3.0.0-b.6+dev
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
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       offset: 512
    //       size: 188
    //     }
    //     {
    //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //       digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
    //       offset: 0
    //       size: 512
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id,
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
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                offset: 512,
                size: NonZeroU64::try_from(188).unwrap(),
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
        ],
    };

    let data = hex!(
        "a1bdf30107f3ee7654c449aeb492e3648ef515a6295379e36c9a5b51ad1ab9fc38375e"
        "1ab531e406c77089b1bcec61142052a650064beaa79c6940a94b4735c213e52a4626d6"
        "3ef9846b1256b389341e826402eec02c2d4288796569a7f085822e47052fb6f8a3c014"
        "2bf7a5da238f11dc54e942650b19b27a0b59c27464e5f5302ac9340a63503845c2dc7f"
        "ada5e073daff421b49cd24eaa3e4dfd855df3ce31789a6c46c5603ba196cff3d08a59f"
        "1d2d35c1471b9c90f848f92fed7e6d0a193fb1b1702873d464737903d93d6576ccc557"
        "c45ba59b62a9a80c74ccc38f1943944d2a4c6d298745ec5f3bd08dd8caf5309980e25f"
        "f39f19b92515dfcb9a9a20ac4acfd559b1de69861de27e1daf80f13a6ae3095b266e8a"
        "36c45c4f8631f981e227101a89467deb34ab33911ffd9966b65e1a8cbac4b9b8cab488"
        "887602f59fd8c06b7e5e3a149c556378a490e12c665841d544b034025c9fac9452b78d"
        "89fbb664d7a061a025b79397de34ebe2cc4ffda93b5bffff6ced1ae1a8e3b8d3bcfe90"
        "0b7688c76896520052bba1429c0768ae073bf0c967cdac"
    );
    (
        now,
        data.to_vec(),
        manifest,
        "blocks are ordered and not overlapping",
    )
}

fn generate_file_manifest_blocks_overlapping(
    alice: &Device,
) -> (DateTime, Vec<u8>, FileManifest, &'static str) {
    // Generated from Parsec v3.0.0-b.6+dev
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
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       offset: 0
    //       size: 20
    //     }
    //     {
    //       id: ext(2, hex!("b82954f1138b4d719b7f5bd78915d20f"))
    //       digest: hex!("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
    //       offset: 10
    //       size: 20
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 30
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id,
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 30,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                offset: 0,
                size: NonZeroU64::try_from(20).unwrap(),
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                offset: 10,
                size: NonZeroU64::try_from(20).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
        ],
    };

    let data = hex!(
        "245868f9163206a68a47157453613cb5413c58cf3cd0f3df7ba0403a3872e20d49cf5b"
        "76f71473bde21aab92e572ffecf0bfc9740cd69a51af0e88fcc48ca73c1e9c1130680b"
        "eb277462839810bc67e2bd66970138f6ada6b7b76c4604536544a604fee49db66b0b48"
        "62d450a957c4dd5d33b168aa240d7a63292664380151bf3394518b2ab8c54ca0c1d437"
        "bbeba35035ae7bd7604da5eba272221e2f7faae44e1a60b97f5cd315624383a08ac8d5"
        "eef5e7f9ef5b8be9eeba70f661004e47be91fb42737496c7e215a3cb7857e9db291d5b"
        "e4cf9d0b6e0bea9efc6723b983d3f8e0785a5b438fc25544134746730696e9a85274e5"
        "84f974788c7ec16d646c3a3d4ed7fdf73d9bc5981f488efad4602909e80205f48b00ca"
        "52c9baedeccf6badc1a84c8e78d94e1490b13b8fd5255e55c3c97e77990188e1823f66"
        "e7177b06ecc91b158da94986eee0d317bb9a02dd2a683a193df56435d17f8bbe2687ad"
        "3aa4b6827c4b90f77c9b49f4a064a88c17ac0c77ae5ef485b3ef68245d04ba28c6e974"
        "0a7168d2c80717d9f44df0f65e0913aa2a"
    );
    (
        now,
        data.to_vec(),
        manifest,
        "blocks are ordered and not overlapping",
    )
}

fn generate_file_manifest_exceed_file_size(
    alice: &Device,
) -> (DateTime, Vec<u8>, FileManifest, &'static str) {
    // Generated from Parsec v3.0.0-b.6+dev
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
    //       offset: 0
    //       size: 512
    //     }
    //     {
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       offset: 512
    //       size: 188
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 699
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id,
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 699,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                offset: 0,
                size: NonZeroU64::try_from(512).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                offset: 512,
                size: NonZeroU64::try_from(188).unwrap(),
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
        ],
    };

    let data = hex!(
    "f83d95f290e86f6805d3678939d1741f6274500fa04a62a2e6a3d75e6138b4c3dbc12a"
    "a55ad1867cb6798f646bac3ced176e28a7285c97bfbd6533b1d425fbcd94f69e4aca55"
    "3db2b0d6f39448be90642385461723ca5b564c326529293c1e29892c5f22dcb6ef010e"
    "8dab1d98c5b5c7ab4512818c7b197e5ac754928e467126cdb978e4c4d08989d05a14a6"
    "76a88eac48f8a48e647bfe0e8b94397b9c2c5c8572812b3feb5f57d7919b408a7f8125"
    "026a18dc1025907878153040757d855ffef7c2d6b7e75095c9baa8136d9846ab422640"
    "3c77e4622ba6cfdf086800d7307b4292e81adc1e89b581f9c0ad67d64dc88f271684fe"
    "5a3d1b82ca5cc6ebd9da4adaee9b731ecc50c24ab28ebab3423ff7b1c4022bae6b250d"
    "8dab5f0dd8a16c214b115f5d8ed2036d0cfae7cf599b2d3ed30a982a5fe3a1ec14d179"
    "d1b1b75a128108c09d1b7e4c44fd772af54245d57c872b6bc214d983d7731f094b32f0"
    "f8d944aaabcad596599acc7bdd5674a5404659d840543c636a5f137bbec12416cf585c"
    "73d3173feb46c5cc738086c85cd57770e9bc2327"
    );
    (now, data.to_vec(), manifest, "file size is not exceeded")
}

fn generate_file_manifest_same_block_span(
    alice: &Device,
) -> (DateTime, Vec<u8>, FileManifest, &'static str) {
    // Generated from Parsec v3.0.0-b.6+dev
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
    //       offset: 0
    //       size: 10
    //     }
    //     {
    //       id: ext(2, hex!("d7e3af6a03e1414db0f4682901e9aa4b"))
    //       digest: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //       offset: 10
    //       size: 10
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 20
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id,
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 20,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![
            BlockAccess {
                id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
                offset: 0,
                size: NonZeroU64::try_from(10).unwrap(),
                digest: HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
            },
            BlockAccess {
                id: BlockID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap(),
                offset: 10,
                size: NonZeroU64::try_from(10).unwrap(),
                digest: HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
            },
        ],
    };

    let data = hex!(
        "c494e75876c58fec8f7aaa204ce29fff61a861fcf2fad85fe0ffce28c6c3fa983e9b97"
        "107c0fd9669e6c380cb0d61f56e0cf0788a29228447341eaf2318c2f79dc88caadee87"
        "6e52d8cc92998bc9ede625d8e5829dce1324078d2bd5b3d8e0e0b2040bef9657f4bf9c"
        "5428dc4746b4dbfe42caa9cbf2b9f41121c0aac44e46509a703df15c1c374d763e02d8"
        "0c48570fe45a525e199ed6f8821b00fee9947c393f95414ec7d7379eaa391361cb91ca"
        "aca429b9653d3982b20dceca805ff35268ad980976e60394f8b66f90eb7a863c47c9f4"
        "615c4d3f1c0ccf88b88ba8394a2dfa37ddf4aee3d6d8f97f53546b3d71456f1dc31b80"
        "045ec7280d0076ceca23b3aac849013fc56cc17ea5333ca5f69cfe4201cf09659a17e8"
        "d4cc1cf733563b1bfa44d361391989b17c34c6c80c5c5eb290f60791a39b24e38a3d87"
        "d33da42d3d2a6d811d75fc1f39225e6915d50fb7d01f43dab9db8e267e172ba2b86be7"
        "eab1391de900e224a9a02d631136348a629e2021ccdac914c201ebb9307c9210ce9967"
        "6ff0e5dc8e7039a227bf30d4e2c5cebf18"
    );
    (
        now,
        data.to_vec(),
        manifest,
        "blocks are not sharing the same block span",
    )
}

fn generate_file_manifest_in_between_block_span(
    alice: &Device,
) -> (DateTime, Vec<u8>, FileManifest, &'static str) {
    // Generated from Parsec v3.0.0-b.6+dev
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
    //       offset: 10
    //       size: 512
    //     }
    //   ]
    //   blocksize: 512
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 522
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id,
        timestamp: now,
        id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
        parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
        version: 42,
        created: now,
        updated: now,
        size: 522,
        blocksize: Blocksize::try_from(512).unwrap(),
        blocks: vec![BlockAccess {
            id: BlockID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap(),
            offset: 10,
            size: NonZeroU64::try_from(512).unwrap(),
            digest: HashDigest::from(hex!(
                "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
            )),
        }],
    };

    let data = hex!(
    "2095d44729cc85ea4ab18ba96e74ac5413620eb6371ba6ff3643bcf9358ebee45f676e"
    "eb8683989f61adda035c5fc1d2fa95665d7c2106f65d8555b625f136efd4eadf562c0a"
    "549a4358035e36ef959941fa12962251a0d2b22104cbfd291b2af030de9d1cfa2cc5c1"
    "cac22b86247c4925ac42efcdde278b1f91429a5b15f4ad8fb059562be51621e9cafec0"
    "a02d66a168246cc995fc20ecf179fe9ea56c244086f8bf6f27b746b3df3a5f84044fa3"
    "adb27f7a8bcbe88b7de57668d189dc6b255761e3ca1b11bffa23854464894a194b1264"
    "a7b6c7f24b40d80f551cc2afdc4800f0dd034bc4b661ba6e7b59767f6b78c27d592e74"
    "dd76ec29d7c609730d2a094d2d39247db1cf048774edb53688d55fe1dd06ad703d2c65"
    "cb14b8e6d0fe3524fa295bf6298d0a909a63126f576365ca586b139bac8aea19c65722"
    "7ea635684e8c044b9fef16cf67a1a16032cb47c03bbd5c48fde55ca4"
    );
    (
        now,
        data.to_vec(),
        manifest,
        "blocks are not spanning over multiple block spans",
    )
}

#[rstest]
fn serde_file_manifest_invalid_blocks(
    alice: &Device,
    #[values(
        "blocks_not_sorted",
        "blocks_overlapping",
        "exceed_file_size",
        "same_block_span",
        "in_between_block_span"
    )]
    kind: &str,
) {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));
    let (now, data, manifest, invariant) = match kind {
        "blocks_not_sorted" => generate_file_manifest_blocks_not_sorted(alice),
        "blocks_overlapping" => generate_file_manifest_blocks_overlapping(alice),
        "exceed_file_size" => generate_file_manifest_exceed_file_size(alice),
        "same_block_span" => generate_file_manifest_same_block_span(alice),
        "in_between_block_span" => generate_file_manifest_in_between_block_span(alice),
        _ => unreachable!(),
    };
    let fresh_data = manifest
        .dump_sign_and_encrypt_with_data_integrity_checks_disabled(&alice.signing_key, &key);

    // Check the freshly generated data
    let outcome = ChildManifest::decrypt_verify_and_load(
        &fresh_data,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    );

    assert_eq!(
        outcome,
        Err(DataError::DataIntegrity {
            data_type: "libparsec_types::manifest::FileManifest",
            invariant
        }),
        "The freshly generated data did not break the expected invariant",
    );

    // Check data generated from older versions
    let outcome = ChildManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        alice.device_id,
        now,
        None,
        None,
    );

    assert_eq!(
        outcome,
        Err(DataError::DataIntegrity {
            data_type: "libparsec_types::manifest::FileManifest",
            invariant
        }),
        "The data generated from older version did not break the expected invariant. Fresh payload:\n{}",
        encode_and_format_with_70_width(fresh_data)
    );
}
