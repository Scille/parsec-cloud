// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, str::FromStr};

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[test]
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
        "789c6b5b99925a96999c1a9f99b22a3107c87000f20dd717e5e797c497a51665a655c667a7"
        "561e51d8a75956acf3a667a5dfdbf35cebaf089f159d9cfce0d79d7919e13ae50b1fd6cede"
        "bb066a4c4e62526ace7adf4a0590310ab989c9199979a9cb0b8af2d3327352973abaf87afa"
        "2d29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f905899939f98b226"
        "a33437312f3e23312f252775d24688c3522b12730b7252f592f373373982442a157c93c10c"
        "b7c4e45400b9cd57ac"
    );
    let expected = PkiEnrollmentAnswerPayload {
        device_id: DeviceID::from_str("alice@dev1").unwrap(),
        device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
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
        "789c6bd956945a589a5a5c929a129f925a96999c1a9f9398949ab3deb75201c83754c84d4c"
        "cec8cc4b5d55509a9493991c9f9d5a7944e1e126ee36ae8af7e5bd3ce5898ac7996a6ef23c"
        "f0f6d597d294f2b829f8d1b5c87755596a51665a2544714b88e8d926710f868090db2bae84"
        "ad91586b57b8e6fe0c3ec96b2b552585934efe5c52525990bab7203b333e35af283f272737"
        "35af24beb8342937b324be20b132273f3105007e8d4634"
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
    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}

