// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod signature_verification;

use crate::{
    encrypt_message,
    errors::{
        InvalidPemContent, ValidatePayloadError, VerifyCertificateError, VerifySignatureError,
    },
    shared::signature_verification::{RsassaPssSha256SignatureVerifier, SUPPORTED_SIG_ALGS},
    EncryptedMessage, SignatureAlgorithm,
};
use libparsec_types::{
    DateTime, EnrollmentID, LocalPendingEnrollment, ParsecPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload, PrivateParts, SecretKey, X509CertificateReference,
};
use rustls_pki_types::{pem::PemObject, CertificateDer, TrustAnchor};
use webpki::{EndEntityCert, Error as WebPkiError, KeyUsage};

pub struct Certificate<'a> {
    internal: CertificateDer<'a>,
}

impl<'a> Certificate<'a> {
    pub fn try_from_pem(raw: &'a [u8]) -> Result<Self, InvalidPemContent> {
        CertificateDer::from_pem_slice(raw)
            .map(Self::new)
            .map_err(Into::into)
    }

    pub fn from_der(raw: &'a [u8]) -> Self {
        Self::new(raw.into())
    }

    pub fn new(cert: CertificateDer<'a>) -> Self {
        Self { internal: cert }
    }

    pub fn into_owned(&self) -> Certificate<'static> {
        Certificate::new(self.internal.clone().into_owned())
    }

    pub fn to_end_certificate(&self) -> Result<EndEntityCert<'_>, WebPkiError> {
        EndEntityCert::try_from(&self.internal)
    }
}

impl Certificate<'static> {
    pub fn from_der_owned(raw: Vec<u8>) -> Self {
        Self::new(raw.into())
    }
}

impl AsRef<[u8]> for Certificate<'_> {
    fn as_ref(&self) -> &[u8] {
        self.internal.as_ref()
    }
}

pub struct SignedMessage {
    pub algo: SignatureAlgorithm,
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
}

pub fn verify_message<'message, 'a>(
    signed_message: &'message SignedMessage,
    certificate: &'a EndEntityCert<'a>,
) -> Result<&'message [u8], VerifySignatureError> {
    let verifier = match signed_message.algo {
        SignatureAlgorithm::RsassaPssSha256 => &RsassaPssSha256SignatureVerifier,
    };
    certificate
        .verify_signature(verifier, &signed_message.message, &signed_message.signature)
        .map(|_| signed_message.message.as_ref())
        .map_err(|e| match e {
            WebPkiError::InvalidSignatureForPublicKey => VerifySignatureError::InvalidSignature,
            e => VerifySignatureError::UnexpectedError(e),
        })
}

pub fn create_local_pending(
    cert_ref: &X509CertificateReference,
    addr: ParsecPkiEnrollmentAddr,
    enrollment_id: EnrollmentID,
    submitted_on: DateTime,
    payload: PkiEnrollmentSubmitPayload,
    private_parts: PrivateParts,
) -> Result<LocalPendingEnrollment, crate::errors::CreateLocalPendingError> {
    let key = SecretKey::generate();
    let EncryptedMessage {
        cert_ref,
        algo,
        ciphered: encrypted_key,
    } = encrypt_message(key.as_ref(), cert_ref)?;
    let ciphered_private_parts = key.encrypt(&private_parts.dump()).into();

    let local_pending = LocalPendingEnrollment {
        cert_ref,
        addr,
        submitted_on,
        enrollment_id,
        payload,
        encrypted_key,
        encrypted_key_algo: algo,
        ciphertext: ciphered_private_parts,
    };
    Ok(local_pending)
}

pub fn verify_certificate<'der>(
    certificate: &'der EndEntityCert<'der>,
    trusted_roots: &'der [TrustAnchor<'_>],
    intermediate_certs: &'der [CertificateDer<'der>],
    now: DateTime,
    key_usage: KeyUsage,
) -> Result<webpki::VerifiedPath<'der>, VerifyCertificateError> {
    let time = rustls_pki_types::UnixTime::since_unix_epoch(
        now.duration_since_unix_epoch()
            .map_err(VerifyCertificateError::DateTimeOutOfRange)?,
    );
    certificate
        .verify_for_usage(
            SUPPORTED_SIG_ALGS,
            trusted_roots,
            intermediate_certs,
            time,
            key_usage,
            // TODO: Build the revocation options from a CRLS
            // webpki::RevocationOptionsBuilder require a non empty list, for now we provide None
            // instead
            None,
            // We do not have additional constrain to reject a valid path.
            None,
        )
        .map_err(VerifyCertificateError::Untrusted)
}

pub fn load_submit_payload(
    der_certificate: &[u8],
    signed_message: &SignedMessage,
    now: DateTime,
) -> Result<PkiEnrollmentSubmitPayload, crate::errors::LoadSubmitPayloadError> {
    let validated_payload = validate_payload(der_certificate, signed_message, now)?;
    PkiEnrollmentSubmitPayload::load(validated_payload).map_err(Into::into)
}

pub fn validate_payload<'message>(
    der_certificate: &[u8],
    signed_message: &'message SignedMessage,
    now: DateTime,
) -> Result<&'message [u8], ValidatePayloadError> {
    let binding = Certificate::from_der(der_certificate);
    let untrusted_cert = binding
        .to_end_certificate()
        .map_err(ValidatePayloadError::InvalidCertificateDer)?;
    let trusted_anchor = crate::list_trusted_root_certificate_anchor()?;
    let verified_path = verify_certificate(
        &untrusted_cert,
        &trusted_anchor,
        // TODO: Consider listing intermediate certificate
        &[],
        now,
        KeyUsage::client_auth(),
    )?;
    let trusted_cert = verified_path.end_entity();

    verify_message(signed_message, trusted_cert).map_err(Into::into)
}
