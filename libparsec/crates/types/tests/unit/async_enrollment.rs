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
            //   encrypted_key: {
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
                "0028b52ffd0058d50b00f616543e30ab261dc080454eb5d488d6d2a3f62896ed1ad859"
                "e36948363f22646f7bb225a32997d8b45d5ef3047daa6f574f85e6bda7ac74c677403a"
                "430f0788ad053e0047004400dc2b4508d26c369b0dc604d9c41f35edc82728ac373de3"
                "b71a0d0635a713b0f262a5f156bc2c919a5b9fd2b70c3e9dfde93d18fbcafa92ebe1d4"
                "353f150214d61be6e200bd83ecd333b6deb47dca570f1e53382824174238b03470a201"
                "143e542c5822541a9b584014e05c1a48582e1f1a6afb9c3c81549283143b1e8408481a"
                "6b439e0b2167a00b8b8a484127e3b11175643a1b51e7e34423f4e8e4264728bdedb080"
                "81c78609d743b47d5257647d93b34dc03830d719bf08e5c04986bfd6ebb477aadc9b1d"
                "db09131a9f6b71a4320284ae87a8d4dc124b9d239ff18e6296a29101206420ca5c5854"
                "ac9304569c5cf781d02311929a4911d89a454a2260901cab8f04cfa37e2e630fcf180e"
                "00c000c033aa0e304f65cc1c6ef56449e22033e9d919eb2cab75ca22eb3242ad52030b"
                "bb11a4"
            )
            .as_ref(),
            AsyncEnrollmentLocalPending {
                server_url: alice.organization_addr.clone().into(),
                organization_id: alice.organization_id().to_owned(),
                submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                enrollment_id: AsyncEnrollmentID::from_hex("008ed852852b413b9f765ef290a7b23b")
                    .unwrap(),
                requested_device_label: alice.device_label.clone(),
                requested_human_handle: alice.human_handle.clone(),
                encrypted_key: AsyncEnrollmentLocalPendingEncryptedKey::PKI {
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
            //   encrypted_key: {
            //     type: 'OPEN_BAO',
            //     openbao_ciphertext_key_path: '65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32',
            //     openbao_entity_id: '65732d02-bb5f-7ce7-eae4-69067383b61d',
            //     openbao_preferred_auth_id: 'auth/my_sso',
            //   }
            //   ciphertext_signing_key: 0x3c636970686572746578745f7369676e696e675f6b65793e
            //   ciphertext_private_key: 0x3c636970686572746578745f707269766174655f6b65793e
            hex!(
                "0028b52ffd0058b50c00a6195b3e206ddb1c004c8270aa001061058e699d6655168165"
                "0c8107a41b67b6de5bda8e47d62c97926b80063fea0f6e5ec118ad6da86df8ad6f0850"
                "0104c072a01f47004e004a000836a26c5a2576102475d6ad1d0f4551940556e1c4a7c5"
                "9cc6272adb631fcae99ec7c2627c09d8ccabf5fd769f69b0a88cf4b6a7837536a6c7e6"
                "ac97be21cd99fb709b318609011e163e237557538769d7a9bca0c32695ed7192333ae6"
                "9f9b7d28b7c78ec7d9b507bf20978f8d0a231c6c172e18021218845088488444c91111"
                "315d240d0044242168ed788c25f6b48c837533b3d6d28b2aa249a25092437177673a9c"
                "9ccaf29e035122144d1609637361673b5fd358e3f8feb21276758fde560f19cc815680"
                "b120932142bea6e3b1cd17d2731cba043063920fe5c47c180817c76e93d75e14080022"
                "5fd31695d4921948cdb2ced87361cd6d88b22facb98ebff7d5afed30601365672c31bd"
                "071445584516cba340d18b1005551505657a52f4a877aae24fac040e00c000c033aab8"
                "8b472979cff856e2e682786bc63b339658d6ed9419eb3257ad52030bbb11a4"
            )
            .as_ref(),
            AsyncEnrollmentLocalPending {
                server_url: alice.organization_addr.clone().into(),
                organization_id: alice.organization_id().to_owned(),
                submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                enrollment_id: AsyncEnrollmentID::from_hex("008ed852852b413b9f765ef290a7b23b")
                    .unwrap(),
                requested_device_label: alice.device_label.clone(),
                requested_human_handle: alice.human_handle.clone(),
                encrypted_key: AsyncEnrollmentLocalPendingEncryptedKey::OpenBao {
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
