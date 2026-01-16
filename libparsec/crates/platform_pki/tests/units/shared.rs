// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rustls_pki_types::CertificateDer;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    test_fixture::{test_pki, TestPKI},
    verify_certificate,
};

#[rstest]
fn test_verify_cert_ok(test_pki: &TestPKI) {
    let trusted_roots = test_pki
        .root
        .values()
        .map(|cert| {
            webpki::anchor_from_trusted_cert(&rustls_pki_types::CertificateDer::from_slice(
                cert.der_certificate.as_ref(),
            ))
            .unwrap()
            .to_owned()
        })
        .collect::<Vec<_>>();
    let der_certificate = &test_pki.cert["bob"].der_certificate;
    let binding = crate::shared::Certificate::from_der(der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .expect("Should be a valid certificate data");

    verify_certificate(&untrusted_cert, &trusted_roots, &[], DateTime::now()).unwrap();
}

#[rstest]
fn test_verify_unknown_issuer(test_pki: &TestPKI) {
    let der_certificate = &test_pki.cert["bob"].der_certificate;
    let binding = crate::shared::Certificate::from_der(der_certificate);

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
fn test_verify_with_intermediate(test_pki: &TestPKI) {
    let trusted_roots = test_pki
        .root
        .values()
        .map(|cert| {
            webpki::anchor_from_trusted_cert(&rustls_pki_types::CertificateDer::from_slice(
                cert.der_certificate.as_ref(),
            ))
            .unwrap()
            .to_owned()
        })
        .collect::<Vec<_>>();
    let intermediate_certs = test_pki
        .intermediate
        .values()
        .map(|cert| CertificateDer::from_slice(&cert.der_certificate))
        .collect::<Vec<_>>();
    let der_certificate = &test_pki.cert["mallory-sign"].der_certificate;
    let binding = crate::shared::Certificate::from_der(der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .expect("Should be a valid certificate data");

    verify_certificate(
        &untrusted_cert,
        &trusted_roots,
        &intermediate_certs,
        DateTime::now(),
    )
    .unwrap();
}
