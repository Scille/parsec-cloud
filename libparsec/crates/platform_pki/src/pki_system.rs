// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::{PkiCertificate, UserX509CertificateDetails, UserX509CertificateLoadError};

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemInitError {
    #[error("Not available")]
    NotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemOpenCertificateError {
    #[error("PKI Certificate not found")]
    NotFound,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemListUserCertificateError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// On web, OS certificates store is accessed using SCWS (a native client running
/// on the machine and exposing an HTTP API on localhost).
/// SCWS requires mutual authentication using which requires communicating with
/// the Parsec server (so that the private key authenticating our application cannot
/// be ripped from the web page code).
pub struct PkiScwsConfig {
    pub parsec_addr: libparsec_types::ParsecAddr,
    pub proxy: libparsec_platform_http_proxy::ProxyConfig,
}

pub enum AvailablePkiCertificate {
    Valid {
        reference: X509CertificateReference,
        /// The most human-readable name for the certificate.
        ///
        /// Typically defaults to `X509CertificateDetails.common_name`, but
        /// some OS store certificate allow to give a custom name (since
        /// COMMON_NAME is often not that friendly...).
        ///
        /// See: https://learn.microsoft.com/en-us/uwp/api/windows.security.cryptography.certificates.certificate.friendlyname
        friendly_name: String, // May be different that `X509CertificateDetails.common_name` TODO: give an example
        details: UserX509CertificateDetails,
    },
    Invalid {
        reference: X509CertificateReference,
        invalid_reason: UserX509CertificateLoadError,
    },
}

impl AvailablePkiCertificate {
    pub fn load_der(der: &[u8]) -> Self {
        use sha2::Digest as _;

        let digest = sha2::Sha256::digest(der);
        let hash = X509CertificateHash::SHA256(Box::new(digest.into()));

        let reference: X509CertificateReference = hash.into();
        match UserX509CertificateDetails::load_der(der) {
            Ok(details) => Self::Valid {
                reference,
                friendly_name: details.common_name.clone(),
                details,
            },
            Err(invalid_reason) => Self::Invalid {
                reference,
                invalid_reason,
            },
        }
    }
}

#[derive(Debug)]
pub struct PkiSystem {
    #[cfg(not(feature = "test-with-testbed"))]
    pub(crate) platform: crate::platform::PlatformPkiSystem,
    #[cfg(feature = "test-with-testbed")]
    pub(crate) platform: crate::testbed::MaybeWithTestbed<
        crate::platform::PlatformPkiSystem,
        crate::testbed::TestbedPkiSystem,
    >,
}

impl PkiSystem {
    pub async fn init(
        #[cfg_attr(not(feature = "test-with-testbed"), expect(unused))]
        config_dir: &std::path::Path,
        scws_config: Option<PkiScwsConfig>,
    ) -> Result<Self, PkiSystemInitError> {
        #[cfg(feature = "test-with-testbed")]
        let platform = {
            if let Some(testbed) = crate::testbed::maybe_init_testbed(config_dir) {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed)
            } else {
                crate::testbed::MaybeWithTestbed::WithPlatform(
                    crate::platform::PlatformPkiSystem::init(scws_config).await?,
                )
            }
        };

        #[cfg(not(feature = "test-with-testbed"))]
        let platform = { crate::platform::PlatformPkiSystem::init(scws_config).await? };

        Ok(Self { platform })
    }

    pub async fn open_certificate(
        &self,
        cert_ref: &X509CertificateReference,
    ) -> Result<PkiCertificate, PkiSystemOpenCertificateError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.open_certificate(cert_ref).await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.open_certificate(cert_ref).await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.open_certificate(cert_ref).await
        }
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.list_user_certificates().await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.list_user_certificates().await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.list_user_certificates().await
        }
    }
}
