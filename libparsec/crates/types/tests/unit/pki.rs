// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, str::FromStr};

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[test]
fn serde_pki_enrollment_answer_payload() {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   user_id: ext(2, a11cec00100000000000000000000000)
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   device_label: "My dev1 machine"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let raw = &hex!(
        "ff87a474797065bd706b695f656e726f6c6c6d656e745f616e737765725f7061796c6f"
        "6164a7757365725f6964d802a11cec00100000000000000000000000a9646576696365"
        "5f6964d802de10a11cec0010000000000000000000ac6465766963655f6c6162656caf"
        "4d792064657631206d616368696e65ac68756d616e5f68616e646c6592b1616c696365"
        "406578616d706c652e636f6db2416c69636579204d63416c69636546616365a770726f"
        "66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420be2976732cec8c"
        "a94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "pki_enrollment_submit_payload"
    //   public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //   requested_device_label: "My dev1 machine"
    //   verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    &hex!(
        "ff84a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061796c6f"
        "6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71ac"
        "df980e19d6a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c"
        "776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565737465645f6465"
        "766963655f6c6162656caf4d792064657631206d616368696e65"
    )[..],
    PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")),
        requested_device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::try_from(hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")).unwrap()
    }
)]
fn serde_pki_enrollment_submit_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentSubmitPayload,
) {
    println!("***expected: {:?}", expected.dump());

    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}

#[rstest]
#[case::full(
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: local_pending_enrollment
    //   server_url: https://parsec.example.com/
    //   organization_id: my_org
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //   enrollment_id: ext(2, d4e678ea63cc4025a0739c6c46476794)
    //   submit_payload: {
    //     type: "pki_enrollment_submit_payload"
    //     public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //     requested_device_label: "My dev1 machine"
    //     verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   x509_certificate: {
    //     type: "x509_certificate"
    //     certificate_id: "foo"
    //     certificate_sha1: hex!("666f6f")
    //     der_x509_certificate: hex!("666f6f")
    //     issuer: {foo:"bar"}
    //     subject: {foo:"bar"}
    //   }
    &hex!(
        "ff89a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74aa7365"
        "727665725f75726cbb68747470733a2f2f7061727365632e6578616d706c652e636f6d"
        "2faf6f7267616e697a6174696f6e5f6964a66d795f6f7267b0783530395f6365727469"
        "66696361746586a474797065b0783530395f6365727469666963617465a66973737565"
        "7281a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f783530"
        "395f6365727469666963617465c403666f6fb063657274696669636174655f73686131"
        "c403666f6fae63657274696669636174655f6964a3666f6fac7375626d69747465645f"
        "6f6ed70100035d15590f4000ad656e726f6c6c6d656e745f6964d802d4e678ea63cc40"
        "25a0739c6c46476794ae7375626d69745f7061796c6f616484a474797065bd706b695f"
        "656e726f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b65"
        "79c420845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"
        "aa7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f"
        "1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d"
        "792064657631206d616368696e65ad656e637279707465645f6b6579c403666f6faa63"
        "697068657274657874c403666f6f"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: local_pending_enrollment
    //   server_url: https://parsec.example.com/
    //   organization_id: my_org
    //   ciphertext: hex!("666f6f")
    //   encrypted_key: hex!("666f6f")
    //    enrollment_id: ext(2, d4e678ea63cc4025a0739c6c46476794)
    //   submit_payload: {
    //     type: "pki_enrollment_submit_payload"
    //     public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //     requested_device_label: "My dev1 machine"
    //     verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   x509_certificate: {
    //     type: "x509_certificate"
    //     certificate_id: None
    //     certificate_sha1: hex!("666f6f")
    //     der_x509_certificate: hex!("666f6f")
    //     issuer: {foo:"bar"}
    //     subject: {foo:"bar"}
    //   }
    &hex!(
        "ff89a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74aa7365"
        "727665725f75726cbb68747470733a2f2f7061727365632e6578616d706c652e636f6d"
        "2faf6f7267616e697a6174696f6e5f6964a66d795f6f7267b0783530395f6365727469"
        "66696361746586a474797065b0783530395f6365727469666963617465a66973737565"
        "7281a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f783530"
        "395f6365727469666963617465c403666f6fb063657274696669636174655f73686131"
        "c403666f6fae63657274696669636174655f6964c0ac7375626d69747465645f6f6ed7"
        "0100035d15590f4000ad656e726f6c6c6d656e745f6964d802d4e678ea63cc4025a073"
        "9c6c46476794ae7375626d69745f7061796c6f616484a474797065bd706b695f656e72"
        "6f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420"
        "845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075"
        "626c69635f6b6579c420e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a"
        "48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d792064"
        "657631206d616368696e65ad656e637279707465645f6b6579c403666f6faa63697068"
        "657274657874c403666f6f"
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
