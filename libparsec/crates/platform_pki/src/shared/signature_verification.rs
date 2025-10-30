// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rsa::{pkcs1::DecodeRsaPublicKey, pss, signature::Verifier};
use rustls_pki_types::{InvalidSignature, SignatureVerificationAlgorithm};
use sha2::Sha256;

pub const SUPPORTED_SIG_ALGS: &[&dyn SignatureVerificationAlgorithm] =
    &[&RsassaPssSha256SignatureVerifier];

#[derive(Debug)]
pub(super) struct RsassaPssSha256SignatureVerifier;

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
        let pubkey = rsa::RsaPublicKey::from_pkcs1_der(public_key).map_err(|_| InvalidSignature)?;
        let verifying_key = pss::VerifyingKey::<Sha256>::new(pubkey);

        let signature = pss::Signature::try_from(signature).map_err(|_| InvalidSignature)?;
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
