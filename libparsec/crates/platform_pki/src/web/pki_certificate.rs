// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;

use crate::{
    PkiCertificateGetDerError, PkiCertificateGetValidationPathError,
    PkiCertificateRequestPrivateKeyError, PkiCertificateToReferenceError, PkiPrivateKey,
    X509ValidationPathOwned,
};

#[derive(Debug, Clone)]
pub struct PlatformPkiCertificate(scwsapi::Certificate);

impl PlatformPkiCertificate {
    pub async fn get_der(&self) -> Result<CertificateDer<'static>, PkiCertificateGetDerError> {
        self.0
            .get_der()
            .await
            .map_err(|e| PkiCertificateGetDerError::Internal(e.into()))
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<PkiPrivateKey, PkiCertificateRequestPrivateKeyError> {
        unimplemented!("platform not supported");
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, PkiCertificateToReferenceError> {
        unimplemented!("platform not supported");
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        unimplemented!("platform not supported")
    }
}
