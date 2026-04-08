// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum PkiPrivateKeySignError {
    #[error("unsupported signature algorithm")]
    UnsupportedAlgorithm,
    #[error("error during signature: {0}")]
    Sign(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiPrivateKeyDecryptError {
    #[error("unsupported encryption algorithm")]
    UnsupportedAlgorithm,
    #[error("error during decryption: {0}")]
    Decrypt(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug)]
pub struct PkiPrivateKey {
    #[cfg(not(feature = "test-with-testbed"))]
    pub(crate) platform: crate::platform::PlatformPkiPrivateKey,
    #[cfg(feature = "test-with-testbed")]
    pub(crate) platform: crate::testbed::MaybeWithTestbed<
        crate::platform::PlatformPkiPrivateKey,
        crate::testbed::TestbedPkiPrivateKey,
    >,
}

impl PkiPrivateKey {
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), PkiPrivateKeySignError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.sign(message).await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.sign(message).await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.sign(message).await
        }
    }

    pub async fn decrypt(
        &self,
        algorithm: PKIEncryptionAlgorithm,
        ciphertext: &[u8],
    ) -> Result<Bytes, PkiPrivateKeyDecryptError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.decrypt(algorithm, ciphertext).await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.decrypt(algorithm, ciphertext).await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.decrypt(algorithm, ciphertext).await
        }
    }
}
