// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;

use crate::{
    PkiCertificateGetDerError, PkiCertificateGetValidationPathError,
    PkiCertificateRequestPrivateKeyError, PkiCertificateToReferenceError, PkiPrivateKey,
    X509ValidationPathOwned,
};

#[derive(Debug)]
pub struct PlatformPkiCertificate {}

impl PlatformPkiCertificate {
    pub async fn get_der(&self) -> Result<CertificateDer<'static>, PkiCertificateGetDerError> {
        unimplemented!("platform not supported");
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
