// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This module provides testbed support for the PKI crate.
// When the `test-with-testbed` feature is enabled, PKI operations check if they
// are running within a testbed environment and use embedded test certificates
// instead of the platform certificate store.

#![allow(clippy::unwrap_used)]

use std::path::Path;
use std::sync::Arc;

use libparsec_testbed::test_get_testbed;
use libparsec_types::prelude::*;
use rsa::RsaPrivateKey;
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::CertificateDer;
use sha2::Digest;

use crate::{
    AvailablePkiCertificate, PkiCertificate, PkiCertificateGetDerError,
    PkiCertificateGetValidationPathError, PkiCertificateRequestPrivateKeyError,
    PkiCertificateToReferenceError, PkiPrivateKey, PkiPrivateKeyDecryptError,
    PkiPrivateKeySignError, PkiSystemListUserCertificateError, PkiSystemOpenCertificateError,
    X509CertificateDer, X509TrustAnchor, X509ValidationPathOwned,
};

// Embedded test certificates (PEM format, converted to DER at init time)
const ALICE_PEM: &[u8] = include_bytes!("../test-pki/Cert/alice.crt");
const BOB_PEM: &[u8] = include_bytes!("../test-pki/Cert/bob.crt");
const MALLORY_SIGN_PEM: &[u8] = include_bytes!("../test-pki/Cert/mallory-sign.crt");
const MALLORY_ENCRYPT_PEM: &[u8] = include_bytes!("../test-pki/Cert/mallory-encrypt.crt");

// Embedded private keys (PKCS#8 PEM format)
const ALICE_KEY_PEM: &[u8] = include_bytes!("../test-pki/Cert/alice.key");
const BOB_KEY_PEM: &[u8] = include_bytes!("../test-pki/Cert/bob.key");
const MALLORY_SIGN_KEY_PEM: &[u8] = include_bytes!("../test-pki/Cert/mallory-sign.key");
const MALLORY_ENCRYPT_KEY_PEM: &[u8] = include_bytes!("../test-pki/Cert/mallory-encrypt.key");

// Trust anchors (root CAs)
const BLACK_MESA_PEM: &[u8] = include_bytes!("../test-pki/Root/black_mesa.crt");
const APERTURE_SCIENCE_PEM: &[u8] = include_bytes!("../test-pki/Root/aperture_science.crt");

// Intermediate CAs
const GLADOS_DEV_TEAM_PEM: &[u8] = include_bytes!("../test-pki/Intermediate/glados_dev_team.crt");

fn load_cert_der(pem: &[u8]) -> X509CertificateDer<'static> {
    X509CertificateDer::from_pem_slice(pem).expect("Failed to read PEM certificate")
}

fn load_private_key(pem: &[u8]) -> RsaPrivateKey {
    use rsa::pkcs8::DecodePrivateKey;

    let pem_str = std::str::from_utf8(pem).expect("PEM key is valid UTF-8");
    RsaPrivateKey::from_pkcs8_pem(pem_str).expect("Failed to load PKCS#8 private key")
}

fn compute_cert_ref(der: &[u8]) -> X509CertificateReference {
    let digest = sha2::Sha256::digest(der);
    let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
    X509CertificateReference::from(hash)
}

#[derive(Debug)]
pub(crate) struct TestbedCertEntry {
    pub cert_der: X509CertificateDer<'static>,
    pub cert_ref: X509CertificateReference,
    pub private_key: RsaPrivateKey,
}

#[derive(Debug)]
struct TestbedCertificates {
    pub user_certs: Vec<Arc<TestbedCertEntry>>,
    pub trust_anchors: Vec<X509TrustAnchor<'static>>,
    pub intermediate_certs: Vec<X509CertificateDer<'static>>,
}

#[derive(Debug)]
pub(crate) enum MaybeWithTestbed<P: std::fmt::Debug, T: std::fmt::Debug> {
    WithPlatform(P),
    WithTestbed(T),
}

pub(crate) fn maybe_init_testbed(config_dir: &Path) -> Option<TestbedPkiSystem> {
    test_get_testbed(config_dir).map(|_env| TestbedPkiSystem::new())
}

/*
 * TestbedPkiSystem
 */

#[derive(Debug)]
pub(crate) struct TestbedPkiSystem {
    certificates: Arc<TestbedCertificates>,
}

