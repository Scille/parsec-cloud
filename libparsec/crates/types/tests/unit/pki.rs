// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, str::FromStr};

use libparsec_tests_lite::prelude::*;
use serde_test::{Configure, Token};

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
#[case::fake_sha256(
    "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
    X509CertificateHash::fake_sha256()
)]
#[case::sha256(
    "sha256-qNfTEzy56Dy76VIZhGzLeb6QiW40lDu1PS0HqoKBBOk=",
    X509CertificateHash::SHA256(Box::new(hex!("a8d7d3133cb9e83cbbe95219846ccb79be90896e34943bb53d2d07aa828104e9")))
)]
fn serde_cert_hash(#[case] raw: &'static str, #[case] expected: X509CertificateHash) {
    serde_test::assert_tokens(&expected, &[Token::BorrowedStr(raw)]);
}

#[test]
fn serde_cert_ref() {
    let cert_ref = X509CertificateReference::from(X509CertificateHash::fake_sha256())
        .add_or_replace_uri(X509WindowsCngURI::from(&b"DEADBEEF"[..]));
    let expected_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uris"),
        Token::Seq { len: Some(1) },
        Token::NewtypeVariant {
            name: "X509URIFlavorValue",
            // cspell:disable-next-line
            variant: "windowscng",
        },
        Token::NewtypeStruct {
            name: "X509WindowsCngURI",
        },
        Token::Bytes(b"DEADBEEF"),
        Token::SeqEnd,
        Token::Str("hash"),
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd,
    ];

    // We test both readable and compact form, both should result in the same representation as we
    // do not do any special treatment
    serde_test::assert_tokens(&cert_ref.clone().readable(), &expected_tokens);
    serde_test::assert_tokens(&cert_ref.compact(), &expected_tokens);
}

#[test]
fn serde_cert_ref_skip_unknown_uris() {
    let expected_cert_ref = X509CertificateReference::from(X509CertificateHash::fake_sha256());
    let got_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uris"),
        Token::Seq { len: Some(2) },
        // Unknown variant of X509URIFlavorValue
        Token::NewtypeVariant {
            name: "X509URIFlavorValue",
            variant: "a_unknown_uri_variant",
        },
        Token::Bytes(b"test"),
        // Invalid windows cng variant, provided with 42u8 instead of a byte array.
        Token::NewtypeVariant {
            name: "X509URIFlavorValue",
            // cspell:disable-next-line
            variant: "windowscng",
        },
        Token::I8(42),
        Token::SeqEnd,
        Token::Str("hash"),
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd,
    ];

    serde_test::assert_de_tokens(&expected_cert_ref.compact(), &got_tokens);
}

#[test]
fn serde_pki_enrollment_answer_payload() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'pki_enrollment_answer_payload'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10a11cec0010000000000000000000)
    //   device_label: 'My dev1 machine'
    //   human_handle: ['alice@example.com', 'Alicey McAliceFace']
    //   profile: 'ADMIN'
    //   root_verify_key: 0xbe2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd
    let raw = &hex!(
    "0028b52ffd0058950600140c87a474797065bd706b695f656e726f6c6c6d656e745f61"
    "6e737765725f7061796c6f6164a7757365725f6964d802a11cec001000a96465766963"
    "65de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68616e"
    "646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d63466163"
    "65a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420be29"
    "76732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf22e0"
    "67c68292eda7d8b40128"
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
fn serde_pki_enrollment_submit_payload() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'pki_enrollment_submit_payload'
    //   verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9
    //   public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d
    //   requested_device_label: 'My dev1 machine'
    let raw = &hex!(
    "0028b52ffd005829050084a474797065bd706b695f656e726f6c6c6d656e745f737562"
    "6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054db"
    "a8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c69635f6b6579c420e1"
    "b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db6726571"
    "7565737465645f6465766963655f6c6162656caf4d792064657631206d616368696e65"
    );
    let expected = PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!(
            "e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d"
        )),
        requested_device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::try_from(hex!(
            "845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"
        ))
        .unwrap(),
    };

    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}

