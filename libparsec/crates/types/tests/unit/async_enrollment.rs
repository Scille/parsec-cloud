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
fn async_enrollment_local_pending(alice: &Device) {
    for (raw, expected) in [
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   type: 'async_enrollment_local_pending'
            //   server_url: 'http://alice_dev1.example.com:9999/'
            //   organization_id: 'CoolOrg'
            //   submitted_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
            //   enrollment_id: ext(2, 0x008ed852852b413b9f765ef290a7b23b)
            //   requested_device_label: 'My dev1 machine'
            //   requested_human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
            //   identity_system: {
            //     type: 'PKI',
            //     algorithm_for_encrypted_key: 'RSAES-OAEP-SHA256',
            //     certificate_ref: {
            //       uris: [ { windowscng: 0x666f6f, }, ],
            //       hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=',
            //     },
            //     encrypted_key: 0x3c656e637279707465645f6b65793e,
            //   }
            //   ciphertext_signing_key: 0x3c636970686572746578745f7369676e696e675f6b65793e
            //   ciphertext_private_key: 0x3c636970686572746578745f707269766174655f6b65793e
            hex!(
                "0028b52ffd0058150c00f697563e30ab261dc080454eb5d488d6d2a3f62896ed1ad859"
                "e36948363f22446e7bb225a32997d8b47d5ff3047daa6f574f85e6bda7ac74c677403a"
                "430f0788ad05410049004600db0cc23293640469369bcd06638a6de2909ad8c82728ac"
                "3b7d5bc8341a0caafb095881b252794b7e9648bc8e4f29642d426dfe0461947c6d7c4d"
                "e6e1d4d59f0a01e77d8d4d180aeb0e737180e04118d4b7b5eed4bde7ab0797291c1492"
                "0b211c581a38d1000a1f2a162c112a6e4d2c200a702e0d242c970f1cd5bd7b27906a72"
                "9064078c1101895be51817161591824ec66323eac87436a2cec7c9f510756fea928daf"
                "327a747abd324a907558c0c063c384e93dafa9576ff3dd82de5882c681b9de16462807"
                "4e321c3a664f994dd50b1a6f66ada4320284ae87a8c4cbd24eed91b7fca3b6a5686400"
                "0899188832171615ec4d022b6fb2f789d1231192b84d91e878454a2260982bab8f04cf"
                "a5be595bcfb67c03050d00c000c033aa0e304f65cc1caef08e96c621de588db104a6cc"
                "2e97b96a951a58d88d2005"
            ).as_ref(),
            AsyncEnrollmentLocalPending {
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
                    .add_or_replace_uri(X509WindowsCngURI::from(Bytes::from_static(b"foo"))),
                    algorithm_for_encrypted_key: PKIEncryptionAlgorithm::RsaesOaepSha256,
                },
                ciphertext_signing_key: Bytes::from_static(b"<ciphertext_signing_key>"),
                ciphertext_private_key: Bytes::from_static(b"<ciphertext_private_key>"),
            },
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   type: 'async_enrollment_local_pending'
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
            //   ciphertext_signing_key: 0x3c636970686572746578745f7369676e696e675f6b65793e
            //   ciphertext_private_key: 0x3c636970686572746578745f707269766174655f6b65793e
            hex!(
                "0028b52ffd0058ad0c0056195a3e306d9b0ec000617e168638eaa3043459f61a5d419a"
                "a80f0f17d068b3fddeddd1483792f6be2718b96473001b9ab8316c24326c32eb2664e7"
                "c38302b8d00f45004d004a001db179d2be2e424f5759aa71a6699a2e2e0a292eebe538"
                "4e51d11ef9b0a5739c8b7aaf3560236dc6f69a3db6a19a1de72b4b07a9e43b7f4949ed"
                "3cc3b9612ec4eb7633500897713a8ba7bb32ebb32de82e49457b946c1a1af9e7241fb6"
                "f6c8f1177b06e116e4f2d161a1c4030dc30553604283900a110021b13d444848970907"
                "0122132326e5f87bfb7a19d6a1b219496982524499a249c5dd27d2a134a12cef4d1e4a"
                "7c20c912656caceb2a6fdfe0cbf16b6fd5864cdd9baf1444068380176430c8689070fd"
                "621bc6dbed3b1f5b73e814400dc976d812f371c0e34218bf4e6b5b310c0810b17d83ab"
                "d9b36a0c8462186f9763ddcc7188cdd6cd1cc75b7beab67e20b089edf140115290c5f2"
                "280ff4e2333d145541915e04bde97db2b8132bd1832ca00f00c000c033aab88b472979"
                "cff856e2e682d0c21cb4663c7363c965dd4e99b12e73d52a35b0b01b410a"
            ).as_ref(),
            AsyncEnrollmentLocalPending {
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
                ciphertext_signing_key: Bytes::from_static(b"<ciphertext_signing_key>"),
                ciphertext_private_key: Bytes::from_static(b"<ciphertext_private_key>"),
            },
        ),
    ] {
        println!("***expected: {:?}", expected.dump());

        let payload = AsyncEnrollmentLocalPending::load(raw).unwrap();
        p_assert_eq!(payload, expected);

        // Also test roundtrip

        let raw2 = payload.dump();
        let payload2 = AsyncEnrollmentLocalPending::load(&raw2).unwrap();

        p_assert_eq!(payload2, expected);
    }
}
