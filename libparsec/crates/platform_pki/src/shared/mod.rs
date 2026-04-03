// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    errors::{
        GetRootCertificateInfoFromTrustchainError, GetValidationPathForCertError,
        ListCertificatesError, ListUserCertificatesError, VerifyMessageError, VerifySignatureError,
    },
    get_der_encoded_certificate,
    platform::list_user_certificates_der,
    x509::{DistinguishedNameValue, X509CertificateInformation},
    GetDerEncodedCertificateError, PkiSignatureAlgorithm, X509CertificateDer,
};

use bytes::Bytes;
use rustls_pki_types::{pem::PemObject, TrustAnchor};
use sha2::Digest;
pub use webpki::EndEntityCert as X509EndCertificate;
use webpki::{Error as WebPkiError, KeyUsage};

use libparsec_types::prelude::*;
use std::str::FromStr;
use x509_cert::time::Time;

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

// Internal API, but `pub` is needed by `examples/verify_message.rs`
pub fn verify_message<'message, 'a>(
    signed_message: &'message SignedMessage,
    certificate: &'a X509EndCertificate<'a>,
) -> Result<&'message [u8], VerifySignatureError> {
    let verifier = match signed_message.algo {
        PkiSignatureAlgorithm::RsassaPssSha256 => webpki::ring::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
    };
    certificate
        .verify_signature(verifier, &signed_message.message, &signed_message.signature)
        .map(|_| signed_message.message.as_ref())
        .map_err(VerifySignatureError::InvalidSignature)
}

// Internal API, but `pub` is needed by `examples/verify_certificate.rs`
pub fn verify_certificate<'der>(
    certificate: &'der X509EndCertificate<'der>,
    trusted_roots: &'der [TrustAnchor<'_>],
    intermediate_certs: &'der [X509CertificateDer<'der>],
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

