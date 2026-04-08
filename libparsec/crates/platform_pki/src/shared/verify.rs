// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) use webpki::EndEntityCert as X509EndCertificate;
use webpki::KeyUsage;

use libparsec_types::prelude::*;

use crate::{X509CertificateDer, X509TrustAnchor};

pub(crate) fn verify_certificate<'der>(
    certificate: &'der X509EndCertificate<'der>,
    intermediate_certs: &'der [X509CertificateDer<'der>],
    trusted_roots: &'der [X509TrustAnchor<'_>],
    now: DateTime,
) -> Result<webpki::VerifiedPath<'der>, webpki::Error> {
    let time = rustls_pki_types::UnixTime::since_unix_epoch(
        // `duration_since_unix_epoch()` returns an error if `now` is negative (i.e.
        // smaller than UNIX EPOCH), which is never supposed to happen since EPOCH
        // already occurred and the arrow of time only goes in one direction!
        now.duration_since_unix_epoch()
            .expect("current time always > EPOCH"),
    );
    static SUPPORTED_ALGS: &[&dyn rustls_pki_types::SignatureVerificationAlgorithm] = &[
        &RsaPssSha256WebpkiAlgorithmSupport,
        &RsaPkcs1Sha256WebpkiAlgorithmSupport,
    ];

    certificate.verify_for_usage(
        SUPPORTED_ALGS,
        trusted_roots,
        intermediate_certs,
        time,
        KeyUsage::client_auth(),
        // TODO: Build the revocation options from a CRLS
        // webpki::RevocationOptionsBuilder require a non empty list, for now we provide None
        // instead
        None,
        // We do not have additional constrain to reject a valid path.
        None,
    )
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyMessageError {
    #[error("X509 certificate cannot be trusted: {0}")]
    X509CertificateUntrusted(webpki::Error),
    #[error("Invalid signature: {0}")]
    InvalidSignature(webpki::Error),
}

pub fn verify_message<'a>(
    message: &[u8],
    signature: &[u8],
    algorithm: PkiSignatureAlgorithm,
    certificate: &[u8],
    intermediate_certs: impl Iterator<Item = &'a [u8]>,
    trusted_roots: &[X509TrustAnchor<'_>],
    now: DateTime,
) -> Result<(), VerifyMessageError> {
    // 1) Verify the certificate trustchain

    let certificate_der = X509CertificateDer::from(certificate);
    let certificate = X509EndCertificate::try_from(&certificate_der)
        .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    verify_certificate(
        &certificate,
        &intermediate_certs
            .map(X509CertificateDer::from)
            .collect::<Vec<_>>(),
        trusted_roots,
        now,
    )
    .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    // 2) Verify the message signature

    let verifier = match algorithm {
        PkiSignatureAlgorithm::RsassaPssSha256 => RsaPssSha256WebpkiAlgorithmSupport,
    };
    certificate
        .verify_signature(&verifier, message, signature)
        .map_err(VerifyMessageError::InvalidSignature)?;

    Ok(())
}

#[derive(Debug)]
struct RsaPssSha256WebpkiAlgorithmSupport;

impl rustls_pki_types::SignatureVerificationAlgorithm for RsaPssSha256WebpkiAlgorithmSupport {
    fn verify_signature(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), rustls_pki_types::InvalidSignature> {
        use rsa::{pkcs1::DecodeRsaPublicKey, signature::Verifier};

        let public_key = rsa::RsaPublicKey::from_pkcs1_der(public_key)
            .map(rsa::pss::VerifyingKey::<rsa::sha2::Sha256>::from)
            .map_err(|_| rustls_pki_types::InvalidSignature)?;

        let signature = rsa::pss::Signature::try_from(signature)
            .map_err(|_| rustls_pki_types::InvalidSignature)?;

        public_key
            .verify(message, &signature)
            .map_err(|_| rustls_pki_types::InvalidSignature)
    }

    fn public_key_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_ENCRYPTION
    }

    fn signature_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_PSS_SHA256
    }
}

#[derive(Debug)]
struct RsaPkcs1Sha256WebpkiAlgorithmSupport;

impl rustls_pki_types::SignatureVerificationAlgorithm for RsaPkcs1Sha256WebpkiAlgorithmSupport {
    fn verify_signature(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), rustls_pki_types::InvalidSignature> {
        use rsa::{pkcs1::DecodeRsaPublicKey, pkcs1v15, signature::Verifier};

        let public_key = rsa::RsaPublicKey::from_pkcs1_der(public_key)
            .map(pkcs1v15::VerifyingKey::<rsa::sha2::Sha256>::new_with_prefix)
            .map_err(|_| rustls_pki_types::InvalidSignature)?;

        let signature = pkcs1v15::Signature::try_from(signature)
            .map_err(|_| rustls_pki_types::InvalidSignature)?;

        public_key
            .verify(message, &signature)
            .map_err(|_| rustls_pki_types::InvalidSignature)
    }

    fn public_key_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_ENCRYPTION
    }

    fn signature_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_PKCS1_SHA256
    }
}
