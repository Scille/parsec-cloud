// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
fn async_enrollment_submit_payload(alice: &Device) {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   type: 'async_enrollment_submit_payload'
    //   verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9
    //   public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d
    //   requested_device_label: 'My dev1 machine'
    //   requested_human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
    let raw: &[u8] = hex!(
        "0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e74"
        "5f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748"
        "005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b"
        "860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565"
        "737465645f6465766963655f6c6162656caf4d792064657631206d616368696e656875"
        "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
        "79204d634661636503006111e550ca8406077506"
    )
    .as_ref();

    let expected = AsyncEnrollmentSubmitPayload {
        verify_key: alice.verify_key().to_owned(),
        public_key: alice.public_key().to_owned(),
        requested_device_label: alice.device_label.clone(),
        requested_human_handle: alice.human_handle.clone(),
    };
    println!("***expected: {:?}", expected.dump());

    let payload = AsyncEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(payload, expected);

    // Also test roundtrip

    let raw2 = payload.dump();
    let payload2 = AsyncEnrollmentSubmitPayload::load(&raw2).unwrap();

    p_assert_eq!(payload2, expected);
}

#[rstest]
fn async_enrollment_accept_payload(alice: &Device) {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   type: 'async_enrollment_accept_payload'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10a11cec0010000000000000000000)
    //   device_label: 'My dev1 machine'
    //   human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
    //   profile: 'ADMIN'
    //   root_verify_key: 0xbe2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd
    let raw: &[u8] = hex!(
        "0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e74"
        "5f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576"
        "696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68"
        "616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d6346"
        "616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420"
        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf"
        "22e067c68292eda7d8b60128"
    )
    .as_ref();

    let expected = AsyncEnrollmentAcceptPayload {
        user_id: alice.user_id,
        device_id: alice.device_id,
        device_label: alice.device_label.clone(),
        human_handle: alice.human_handle.clone(),
        profile: UserProfile::Admin,
        root_verify_key: alice.root_verify_key().to_owned(),
    };
    println!("***expected: {:?}", expected.dump());

    let payload = AsyncEnrollmentAcceptPayload::load(raw).unwrap();
    p_assert_eq!(payload, expected);

    // Also test roundtrip

    let raw2 = payload.dump();
    let payload2 = AsyncEnrollmentAcceptPayload::load(&raw2).unwrap();

    p_assert_eq!(payload2, expected);
}

#[rstest]
fn async_enrollment_local_pending() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   type: 'async_enrollment_local_pending'
    //   cleartext_content: 0x3c636c656172746578745f636f6e74656e743e
    //   ciphertext_cleartext_content_digest: 0x3c636970686572746578745f636c656172746578745f636f6e74656e745f6469676573743e
    //   ciphertext_signing_key: 0x3c636970686572746578745f7369676e696e675f6b65793e
    //   ciphertext_private_key: 0x3c636970686572746578745f707269766174655f6b65793e
    let raw: &[u8] = hex!(
        "0028b52ffd0058bd03003246161eb0271d4c5d340573482bb0db92f7d89bccbab44392"
        "bb46b775d050a0ce22c56b0ca912a03113ee3c7f722d0a2268bb7c32390f5262a05190"
        "a3edbd47787bf02a6345c57926773a3588f4f62ea822751f9e2dc370629b40040a00c0"
        "00c033aa0ef0184080f947cac0922251280498cab8de3ca1"
    )
    .as_ref();
    let expected = AsyncEnrollmentLocalPending {
        cleartext_content: Bytes::from_static(b"<cleartext_content>"),
        ciphertext_cleartext_content_digest: Bytes::from_static(
            b"<ciphertext_cleartext_content_digest>",
        ),
        ciphertext_signing_key: Bytes::from_static(b"<ciphertext_signing_key>"),
        ciphertext_private_key: Bytes::from_static(b"<ciphertext_private_key>"),
    };
    println!("***expected: {:?}", expected.dump());

    let payload = AsyncEnrollmentLocalPending::load(raw).unwrap();
    p_assert_eq!(payload, expected);

    // Also test roundtrip

    let raw2 = payload.dump();
    let payload2 = AsyncEnrollmentLocalPending::load(&raw2).unwrap();

    p_assert_eq!(payload2, expected);
}

