// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::parsec_test;

use super::utils::{certificates, initialize_pki_system, InstalledCertificates};
use crate::AvailablePkiCertificate;

// Root certificates subject fields
const BLACK_MESA_CA_SUBJECT: &[u8] =
    b"1\x160\x14\x06\x03U\x04\x03\x0c\rBlack Mesa CA1\x130\x11\x06\x03U\x04\n\x0c\nBlack Mesa";

#[parsec_test]
async fn list_user_certificates(certificates: &InstalledCertificates) {
    let pki = initialize_pki_system().await;

    let certs = pki.list_user_certificates().await.unwrap();
    // Should have at least the test certificates installed
    assert!(
        !certs.is_empty(),
        "Expected at least one user certificate in the store"
    );

    // Should at least found Alice's certificate
    let alice_reference = certificates.alice_cert_ref();
    assert!(certs.iter().find(|c| matches!(c, AvailablePkiCertificate::Valid{ reference, .. } if *reference == alice_reference)).is_some());
}

#[parsec_test]
async fn open_certificate_and_get_validation_path() {
    let pki = initialize_pki_system().await;

    // Use Alice's known cert hash to find her certificate
    let alice_ref = crate::X509CertificateReference::from(
        super::utils::ALICE_SHA256_CERT_HASH
            .parse::<crate::X509CertificateHash>()
            .unwrap(),
    );
    let cert = pki
        .open_certificate(&alice_ref)
        .await
        .unwrap()
        .expect("Alice's certificate should be installed");

    // Should be able to get the validation path
    let path = cert.get_validation_path().await.unwrap();
    assert!(!path.leaf.is_empty());
    assert!(
        path.root
            .subject
            .as_ref()
            .windows(BLACK_MESA_CA_SUBJECT.len())
            .any(|w| w == BLACK_MESA_CA_SUBJECT),
        "Missing expected root certificate with subject: {:?}",
        String::from_utf8_lossy(BLACK_MESA_CA_SUBJECT),
    );
}
