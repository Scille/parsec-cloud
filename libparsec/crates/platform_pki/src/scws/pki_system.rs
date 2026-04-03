// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    PkiCertificate, PkiScwsConfig, PkiSystemFindCertificateError, PkiSystemInitError,
    PkiSystemListUserCertificateError,
};
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct PlatformPkiSystem {}

impl PlatformPkiSystem {
    pub async fn init(_scws_config: Option<PkiScwsConfig<'_>>) -> Result<Self, PkiSystemInitError> {
        Err(PkiSystemInitError::NotAvailable)
    }

    pub async fn find_certificate(
        &self,
        _cert_ref: &X509CertificateReference,
    ) -> Result<Option<PkiCertificate>, PkiSystemFindCertificateError> {
        unimplemented!("platform not supported")
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<PkiCertificate>, PkiSystemListUserCertificateError> {
        unimplemented!("platform not supported");
    }
}