#[rstest]
#[case::full(
    // Generated from Python implementation (Parsec v3.0.0-a.0 2024-04-19)
    // Content:
    //   "type": "local_pending_enrollment",
    //   "addr": "parsec://parsec.example.com/my_org?action=pki_enrollment",
    //   "ciphertext": b"foo",
    //   "encrypted_key": b"foo",
    //   "enrollment_id": msgpack.ExtType(2, bytes.fromhex("d4e678ea63cc4025a0739c6c46476794")),
    //   "submit_payload": {
    //       "type": "pki_enrollment_submit_payload",
    //       "public_key": bytes.fromhex(
    //           "e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d"
    //       ),
    //       "requested_device_label": "My dev1 machine",
    //       "verify_key": bytes.fromhex(
    //           "845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"
    //       ),
    //   },
    //   "submitted_on": msgpack.ExtType(code=1, struct.pack('>d', 946771200.0)),
    //   "x509_certificate": {
    //       "type": "x509_certificate",
    //       "certificate_id": "foo",
    //       "certificate_sha256": f"foo",
    //       "der_x509_certificate": f"foo",
    //       "issuer": {"foo": "bar"},
    //       "subject": {"foo": "bar"},
    //   },
    //
    &hex!(
        "88a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74a4"
        "61646472d9387061727365633a2f2f7061727365632e6578616d706c652e636f"
        "6d2f6d795f6f72673f616374696f6e3d706b695f656e726f6c6c6d656e74aa63"
        "697068657274657874c403666f6fad656e637279707465645f6b6579c403666f"
        "6fad656e726f6c6c6d656e745f6964d802d4e678ea63cc4025a0739c6c464767"
        "94ae7375626d69745f7061796c6f616484a474797065bd706b695f656e726f6c"
        "6c6d656e745f7375626d69745f7061796c6f6164aa7075626c69635f6b6579c4"
        "20e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f14572"
        "4db67265717565737465645f6465766963655f6c6162656caf4d792064657631"
        "206d616368696e65aa7665726966795f6b6579c420845415cd821748005054db"
        "a8d456ac18ad3e71acdf980e19d6a925191362c9f9ac7375626d69747465645f"
        "6f6ed70141cc374a80000000b0783530395f636572746966696361746586a474"
        "797065b0783530395f6365727469666963617465ae6365727469666963617465"
        "5f6964a3666f6fb263657274696669636174655f736861323536a3666f6fb464"
        "65725f783530395f6365727469666963617465a3666f6fa669737375657281a3"
        "666f6fa3626172a77375626a65637481a3666f6fa3626172"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha256: b"foo".as_ref().into(),
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
    // Generated from Python implementation (Parsec v3.0.0-a.0 2024-04-19)
    // Content:
    //   "type": "local_pending_enrollment",
    //   "addr": "parsec://parsec.example.com/my_org?action=pki_enrollment",
    //   "ciphertext": b"foo",
    //   "encrypted_key": b"foo",
    //   "enrollment_id": msgpack.ExtType(2, bytes.fromhex("d4e678ea63cc4025a0739c6c46476794")),
    //   "submit_payload": {
    //       "type": "pki_enrollment_submit_payload",
    //       "public_key": bytes.fromhex(
    //           "e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d"
    //       ),
    //       "requested_device_label": "My dev1 machine",
    //       "verify_key": bytes.fromhex(
    //           "845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"
    //       ),
    //   },
    //   "submitted_on": msgpack.ExtType(code=1, struct.pack('>d', 946771200.0)),
    //   "x509_certificate": {
    //       "type": "x509_certificate",
    //       "certificate_id": None,
    //       "certificate_sha256": f"foo",
    //       "der_x509_certificate": f"foo",
    //       "issuer": {"foo": "bar"},
    //       "subject": {"foo": "bar"},
    //   },
    //
    &hex!(
        "88a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74a4"
        "61646472d9387061727365633a2f2f7061727365632e6578616d706c652e636f"
        "6d2f6d795f6f72673f616374696f6e3d706b695f656e726f6c6c6d656e74aa63"
        "697068657274657874c403666f6fad656e637279707465645f6b6579c403666f"
        "6fad656e726f6c6c6d656e745f6964d802d4e678ea63cc4025a0739c6c464767"
        "94ae7375626d69745f7061796c6f616484a474797065bd706b695f656e726f6c"
        "6c6d656e745f7375626d69745f7061796c6f6164aa7075626c69635f6b6579c4"
        "20e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f14572"
        "4db67265717565737465645f6465766963655f6c6162656caf4d792064657631"
        "206d616368696e65aa7665726966795f6b6579c420845415cd821748005054db"
        "a8d456ac18ad3e71acdf980e19d6a925191362c9f9ac7375626d69747465645f"
        "6f6ed70141cc374a80000000b0783530395f636572746966696361746586a474"
        "797065b0783530395f6365727469666963617465ae6365727469666963617465"
        "5f6964c0b263657274696669636174655f736861323536a3666f6fb46465725f"
        "783530395f6365727469666963617465a3666f6fa669737375657281a3666f6f"
        "a3626172a77375626a65637481a3666f6fa3626172"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha256: b"foo".as_ref().into(),
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
fn serde_local_pending_enrollment_legacy_v2(
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

#[rstest]
#[case::full(
    // Generated from Parsec v3.0.0-b.6+dev (2024-04-19)
    // Content:
    //   type: "local_pending_enrollment"
    //   addr: "parsec3://parsec.example.com/my_org?a=pki_enrollment"
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
        "88a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74b0783530395f"
        "636572746966696361746586a474797065b0783530395f6365727469666963617465a66973"
        "7375657281a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f7835"
        "30395f6365727469666963617465c403666f6fb263657274696669636174655f7368613235"
        "36c403666f6fae63657274696669636174655f6964a3666f6fa461646472d9347061727365"
        "63333a2f2f7061727365632e6578616d706c652e636f6d2f6d795f6f72673f613d706b695f"
        "656e726f6c6c6d656e74ac7375626d69747465645f6f6ed70141cc374a80000000ad656e72"
        "6f6c6c6d656e745f6964d802d4e678ea63cc4025a0739c6c46476794ae7375626d69745f70"
        "61796c6f616484a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061"
        "796c6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71"
        "acdf980e19d6a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c77"
        "6121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963"
        "655f6c6162656caf4d792064657631206d616368696e65ad656e637279707465645f6b6579"
        "c403666f6faa63697068657274657874c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha256: b"foo".as_ref().into(),
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
    // Generated from Parsec v3.0.0-b.6+dev (2024-04-19)
    // Content:
    //   type: "local_pending_enrollment"
    //   addr: "parsec3://parsec.example.com/my_org?a=pki_enrollment"
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
        "88a474797065b86c6f63616c5f70656e64696e675f656e726f6c6c6d656e74b0783530395f"
        "636572746966696361746586a474797065b0783530395f6365727469666963617465a66973"
        "7375657281a3666f6fa3626172a77375626a65637481a3666f6fa3626172b46465725f7835"
        "30395f6365727469666963617465c403666f6fb263657274696669636174655f7368613235"
        "36c403666f6fae63657274696669636174655f6964c0a461646472d934706172736563333a"
        "2f2f7061727365632e6578616d706c652e636f6d2f6d795f6f72673f613d706b695f656e72"
        "6f6c6c6d656e74ac7375626d69747465645f6f6ed70141cc374a80000000ad656e726f6c6c"
        "6d656e745f6964d802d4e678ea63cc4025a0739c6c46476794ae7375626d69745f7061796c"
        "6f616484a474797065bd706b695f656e726f6c6c6d656e745f7375626d69745f7061796c6f"
        "6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18ad3e71acdf98"
        "0e19d6a925191362c9f9aa7075626c69635f6b6579c420e1b20b860a78ef778d0c776121c7"
        "027cd90ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c"
        "6162656caf4d792064657631206d616368696e65ad656e637279707465645f6b6579c40366"
        "6f6faa63697068657274657874c403666f6f"
    )[..],
    Box::new(|alice: &Device| {
        LocalPendingEnrollment {
            x509_certificate: X509Certificate {
                issuer: HashMap::from([("foo".into(), "bar".into())]),
                subject: HashMap::from([("foo".into(), "bar".into())]),
                der_x509_certificate: b"foo".as_ref().into(),
                certificate_sha256: b"foo".as_ref().into(),
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
