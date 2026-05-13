// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    X509CertificateHash::SHA256(hex!("a8d7d3133cb9e83cbbe95219846ccb79be90896e34943bb53d2d07aa828104e9"))
)]
fn serde_cert_hash(#[case] raw: &'static str, #[case] expected: X509CertificateHash) {
    serde_test::assert_tokens(&expected, &[Token::BorrowedStr(raw)]);
}

#[test]
fn serde_cert_ref() {
    // Here we cannot test serialization & deserialization together with
    // `serde_test::assert_tokens` since `uri` field behave differently:
    // - In serialization it is provided as an optional field
    // - In deserialization it is either present or absent

    let cert_ref = X509CertificateReference {
        uri: X509Pkcs11URI {
            id: Some(b"My ID".into()),
            label: Some(b"My label".into()),
            der_issuer: b"Black Mesa".into(),
            der_subject: b"Test".into(),
            serial: b"abcde".into(),
        }
        .into(),
        hash: X509CertificateHash::fake_sha256(),
    };

    // Serialization

    let expected_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uri"), // Start `uri` field
        Token::Some,
        Token::Struct {
            name: "X509Pkcs11URI",
            len: 5,
        },
        Token::Str("id"),
        Token::Some,
        Token::Bytes(b"My ID"),
        Token::Str("label"),
        Token::Some,
        Token::Bytes(b"My label"),
        Token::Str("der_issuer"),
        Token::Bytes(b"Black Mesa"),
        Token::Str("der_subject"),
        Token::Bytes(b"Test"),
        Token::Str("serial"),
        Token::Bytes(b"abcde"),
        Token::StructEnd,   // End X509Pkcs11URI
        Token::Str("hash"), // Start `hash` field
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd, // End X509CertificateReference
    ];

    // We test both readable and compact form, both should result in the same representation as we
    // do not do any special treatment
    serde_test::assert_ser_tokens(&cert_ref.clone().readable(), &expected_tokens);
    serde_test::assert_ser_tokens(&cert_ref.clone().compact(), &expected_tokens);

    // Deserialization (with `uri` field present)

    let expected_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uri"), // Start `uri` field
        Token::Struct {
            name: "X509Pkcs11URI",
            len: 5,
        },
        Token::Str("id"),
        Token::Some,
        Token::Bytes(b"My ID"),
        Token::Str("label"),
        Token::Some,
        Token::Bytes(b"My label"),
        Token::Str("der_issuer"),
        Token::Bytes(b"Black Mesa"),
        Token::Str("der_subject"),
        Token::Bytes(b"Test"),
        Token::Str("serial"),
        Token::Bytes(b"abcde"),
        Token::StructEnd,   // End X509Pkcs11URI
        Token::Str("hash"), // Start `hash` field
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd, // End X509CertificateReference
    ];

    // We test both readable and compact form, both should result in the same representation as we
    // do not do any special treatment
    serde_test::assert_de_tokens(&cert_ref.clone().readable(), &expected_tokens);
    serde_test::assert_de_tokens(&cert_ref.clone().compact(), &expected_tokens);

    // Deserialization (with `uri` field missing)

    let expected_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 1,
        },
        Token::Str("hash"), // Start `hash` field
        Token::BorrowedStr("sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
        Token::StructEnd, // End X509CertificateReference
    ];

    // We test both readable and compact form, both should result in the same representation as we
    // do not do any special treatment
    serde_test::assert_de_tokens(&cert_ref.clone().readable(), &expected_tokens);
    serde_test::assert_de_tokens(&cert_ref.compact(), &expected_tokens);
}

#[test]
fn serde_cert_ref_skip_legacy_uris_field() {
    let expected_cert_ref = X509CertificateReference::from(X509CertificateHash::fake_sha256());
    let got_tokens = [
        Token::Struct {
            name: "X509CertificateReference",
            len: 2,
        },
        Token::Str("uris"),
        Token::Seq { len: Some(3) },
        // Valid variant of X509URIFlavorValue
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
fn cert_ref_eq_ignore_uri() {
    let cert1_hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
        .parse::<X509CertificateHash>()
        .unwrap();
    let cert2_hash = "sha256-0000000000000000000000000000000000000000000="
        .parse::<X509CertificateHash>()
        .unwrap();

    let cert1_ref1 = X509CertificateReference {
        uri: X509Pkcs11URI {
            id: None,
            label: None,
            der_issuer: b"Black Mesa".into(),
            der_subject: b"Test".into(),
            serial: b"abcde".into(),
        }
        .into(),
        hash: cert1_hash,
    };
    let cert1_ref2 = X509CertificateReference {
        uri: X509Pkcs11URI {
            id: None,
            label: None,
            der_issuer: b"Black Mesa".into(),
            der_subject: b"Test".into(),
            serial: b"12345".into(), // Different serial!
        }
        .into(),
        hash: cert1_hash,
    };
    p_assert_eq!(cert1_ref1, cert1_ref2);

    let cert1_ref3 = X509CertificateReference {
        uri: crate::Maybe::Absent,
        hash: cert1_hash,
    };
    p_assert_eq!(cert1_ref1, cert1_ref3);

    let cert2_ref1 = X509CertificateReference {
        uri: cert1_ref1.uri.clone(),
        hash: cert2_hash,
    };
    p_assert_ne!(cert1_ref1, cert2_ref1);
}
