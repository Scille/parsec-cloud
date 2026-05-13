// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::parsec_test;
use x509_cert::der::Decode;

use crate::test::utils::{certificates, initialize_pki_system, InstalledCertificates};

/// Check that serial number contained in the certificate reference is similar to the serial number in
/// the certificate DER.
///
/// This test was notably made to check if we correctly handle the serial endian on Windows platform
/// as `CERT_INFO` would provide that value in little-endian over the standard big-endian.
#[parsec_test]
async fn ensure_corresponding_serial_number(certificates: &InstalledCertificates) {
    let pki = initialize_pki_system().await;

    let certs = pki.list_user_certificates().await.unwrap();
    let alice_reference = certificates.alice_cert_ref();

    let got_ref = certs
        .iter()
        .find_map(|c| match c {
            crate::AvailablePkiCertificate::Valid { reference, .. }
                if reference.hash == alice_reference.hash =>
            {
                Some(reference)
            }
            _ => None,
        })
        .unwrap();

    let got_uri = got_ref.uri.to_owned().expect("No pkcs11 uri for reference");

    let alice_der = certificates.alice_der_cert();
    let alice_parsed_cert = x509_cert::der::SliceReader::new(&alice_der)
        .and_then(|mut r| x509_cert::Certificate::decode(&mut r))
        .expect("Cannot parse alice cert");

    assert_eq!(
        got_uri.serial,
        alice_parsed_cert.tbs_certificate.serial_number.as_bytes(),
        "Serial in URI should be the same in the certificate"
    );

    let got_cert = pki
        .open_certificate(got_ref)
        .await
        .expect("Cannot open certificate");
    let crate::testbed::MaybeWithTestbed::WithPlatform(got_cert) = got_cert.platform else {
        panic!("Used fake testbed pki, nulling the whole test");
    };
    #[cfg(not(windows))]
    drop(got_cert);
    #[cfg(windows)]
    {
        let raw_cert_context = crate::platform::schannel_utils::cert_context_to_raw(&got_cert.0);
        // SAFETY: The raw pointer come from the inner valid pointer of `cert_context`
        // that is of type `Cryptography::CERT_CONTEXT`
        let cert_info = unsafe { *(*raw_cert_context).pCertInfo };
        // SAFETY: Issuer is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
        let cert_info_issuer = unsafe {
            std::slice::from_raw_parts(cert_info.Issuer.pbData, cert_info.Issuer.cbData as usize)
        };
        // SAFETY: SerialNumber is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
        let cert_info_serial = unsafe {
            std::slice::from_raw_parts(
                cert_info.SerialNumber.pbData,
                cert_info.SerialNumber.cbData as usize,
            )
        };

        let mut got_uri = got_uri.clone();
        assert_eq!(
            cert_info_issuer, got_uri.der_issuer,
            "issuer is in DER format for both side"
        );
        assert_ne!(
            cert_info_serial, got_uri.serial,
            "Should not match as cert_info_serial is in little-endian"
        );

        let (issuer, serial) = crate::platform::get_issuer_serial_from_pkcs11_uri(&mut got_uri);
        assert_eq!(cert_info_issuer, issuer);
        assert_eq!(cert_info_serial, serial, "get_issuer_serial_from_pkcs11_uri transform the serial to be similar to the one obtain from cert info");
    }
}
