// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) use webpki::EndEntityCert as X509EndCertificate;
use webpki::KeyUsage;

use libparsec_types::prelude::*;

use crate::{X509CertificateDer, X509CertificateRevocationList, X509TrustAnchor};

pub(crate) fn verify_certificate<'der>(
    certificate: &'der X509EndCertificate<'der>,
    intermediate_certs: &'der [X509CertificateDer<'der>],
    trusted_roots: &'der [X509TrustAnchor<'_>],
    certificate_revocation_lists: &[X509CertificateRevocationList],
    now: DateTime,
) -> Result<webpki::VerifiedPath<'der>, webpki::Error> {
    let crls_as_ref: Vec<&_> = certificate_revocation_lists.iter().collect();
    let revocation_options = match webpki::RevocationOptionsBuilder::new(&crls_as_ref) {
        Ok(revocation_options) => Some(revocation_options.build()),
        // No CRL provided, this is fine for us
        Err(webpki::CrlsRequired { .. }) => None,
    };

    let time = rustls_pki_types::UnixTime::since_unix_epoch(
        // `duration_since_unix_epoch()` returns an error if `now` is negative (i.e.
        // smaller than UNIX EPOCH), which is never supposed to happen since EPOCH
        // already occurred and the arrow of time only goes in one direction!
        now.duration_since_unix_epoch()
            .expect("current time always > EPOCH"),
    );

    certificate.verify_for_usage(
        webpki::ALL_VERIFICATION_ALGS,
        trusted_roots,
        intermediate_certs,
        time,
        KeyUsage::client_auth(),
        revocation_options,
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

#[expect(clippy::too_many_arguments)]
pub fn verify_message<'a>(
    message: &[u8],
    signature: &[u8],
    algorithm: PkiSignatureAlgorithm,
    der_certificate: &[u8],
    der_intermediate_certs: impl Iterator<Item = &'a [u8]>,
    trusted_roots: &[X509TrustAnchor<'_>],
    certificate_revocation_lists: &[X509CertificateRevocationList],
    now: DateTime,
) -> Result<(), VerifyMessageError> {
    // 1) Verify the certificate trustchain

    let certificate_der = X509CertificateDer::from(der_certificate);
    let certificate = X509EndCertificate::try_from(&certificate_der)
        .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    verify_certificate(
        &certificate,
        &der_intermediate_certs
            .map(X509CertificateDer::from)
            .collect::<Vec<_>>(),
        trusted_roots,
        certificate_revocation_lists,
        now,
    )
    .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    // 2) Verify the message signature

    let verifier = match algorithm {
        PkiSignatureAlgorithm::RsassaPssSha256 => webpki::ring::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
    };
    certificate
        .verify_signature(verifier, message, signature)
        .map_err(VerifyMessageError::InvalidSignature)?;

    Ok(())
}
