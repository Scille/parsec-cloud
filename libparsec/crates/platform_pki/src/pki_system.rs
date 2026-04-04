// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::PkiCertificate;

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemInitError {
    #[error("Not availble")]
    NotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemFindCertificateError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PkiSystemListUserCertificateError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

/// On web, OS certificates store is access using SCWS (a native client running
/// on the machine and exposing an HTTP API on localhost).
/// SCWS requires mutual authentication using which requires communicating with
/// the Parsec server (so that the private key authenticating our application cannot
/// be ripped from the web page code).
pub struct PkiScwsConfig<'a> {
    pub parsec_addr: &'a libparsec_types::ParsecAddr,
    pub proxy: &'a libparsec_platform_http_proxy::ProxyConfig,
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
        scws_config: Option<PkiScwsConfig<'_>>,
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

    pub async fn find_certificate(
        &self,
        cert_ref: &X509CertificateReference,
    ) -> Result<Option<PkiCertificate>, PkiSystemFindCertificateError> {
        #[cfg(feature = "test-with-testbed")]
        {
            match &self.platform {
                crate::testbed::MaybeWithTestbed::WithTestbed(testbed) => {
                    testbed.find_certificate(cert_ref).await
                }
                crate::testbed::MaybeWithTestbed::WithPlatform(platform) => {
                    platform.find_certificate(cert_ref).await
                }
            }
        }
        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.platform.find_certificate(cert_ref).await
        }
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<PkiCertificate>, PkiSystemListUserCertificateError> {
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
