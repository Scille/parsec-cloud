// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    encrypt_message,
    errors::{
        GetValidationPathForCertError, ListCertificatesError, LoadAnswerPayloadError,
        ValidatePayloadError, VerifyCertificateError, VerifyMessageError, VerifySignatureError,
    },
    get_der_encoded_certificate, CreateLocalPendingError, EncryptMessageError, EncryptedMessage,
    GetDerEncodedCertificateError, PkiSignatureAlgorithm,
};

use bytes::Bytes;
use rustls_pki_types::{pem::PemObject, CertificateDer, TrustAnchor};
pub use webpki::EndEntityCert as X509EndCertificate;
use webpki::{Error as WebPkiError, KeyUsage};

use libparsec_types::prelude::*;

#[derive(Clone)]
pub struct Certificate<'a> {
    internal: CertificateDer<'a>,
}

impl<'a> Certificate<'a> {
    pub fn try_from_pem(raw: &'a [u8]) -> anyhow::Result<Self> {
        CertificateDer::from_pem_slice(raw)
            .map(Self::new)
            .map_err(|err| anyhow::anyhow!("Invalid PEM content: {err}"))
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

    pub fn to_end_certificate(&self) -> Result<X509EndCertificate<'_>, WebPkiError> {
        X509EndCertificate::try_from(&self.internal)
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

#[derive(Clone)]
pub struct SignedMessage {
    pub algo: PkiSignatureAlgorithm,
    pub signature: Bytes,
    pub message: Bytes,
}

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

pub fn create_local_pending(
    cert_ref: &X509CertificateReference,
    addr: ParsecPkiEnrollmentAddr,
    enrollment_id: PKIEnrollmentID,
    submitted_on: DateTime,
    payload: PkiEnrollmentSubmitPayload,
    private_parts: PrivateParts,
) -> Result<PKILocalPendingEnrollment, CreateLocalPendingError> {
    let key = SecretKey::generate();
    let EncryptedMessage {
        cert_ref,
        algo,
        ciphered: encrypted_key,
    } = encrypt_message(key.as_ref(), cert_ref).map_err(|err| match err {
        EncryptMessageError::CannotOpenStore(err) => CreateLocalPendingError::CannotOpenStore(err),
        EncryptMessageError::NotFound => CreateLocalPendingError::NotFound,
        EncryptMessageError::CannotGetCertificateInfo(err) => {
            CreateLocalPendingError::CannotGetCertificateInfo(err)
        }
        EncryptMessageError::CannotAcquireKeypair(err) => {
            CreateLocalPendingError::CannotAcquireKeypair(err)
        }
        EncryptMessageError::CannotEncrypt(err) => CreateLocalPendingError::CannotEncrypt(err),
    })?;
    let ciphered_private_parts = key.encrypt(&private_parts.dump()).into();

    let local_pending = PKILocalPendingEnrollment {
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
    certificate: &'der X509EndCertificate<'der>,
    trusted_roots: &'der [TrustAnchor<'_>],
    intermediate_certs: &'der [CertificateDer<'der>],
    now: DateTime,
    key_usage: KeyUsage,
) -> Result<webpki::VerifiedPath<'der>, VerifyCertificateError> {
    let time = rustls_pki_types::UnixTime::since_unix_epoch(
        // `duration_since_unix_epoch()` returns an error if `now` is negative (i.e.
        // smaller than UNIX EPOCH), which is never supposed to happen since EPOCH
        // already occurred and the arrow of time only goes in one direction!
        now.duration_since_unix_epoch()
            .expect("current time always > EPOCH"),
    );
    certificate
        .verify_for_usage(
            webpki::ALL_VERIFICATION_ALGS,
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
    let time = rustls_pki_types::UnixTime::since_unix_epoch(
        // `duration_since_unix_epoch()` returns an error if `now` is negative (i.e.
        // smaller than UNIX EPOCH), which is never supposed to happen since EPOCH
        // already occurred and the arrow of time only goes in one direction!
        now.duration_since_unix_epoch()
            .expect("current time always > EPOCH"),
    );

    // 1) Verify the certificate trustchain

    let certificate_der = CertificateDer::from(certificate);
    let certificate = X509EndCertificate::try_from(&certificate_der)
        .map_err(VerifyMessageError::X509CertificateUntrusted)?;

    certificate
        .verify_for_usage(
            webpki::ALL_VERIFICATION_ALGS,
            trusted_roots,
            &intermediate_certs
                .map(CertificateDer::from)
                .collect::<Vec<_>>(),
            time,
            KeyUsage::client_auth(),
            // TODO: Build the revocation options from a CRLS
            // webpki::RevocationOptionsBuilder require a non empty list, for now we provide None
            // instead
            None,
            // We do not have additional constrain to reject a valid path.
            None,
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

pub fn load_submit_payload<'cert>(
    signed_message: &SignedMessage,
    der_certificate: &[u8],
    intermediate_certs: impl Iterator<Item = &'cert [u8]>,
    trusted_roots: &[TrustAnchor<'_>],
    now: DateTime,
) -> Result<PkiEnrollmentSubmitPayload, crate::errors::LoadSubmitPayloadError> {
    let validated_payload = validate_payload(
        der_certificate,
        signed_message,
        trusted_roots,
        intermediate_certs,
        now,
    )?;
    PkiEnrollmentSubmitPayload::load(validated_payload).map_err(Into::into)
}

pub fn validate_payload<'message, 'cert>(
    der_certificate: &[u8],
    signed_message: &'message SignedMessage,
    trusted_roots: &[TrustAnchor<'_>],
    intermediate_certs: impl Iterator<Item = &'cert [u8]>,
    now: DateTime,
) -> Result<&'message [u8], ValidatePayloadError> {
    let intermediate_certs: Vec<_> = intermediate_certs.map(CertificateDer::from).collect();
    let binding = Certificate::from_der(der_certificate);

    let untrusted_cert = binding
        .to_end_certificate()
        .map_err(ValidatePayloadError::InvalidCertificateDer)?;
    let verified_path = verify_certificate(
        &untrusted_cert,
        trusted_roots,
        &intermediate_certs,
        now,
        KeyUsage::client_auth(),
    )
    .map_err(|err| match err {
        VerifyCertificateError::Untrusted(err) => ValidatePayloadError::Untrusted(err),
    })?;
    let trusted_cert = verified_path.end_entity();

    verify_message(signed_message, trusted_cert).map_err(|err| match err {
        VerifySignatureError::InvalidSignature(e) => ValidatePayloadError::InvalidSignature(e),
    })
}

pub fn load_answer_payload<'cert>(
    signed_message: &SignedMessage,
    der_certificate: &[u8],
    intermediate_certs: impl Iterator<Item = &'cert [u8]>,
    trusted_roots: &[TrustAnchor<'_>],
    now: DateTime,
) -> Result<PkiEnrollmentAnswerPayload, LoadAnswerPayloadError> {
    let validated_payload = validate_payload(
        der_certificate,
        signed_message,
        trusted_roots,
        intermediate_certs,
        now,
    )?;
    PkiEnrollmentAnswerPayload::load(validated_payload).map_err(Into::into)
}

pub struct ValidationPathOwned {
    pub root: TrustAnchor<'static>,
    pub intermediate_certs: Vec<Bytes>,
}

pub fn get_validation_path_for_cert(
    cert_ref: &X509CertificateReference,
    now: DateTime,
) -> Result<ValidationPathOwned, GetValidationPathForCertError> {
    let trusted_anchor =
        crate::list_trusted_root_certificate_anchors().map_err(|err| match err {
            ListCertificatesError::CannotOpenStore(err) => {
                GetValidationPathForCertError::CannotOpenStore(err)
            }
        })?;
    let intermediate_certs = crate::list_intermediate_certificates().map_err(|err| match err {
        ListCertificatesError::CannotOpenStore(err) => {
            GetValidationPathForCertError::CannotOpenStore(err)
        }
    })?;
    let base_raw_cert = get_der_encoded_certificate(cert_ref)
        .map(|v| Certificate::from_der_owned(v.der_content.into()))
        .map_err(|err| match err {
            GetDerEncodedCertificateError::CannotOpenStore(err) => {
                GetValidationPathForCertError::CannotOpenStore(err)
            }
            GetDerEncodedCertificateError::NotFound => GetValidationPathForCertError::NotFound,
            GetDerEncodedCertificateError::CannotGetCertificateInfo(err) => {
                GetValidationPathForCertError::CannotGetCertificateInfo(err)
            }
        })?;
    let base_cert = base_raw_cert
        .to_end_certificate()
        .map_err(GetValidationPathForCertError::InvalidCertificateDer)?;

    let path = verify_certificate(
        &base_cert,
        &trusted_anchor,
        &intermediate_certs,
        now,
        KeyUsage::client_auth(),
    )
    .map_err(|err| match err {
        VerifyCertificateError::Untrusted(err) => {
            GetValidationPathForCertError::InvalidCertificateUntrusted(err)
        }
    })?;

    let intermediate_certs = path
        .intermediate_certificates()
        .map(|cert| cert.der().to_vec().into())
        .collect();
    let root = path.anchor().to_owned();

    Ok(ValidationPathOwned {
        root,
        intermediate_certs,
    })
}
