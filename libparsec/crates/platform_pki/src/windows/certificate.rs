// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

pub struct PlatformPkiCertificate(schannel::cert_context::CertContext);

impl std::fmt::Debug for PlatformPkiCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlatformPkiCertificate")
            .finish_non_exhaustive()
    }
}

impl From<schannel::cert_context::CertContext> for PlatformPkiCertificate {
    fn from(value: schannel::cert_context::CertContext) -> Self {
        Self(value)
    }
}

impl PlatformPkiCertificate {
    pub async fn get_der(
        &self,
    ) -> Result<crate::X509CertificateDer<'static>, crate::PkiCertificateGetDerError> {
        Ok(crate::X509CertificateDer::from_slice(self.0.to_der()).into_owned())
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<crate::PkiPrivateKey, crate::PkiCertificateRequestPrivateKeyError> {
        self.0
            .private_key()
            .compare_key(true)
            .acquire()
            .map(Into::into)
            .map(super::wrap_platform_private_key)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    crate::PkiCertificateRequestPrivateKeyError::NotFound
                }
                _ => crate::PkiCertificateRequestPrivateKeyError::Internal(e.into()),
            })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, crate::PkiCertificateToReferenceError> {
        let uri = super::get_certificate_uri(&self.0);
        let hash = self
            .0
            .fingerprint(schannel::cert_context::HashAlgorithm::sha256())
            .and_then(|buf| {
                buf.try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Not a sha256 hash")
                })
            })
            .map(X509CertificateHash::SHA256)
            .context("Cannot get cert fingerprint")
            .map_err(crate::PkiCertificateToReferenceError::Internal)?;

        Ok(X509CertificateReference::from(hash).add_or_replace_uri(uri))
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<crate::X509ValidationPathOwned, crate::PkiCertificateGetValidationPathError> {
        let raw_trusted_roots = super::list_trusted_root_certificate_anchors()
            .await
            .context("Cannot list trusted roots")
            .map_err(crate::PkiCertificateGetValidationPathError::Internal)?;
        let raw_intermediates = super::list_intermediate_certificates()
            .await
            .context("Cannot list intermediates certificates")
            .map_err(crate::PkiCertificateGetValidationPathError::Internal)?;
        let leaf = self
            .get_der()
            .await
            .context("Cannot get certificate content")
            .map_err(crate::PkiCertificateGetValidationPathError::Internal)?;
        let end_cert = crate::X509EndCertificate::try_from(&leaf)
            .context("Invalid leaf certificate")
            .map_err(crate::PkiCertificateGetValidationPathError::Internal)?;
        let now = DateTime::now();
        let path =
            crate::verify_certificate(&end_cert, &raw_intermediates, &raw_trusted_roots, now)
                .inspect_err(|e| log::warn!("Failed to verify certificate: {e}"))
                .map_err(|_| crate::PkiCertificateGetValidationPathError::Untrusted)?;

        let intermediates = path
            .intermediate_certificates()
            .map(|cert| cert.der().into_owned())
            .collect();
        let root = path.anchor().to_owned();

        Ok(crate::X509ValidationPathOwned {
            root,
            intermediates,
            leaf,
        })
    }
}
