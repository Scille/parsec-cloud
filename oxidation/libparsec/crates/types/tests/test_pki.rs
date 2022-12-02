// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::{collections::HashMap, str::FromStr};
use tests_fixtures::{alice, Device};

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_types::{
    BackendPkiEnrollmentAddr, DeviceID, DeviceLabel, EnrollmentID, LocalPendingEnrollment,
    PkiEnrollmentAnswerPayload, PkiEnrollmentSubmitPayload, UserProfile, X509Certificate,
};

#[rstest]
#[case::without(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   device_id: "alice@dev1"
    //   device_label: None
    //   human_handle: None
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //
    &hex!(
        "789c6b5b5e50949f969993bad4d1c5d7d36f4d4a6a5966726a7c4e62526ace81f545f9f925"
        "f165a945996995f1d9a9954714f6699615ebbce959e9f7f63cd7fa2bc2674527273ff87567"
        "5e46b84ef9c287b5b3f7ae84eacf4c59959803643800f9866b324a7313f3e23312f3527252"
        "0f2c29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f905899939f9802"
        "00271d4432"
    )[..],
    PkiEnrollmentAnswerPayload {
        device_id: DeviceID::from_str("alice@dev1").unwrap(),
        device_label: None,
        human_handle: None,
        profile: UserProfile::Admin,
        root_verify_key: libparsec_crypto::VerifyKey::from(hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"))
    }
)]
#[case::full(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //
    &hex!(
        "789c6b5b99925a96999c1a9f99b22a3107c87000f20dd717e5e797c497a51665a655c667a7"
        "561e51d8a75956acf3a667a5dfdbf35cebaf089f159d9cfce0d79d7919e13ae50b1fd6cede"
        "bb066a4c4e62526ace7adf4a0590310ab989c9199979a9cb0b8af2d3327352973abaf87afa"
        "2d29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f905899939f98b226"
        "a33437312f3e23312f252775d24688c3522b12730b7252f592f373373982442a157c93c10c"
        "b7c4e45400b9cd57ac"
    )[..],
    PkiEnrollmentAnswerPayload {
        device_id: DeviceID::from_str("alice@dev1").unwrap(),
        device_label: Some(DeviceLabel::from_str("My dev1 machine").unwrap()),
        human_handle: Some(("alice@example.com", "Alicey McAliceFace").try_into().unwrap()),
        profile: UserProfile::Admin,
        root_verify_key: VerifyKey::from(hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"))
    }
)]
fn serde_pki_enrollment_answer_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentAnswerPayload,
) {
    let data = PkiEnrollmentAnswerPayload::load(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentAnswerPayload::load(&raw).unwrap();

    assert_eq!(data, expected);
}

#[rstest]
#[case(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_submit_payload"
    //   public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //   requested_device_label: "My dev1 machine"
    //   verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //
    &hex!(
        "789c6bd956945a589a5a5c929a129f925a96999c1a9f9398949ab3deb75201c83754c84d4c"
        "cec8cc4b5d55509a9493991c9f9d5a7944e1e126ee36ae8af7e5bd3ce5898ac7996a6ef23c"
        "f0f6d597d294f2b829f8d1b5c87755596a51665a2544714b88e8d926710f868090db2bae84"
        "ad91586b57b8e6fe0c3ec96b2b552585934efe5c52525990bab7203b333e35af283f272737"
        "35af24beb8342937b324be20b132273f3105007e8d4634"
    )[..],
    PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")),
        requested_device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::from(hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"))
    }
)]
fn serde_pki_enrollment_submit_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentSubmitPayload,
) {
    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    assert_eq!(data, expected);
}

