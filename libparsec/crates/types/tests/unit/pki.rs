// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, str::FromStr};

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[test]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_pki_enrollment_answer_payload() {
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //
    let raw = &hex!(
        "789c6b5b99925a96999c1a9f99b22a3107c87000f20dd717e5e797c497a51665a655c6"
        "67a7561e51d8a75956acf3a667a5dfdbf35cebaf089f159d9cfce0d79d7919e13ae50b"
        "1fd6cedebb066a4c4e62526ace7adf4a0590310ab989c9199979a9cb0b8af2d3327352"
        "973abaf87afa2d29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f"
        "905899939f98b226a33437312f3e23312f252775d24688c3522b12730b7252f592f373"
        "373982442a157c93c10cb7c4e45400b9cd57ac"
    );
    let expected = PkiEnrollmentAnswerPayload {
        user_id: "alice".parse().unwrap(),
        device_id: "alice@dev1".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        human_handle: ("alice@example.com", "Alicey McAliceFace")
            .try_into()
            .unwrap(),
        profile: UserProfile::Admin,
        root_verify_key: VerifyKey::try_from(hex!(
            "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
        ))
        .unwrap(),
    };

    let data = PkiEnrollmentAnswerPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentAnswerPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
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
        "789c6bd956945a589a5a5c929a129f925a96999c1a9f9398949ab3deb75201c83754c8"
        "4d4ccec8cc4b5d55509a9493991c9f9d5a7944e1e126ee36ae8af7e5bd3ce5898ac799"
        "6a6ef23cf0f6d597d294f2b829f8d1b5c87755596a51665a2544714b88e8d926710f86"
        "8090db2bae84ad91586b57b8e6fe0c3ec96b2b552585934efe5c52525990bab7203b33"
        "3e35af283f27273735af24beb8342937b324be20b132273f3105007e8d4634"
    )[..],
    PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")),
        requested_device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::try_from(hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")).unwrap()
    }
)]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_pki_enrollment_submit_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentSubmitPayload,
) {
    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}

#[rstest]
#[case::full(
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "local_pending_enrollment"
    //   server_url: http://parsec.example.com
    //   organization_id: "my_org"
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //   enrollment_id: ext(2, hex!("d4e678ea63cc4025a0739c6c46476794"))
    //   submit_payload: {
    //     type: "pki_enrollment_submit_payload"
    //     public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //     requested_device_label: "My dev1 machine"
    //     verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //   }
    //   submitted_on: ext(1, 946771200.0)
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
        "89a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74aa736572"
        "7665725f75726cbb68747470733a2f2f7061727365632e6578616d706c652e636f6d2f"
        "af6f7267616e697a6174696f6e5f6964a66d795f6f7267b0783530395f636572746966"
        "696361746586a474797065b0783530395f6365727469666963617465a6697373756572"
        "81a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f78353039"
        "5f6365727469666963617465c403666f6fb063657274696669636174655f73686131c4"
        "03666f6fae63657274696669636174655f6964a3666f6fac7375626d69747465645f6f"
        "6ed70141cc374a80000000ad656e726f6c6c6d656e745f6964d802d4e678ea63cc4025"
        "a0739c6c46476794ae7375626d69745f7061796c6f616484a474797065bd706b695f65"
        "6e726f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579"
        "c420845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa"
        "7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a"
        "291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d79"
        "2064657631206d616368696e65ad656e637279707465645f6b6579c403666f6faa6369"
        "7068657274657874c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha1: b"foo".as_ref().into(),
                certificate_id: Some("foo".into()),
            },
            addr: ParsecPkiEnrollmentAddr::from_str(
                "parsec3://parsec.example.com/my_org?a=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T00:00:00Z".parse().unwrap(),
            enrollment_id: EnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            submit_payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                requested_device_label: alice.device_label.clone(),
            },
            encrypted_key: b"foo".as_ref().into(),
            ciphertext: b"foo".as_ref().into(),
        }
    })
)]
#[case::without(
    // Generated from Parsec v3.0.0-b.6+dev
    // Content:
    //   type: "local_pending_enrollment"
    //   server_url: http://parsec.example.com
    //   organization_id: "my_org"
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //   enrollment_id: ext(2, hex!("d4e678ea63cc4025a0739c6c46476794"))
    //   submit_payload: {
    //     type: "pki_enrollment_submit_payload"
    //     public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //     requested_device_label: "My dev1 machine"
    //     verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //   }
    //   submitted_on: ext(1, 946771200.0)
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
        "89a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74aa736572"
        "7665725f75726cbb68747470733a2f2f7061727365632e6578616d706c652e636f6d2f"
        "af6f7267616e697a6174696f6e5f6964a66d795f6f7267b0783530395f636572746966"
        "696361746586a474797065b0783530395f6365727469666963617465a6697373756572"
        "81a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f78353039"
        "5f6365727469666963617465c403666f6fb063657274696669636174655f73686131c4"
        "03666f6fae63657274696669636174655f6964c0ac7375626d69747465645f6f6ed701"
        "41cc374a80000000ad656e726f6c6c6d656e745f6964d802d4e678ea63cc4025a0739c"
        "6c46476794ae7375626d69745f7061796c6f616484a474797065bd706b695f656e726f"
        "6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c42084"
        "5415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa707562"
        "6c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48"
        "d911f145724db67265717565737465645f6465766963655f6c6162656caf4d79206465"
        "7631206d616368696e65ad656e637279707465645f6b6579c403666f6faa6369706865"
        "7274657874c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha1: b"foo".as_ref().into(),
                certificate_id: None,
            },
            addr: ParsecPkiEnrollmentAddr::from_str(
                "parsec3://parsec.example.com/my_org?a=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T00:00:00Z".parse().unwrap(),
            enrollment_id: EnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            submit_payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                requested_device_label: alice.device_label.clone(),
            },
            encrypted_key: b"foo".as_ref().into(),
            ciphertext: b"foo".as_ref().into(),
        }
    })
)]
#[ignore = "TODO: scheme has changed, must regenerate the dump"]
fn serde_local_pending_enrollment(
    alice: &Device,
    #[case] raw: &[u8],
    #[case] generate_expected: Box<dyn FnOnce(&Device) -> LocalPendingEnrollment>,
) {
    let expected = generate_expected(alice);

    let data = LocalPendingEnrollment::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = LocalPendingEnrollment::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}
