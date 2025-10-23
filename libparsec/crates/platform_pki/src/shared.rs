// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    encrypt_message,
    errors::{InvalidPemContent, VerifySignatureError},
    EncryptedMessage, SignatureAlgorithm,
};
use libparsec_types::{
    DateTime, EnrollmentID, LocalPendingEnrollment, ParsecPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload, PrivateParts, SecretKey, X509CertificateReference,
};
use rsa::{
    pkcs1,
    pss::{Signature, VerifyingKey},
    signature::Verifier,
    RsaPublicKey,
};
use rustls_pki_types::{
    pem::PemObject, CertificateDer, InvalidSignature, SignatureVerificationAlgorithm, TrustAnchor,
};
use sha2::Sha256;
use webpki::{anchor_from_trusted_cert, EndEntityCert, Error as WebPkiError};

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

impl<'a> TryFrom<&'a Certificate<'a>> for TrustAnchor<'a> {
    type Error = WebPkiError;

    fn try_from(value: &'a Certificate<'a>) -> Result<Self, Self::Error> {
        anchor_from_trusted_cert(&value.internal)
    }
}

pub struct SignedMessage {
    pub algo: SignatureAlgorithm,
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
}

pub fn verify_message<'message>(
    signed_message: &'message SignedMessage,
    certificate: Certificate<'_>,
) -> Result<&'message [u8], VerifySignatureError> {
    let verifier = match signed_message.algo {
        SignatureAlgorithm::RsassaPssSha256 => &RsassaPssSha256SignatureVerifier,
    };
    EndEntityCert::try_from(&certificate.internal)
        .map_err(VerifySignatureError::InvalidCertificateDer)?
        .verify_signature(verifier, &signed_message.message, &signed_message.signature)
        .map(|_| signed_message.message.as_ref())
        .map_err(|e| match e {
            WebPkiError::InvalidSignatureForPublicKey => VerifySignatureError::InvalidSignature,
            e => VerifySignatureError::UnexpectedError(e),
        })
}

#[derive(Debug)]
struct RsassaPssSha256SignatureVerifier;

impl SignatureVerificationAlgorithm for RsassaPssSha256SignatureVerifier {
    fn verify_signature(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), InvalidSignature> {
        // Webpki already checked that the key part correspond to an RSA public key.
        //
        // We are not using `pkcs8::DecodePublicKey::from_public_key_der` as
        // `public_key` is already the unwrapped key from the `subjectPublicKeyInfo` structure
        // which it's the expected data from the above method.
        //
        // Instead, we use `pkcs1::RsaPublicKey::try_from(&[u8])` which only expect the key element
        // (without the algorithm identifier).
        let pubkey = pkcs1::RsaPublicKey::try_from(public_key)
            .map_err(|_| InvalidSignature)
            // But `rsa` does not provide a conversion between the `pkcs1` and its `RsaPublicKey`, so
            // we need to perform the manual conversion
            .and_then(|pubkey| {
                let n = rsa::BigUint::from_bytes_be(pubkey.modulus.as_bytes());
                let e = rsa::BigUint::from_bytes_be(pubkey.public_exponent.as_bytes());
                RsaPublicKey::new(n, e).map_err(|_| InvalidSignature)
            })?;
        let verifying_key = VerifyingKey::<Sha256>::new(pubkey);

        let signature = Signature::try_from(signature).map_err(|_| InvalidSignature)?;
        verifying_key
            .verify(message, &signature)
            .map_err(|_| InvalidSignature)
    }

    fn public_key_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_ENCRYPTION
    }

    fn signature_alg_id(&self) -> rustls_pki_types::AlgorithmIdentifier {
        rustls_pki_types::alg_id::RSA_PSS_SHA256
    }
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
