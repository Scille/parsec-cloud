// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;

use crate::{PkiPrivateKey, X509CertificateDer, X509TrustAnchor};

#[derive(Debug, thiserror::Error)]
pub enum PkiCertificateGetDerError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiCertificateRequestPrivateKeyError {
    #[error("private key not found")]
    NotFound,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiCertificateToReferenceError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiCertificateGetValidationPathError {
    #[error("certificate not trusted")]
    Untrusted,
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub struct X509ValidationPathOwned {
    pub leaf: X509CertificateDer<'static>,
    pub intermediates: Vec<X509CertificateDer<'static>>,
    pub root: X509TrustAnchor<'static>,
}

// #[derive(Debug, thiserror::Error)]
// pub enum PkiCertificateEncryptError {
//     #[error("Cannot acquire public key: {0}")]
//     CannotAcquirePubkey(#[from] crate::x509::GetCertificatePublicKeyError),
//     #[error("Cannot encrypt message: {0}")]
//     CannotEncrypt(#[from] rsa::errors::Error),
// }

// #[derive(Debug, thiserror::Error)]
// pub enum PkiCertificateVerifyError {
//     #[error("Invalid signature: {0}")]
//     InvalidSignature(webpki::Error),
// }

/// Represents a validated X509 end-entity certificate (i.e. not a CA/intermediate/root)
/// present on the certificate store.
///
/// To manipulate a X509 certificate outside of the certificate store you should instead
/// use `x509_certificate_verify_message` & `x509_certificate_encrypt_message` functions.
#[derive(Debug)]
pub struct PkiCertificate {
    #[cfg(not(feature = "test-with-testbed"))]
    pub(crate) platform: crate::platform::PlatformPkiCertificate,
    #[cfg(feature = "test-with-testbed")]
    pub(crate) platform: crate::testbed::MaybeWithTestbed<
        crate::platform::PlatformPkiCertificate,
        crate::testbed::TestbedPkiCertificate,
    >,
}

impl PkiCertificate {
    pub async fn get_der(&self) -> Result<CertificateDer<'static>, PkiCertificateGetDerError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => testbed.get_der().await,
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.get_der().await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.get_der().await
        }
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<PkiPrivateKey, PkiCertificateRequestPrivateKeyError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.request_private_key().await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.request_private_key().await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.request_private_key().await
        }
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, PkiCertificateToReferenceError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.to_reference().await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.to_reference().await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.to_reference().await
        }
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.get_validation_path().await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.get_validation_path().await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.get_validation_path().await
        }
    }
}