#[rstest]
fn async_enrollment_local_pending_cleartext_content(alice: &Device) {
    for (raw, expected) in [
        (
            // Generated from Parsec 3.7.2-a.0+dev
            // Content:
            //   type: 'async_enrollment_local_pending_cleartext_content'
            //   server_url: 'http://alice_dev1.example.com:9999/'
            //   organization_id: 'CoolOrg'
            //   submitted_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
            //   enrollment_id: ext(2, 0x008ed852852b413b9f765ef290a7b23b)
            //   requested_device_label: 'My dev1 machine'
            //   requested_human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
            //   identity_system: {
            //     type: 'PKI',
            //     algorithm: 'RSAES-OAEP-SHA256',
            //     certificate_ref: {
            //       uris: [
            //         {
            //           windowscng: { issuer: [ 102, 111, 111, ], serial_number: [ 98, 97, 114, ], },
            //         },
            //       ],
            //       hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=',
            //     },
            //     encrypted_key: 0x3c656e637279707465645f6b65793e,
            //   }
            hex!(
                "0028b52ffd0058c50b004618573e308d930ec000611aa41123d918f7cf2e632626bc69"
                "06b99487fc9110b95dc9965ba724d2d26ef29b66c43d4c1c1c445635165f75ac56e131"
                "96421714db163e004b004800b151fc5150414e5161ddb1b77d8ac16450ddd380d09315"
                "93b7e42749f39b63630df296fad798c4ebf8147bdae263cd87ef45c9678dafa90542a7"
                "aef66408e7e8636f6bdd31f79eaf285c8ed0988c6c3821c1f2a0890860f038d1400170"
                "e2f643317140538910805229c191b9772fe454538424395e8c0c46dc328edb5a615925"
                "1941984c2693c980010d8471611139d85c3a34228ecb4623da7894c0a95746e9698807"
                "103a3e5c80ef794d76f536df2de84d1bb40dcbf5b62f325950d2e1cfb14e8c1556d379"
                "a7a6d05261dd59ae0dd00b0a100820717d02736fea928dff49105d302e2c749a0642d3"
                "d4c923952322cf8115b3a72b3f3f317644e4ab39174488c46d7e44c7fb416281c35c59"
                "79423a97ecad6d3bdbf2010800661e018e43ecb21a63094c995d2e73d52a3510d3308c"
                "14"
            )
            .as_ref(),
            AsyncEnrollmentLocalPendingCleartextContent {
                server_url: alice.organization_addr.clone().into(),
                organization_id: alice.organization_id().to_owned(),
                submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                enrollment_id: AsyncEnrollmentID::from_hex("008ed852852b413b9f765ef290a7b23b")
                    .unwrap(),
                requested_device_label: alice.device_label.clone(),
                requested_human_handle: alice.human_handle.clone(),
                identity_system: AsyncEnrollmentLocalPendingIdentitySystem::PKI {
                    encrypted_key: Bytes::from_static(b"<encrypted_key>"),
                    certificate_ref: X509CertificateReference::from(
                        X509CertificateHash::fake_sha256(),
                    )
                    .add_or_replace_uri(X509WindowsCngURI {
                        issuer: b"foo".into(),
                        serial_number: b"bar".into(),
                    }),
                    algorithm: PKIEncryptionAlgorithm::RsaesOaepSha256,
                },
            },
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   type: 'async_enrollment_local_pending_cleartext_content'
            //   server_url: 'http://alice_dev1.example.com:9999/'
            //   organization_id: 'CoolOrg'
            //   submitted_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
            //   enrollment_id: ext(2, 0x008ed852852b413b9f765ef290a7b23b)
            //   requested_device_label: 'My dev1 machine'
            //   requested_human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
            //   identity_system: {
            //     type: 'OPEN_BAO',
            //     openbao_ciphertext_key_path: '65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32',
            //     openbao_entity_id: '65732d02-bb5f-7ce7-eae4-69067383b61d',
            //     openbao_preferred_auth_id: 'auth/my_sso',
            //   }
            hex!(
                "0028b52ffd0058050c00a618573c306d9b0ec000617e160686941d91a3dbcaf4116b84"
                "264630044292f56d778fd9fe24ed5d4e3072c9e60036f4914507b56193513338dbcf88"
                "0858d10f40004d00470016167d139797c5199f906887fb1725378dc5d55d09c8c89a6d"
                "bd568f69e8f99ef46d67bc2b5fbebdcc08e7af95fca8dc9eb7a4a4757edfdc307632bf"
                "b6c630211b0646deb1dcbfa81d2e7c63cf1ebc7aa070685418d140bb40a91090b0a083"
                "c20340078c1d1e22248a0483000f8910932e7c77b45c8671b8548ba4f480068c3b696c"
                "8610275f2ac534d3344d010e7c2069125d64aae66bb73b7875c5b794ddfc4541582a06"
                "36c185024b8608d61bdbb0dd3af63c8c983f46f03020db5f94158e021b162ee1e46a27"
                "5b59f519abc71644a21d420214c376b39aaa33de00e3569df1c2d77aeaf5fa302003e3"
                "c603c147419389731ec8c1677a288ac2915c04b9c97daa78032591832a202972a62842"
                "717e44fa9322d4c4b9c8130b005b09910b420b730099f14c9045d165b5cb129d32635d"
                "e6aa556a20a6611829"
            )
            .as_ref(),
            AsyncEnrollmentLocalPendingCleartextContent {
                server_url: alice.organization_addr.clone().into(),
                organization_id: alice.organization_id().to_owned(),
                submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                enrollment_id: AsyncEnrollmentID::from_hex("008ed852852b413b9f765ef290a7b23b")
                    .unwrap(),
                requested_device_label: alice.device_label.clone(),
                requested_human_handle: alice.human_handle.clone(),
                identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                    openbao_preferred_auth_id: "auth/my_sso".to_string(),
                    openbao_entity_id: "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
                    openbao_ciphertext_key_path:
                        "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32"
                            .to_string(),
                },
            },
        ),
    ] {
        println!("***expected: {:?}", expected.dump());

        let payload = AsyncEnrollmentLocalPendingCleartextContent::load(raw).unwrap();
        p_assert_eq!(payload, expected);

        // Also test roundtrip

        let raw2 = payload.dump();
        let payload2 = AsyncEnrollmentLocalPendingCleartextContent::load(&raw2).unwrap();

        p_assert_eq!(payload2, expected);
    }
}
