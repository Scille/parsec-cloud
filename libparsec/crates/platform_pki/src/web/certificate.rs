// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;
use sha2::Digest as _;
use tokio::sync::OnceCell;

use super::scws_client::ScwsClientError;
use super::scws_types::TokenObject;
use super::PrivateKey;
use super::ScwsSession;

pub struct Certificate {
    pub(super) session: Arc<ScwsSession>,
    pub(super) object: TokenObject,
    der_cache: OnceCell<Vec<u8>>,
}

impl Certificate {
    pub(super) fn new(session: Arc<ScwsSession>, object: TokenObject) -> Self {
        Self {
            session,
            object,
            der_cache: OnceCell::new(),
        }
    }

    async fn get_der_bytes(&self) -> Result<&Vec<u8>, ScwsClientError> {
        self.der_cache
            .get_or_try_init(|| async {
                self.session
                    .client
                    .export_object_der(&self.session.env_id, &self.object.handle)
                    .await
            })
            .await
    }

    pub async fn get_der(&self) -> Result<CertificateDer<'static>, crate::GetCertificateDerError> {
        let der_bytes = self
            .get_der_bytes()
            .await
            .map_err(|e| crate::GetCertificateDerError::Internal(e.into()))?;
        Ok(CertificateDer::from(der_bytes.clone()).into_owned())
    }

    pub async fn request_private_key(&self) -> Result<PrivateKey, crate::RequestPrivateKeyError> {
        let ck_id = self
            .object
            .ck_id
            .as_deref()
            .ok_or(crate::RequestPrivateKeyError::NotFound)?;

        let key_handle = self
            .session
            .objects
            .iter()
            .find(|obj| obj.r#type == "privateKey" && obj.ck_id.as_deref() == Some(ck_id))
            .map(|obj| obj.handle.clone())
            .ok_or(crate::RequestPrivateKeyError::NotFound)?;

        Ok(PrivateKey {
            session: Arc::clone(&self.session),
            key_handle,
        })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, crate::GetCertificateReferenceError> {
        let der_bytes = self
            .get_der_bytes()
            .await
            .map_err(|e| crate::GetCertificateReferenceError::Internal(e.into()))?;

        let digest = sha2::Sha256::digest(der_bytes);
        let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
        Ok(X509CertificateReference::from(hash))
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<crate::ValidationPathOwned, crate::ValidationPathError> {
        let trust = self
            .session
            .client
            .get_certificate_trust(&self.session.env_id, &self.object.handle)
            .await
            .map_err(|e| crate::ValidationPathError::Internal(e.into()))?;

        if trust.trust_status != "ok" {
            return Err(crate::ValidationPathError::Untrusted);
        }

        // The leaf is this certificate itself
        let leaf = self
            .get_der()
            .await
            .map_err(|e| crate::ValidationPathError::Internal(e.into()))?;

        // Extract intermediates and root from cert_path
        let mut intermediates = Vec::new();
        let mut root_anchor = None;

        for entry in &trust.cert_path {
            let der_bytes = self
                .session
                .client
                .export_object_der(&self.session.env_id, &entry.handle)
                .await
                .map_err(|e| crate::ValidationPathError::Internal(e.into()))?;

            let cert_der = CertificateDer::from(der_bytes);

            if entry.root_cert.unwrap_or(false) {
                let anchor = webpki::anchor_from_trusted_cert(&cert_der)
                    .map(|a| a.to_owned())
                    .map_err(|e| crate::ValidationPathError::Internal(e.into()))?;
                root_anchor = Some(anchor);
            } else if entry.ca_cert.unwrap_or(false) {
                intermediates.push(cert_der.into_owned());
            }
        }

        let root = root_anchor.ok_or(crate::ValidationPathError::Untrusted)?;

        Ok(crate::ValidationPathOwned {
            leaf,
            intermediates,
            root,
        })
    }
}