// TODO: rename to `verify_message` once pki-enrollment specific code is removed
// see https://github.com/Scille/parsec-cloud/issues/12054
pub fn verify_message2<'a>(
    message: &[u8],
    signature: &[u8],
    algorithm: PkiSignatureAlgorithm,
    certificate: &[u8],
    intermediate_certs: impl Iterator<Item = &'a [u8]>,
    trusted_roots: &[TrustAnchor<'_>],
    now: DateTime,
) -> Result<(), VerifyMessageError> {
    // 1) Verify the certificate trustchain

    let certificate_der = X509CertificateDer::from(certificate);
    let certificate = X509EndCertificate::try_from(&certificate_der)
        .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    verify_certificate(
        &certificate,
        trusted_roots,
        &intermediate_certs
            .map(X509CertificateDer::from)
            .collect::<Vec<_>>(),
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

pub struct X509ValidationPathOwned {
    pub leaf: X509CertificateDer<'static>,
    pub intermediates: Vec<X509CertificateDer<'static>>,
    pub root: TrustAnchor<'static>,
}

pub async fn get_validation_path_for_cert(
    cert_ref: &X509CertificateReference,
    now: DateTime,
) -> Result<X509ValidationPathOwned, GetValidationPathForCertError> {
    let all_trusted_roots = crate::list_trusted_root_certificate_anchors()
        .await
        .map_err(|err| match err {
            ListCertificatesError::CannotOpenStore(err) => {
                GetValidationPathForCertError::CannotOpenStore(err)
            }
        })?;
    let all_intermediates =
        crate::list_intermediate_certificates()
            .await
            .map_err(|err| match err {
                ListCertificatesError::CannotOpenStore(err) => {
                    GetValidationPathForCertError::CannotOpenStore(err)
                }
            })?;
    let leaf = get_der_encoded_certificate(cert_ref)
        .await
        .map_err(|err| match err {
            GetDerEncodedCertificateError::CannotOpenStore(err) => {
                GetValidationPathForCertError::CannotOpenStore(err)
            }
            GetDerEncodedCertificateError::NotFound => GetValidationPathForCertError::NotFound,
        })?;

    let leaf_cert_der = X509CertificateDer::from_slice(&leaf);
    let leaf_end_cert = X509EndCertificate::try_from(&leaf_cert_der)
        .map_err(GetValidationPathForCertError::InvalidCertificateDer)?;
    let path = verify_certificate(&leaf_end_cert, &all_trusted_roots, &all_intermediates, now)
        .map_err(GetValidationPathForCertError::InvalidCertificateUntrusted)?;

    let intermediates = path
        .intermediate_certificates()
        .map(|cert| cert.der().to_vec().into())
        .collect();
    let root = path.anchor().to_owned();

    Ok(X509ValidationPathOwned {
        root,
        intermediates,
        leaf,
    })
}

#[derive(Debug)]
pub struct RootCertificateInfo {
    pub common_name: String,
    pub subject: Vec<u8>,
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

pub enum InvalidCertificateReason {
    UnableToParseTime,
    UnableToParseCert,
    InvalidEmail,
}

pub enum CertificateWithDetails {
    Valid {
        handle: X509CertificateReference,
        friendly_name: Option<String>, // May be different that CertificateWithDetails.name
        details: CertificateDetails,
    },
    Invalid {
        handle: X509CertificateReference,
        friendly_name: Option<String>,
        invalid_reason: InvalidCertificateReason,
    },
}

pub struct CertificateDetails {
    pub name: Option<String>, // Common name of the certificate
    pub subject: Vec<DistinguishedNameValue>,
    pub issuer: Vec<DistinguishedNameValue>,
    pub not_before: DateTime,
    pub not_after: DateTime,
    pub serial: Bytes,
    pub emails: Vec<EmailAddress>,
    pub can_sign: bool,
    pub can_encrypt: bool,
}

pub fn get_cert_details(cert: &[u8]) -> Result<CertificateDetails, InvalidCertificateReason> {
    let info: X509CertificateInformation = X509CertificateInformation::load_der(cert)
        .map_err(|_| InvalidCertificateReason::UnableToParseCert)?;
    let emails = info
        .emails()
        .map(EmailAddress::from_str)
        .collect::<Result<_, _>>()
        .map_err(|_| InvalidCertificateReason::InvalidEmail)?;

    let name = info.common_name().map(|v| v.to_string());
    let subject = info.subject;
    let issuer = info.issuer;

    let can_encrypt = info.extensions.can_encrypt();
    let can_sign = info.extensions.can_sign();
    let serial = info.serial;

    let not_before = time_to_datetime(info.validity.not_before)?;
    let not_after = time_to_datetime(info.validity.not_after)?;

    Ok(CertificateDetails {
        name,
        subject,
        issuer,
        not_before,
        not_after,
        serial,
        emails,
        can_sign,
        can_encrypt,
    })
}

fn time_to_datetime(time: Time) -> Result<DateTime, InvalidCertificateReason> {
    let time = time.to_date_time();
    DateTime::from_ymd_hms_us(
        time.year().into(),
        time.month().into(),
        time.day().into(),
        time.hour().into(),
        time.minutes().into(),
        time.seconds().into(),
        0,
    )
    .map_err(|_| InvalidCertificateReason::UnableToParseTime)
}

pub async fn list_user_certificates_with_details(
) -> Result<Vec<CertificateWithDetails>, ListUserCertificatesError> {
    Ok(list_user_certificates_der()
        .await?
        .into_iter()
        .map(|cert| {
            let digest = sha2::Sha256::digest(&cert);
            let hash = X509CertificateHash::SHA256(Box::new(digest.into()));

            let cert_handle: X509CertificateReference = hash.into();
            let friendly_name = None; // TODO
            match get_cert_details(cert.as_ref()) {
                Ok(details) => CertificateWithDetails::Valid {
                    handle: cert_handle,
                    friendly_name,
                    details,
                },
                Err(invalid_reason) => CertificateWithDetails::Invalid {
                    handle: cert_handle,
                    friendly_name,
                    invalid_reason,
                },
            }
        })
        .collect())
}
