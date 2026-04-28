// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::fmt::Debug;

use crate::{PkiPrivateKeyDecryptError, PkiPrivateKeySignError};
use libparsec_types::prelude::{anyhow, Bytes, PKIEncryptionAlgorithm, PkiSignatureAlgorithm};

#[derive(Debug)]
pub struct PlatformPkiPrivateKey(pub(super) scwsapi::PrivateKey);

impl PlatformPkiPrivateKey {
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), PkiPrivateKeySignError> {
        self.0
            .sign(message)
            .await
            .map_err(|e| match e {
                scwsapi::SignError::CannotGenerateSignatureConfig(_) => {
                    PkiPrivateKeySignError::Internal(e.into())
                }
                scwsapi::SignError::SignError(js_value) => PkiPrivateKeySignError::Sign(
                    anyhow::anyhow!("Cannot sign message ({js_value:?})"),
                ),
            })
            .and_then(|(inner_algo, signature)| {
                let algo = match inner_algo {
                    scwsapi::SignatureAlgorithm::RsassaPssSha256 => {
                        PkiSignatureAlgorithm::RsassaPssSha256
                    }
                    scwsapi::SignatureAlgorithm::Pkcs1Sha256 => {
                        return Err(PkiPrivateKeySignError::UnsupportedAlgorithm)
                    }
                };
                Ok((algo, signature.into()))
            })
    }

    pub async fn decrypt(
        &self,
        algorithm: PKIEncryptionAlgorithm,
        ciphertext: &[u8],
    ) -> Result<Bytes, PkiPrivateKeyDecryptError> {
        let scws_algo = match algorithm {
            PKIEncryptionAlgorithm::RsaesOaepSha256 => scwsapi::EncryptionAlgorithm::RsaOaepSha256,
        };
        self.0
            .decrypt(scws_algo, ciphertext)
            .await
            .map(Bytes::from)
            .map_err(|e| match e {
                scwsapi::DecryptError::CannotGenerateEncryptionConfig(_) => {
                    PkiPrivateKeyDecryptError::Internal(e.into())
                }
                scwsapi::DecryptError::Decrypt(js_value) => PkiPrivateKeyDecryptError::Decrypt(
                    anyhow::anyhow!("Cannot decrypt ciphertext ({js_value:?})"),
                ),
            })
    }
}
