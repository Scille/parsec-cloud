// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    AvailablePkiCertificate, PkiCertificate, PkiScwsConfig, PkiSystemInitError,
    PkiSystemListUserCertificateError, PkiSystemOpenCertificateError,
};
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct PlatformPkiSystem {}

impl PlatformPkiSystem {
    pub async fn init(
        _config_dir: &std::path::Path,
        _scws_config: Option<PkiScwsConfig>,
    ) -> Result<Self, PkiSystemInitError> {
        Err(PkiSystemInitError::NotAvailable)
    }

    pub async fn open_certificate(
        &self,
        _cert_ref: &X509CertificateReference,
    ) -> Result<PkiCertificate, PkiSystemOpenCertificateError> {
        unimplemented!("platform not supported")
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
        unimplemented!("platform not supported");
    }
}
