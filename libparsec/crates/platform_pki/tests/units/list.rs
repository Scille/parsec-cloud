// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::utils::{certificates, initialize_pki_system, InstalledCertificates};
use crate::{
    AvailablePkiCertificate, PkiCertificateGetValidationPathError, PkiSystemOpenCertificateError,
};

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

    // Helper function to find a certificate by reference and check its can_sign/can_encrypt flags
    fn assert_certificate(
        certs: &[AvailablePkiCertificate],
        reference: &crate::X509CertificateReference,
        expected_can_sign: bool,
        expected_can_encrypt: bool,
    ) {
        let cert = certs.iter().find(|c| matches!(c, AvailablePkiCertificate::Valid { reference: ref r, .. } if r.hash == reference.hash));
        if let AvailablePkiCertificate::Valid { details, .. } = cert.expect("certificate not found")
        {
            p_assert_eq!(details.can_sign, expected_can_sign);
            p_assert_eq!(details.can_encrypt, expected_can_encrypt);
        }
    }

    // Check that Alice certificate has both can_sign==true and can_encrypt==true
    // (since Alice certificate contains no key usage information)
    assert_certificate(&certs, &certificates.alice_cert_ref(), true, true);

    // Check Mallory certificate with only `digitalSignature` key usage
    assert_certificate(&certs, &certificates.mallory_sign_cert_ref(), true, false);

    // Check Mallory certificate with only `keyEncipherment` key usage
    assert_certificate(
        &certs,
        &certificates.mallory_encrypt_cert_ref(),
        false,
        true,
    );

    // Check Mallory certificate with both `digitalSignature` & `keyEncipherment` key usages
    assert_certificate(&certs, &certificates.mallory_both_cert_ref(), true, true);

    // Check Mallory with wrong `dataEncipherment` key usage
    assert_certificate(
        &certs,
        &certificates.mallory_encrypt_data_encipherment_cert_ref(),
        false,
        true,
    );
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

#[parsec_test]
async fn certificate_and_get_validation_path_ko_revoked() {
    let pki = initialize_pki_system().await;

    let revoked_breen_ref = crate::X509CertificateReference::from(
        super::utils::REVOKED_BREEN_SHA256_CERT_HASH
            .parse::<crate::X509CertificateHash>()
            .unwrap(),
    );
    let cert = pki
        .open_certificate(&revoked_breen_ref)
        .await
        .expect("revoked_breen's certificate should be installed");
    p_assert_matches!(
        cert.get_validation_path()
            .await
            .map(|_| "<not displayable>"),
        Err(PkiCertificateGetValidationPathError::Untrusted)
    );

    // Gordon is not itself revoked, however the intermediate CA that signed it is
    let gordon_ref = crate::X509CertificateReference::from(
        super::utils::GORDON_SHA256_CERT_HASH
            .parse::<crate::X509CertificateHash>()
            .unwrap(),
    );
    let cert = pki
        .open_certificate(&gordon_ref)
        .await
        .expect("gordon's certificate should be installed");
    p_assert_matches!(
        cert.get_validation_path()
            .await
            .map(|_| "<not displayable>"),
        Err(PkiCertificateGetValidationPathError::Untrusted)
    );
}

#[parsec_test]
async fn open_certificate_ko_not_found() {
    let pki = initialize_pki_system().await;

    let dummy_ref =
        crate::X509CertificateReference::from(crate::X509CertificateHash::fake_sha256());
    p_assert_matches!(
        pki.open_certificate(&dummy_ref).await,
        Err(PkiSystemOpenCertificateError::NotFound)
    );
}
