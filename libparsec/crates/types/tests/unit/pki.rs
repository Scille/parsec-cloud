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
        human_handle: HumanHandle::new("alice@example.com".parse().unwrap(), "Alice McAlice")
            .unwrap(),
    };
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
       // Generated from Parsec 3.5.1-a.0+dev
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
    //     device_label: 'My dev1 machine',
    //     human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ],
    //   }
    //   encrypted_key: 0x666f6f
    //   ciphertext: 0x666f6f
    hex!(
        "0028b52ffd00583d0d00c6d85d421093d301002b4ae167f8a13c9214a418f9027e6b43"
        "8239c64094eee610f85dd4b6544699e48a626eb2934c533554a8c764abcaea39d8daee"
        "9d130f9e8407c375d22445004a005b00aec10206190c68bed3ae96e72ebed7d5e07141"
        "f35f3ad7734943796dd625dc74aeec741c5059abe5024ecccd0ecda6a31afb82f0676c"
        "078afe624b07971fb668a9d513200225a809128f0ce691ed90d9d878ee481a0f010b21"
        "17260d801af1ddeb5afda230bafd57a2e72e266b5107f35f4d2537fbb1a8dc18288ca2"
        "984b89fe2b1187d918a5dec2d275d92bafb3393328230a463a9f09394066366c6024d8"
        "1590102b3db2002c026274b0430a93a31f31280322242a68880c208163cbf0808460ab"
        "2093e2460a050dca810e3342523c704a5aab38adf2d9a1917315f9e2c3db87da4ae732"
        "0224e10406ffd554aa59d7d996741efd57d3eb2ab5c9b149cd7bc21f8ed6ec10428ca7"
        "d9e1ae10d1f0c5b55cdc0bb3b68e6ef15e1c93503f8ef679814d2d74cd1eeab77d6604"
        "1f0502100003238a7551425dc61054c33e2a05ca464234e2d03a9a6fb339e09bd37488"
        "e39215d2d02c4667da1b0bf80c"
    ).as_ref(),
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
                device_label: alice.device_label.clone(),
                human_handle: alice.human_handle.clone()
            },
            encrypted_key: b"foo".as_ref().into(),
            ciphertext: b"foo".as_ref().into(),
        }
    })
)]
#[case::without(
    // Generated from Parsec 3.5.1-a.0+dev
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
    //     certificate_id: None,
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   enrollment_id: ext(2, 0xd4e678ea63cc4025a0739c6c46476794)
    //   submit_payload: {
    //     type: 'pki_enrollment_submit_payload',
    //     verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9,
    //     public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d,
    //     device_label: 'My dev1 machine',
    //     human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ],
    //   }
    //   encrypted_key: 0x666f6f
    //   ciphertext: 0x666f6f
     hex!(
        "0028b52ffd00582d0d0096585d422093d20180781b790cad92ee6292042ffbb7cd8e5f"
        "c48d64212143e2fcec6e644b659449ae2da64d769269ea4dc739b6f630495a402b854c"
        "ccee4293585a6f2d692445004c005b008e818205994c68bed3ae97e72ebed7cde07143"
        "f3633cd7b34943796dd626dc74aef4781a5059abd5024ecccd16cda6a31afb82f0676c"
        "178afe624b07971fb268a955131f02115020095aa2e4238379643b6838369e3bd2e633"
        "a082888569035223be7b5dab5f1446b717cf614ed6a24ee6bf9a4a6ef65b51b92f5018"
        "3d319772f9af5c36ccc628f59796aecb5e799dcd01107c10a849214940e20149c81152"
        "a3810303c1b2848060e99101680930a3831d52981cfd884115182949416344c0081c1b"
        "0607a4031b051a143754266c520c709821a2d2a153d27ac56995cf168d9caf48171ede"
        "3ed4563a1811a1ffd554aa59d7d996741efd57d3eb2ab5c9b149cd8bc2a08ed6ecf0c1"
        "cca7d9e1aa08d1f0c5b55cdc0bb3b68e6ef15e9d1351bf8e068a814d2d74cd2eeab781"
        "66100003238a75512a5dc61054c33e2a45c9461234e2d03ada6cb30dc022a7e910c725"
        "2ba4a1598cceb43716f019"
    ).as_ref(),
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
                device_label: alice.device_label.clone(),
                human_handle: alice.human_handle.clone()
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
    println!("***expected: {:?}", expected.dump());
    let data = LocalPendingEnrollment::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = LocalPendingEnrollment::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}