#[rstest]
#[case::full(
    // Generated from Rust implementation (Parsec v2.14.1+dev)
    // Content:
    //   type: "local_pending_enrollment"
    //   addr: "parsec://parsec.example.com/my_org?action=pki_enrollment"
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //   enrollment_id: ext(2, hex!("d4e678ea63cc4025a0739c6c46476794"))
    //   submit_payload: hex!(
    //     "789c01a5005aff84a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061796c"
    //     "6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71acdf980e19"
    //     "d6a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce0"
    //     "4b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d7920"
    //     "64657631206d616368696e65610b4634"
    //   )
    //   submitted_on: ext(1, 946774800.0)
    //   x509_certificate: {
    //     type: "x509_certificate"
    //     certificate_id: "foo"
    //     certificate_sha1: hex!("666f6f")
    //     der_x509_certificate: hex!("666f6f")
    //     issuer: {foo:"bar"}
    //     subject: {foo:"bar"}
    //   }
    //
    &hex!(
        "88aa63697068657274657874c403666f6fa474797065b86c6f63616c5f70656e64696e675f"
        "656e726f6c6c6d656e74a461646472d9387061727365633a2f2f7061727365632e6578616d"
        "706c652e636f6d2f6d795f6f72673f616374696f6e3d706b695f656e726f6c6c6d656e74ac"
        "7375626d69747465645f6f6ed70141cc375188000000ad656e726f6c6c6d656e745f6964d8"
        "02d4e678ea63cc4025a0739c6c46476794ad656e637279707465645f6b6579c403666f6fae"
        "7375626d69745f7061796c6f6164c4b0789c01a5005aff84a474797065bd706b695f656e72"
        "6f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c4208454"
        "15cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963"
        "5f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f14572"
        "4db67265717565737465645f6465766963655f6c6162656caf4d792064657631206d616368"
        "696e65610b4634b0783530395f636572746966696361746586a669737375657281a3666f6f"
        "a3626172a474797065b0783530395f6365727469666963617465b063657274696669636174"
        "655f73686131c403666f6fa77375626a65637481a3666f6fa3626172b46465725f78353039"
        "5f6365727469666963617465c403666f6fae63657274696669636174655f6964a3666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".to_vec(),
                certificate_sha1: b"foo".to_vec(),
                certificate_id: Some("foo".into()),
            },
            addr: BackendPkiEnrollmentAddr::from_str(
                "parsec://parsec.example.com/my_org?action=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T01".parse().unwrap(),
            enrollment_id: EnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            submit_payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                requested_device_label: alice.device_label.clone().unwrap(),
            },
            encrypted_key: b"foo".to_vec(),
            ciphertext: b"foo".to_vec(),
        }
    })
)]
#[case::without(
    // Generated from Rust implementation (Parsec v2.14.1+dev)
    // Content:
    //   type: "local_pending_enrollment"
    //   addr: "parsec://parsec.example.com/my_org?action=pki_enrollment"
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //   enrollment_id: ext(2, hex!("d4e678ea63cc4025a0739c6c46476794"))
    //   submit_payload: hex!(
    //     "789c01a5005aff84a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061796c"
    //     "6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71acdf980e19"
    //     "d6a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce0"
    //     "4b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d7920"
    //     "64657631206d616368696e65610b4634"
    //   )
    //   submitted_on: ext(1, 946774800.0)
    //   x509_certificate: {
    //     type: "x509_certificate"
    //     certificate_id: None
    //     certificate_sha1: hex!("666f6f")
    //     der_x509_certificate: hex!("666f6f")
    //     issuer: {foo:"bar"}
    //     subject: {foo:"bar"}
    //   }
    //
    &hex!(
        "88b0783530395f636572746966696361746586b46465725f783530395f6365727469666963"
        "617465c403666f6fa77375626a65637481a3666f6fa3626172a669737375657281a3666f6f"
        "a3626172ae63657274696669636174655f6964c0b063657274696669636174655f73686131"
        "c403666f6fa474797065b0783530395f6365727469666963617465ad656e726f6c6c6d656e"
        "745f6964d802d4e678ea63cc4025a0739c6c46476794aa63697068657274657874c403666f"
        "6fa461646472d9387061727365633a2f2f7061727365632e6578616d706c652e636f6d2f6d"
        "795f6f72673f616374696f6e3d706b695f656e726f6c6c6d656e74ac7375626d6974746564"
        "5f6f6ed70141cc375188000000ae7375626d69745f7061796c6f6164c4b0789c01a5005aff"
        "84a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061796c6f6164aa"
        "7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71acdf980e19d6"
        "a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd9"
        "0ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c616265"
        "6caf4d792064657631206d616368696e65610b4634a474797065b86c6f63616c5f70656e64"
        "696e675f656e726f6c6c6d656e74ad656e637279707465645f6b6579c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".to_vec(),
                certificate_sha1: b"foo".to_vec(),
                certificate_id: None,
            },
            addr: BackendPkiEnrollmentAddr::from_str(
                "parsec://parsec.example.com/my_org?action=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T01".parse().unwrap(),
            enrollment_id: EnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            submit_payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                requested_device_label: alice.device_label.clone().unwrap(),
            },
            encrypted_key: b"foo".to_vec(),
            ciphertext: b"foo".to_vec(),
        }
    })
)]
fn serde_local_pending_enrollment(
    alice: &Device,
    #[case] raw: &[u8],
    #[case] generate_expected: Box<dyn FnOnce(&Device) -> LocalPendingEnrollment>,
) {
    let expected = generate_expected(alice);

    let data = LocalPendingEnrollment::load(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = LocalPendingEnrollment::load(&raw).unwrap();

    assert_eq!(data, expected);
}
