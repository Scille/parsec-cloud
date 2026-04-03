// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::X509CertificateDer;

use bytes::Bytes;
use rustls_pki_types::pem::PemObject;
pub use webpki::EndEntityCert as X509EndCertificate;

use crate::X509TrustAnchor;
use webpki::{Error as WebPkiError, KeyUsage};

use libparsec_types::prelude::*;

#[derive(Clone)]
pub struct DerCertificate<'a> {
    internal: X509CertificateDer<'a>,
}

impl<'a> DerCertificate<'a> {
    pub fn try_from_pem(raw: &'a [u8]) -> anyhow::Result<Self> {
        X509CertificateDer::from_pem_slice(raw)
            .map(Self::new)
            .map_err(|err| anyhow::anyhow!("Invalid PEM content: {err}"))
    }

    pub fn from_der(raw: &'a [u8]) -> Self {
        Self::new(raw.into())
    }

    pub fn new(cert: X509CertificateDer<'a>) -> Self {
        Self { internal: cert }
    }

    pub fn into_owned(&self) -> DerCertificate<'static> {
        DerCertificate::new(self.internal.clone().into_owned())
    }

    pub fn to_end_certificate(&self) -> Result<X509EndCertificate<'_>, WebPkiError> {
        X509EndCertificate::try_from(&self.internal)
    }
}

impl DerCertificate<'static> {
    pub fn from_der_owned(raw: Vec<u8>) -> Self {
        Self::new(raw.into())
    }
}

impl AsRef<[u8]> for DerCertificate<'_> {
    fn as_ref(&self) -> &[u8] {
        self.internal.as_ref()
    }
}

impl<'a> From<X509CertificateDer<'a>> for DerCertificate<'a> {
    fn from(value: X509CertificateDer<'a>) -> Self {
        Self { internal: value }
    }
}

#[derive(Clone)]
pub struct SignedMessage {
    pub algo: PkiSignatureAlgorithm,
    pub signature: Bytes,
    pub message: Bytes,
}

// Internal API, but `pub` is needed by `examples/verify_certificate.rs`
pub fn verify_certificate<'der>(
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
    certificate.verify_for_usage(
        webpki::ALL_VERIFICATION_ALGS,
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
        PkiSignatureAlgorithm::RsassaPssSha256 => webpki::ring::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
    };
    certificate
        .verify_signature(verifier, message, signature)
        .map_err(VerifyMessageError::InvalidSignature)?;

    Ok(())
}

#[derive(Debug)]
pub struct RootCertificateInfo {
    pub common_name: String,
    pub subject: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetRootCertificateInfoFromTrustchainError {
    #[error("Invalid certificate: invalid DER format: {0}")]
    InvalidCertificateDer(anyhow::Error),
    #[error("Invalid certificate: missing common name")]
    InvalidCertificateNoCommonName,
}

pub fn get_root_certificate_info_from_trustchain<'cert>(
    submitter_der_x509_certificate: &[u8],
    intermediate_der_x509_certificates: impl Iterator<Item = &'cert [u8]>,
) -> Result<RootCertificateInfo, GetRootCertificateInfoFromTrustchainError> {
    // 1. Walk up the chain until we reach the root

    let submitter_der_x509_certificate_der =
        X509CertificateDer::from_slice(submitter_der_x509_certificate);
    let submitter_der_x509_certificate_end =
        X509EndCertificate::try_from(&submitter_der_x509_certificate_der).map_err(|err| {
            GetRootCertificateInfoFromTrustchainError::InvalidCertificateDer(err.into())
        })?;

    let intermediate_der_x509_certificates_der = intermediate_der_x509_certificates
        .map(X509CertificateDer::from_slice)
        .collect::<Vec<_>>();
    let mut intermediate_der_x509_certificates_end = intermediate_der_x509_certificates_der
        .iter()
        .map(X509EndCertificate::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            GetRootCertificateInfoFromTrustchainError::InvalidCertificateDer(err.into())
        })?;

    let mut current = submitter_der_x509_certificate_end;
    while let Some(i) = intermediate_der_x509_certificates_end
        .iter()
        .position(|cert| cert.subject() == current.issuer())
    {
        current = intermediate_der_x509_certificates_end.swap_remove(i);
    }

    // 2. Extract root subject & common name from the certificate issuer

    let subject = current.issuer().to_vec();

    let common_name = match crate::x509::extract_common_name_from_subject(&subject) {
        Ok(Some(common_name)) => common_name,
        Ok(None) => {
            return Err(GetRootCertificateInfoFromTrustchainError::InvalidCertificateNoCommonName)
        }
        Err(err) => {
            return Err(
                GetRootCertificateInfoFromTrustchainError::InvalidCertificateDer(err.into()),
            )
        }
    };

    Ok(RootCertificateInfo {
        subject,
        common_name,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptMessageError {
    #[error("Cannot acquire public key: {0}")]
    CannotAcquirePubkey(#[from] crate::x509::GetCertificatePublicKeyError),
    #[error("Cannot encrypt message: {0}")]
    CannotEncrypt(#[from] rsa::errors::Error),
}

pub async fn encrypt_message(
    certificate: X509CertificateDer<'_>,
    message: &[u8],
) -> Result<(PKIEncryptionAlgorithm, Bytes), EncryptMessageError> {
    let pubkey = crate::x509::get_certificate_public_key(certificate.as_ref())?;
    match pubkey {
        crate::x509::PublicKey::Rsa(rsa_public_key) => {
            use rsa::traits::RandomizedEncryptor;

            let enc_key = rsa::oaep::EncryptingKey::<rsa::sha2::Sha256>::new(rsa_public_key);
            enc_key
                .encrypt_with_rng(&mut rsa::rand_core::OsRng, message)
                .map_err(Into::into)
                .map(|v| (PKIEncryptionAlgorithm::RsaesOaepSha256, Bytes::from(v)))
        }
    }
}
