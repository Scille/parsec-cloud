// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
        // let digest = sha2::Sha256::digest(&self.der);
        // let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
        // Ok(X509CertificateReference::from(hash))
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        unimplemented!("platform not supported")
    }
}
