// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::{SecretKey, SigningKey};

use crate::{
    fixtures::{alice, Device},
    Blocksize, ChildManifest, DataError, FileManifest, VlobID,
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

    assert_eq!(manifest, Err(error));
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

    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
    assert_eq!(
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
