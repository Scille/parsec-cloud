// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{certificates, InstalledCertificates};
use crate::verify_certificate;

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