impl TestbedPkiSystem {
    fn new() -> Self {
        let user_cert_data: Vec<(&[u8], &[u8])> = vec![
            (ALICE_PEM, ALICE_KEY_PEM),
            (BOB_PEM, BOB_KEY_PEM),
            (MALLORY_SIGN_PEM, MALLORY_SIGN_KEY_PEM),
            (MALLORY_ENCRYPT_PEM, MALLORY_ENCRYPT_KEY_PEM),
        ];

        let user_certs = user_cert_data
            .into_iter()
            .map(|(cert_pem, key_pem)| {
                let cert_der = load_cert_der(cert_pem);
                let cert_ref = compute_cert_ref(cert_der.as_ref());
                let private_key = load_private_key(key_pem);
                Arc::new(TestbedCertEntry {
                    cert_der,
                    cert_ref,
                    private_key,
                })
            })
            .collect();

        let black_mesa_der = load_cert_der(BLACK_MESA_PEM);
        let aperture_science_der = load_cert_der(APERTURE_SCIENCE_PEM);

        let trust_anchors = vec![
            webpki::anchor_from_trusted_cert(&black_mesa_der)
                .unwrap()
                .to_owned(),
            webpki::anchor_from_trusted_cert(&aperture_science_der)
                .unwrap()
                .to_owned(),
        ];

        let intermediate_certs = vec![load_cert_der(GLADOS_DEV_TEAM_PEM)];

        Self {
            certificates: Arc::new(TestbedCertificates {
                user_certs,
                trust_anchors,
                intermediate_certs,
            }),
        }
    }

    pub async fn open_certificate(
        &self,
        cert_ref: &X509CertificateReference,
    ) -> Result<Option<PkiCertificate>, PkiSystemOpenCertificateError> {
        Ok(self
            .certificates
            .user_certs
            .iter()
            .find(|entry| entry.cert_ref.hash == cert_ref.hash)
            .map(|entry| PkiCertificate {
                platform: MaybeWithTestbed::WithTestbed(TestbedPkiCertificate {
                    entry: entry.clone(),
                    certificates: self.certificates.clone(),
                }),
            }))
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
        let certs = self
            .certificates
            .user_certs
            .iter()
            .map(|entry| AvailablePkiCertificate::load_der(&entry.cert_der))
            .collect();
        Ok(certs)
    }
}

/*
 * TestbedPkiCertificates
 */

#[derive(Debug)]
pub(crate) struct TestbedPkiCertificate {
    entry: Arc<TestbedCertEntry>,
    certificates: Arc<TestbedCertificates>,
}

impl TestbedPkiCertificate {
    pub async fn request_private_key(
        &self,
    ) -> Result<PkiPrivateKey, PkiCertificateRequestPrivateKeyError> {
        Ok(PkiPrivateKey {
            platform: MaybeWithTestbed::WithTestbed(TestbedPkiPrivateKey {
                entry: self.entry.clone(),
            }),
        })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, PkiCertificateToReferenceError> {
        Ok(self.entry.cert_ref.clone())
    }

    pub async fn get_der(&self) -> Result<CertificateDer<'static>, PkiCertificateGetDerError> {
        Ok(self.entry.cert_der.clone())
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        let cert = webpki::EndEntityCert::try_from(&self.entry.cert_der).unwrap();

        let intermediate_refs: Vec<X509CertificateDer<'_>> = self
            .certificates
            .intermediate_certs
            .iter()
            .map(|c| X509CertificateDer::from(c.as_ref()))
            .collect();

        let path = crate::verify_certificate(
            &cert,
            &intermediate_refs,
            &self.certificates.trust_anchors,
            DateTime::now(),
        )
        .unwrap();

        let intermediates = path
            .intermediate_certificates()
            .map(|cert| cert.der().to_vec().into())
            .collect();
        let root = path.anchor().to_owned();

        Ok(X509ValidationPathOwned {
            leaf: self.entry.cert_der.clone(),
            intermediates,
            root,
        })
    }
}

/*
 * TestbedPkiPrivateKey
 */

#[derive(Debug)]
pub(crate) struct TestbedPkiPrivateKey {
    entry: Arc<TestbedCertEntry>,
}

impl TestbedPkiPrivateKey {
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), PkiPrivateKeySignError> {
        use rsa::signature::{RandomizedSigner, SignatureEncoding};

        // Use the hash size as the salt size (i.e. 32 bytes for sha256) as it is what expects
        // `webpki` when doing the signature verification
        let signing_key = rsa::pss::SigningKey::<rsa::sha2::Sha256>::new_with_salt_len(
            self.entry.private_key.clone(),
            32,
        );
        let signature = signing_key.sign_with_rng(&mut rsa::rand_core::OsRng, message);
        Ok((
            PkiSignatureAlgorithm::RsassaPssSha256,
            Bytes::from(signature.to_vec()),
        ))
    }

    pub async fn decrypt(
        &self,
        algorithm: PKIEncryptionAlgorithm,
        ciphertext: &[u8],
    ) -> Result<Bytes, PkiPrivateKeyDecryptError> {
        match algorithm {
            PKIEncryptionAlgorithm::RsaesOaepSha256 => {
                use rsa::traits::RandomizedDecryptor;

                let dec_key = rsa::oaep::DecryptingKey::<rsa::sha2::Sha256>::new(
                    self.entry.private_key.clone(),
                );
                let plaintext = dec_key
                    .decrypt_with_rng(&mut rsa::rand_core::OsRng, ciphertext)
                    .map_err(|e| PkiPrivateKeyDecryptError::Decrypt(e.into()))?;
                Ok(Bytes::from(plaintext))
            }
        }
    }
}
