// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Tests for the SCWS-based web PKI backend.
//!
//! These tests require the mock SCWS service to be running:
//!
//!     cd idopt-scws/mock_scws && uv run mock_scws.py --pki-dir ../../libparsec/crates/platform_pki/test-pki
//!

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

fn make_config() -> crate::PkiConfig<'static> {
    // PkiConfig fields are not used by the SCWS backend, but we need to provide them.
    crate::PkiConfig {
        config_dir: std::path::Path::new("/tmp"),
        addr: Box::leak(Box::new(
            "parsec3://localhost".parse::<libparsec_types::ParsecAddr>().unwrap(),
        )),
        proxy: Box::leak(Box::new(
            libparsec_platform_http_proxy::ProxyConfig::default(),
        )),
    }
}

async fn init_or_skip() -> crate::PkiSystem {
    match crate::PkiSystem::init(make_config()).await {
        Ok(sys) => sys,
        Err(e) => {
            eprintln!(
                "Skipping SCWS test: mock SCWS not available ({e})\n\
                 Start it with: cd idopt-scws/mock_scws && uv run mock_scws.py --pki-dir ../../libparsec/crates/platform_pki/test-pki"
            );
            // Use panic to skip — parsec_test doesn't support #[ignore] dynamically
            panic!("Mock SCWS not running");
        }
    }
}

#[parsec_test]
async fn init() {
    let pki = init_or_skip().await;
    // If we got here, init succeeded. Drop will abort the keepalive task.
    drop(pki);
}

#[parsec_test]
async fn list_user_certificates() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    // The mock with test-pki should have at least alice's certificates
    assert!(
        !certs.is_empty(),
        "Expected at least one user certificate from SCWS"
    );
    // Verify each cert can export DER
    for cert in &certs {
        let der = cert.get_der().await.unwrap();
        assert!(!der.is_empty());
    }
}

#[parsec_test]
async fn find_certificate() {
    let pki = init_or_skip().await;

    // List all certs, pick the first one, get its reference, then find it again
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    let cert_ref = certs[0].to_reference().await.unwrap();
    let found = pki.find_certificate(&cert_ref).await.unwrap();
    assert!(found.is_some(), "Should find the certificate by its hash");
}

#[parsec_test]
async fn find_certificate_not_found() {
    let pki = init_or_skip().await;
    let dummy_ref: X509CertificateReference = X509CertificateHash::fake_sha256().into();
    let found = pki.find_certificate(&dummy_ref).await.unwrap();
    assert!(found.is_none(), "Should not find a certificate with a fake hash");
}

#[parsec_test]
async fn get_der() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    let der = certs[0].get_der().await.unwrap();
    // Verify it's valid DER by loading it as an X509 certificate
    let info = crate::x509::X509CertificateInformation::load_der(&der).unwrap();
    assert!(info.common_name().is_some());
}

#[parsec_test]
async fn to_reference() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    let cert_ref = certs[0].to_reference().await.unwrap();
    // The hash should not be the fake/zero hash
    let X509CertificateHash::SHA256(hash) = &cert_ref.hash;
    assert_ne!(hash.as_ref(), &[0u8; 32]);
}

#[parsec_test]
async fn request_private_key() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    // All listed certs should have hasPrivateKey=true, so requesting should succeed
    let _pkey = certs[0].request_private_key().await.unwrap();
}

#[parsec_test]
async fn sign_and_verify() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    let payload = b"The cake is a lie!";

    // Sign via SCWS
    let pkey = certs[0].request_private_key().await.unwrap();
    let (algo, signature) = pkey.sign(payload.as_ref()).await.unwrap();
    assert_eq!(algo, PkiSignatureAlgorithm::RsassaPssSha256);
    assert!(!signature.is_empty());

    // Verify using the shared (pure-crypto) verification
    let cert_der = certs[0].get_der().await.unwrap();
    let end_cert = webpki::EndEntityCert::try_from(&cert_der).unwrap();
    let signed = crate::SignedMessage {
        algo,
        signature,
        message: bytes::Bytes::copy_from_slice(payload),
    };
    crate::verify_message(&signed, &end_cert).unwrap();
}

#[parsec_test]
async fn encrypt_decrypt_roundtrip() {
    let pki = init_or_skip().await;
    let certs: Vec<_> = pki.list_user_certificates().await.unwrap().collect();
    assert!(!certs.is_empty());

    let payload = b"The cake is a lie!";

    // Encrypt locally using the certificate's public key
    let cert_der = certs[0].get_der().await.unwrap();
    let (algo, encrypted) = crate::encrypt_message(cert_der, payload.as_ref())
        .await
        .unwrap();
    assert_eq!(algo, PKIEncryptionAlgorithm::RsaesOaepSha256);

    // Decrypt via SCWS
    let pkey = certs[0].request_private_key().await.unwrap();
    let decrypted = pkey.decrypt(algo, &encrypted).await.unwrap();
    assert_eq!(&*decrypted, payload);
}
