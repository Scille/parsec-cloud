// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{certificates, InstalledCertificates};
use crate::{
    errors::GetRootCertificateInfoFromTrustchainError, shared::RootCertificateInfo,
    verify_certificate,
};

#[rstest]
fn test_verify_cert_ok(certificates: &InstalledCertificates) {
    let der_certificate = certificates.bob_der_cert();
    let trusted_roots =
        [
            webpki::anchor_from_trusted_cert(&rustls_pki_types::CertificateDer::from_slice(
                &certificates.black_mesa_der_cert(),
            ))
            .unwrap()
            .to_owned(),
        ];
    let binding = crate::shared::Certificate::from_der(&der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .expect("Should be a valid certificate data");

    verify_certificate(&untrusted_cert, &trusted_roots, &[], DateTime::now()).unwrap();
}

#[rstest]
fn test_verify_unknown_issuer(certificates: &InstalledCertificates) {
    let der_certificate = certificates.bob_der_cert();
    let binding = crate::shared::Certificate::from_der(&der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .expect("Should be a valid certificate data");

    let err = verify_certificate(
        &untrusted_cert,
        &[], // No trusted root, so always invalid
        &[],
        DateTime::now(),
    )
    .map(|_| ())
    .expect_err("Should not be trusted");

    assert!(matches!(err, webpki::Error::UnknownIssuer));
}

#[rstest]
fn test_verify_with_intermediate(certificates: &InstalledCertificates) {
    let der_certificate = certificates.mallory_sign_der_cert();
    let trusted_roots =
        [
            webpki::anchor_from_trusted_cert(&rustls_pki_types::CertificateDer::from_slice(
                &certificates.aperture_science_der_cert(),
            ))
            .unwrap()
            .to_owned(),
        ];
    let glados_dev_team_der_cert = certificates.glados_dev_team_der_cert();
    let intermediate_certs = [rustls_pki_types::CertificateDer::from_slice(
        &glados_dev_team_der_cert,
    )];
    let binding = crate::shared::Certificate::from_der(&der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .expect("Should be a valid certificate data");

    verify_certificate(
        &untrusted_cert,
        &trusted_roots,
        intermediate_certs.as_ref(),
        DateTime::now(),
    )
    .unwrap();
}

#[rstest]
fn test_get_root_certificate_info_from_trustchain_ok(
    #[values(
        "no_intermediates",
        "with_intermediates",
        // When validating the trustchain, any unused intermediates is considered an error,
        // however `get_root_certificate_info_from_trustchain` is not performing a full
        // blown validation but only extract some useful info about the root certificate
        // (which is expected be used later on for the actual trustchain validation!).
        "too_many_intermediates",
    )]
    kind: &str,
    certificates: &InstalledCertificates,
) {
    let (leaf, intermediates, expected_common_name, expected_subject) = match kind {
        "no_intermediates" => {
            (
                certificates.alice_der_cert(),
                vec![],
                "Black Mesa CA",
                b"1\x160\x14\x06\x03U\x04\x03\x0c\rBlack Mesa CA1\x130\x11\x06\x03U\x04\n\x0c\nBlack Mesa".as_ref(),
            )
        }

        "with_intermediates" => {
            (
                certificates.mallory_encrypt_der_cert(),
                vec![
                    certificates.glados_dev_team_der_cert(),
                ],
                "Aperture Science CA",
                b"1\x1c0\x1a\x06\x03U\x04\x03\x0c\x13Aperture Science CA1\x190\x17\x06\x03U\x04\n\x0c\x10Aperture Science".as_ref(),
            )
        }

        "too_many_intermediates" => {
            (
                certificates.mallory_encrypt_der_cert(),
                vec![
                    certificates.glados_dev_team_der_cert(),
                    certificates.glados_dev_team_der_cert(),
                ],
                "Aperture Science CA",
                b"1\x1c0\x1a\x06\x03U\x04\x03\x0c\x13Aperture Science CA1\x190\x17\x06\x03U\x04\n\x0c\x10Aperture Science".as_ref(),
            )
        }

        unknown => panic!("Unknown kind: {unknown}"),
    };

    p_assert_matches!(
        crate::get_root_certificate_info_from_trustchain(
            &leaf,
            intermediates.iter().map(|cert| cert.as_ref()),
        ),
        Ok(RootCertificateInfo {
            common_name,
            subject,
        })
        if common_name == expected_common_name && subject == expected_subject
    );
}

#[rstest]
fn test_get_root_certificate_info_from_trustchain_ko_invalid_der(
    #[values("invalid_der_leaf", "invalid_der_intermediate")] kind: &str,
    certificates: &InstalledCertificates,
) {
    let (leaf, intermediates) = match kind {
        "invalid_der_leaf" => (Bytes::from_static(b"<invalid der>"), vec![]),

        "invalid_der_intermediate" => (
            certificates.mallory_encrypt_der_cert(),
            vec![Bytes::from_static(b"<invalid der>")],
        ),

        unknown => panic!("Unknown kind: {unknown}"),
    };

    let outcome = crate::get_root_certificate_info_from_trustchain(
        &leaf,
        intermediates.iter().map(|cert| cert.as_ref()),
    );
    p_assert_matches!(
        outcome,
        Err(GetRootCertificateInfoFromTrustchainError::InvalidCertificateDer(err))
        if format!("{}", err) == "TrailingData(SignedData)"
    );
}

#[rstest]
fn test_get_root_certificate_info_from_trustchain_ko_missing_root_common_name() {
    // Alice certificate generated with `misc/gen_test_pki.py` by tweaking it
    // to remove the common name from Black Mesa's certificate config.
    const ALICE_ISSUER_WITHOUT_COMMON_NAME_DER: &[u8] = &hex!(
        "
        3082032e30820216a00302010202145daa4d2b2920821ab7a83c06e5d35305da3e9e1d
        300d06092a864886f70d01010b0500301531133011060355040a0c0a426c61636b204d
        6573613020170d3236303132373134333630355a180f32313233303731383133333630
        355a302531133011060355040a0c0a426c61636b204d657361310e300c06035504030c
        05416c69636530820122300d06092a864886f70d01010105000382010f003082010a02
        82010100b7113dbf377e1b1fa3548d84da2916dc30610bea093462d2f8ee8796594b71
        a4eb53f51791a1ee0844bf74c6a7af6760975de7903b88775880f07f0a994a9dd602d8
        828826c95c3354a09db9801157c443128da319d35768b1ef19ea4001f5f5aa358a867a
        b19ab964a9e594b2c15811e11b41afc4e6aea04874363b3a5654fd5acef0b1c41075fb
        b91e9a7f840e043d5d42c13e268dd264d3077cea9993093f622ba6b81dfe3e50322d01
        60d5eac920f34b0b4f75590d2777e39d9c213e7c64194e3a270dbc0dfeb3b4aab617b8
        528b90a0ba4be81d96f947089520035a199e79b2b64dd44f6f15d17b65b74030d34be6
        482f0569956509c3541707ffa9c2bf0203010001a364306230200603551d1104193017
        8115616c69636540626c61636b5f6d6573612e636f7270301d0603551d0e0416041488
        6908eb3abf3c50f238f88b01fd59b7f363b4aa301f0603551d23041830168014f38d42
        7718b088972ed09e20c86819e57ff14c2b300d06092a864886f70d01010b0500038201
        010071d2f32b960a48ef730758e91775b17b9fad3a02fa7e6369c6bcf57e0de8dca4a3
        1a89d13c0edb982873a6a9ea29e89d60a0671ddbbcddb969ce60c73aa7f7a55c93aed2
        020b048f8a194193d99bd5e6afbb7986b8098809a161ac24ddcdc73d1ddd464bc440cc
        6eed168c80f4350d5d416d1aa822af4a1ae2b251e0b3a4e2349bdf8af4077629100049
        3525aaae468d8337b348757a1db267214f5d99029c354b5ea851df447cd0eb304f09cc
        f64167bd9fa9a25aa52de0641dff31d6c31434935ecde76fcf717f195cad7baec50748
        4f73c53d530d3960e629606ad422f006f060420878cc3eefe8e0a1bfa3012a2e18cd7f
        ddac89a88b2d03aadd5f378b94
    "
    );
    p_assert_matches!(
        crate::get_root_certificate_info_from_trustchain(
            ALICE_ISSUER_WITHOUT_COMMON_NAME_DER,
            [].into_iter(),
        ),
        Err(GetRootCertificateInfoFromTrustchainError::InvalidCertificateNoCommonName)
    );
}
