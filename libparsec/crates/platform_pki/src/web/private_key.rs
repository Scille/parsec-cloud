// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;
use sha2::Digest as _;

use super::scws_client::{hex_encode, ScwsClientError};
use super::ScwsSession;

pub struct PrivateKey {
    pub(super) session: Arc<ScwsSession>,
    pub(super) key_handle: String,
}

impl PrivateKey {
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), crate::SignError> {
        let hash = sha2::Sha256::digest(message);
        let hash_hex = hex_encode(&hash);

        let sig_bytes = self
            .session
            .client
            .sign(
                &self.session.env_id,
                &self.key_handle,
                &hash_hex,
                "sha256",
                32,
            )
            .await
            .map_err(|e| crate::SignError::Sign(e.into()))?;

        Ok((
            PkiSignatureAlgorithm::RsassaPssSha256,
            Bytes::from(sig_bytes),
        ))
    }

    pub async fn decrypt(
        &self,
        algorithm: PKIEncryptionAlgorithm,
        ciphertext: &[u8],
    ) -> Result<Bytes, crate::DecryptError> {
        if algorithm != PKIEncryptionAlgorithm::RsaesOaepSha256 {
            return Err(crate::DecryptError::UnsupportedAlgorithm);
        }

        let ciphertext_hex = hex_encode(ciphertext);

        let plaintext = self
            .session
            .client
            .decrypt(&self.session.env_id, &self.key_handle, &ciphertext_hex)
            .await
            .map_err(|e| crate::DecryptError::Decrypt(e.into()))?;

        Ok(Bytes::from(plaintext))
    }
}

impl From<ScwsClientError> for crate::SignError {
    fn from(e: ScwsClientError) -> Self {
        crate::SignError::Internal(e.into())
    }
}

impl From<ScwsClientError> for crate::DecryptError {
    fn from(e: ScwsClientError) -> Self {
        crate::DecryptError::Internal(e.into())
    }
}
