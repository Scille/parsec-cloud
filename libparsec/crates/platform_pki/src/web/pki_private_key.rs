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
        _algorithm: PKIEncryptionAlgorithm,
        _ciphertext: &[u8],
    ) -> Result<Bytes, PkiPrivateKeyDecryptError> {
        unimplemented!("platform not supported")
    }
}
