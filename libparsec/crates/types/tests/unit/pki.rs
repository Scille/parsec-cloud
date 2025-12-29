// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

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

#[rstest]
#[case::with_cert_uri(
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   type: 'local_pending_enrollment'
    //   server_url: 'https://parsec.example.com/'
    //   organization_id: 'my_org'
    //   x509_certificate_ref: {
    //     uris: [
    //       {
    //         windowscng: { issuer: [ 102, 111, 111, ], serial_number: [ 98, 97, 114, ], },
    //       },
    //     ],
    //     hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=',
    //   }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   enrollment_id: ext(2, 0xd4e678ea63cc4025a0739c6c46476794)
    //   payload: {
    //     type: 'pki_enrollment_submit_payload',
    //     verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9,
    //     public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d,
    //     device_label: 'My dev1 machine',
    //   }
    //   encrypted_key: 0x666f6f
    //   encrypted_key_algo: 'RSAES-OAEP-SHA256'
    //   ciphertext: 0x666f6f
    &hex!(
        "0028b52ffd00582d0d00669961452073d30180781b790c580db836dcab18e8ffc451bf"
        "491a6b88119e697c765bdbd214a594ebbfb4f74e5296bde94cd5896083607dd81df3c7"
        "f7a2b865dbaa93e2b47ea1430246004b005c00df123a58e0e954646fa1b7b54c87d57a"
        "8e1bbcc6ecad4bb55949d2f46eb16b12d9f8e3adaa07f8e6acb1a0cfc1efd137fe72ed"
        "8d52fbb561567dd6be904669e98abe5e367d0810953921800124c326036046b57076ce"
        "be5d1c3e41b03a269e92a8e106290d47cdf4f162f9be36ae421afd1ba3acc558be398c"
        "bda5945589497656277d1d42607fce54c2fa61e7281df31c1a170922e90304694717c0"
        "8380171cd2e9c5ddafc78bcb40c8080a19210346d8386988804270ba1073a2460a4a06"
        "74a0140324658503e3b50af430ff1e7d10baca6db132a3a55ebc26446413245012b8c4"
        "c8090ca9a3cb11a3a1313dd6621b98dc75fed6aecbcfc4536281441d948f55a123fa58"
        "220b296c46237cb116dbccced77b591bd5d53519370e83514f8e1772a17d450a5bfaa8"
        "e729f41f1e0552201105225588c28d8f1487090036a70784ef21a8c63e9ae58a712bd6"
        "881f7b5d63b30d5908a001"
    )[..],
    Box::new(|alice: &Device| {
        PKILocalPendingEnrollment {
            cert_ref: X509CertificateReference::from(X509CertificateHash::fake_sha256())
                .add_or_replace_uri(X509WindowsCngURI{issuer: b"foo".into(), serial_number: b"bar".into() }),
            addr: ParsecPkiEnrollmentAddr::from_str(
                "parsec3://parsec.example.com/my_org?a=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T00:00:00Z".parse().unwrap(),
            enrollment_id: PKIEnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                device_label: alice.device_label.clone(),
            },
            encrypted_key: b"foo".as_ref().into(),
            encrypted_key_algo: PKIEncryptionAlgorithm::RsaesOaepSha256,
            ciphertext: b"foo".as_ref().into(),
        }
    })
)]
#[case::without_cert_uri(
    // Generated from Parsec 3.5.3-a.0+dev
    // Content:
    //   type: 'local_pending_enrollment'
    //   server_url: 'https://parsec.example.com/'
    //   organization_id: 'my_org'
    //   x509_certificate_ref: { uris: [ ], hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=', }
    //   submitted_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z
    //   enrollment_id: ext(2, 0xd4e678ea63cc4025a0739c6c46476794)
    //   payload: {
    //     type: 'pki_enrollment_submit_payload',
    //     verify_key: 0x845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9,
    //     public_key: 0xe1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d,
    //     device_label: 'My dev1 machine',
    //     human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ],
    //   }
    //   encrypted_key: 0x666f6f
    //   encrypted_key_algo: 'RSAES-OAEP-SHA256'
    //   ciphertext: 0x666f6f
    &hex!(
        "0028b52ffd00581d0d00f61860442073d30180781b790c580db836dcab18f4dfaf01ae"
        "218c1d24ac0e81bafbdb96ca28a55cff25b97712395182dbbc13c11e86cfcbe0983fa7"
        "26e0966dab4e8ad3fa25b22446004e005a00f03550f0d95460b73e2e3311ffd1fab933"
        "3c6dec56174f5661894f9bf5e625d0f16debf138c03667c9625eed5d375dc7a84c3d9b"
        "b35eea0fa31fa9db371bd6b9c22d974f1e026e42bdf72bb60587d875726d4f88c022a0"
        "4c12b804090806d4b1e588d1d088ff5219d0000b211736998f19b4bee8397f56d270f7"
        "c0ea9c7c4c5a62062b895b9e40e0bfdaa9e5f2fb3c37fcebf3f51e39dcc8657aa9e773"
        "f8268fdde8867810588946128c7846126e80ac34685c206816900f2c3b36001d02a870"
        "c09f25cda27abc2815102149214344c0081b1d0608a9035d8518143554523222032d31"
        "4254367060385f79dcf537b55e1b98bc79be2eed43d4c9c7c412913a29216b428714b2"
        "48145e2a36fba96abd36d1b3e52ea49e72d82870c4e11cfeb45420873ffb2584cf485b"
        "3eda99263ba72f61679c05a60b0036a70dde295643696d454350e17d34cb15e3aecd88"
        "75bbae9cd9062204d0"
    )[..],
    Box::new(|alice: &Device| {
        PKILocalPendingEnrollment {
            cert_ref: X509CertificateReference::from(X509CertificateHash::fake_sha256()),
            addr: ParsecPkiEnrollmentAddr::from_str(
                "parsec3://parsec.example.com/my_org?a=pki_enrollment",
            )
            .unwrap(),
            submitted_on: "2000-01-02T00:00:00Z".parse().unwrap(),
            enrollment_id: PKIEnrollmentID::from_hex("d4e678ea63cc4025a0739c6c46476794").unwrap(),
            payload: PkiEnrollmentSubmitPayload {
                verify_key: alice.verify_key(),
                public_key: alice.public_key(),
                device_label: alice.device_label.clone(),
            },
            encrypted_key: b"foo".as_ref().into(),
            encrypted_key_algo: PKIEncryptionAlgorithm::RsaesOaepSha256,
            ciphertext: b"foo".as_ref().into(),
        }
    })
)]
fn serde_local_pending_enrollment(
    alice: &Device,
    #[case] raw: &[u8],
    #[case] generate_expected: Box<dyn FnOnce(&Device) -> PKILocalPendingEnrollment>,
) {
    let expected = generate_expected(alice);

    println!("***expected: {:?}", expected.dump());
    let data = PKILocalPendingEnrollment::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PKILocalPendingEnrollment::load(&raw).unwrap();

    p_assert_eq!(data, expected);
}
