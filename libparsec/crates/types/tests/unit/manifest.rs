// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64, str::FromStr};

use libparsec_crypto::{SecretKey, SigningKey};
use libparsec_tests_lite::prelude::*;

use crate::{
    fixtures::{alice, Device},
    BlockAccess, BlockID, Blocksize, ChildManifest, DataError, DateTime, DeviceID, FileManifest,
    FolderManifest, HashDigest, UserManifest, VlobID,
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
        "789ceb5e5e965a549c999fa7b5ac20b12835afe406137b49fffef4656e4e2dea6df1d7"
        "0df6db2d4f2e4a4d2c494db9cee87823eb6acbd9d8e04599293798da8f6dfd6bedea33"
        "65d756d9ac8fc726732f2f2d484155b6a4a4b220756d5a664e6a7c6e625e665a6a71c9"
        "b2a49cfce4ece2094b8a33ab52cf32ed5996585a92915fb42a31273339d52125b5cc70"
        "6549662e5061626e01c2a095105d402d4c0066eb5178"
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
        "86f116d8cad06b67df476f0738750a937330f1cb20475f3c1197ea16d069ab6ce1f867"
        "fd555fd7b4861058c128fa9c68872efed5284135a041e451d4ded93403789c558cb10e"
        "c1501440ab0b62f2013e82d5c220169b18459ebedbb8d5bebebcbe36a9854162b048f8"
        "0269dab489b18b0fe86614fd02bb19096210dbc939c95907d2e770d0d184914518eae0"
        "c890b872628b8498a8418b82578f255aef402c7e29b473e3bc3c0dfb7ba4b9bacad267"
        "b3d3db1dd39a71cbb6959013014ce66af9ea27acdabd278345b1a13ee69107c2419b29"
        "91268048a0bf51e472fa6f020767a0c463d3d6a61f2c855fdcbc0073734fdf"
    );
    let signed_encrypted = hex!(
        "be4be31375b99ebf3e76c54b1e91d05f5739e7e11b631b0c51cf2a91c3ca92c3a074e4"
        "d0449dfd02c55444d9bc500402680daa04bcbf13d8d38f8a296597cb31cc2bc277c7d1"
        "c8bb5bb42321b134055351d9a81ce423d42c85ede0cf8bc8722662ce1227e4bef0645c"
        "7ae934df708b397cdc582d6eabe8155cf3a00bde4cc2f2ff58e9bd415829fb4e985bc5"
        "c170b94bfe05378c03bd86a5c074eacef07dfb5760fe6b072960c70d7ca95c9a1ec6ab"
        "5769e4bd700d7f31258ae24e5a8f1525950fea50d447082c90e4599d30833d2776b268"
        "b0b59804ead23967e5ef3e2cf86ad132cc806f23bfcc2be8a2daae49139f9a3fb0a6cf"
        "53a3affa839e2b3ee8fae63fc991d4b644954559695f0c61219e0551edec7123466a20"
        "e9"
    );

    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let parent = VlobID::from_hex("09e679aa6e1147fbaa5580073202fc7f").unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();

    let expected_file_manifest = FileManifest {
        author: alice.device_id,
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
        .verify(alice.device_id, now, Some(id), Some(0))
        .is_ok());

    p_assert_eq!(
        FileManifest::verify_and_load(
            &signed,
            &alice.verify_key(),
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
        "be4be31375b99ebf3e76c54b1e91d05f5739e7e11b631b0c51cf2a91c3ca92c3a074e4"
        "d0449dfd02c55444d9bc500402680daa04bcbf13d8d38f8a296597cb31cc2bc277c7d1"
        "c8bb5bb42321b134055351d9a81ce423d42c85ede0cf8bc8722662ce1227e4bef0645c"
        "7ae934df708b397cdc582d6eabe8155cf3a00bde4cc2f2ff58e9bd415829fb4e985bc5"
        "c170b94bfe05378c03bd86a5c074eacef07dfb5760fe6b072960c70d7ca95c9a1ec6ab"
        "5769e4bd700d7f31258ae24e5a8f1525950fea50d447082c90e4599d30833d2776b268"
        "b0b59804ead23967e5ef3e2cf86ad132cc806f23bfcc2be8a2daae49139f9a3fb0a6cf"
        "53a3affa839e2b3ee8fae63fc991d4b644954559695f0c61219e0551edec7123466a20"
        "e9"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();

    let dummy_without_compression = hex!(
        "d6e76bcf3b749f3b5e70f2e17ccb4a397c382921bea368c57cca7000ada6bcfad4622f"
        "dce6637a6b7290d8c764b3ca5c1ca6504b873773db6d61546bd4e50f7e3cba3b7f2503"
        "6d35ed0a267b5c4b531e2131884c287ff803994b9d4b2fe5ee1b91fff5963aae8d6c7e"
        "479f042f"
    );
    let dummy = hex!(
        "e2a8070aa052bfda83732d9ab5fb7177b5d9c6c4986f0563dfc18fd136d250b0283632"
        "548e26987d4902e6448133ea1288eed9f0d8a3d4cb78b4c09a469896f326f839fff79e"
        "185ff40f5a42ed59880c6aa1c0576e332b4428bac7b9da4a188198a055e237ea6e0c3d"
        "f4864877f57af6cfb243b0b7"
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
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
            expected_author,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::UnexpectedAuthor {
            expected: expected_author,
            got: Some(alice.device_id),
        })
    );

    // Check that the timestamp is incorrect
    p_assert_eq!(
        FileManifest::decrypt_verify_and_load(
            &data,
            &alice.local_symkey,
            &alice.verify_key(),
            alice.device_id,
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
            alice.device_id,
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
            alice.device_id,
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
    //   size: 700
    let data = hex!(
        "09f77ead3e78f15e5c8d1475c11f704861682b7601250493fb4c7ae5b1387ba9a48c3d"
        "fcd94bd1ba97a0a4c3928e95628588163cbdf9f6024cbbd19749841f7661d346d6dd0c"
        "cc6f95750ca8755d5ada11c7e716453699815ff75c5c39267c5238f6e02171c50bd32d"
        "271506b5a1834ff37a218b882503a607c8f8f4e26f834c2af620d733a130e3328ee2b7"
        "0adeb43319a1a1e047ba2f699362ac236d92bc88ca18dbc67cbaff476b9cf1a0e9ffcc"
        "d5e0ccebc97ee468191a513655d028ca1a973a26b510c38d26007f0f00fd8b71c3db89"
        "7fcec1a3b3f2cc82bbb8319ef2e63ff8a12dbf5c3971352a40af512a69f4263085db6e"
        "0700efc4da802a82585d1c306b311873e5cd01c92e84deb91d9d6bbab1f1cd0464e54d"
        "7a0a9a29b2cf23bf7b00dc543c5c93ef1638d9dbdd86c29be5bc88b977ea34d55dbc39"
        "5eb0b027d7985d67d96c927b8f1ddf5f3f15d1d45fb4ef6bcd622813b73ab1f243cd6b"
        "2a893d2fa90a5f41f7ca285a3f4bcf4a51faad93c4ecb171dde9e0bbb4670d365e5b1c"
        "e04a13b108ce22d4483fa634e1345aeedac2581cbc99d11fd5aef880539fc5cd0f2f75"
        "007a47b609f30349dcbae2e7"
    );
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

    let manifest = FileManifest::decrypt_verify_and_load(
        &data,
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
    let manifest2 = FileManifest::decrypt_verify_and_load(
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
        "3d24370f85424221a83a0dd4bc5d27b0d5816fc62ef3a6c3186a9211f0b2ec0955ec20"
        "204a9b351224b789537d478b2a5d8a047c3fb9b3a039d90ff8a00cb53ee1ae1f2a24ba"
        "fd14dbf674e1c7bcf7a782dd9aa695ff93263e83cb1611d6d18b8348d2f2f184fdbdcb"
        "8979640fe7fcc0c1d39c930dda0ccb243d298e357b0830774fbb630bbe3009982cc880"
        "6c89e709bd4d4e33b43966b3d0cf55440a57bb84bd3c78da48f04512c386afa17a1bfa"
        "2e14fcc0a6210661086b7427eb7ca77b28078b496c984bf5d51da7023e13485a6c34b2"
        "d80d1380cc94b2546f06aa993dbbfe3c6d6aa46820ea7bed353d14892048cfb622144d"
        "99659e6ea429f54cc0e53840c86c9ac81147860b3e"
    );
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let outcome = FileManifest::decrypt_verify_and_load(
        &data,
        &key,
        &alice.verify_key(),
        alice.device_id,
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
        "afc13ea5d28a0da5f17662d5110dd75d983f22efc1c09ddce90fb3ff8fef001aad8317"
        "957874a52cf75882123b1030891e5b46052d5bddfd1885de9fc784e07afa0d415fe7e0"
        "ffde9416d1285a8738a0e634fc1185e441886db61aa8fce5b588f502604fe26d28e295"
        "23f424b65465dcde840192a68b1668376ac86e1949c7613f65e1f760cc031df2306ae5"
        "73d0178d7a388515ffe241562e167af7eb7484a3b0608438a08ff3449759d2622847ef"
        "c05698d772b0fff9f529b1c3582b1137f7121bc2f3dcdb4de32dc3f8daac75602273de"
        "7b7d8b87451c517c1d1476422c51d72375520808e85eb64c0f6a6f2de5ca564df7cca2"
        "86530c527b4bad7594f9e3dad1f40845af2dbe8fbb97201f3c617d5d63824ff36ce61f"
        "ccd31ac8d5cb3c2f1609236a266b46ae9d01b508"
    );
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

    let manifest = FolderManifest::decrypt_verify_and_load(
        &data,
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
    let manifest2 = FolderManifest::decrypt_verify_and_load(
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
        "a7f9f9e4d8e30951e7977a4d142f7f6bd623f7590dff7da3059fcf851cb172f23f4eac"
        "f275d78de986ee2cd64049c54cff626ad777f7928ab5a47f751db9f7896bf64c8b10e2"
        "45e40e6f847f52f503d4d0c9ad812642d5bbf71e2c7d80bc5159c191279b2f9e47a83b"
        "ba7f85d8431d77570a93de678d5a3268c76aaede7790945c653d487c958f689172c25c"
        "1f160c872dc30bb1c4e63157b981d2698ddb5b8253c140e54e3028017d8bd461fcc662"
        "4a26e7daa6be4ef396cc587f368a523bc585e3b606ddf72be6c7f8eaf12f6b2410d783"
        "59782a9b508148cd30fcada6bd0f3f1cec49bd30afc6e1fa2a1fb3a0b8"
    );
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
        &data,
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
        .unwrap_or_else(|| alice.device_id);
    let expected_timestamp = expected_timestamp.unwrap_or(now);

    let manifest = FileManifest {
        author: alice.device_id,
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
                expected_author,
                expected_timestamp,
                expected_id,
                expected_version,
            )
            .map_err(|e| e.to_string())
            .err(),
        expected_result
    );
}