#[rstest]
#[case::full(
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_pending_enrollment'
    //   server_url: 'https://parsec.example.com/'
    //   organization_id: 'my_org'
    //   x509_certificate: {
    //     type: 'x509_certificate',
    //     issuer: { foo: 'bar', },
    //     subject: { foo: 'bar', },
    //     der_x509_certificate: 0x666f6f,
    //     certificate_sha1: 0x666f6f,
    //     certificate_id: 'foo',
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   enrollment_id: ext(2, 0xd4e678ea63cc4025a0739c6c46476794)
    //   submit_payload: {
    //     type: 'pki_enrollment_submit_payload',
    //     verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9,
    //     public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d,
    //     requested_device_label: 'My dev1 machine',
    //   }
    //   encrypted_key: 0x666f6f
    //   ciphertext: 0x666f6f
    &hex!(
    "0028b52ffd00587d0c003617594220719b0380e514d470537524df0b186d14ff3f7b9a"
    "c00a46c45287789e292d6e22257d9992449a4b761259b66746ac14d01692ccf4b9f714"
    "a8c7ece49b6df06eff163e0045005400d6fbe33015d98f7a5f88a129b4de5926cc2308"
    "cf8ac18d4122db87759a1ad87aefa5627ef69eb567db5dcad89cfe949f455b52ae6fc2"
    "e8330557ee8ae01032643f11466440858d0509e31183fa937c6f63546643ee1548f64a"
    "08bcec6b71afa4d17b96276ee40b5466473ca440ee1548c373666daba4f085b1abbecf"
    "f13150b0606b091d4e726c00520298b0117b5259ed76b4a00a703e4ac0e008f8402364"
    "e240742044c185c40c92099811031b5e6c48503c238e4b1e57f5b3d6bd2ea92b5024f4"
    "c9c1fa563c5811204c1036c487888b8863bbe19209ee9534ba1bdfe7ab76ecdc2b49f2"
    "addc47ca22a67d4588794234961eeec3329982f3716c8de5937c2d6726fc283eff4007"
    "8197914e82cef4915043f39221c302013cd1040e0091164424a806382a19cac64c34e2"
    "d03a9a6fb339e09bd37488e39215d2d02c4667da1b0bf80c"
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
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_pending_enrollment'
    //   server_url: 'https://parsec.example.com/'
    //   organization_id: 'my_org'
    //   x509_certificate: {
    //     type: 'x509_certificate',
    //     issuer: { foo: 'bar' },
    //     subject: { foo: 'bar' },
    //     der_x509_certificate: 0x666f6f,
    //     certificate_sha1: 0x666f6f,
    //     certificate_id: None,
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   enrollment_id: ext(2, 0xd4e678ea63cc4025a0739c6c46476794)
    //   submit_payload: {
    //     type: 'pki_enrollment_submit_payload',
    //     verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9,
    //     public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d,
    //     requested_device_label: 'My dev1 machine',
    //   }
    //   encrypted_key: 0x666f6f
    //   ciphertext: 0x666f6f
    &hex!(
    "0028b52ffd00586d0c000697584210739b03002b4ae1676013aa07d8180d24fcf1dba3"
    "7e620c2128f91b87e0b4b88994f4654a12452ed94964590d55e1831f6723391e744da1"
    "999d5aa0c4c28168d3363d0045005300ef8fc354643fea7d6186a6d07a6799308f1f3c"
    "2b0637fe886c1fd6697260ebbdd78af9d97bd69e6d77296373fa537e166d49b9be09a3"
    "4f155cb93ba243191ab29f081342c0c2e68284013083fa937c6f635466432d64af80c0"
    "cbbe16f74a1abd6789e246c6406596c4432ac5bd4ad9f09c59db2a287c61ecaaef737c"
    "0d1630d85aac1e4c766c0150049cc0117b5259ed7abca00c703e2664700680b0112a79"
    "204210aae052a2c608058cc8010e313623279e11c7248fabfa59eb5e9354162712fae4"
    "607d2b1f2c091166082be2130223ead872b804ee9534ba1bdfe7ab76ecdc2b49f2addc"
    "47ca22a67d4588794034961e0ec4329982f3716c8de5937c2d6725fc29400f81478117"
    "918e82ce049a7043f3a221e3220134d1040e0091164424a806382a59c9c61c34e2d03a"
    "da6cb30dc022a7e910c7252ba4a1598cceb43716f019"
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
