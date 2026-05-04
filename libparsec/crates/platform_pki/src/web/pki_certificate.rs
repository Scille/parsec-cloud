// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::VecDeque;

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;

use crate::{
    platform::PlatformPkiPrivateKey, PkiCertificateGetDerError,
    PkiCertificateGetValidationPathError, PkiCertificateRequestPrivateKeyError,
    PkiCertificateToReferenceError, PkiPrivateKey, X509ValidationPathOwned,
};

#[derive(Debug, Clone)]
pub struct PlatformPkiCertificate(pub(super) scwsapi::Certificate);

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
        self.0
            .request_private_key()
            .await
            .map_err(|e| PkiCertificateRequestPrivateKeyError::Internal(e.into()))?
            .ok_or(PkiCertificateRequestPrivateKeyError::NotFound)
            .map(|private_key| PkiPrivateKey {
                platform: PlatformPkiPrivateKey(private_key),
            })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, PkiCertificateToReferenceError> {
        unimplemented!("platform not supported");
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        let trust = self.0.get_trust().await.map_err(|_| {
            PkiCertificateGetValidationPathError::Internal(anyhow::anyhow!("unable to get trust"))
        })?;
        if trust.status().is_err() {
            return Err(PkiCertificateGetValidationPathError::Untrusted);
        }
        let mut certs: VecDeque<_> = trust.cert_path().collect();

        // Current cert
        let leaf = certs
            .pop_front()
            .ok_or(PkiCertificateGetValidationPathError::Internal(
                anyhow::anyhow!("no leaf cert"),
            ))?;
        let leaf = leaf
            .get_der()
            .await
            .map_err(|e| PkiCertificateGetValidationPathError::Internal(e.into()))?;
        // root cert
        let root = certs
            .pop_back()
            .ok_or(PkiCertificateGetValidationPathError::Internal(
                anyhow::anyhow!("no root cert"),
            ))?;
        let root = root
            .get_der()
            .await
            .map_err(|e| PkiCertificateGetValidationPathError::Internal(e.into()))?;
        let root = webpki::anchor_from_trusted_cert(&root)
            .map(|anchor| anchor.to_owned())
            .map_err(|e| PkiCertificateGetValidationPathError::Internal(e.into()))?;

        // Intermediates certs
        let mut intermediates = Vec::with_capacity(certs.len());
        for c in certs {
            intermediates.push(
                c.get_der()
                    .await
                    .map_err(|e| PkiCertificateGetValidationPathError::Internal(e.into()))?,
            )
        }

        Ok(X509ValidationPathOwned {
            leaf,
            root,
            intermediates,
        })
    }
}

impl From<scwsapi::Certificate> for PlatformPkiCertificate {
    fn from(value: scwsapi::Certificate) -> Self {
        Self(value)
    }
}
