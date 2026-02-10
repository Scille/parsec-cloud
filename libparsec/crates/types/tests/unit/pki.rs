// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use libparsec_tests_lite::prelude::*;
use serde_test::{Configure, Token};

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
        .add_or_replace_uri(X509WindowsCngURI {
            issuer: b"DEAD".into(),
            serial_number: b"BEEF".into(),
        });
    let expected_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uris"), // Start uris field
        Token::Seq { len: Some(1) },
        Token::NewtypeVariant {
            name: "X509URIFlavorValue",
            // cspell:disable-next-line
            variant: "windowscng",
        },
        Token::Struct {
            name: "X509WindowsCngURI",
            len: 2,
        },
        Token::Str("issuer"),
        Token::Bytes(b"DEAD"),
        Token::Str("serial_number"),
        Token::Bytes(b"BEEF"),
        Token::StructEnd, // End X509WindowsCngURI
        Token::SeqEnd,
        Token::Str("hash"), // Start hash field
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd, // End X509CertificateReference
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
        profile: UserProfile::Admin,
        root_verify_key: VerifyKey::from(hex!(
            "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"
        )),
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
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   type: 'pki_enrollment_submit_payload'
    //   verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9
    //   public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d
    //   device_label: 'My dev1 machine'
    //   human_handle: [ 'alice@example.com', 'Alice McAlice', ]
    let raw: &[u8] = hex!(
        "0028b52ffd005849060085a474797065bd706b695f656e726f6c6c6d656e745f737562"
        "6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054db"
        "a8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c69635f6b6579c420e1"
        "b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724dac646576"
        "6963655f6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68616e"
        "646c6592b1616c696365406578616d706c652e636f6dad416c696365204d63416c6963"
        "65"
    )
    .as_ref();

    let expected = PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!(
            "e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d"
        )),
        device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::from(hex!(
            "845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"
        )),
    };
    println!("***expected: {:?}", expected.dump());
    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}
