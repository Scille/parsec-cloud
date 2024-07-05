// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU64};

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
    DataError::BadSerialization { format: Some(0), step: "msgpack+validation" },
)]
#[case::dummy(b"dummy", DataError::BadSerialization { format: None, step: "format detection" })]
#[case::dummy_compressed(
    &hex!("789c4b29cdcdad04000667022d"),
    DataError::BadSerialization { format: Some(0), step: "zstd" }
)]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn invalid_deserialize_data(#[case] data: &[u8], #[case] error: DataError) {
    let manifest = ChildManifest::deserialize_data(data);

    p_assert_eq!(manifest, Err(error));
}

#[rstest]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
        ChildManifest::verify_and_load(
            &signed,
            &alice.verify_key(),
            alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap()
        .into_file_manifest()
        .unwrap(),
        expected_file_manifest
    );
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
        .unwrap()
        .into_file_manifest()
        .unwrap(),
        expected_file_manifest
    );

    // Also test round trip
    p_assert_eq!(
        ChildManifest::verify_and_load(
            &expected_file_manifest.dump_and_sign(&alice.signing_key),
            &alice.verify_key(),
            alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap()
        .into_file_manifest()
        .unwrap(),
        expected_file_manifest
    );
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            &expected_file_manifest.dump_sign_and_encrypt(&alice.signing_key, &alice.local_symkey),
            &alice.local_symkey,
            &alice.verify_key(),
            alice.device_id,
            now,
            Some(id),
            Some(0),
        )
        .unwrap()
        .into_file_manifest()
        .unwrap(),
        expected_file_manifest
    );
}

#[rstest]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
        ChildManifest::decrypt_verify_and_load(
            &dummy_without_compression,
            &alice.local_symkey,
            &alice.verify_key(),
            alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::BadSerialization {
            format: Some(0),
            step: "zstd"
        })
    );

    // Check that the serialization is incorrect
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
            &dummy,
            &alice.local_symkey,
            &alice.verify_key(),
            alice.device_id,
            now,
            Some(id),
            Some(0)
        ),
        Err(DataError::BadSerialization {
            format: Some(0),
            step: "msgpack+validation"
        })
    );

    // Check that the encryption is incorrect
    p_assert_eq!(
        ChildManifest::decrypt_verify_and_load(
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
        ChildManifest::decrypt_verify_and_load(
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
        ChildManifest::decrypt_verify_and_load(
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
        ChildManifest::decrypt_verify_and_load(
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
        ChildManifest::decrypt_verify_and_load(
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
        ChildManifest::decrypt_verify_and_load(
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "file_manifest"
    //   author: ext(2, de10a11cec0010000000000000000000)
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
        "b1c255a968225dc9dfb30e7f8d942895b66ade79c5924c1f425579c6dd27750f3de61e"
        "41a2f132e1c0d994493c16ff4df9f8972211519c1b92bf04b88fd4a6d03cbcd2125492"
        "28a7402f7fa1b15161a2c521f6aa6058c52b344ee39bff593e17da2b61c90bd67f4d73"
        "a9abe43140d920dd287a832a28d2319d6d47a3545921df48b96963f61767882e301ed6"
        "a0fe605ade9e86b264cdc9afda41ec217a2e200a663659a350d4560d62086ba5824e55"
        "7ad1f76f3b50148dbcdb7fd3c27f7b53587f76c5bdca9c2d62879143528a8b7392a67a"
        "4b6e27d4d5765620e9b2d89f6a7019df8c17d373aec46d32207d8f27435e11de179e8d"
        "c146bbd0d673bf40379b45390d95eb84793ef52b3500c529dd3ea542cd00713b61030f"
        "924a0d55f5bb40746596942944f5d875eafce6bcfe0921561397899a95dbd9248f29fb"
        "a2cd24ffec9df96d095afb3aebde0aaed3238ec4d480d9958ad1e202a62034c2e7c23e"
        "ab0906a4cd0ca62cb569702adff82bdc9047f0a798dc038af88c5dfdf402d306202e14"
        "9997de859ad44a92e8216b266ea1e8e88b0120f6a816b90c6cd85ff9d8cb435ad6bbb1"
        "251654931c5c68f10b9894ba6cc6cc5c3b2f1f88f5dcbb5f581073762e"
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

    let manifest = ChildManifest::decrypt_verify_and_load(
        &data,
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
}

#[rstest]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
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
        Err(DataError::BadSerialization {
            format: Some(0),
            step: "msgpack+validation"
        })
    );
}

#[rstest]
fn serde_folder_manifest(alice: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "folder_manifest"
    //   author: ext(2, de10a11cec0010000000000000000000)
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
        "816426b30ff69bbfda1232fc85708987c8556a9a25a32d6f3031282abd8a7040a9d2e2"
        "eed40f839e0ef0af0c416585e49e685e9df1e15e1d11b6991689a8e1102a98325a6d7d"
        "3cbf5d129053e0e99e77074b9858f7990b8993c0c79defab4768d4407e7facbc1efa42"
        "2cbc9242dddd389019a418367afe4f4c35295dd24480f80270a2e6e2e22c62439ce8cd"
        "2cf3fa44afa66bb4dc554c6a9d422ce46836f070aca9919ca89555a385c9d1ccf64897"
        "5cc72838e452cdcd74428d00802bc204e1d4dd5cf9b272b827d5333c73da27e3ea3d96"
        "6f311889ee3e6804be13d16105c534fde987b80c58e3910001f7e3076abe2e634215fb"
        "4ae259131c8e5c4070165713301cd7027d0759f6dc1822c91bd30d90c2e7a954983a17"
        "dc08014817994b6a7ff7e42037409218cd3126a9fbf43962f91422664c44ec9e3bb5ec"
        "300cfe5b269e"
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

    let manifest = ChildManifest::decrypt_verify_and_load(
        &data,
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
}

#[rstest]
fn serde_user_manifest(alice: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "user_manifest"
    //   author: ext(2, de10a11cec0010000000000000000000)
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    let data = hex!(
        "92c7286c12e48b9790d55713dd33a0c447ae96b1ee03537ce46c64eac982d957483dd9"
        "9f7725220c2bed5c76f02caca992daec30e8fe9c46986f61555e8c7009a9895b8ab0bb"
        "843cb63c96f02f756fafdee6d991b18fd0090003de20e12a96e23f19298f78d0e451dd"
        "a7559776a519fa5bdd3adb398ccc6fa3222a293c19ac80dd400c6fb61d667848c2071b"
        "6acfe752d20bb6e8a1cd1dd91616c29bf2a19367c197ea8e5dd3b09f2ca886ff8b9c85"
        "9b726fe2bcd9fc6eb56c14067b8383c462c55fe1eff51c6d60f99105820d99bcf70f47"
        "3cca12385f6b184ce9b737d36f1b6489791a4a4183cccf4fa094"
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
    Some(DeviceID::from_hex("6b398b3dc6804bb784bb07b0d7038c63").unwrap()),
    None,
    None,
    None,
    Some("Invalid author: expected `6b398b3d-c680-4bb7-84bb-07b0d7038c63`, got `de10a11c-ec00-1000-0000-000000000000`".to_string())
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
    #[case] expected_author: Option<DeviceID>,
    #[case] expected_timestamp: Option<DateTime>,
    #[case] expected_id: Option<VlobID>,
    #[case] expected_version: Option<u32>,
    #[case] expected_result: Option<String>,
) {
    let now = "2021-12-04T11:50:43.208821Z".parse::<DateTime>().unwrap();
    let id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let version = 42;

    let expected_author = expected_author.unwrap_or(alice.device_id);
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
