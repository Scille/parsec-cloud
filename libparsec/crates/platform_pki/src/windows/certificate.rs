// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::{
    anyhow::Context as _, DateTime, X509CertificateHash, X509CertificateReference,
    X509WindowsCngURI,
};

pub struct X509Certificate(schannel::cert_context::CertContext);

impl From<schannel::cert_context::CertContext> for X509Certificate {
    fn from(value: schannel::cert_context::CertContext) -> Self {
        Self(value)
    }
}

impl X509Certificate {
    pub async fn get_der(
        &self,
    ) -> Result<crate::X509CertificateDer<'static>, crate::GetCertificateDerError> {
        Ok(crate::X509CertificateDer::from_slice(self.0.to_der()).into_owned())
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<super::X509PrivateKey, crate::RequestPrivateKeyError> {
        self.0
            .private_key()
            .compare_key(true)
            .acquire()
            .map(Into::into)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => crate::RequestPrivateKeyError::NotFound,
                _ => crate::RequestPrivateKeyError::Internal(e.into()),
            })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, crate::GetCertificateReferenceError> {
        let uri = self.to_uri();
        let hash = self
            .sha256_fingerprint()
            .context("Cannot get cert fingerprint")
            .map_err(crate::GetCertificateReferenceError::Internal)?;

        Ok(X509CertificateReference::from(hash).add_or_replace_uri(uri))
    }

    fn sha256_fingerprint(&self) -> std::io::Result<X509CertificateHash> {
        self.0
            .fingerprint(schannel::cert_context::HashAlgorithm::sha256())
            .and_then(|buf| {
                buf.try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Not a sha256 hash")
                })
            })
            .map(X509CertificateHash::SHA256)
    }

    fn to_uri(&self) -> X509WindowsCngURI {
        let raw_context = super::schannel_utils::cert_context_to_raw(&self.0);
        // SAFETY: The raw pointer come from the inner valid pointer of `cert_context`
        // that is of type `Cryptography::CERT_CONTEXT`
        let cert_info = unsafe { *(*raw_context).pCertInfo };

        // SAFETY: Issuer is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
        let issuer = unsafe {
            std::slice::from_raw_parts(cert_info.Issuer.pbData, cert_info.Issuer.cbData as usize)
        }
        .to_vec();
        // SAFETY: SerialNumber is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
        let serial_number = unsafe {
            std::slice::from_raw_parts(
                cert_info.SerialNumber.pbData,
                cert_info.SerialNumber.cbData as usize,
            )
        }
        .to_vec();

        X509WindowsCngURI {
            issuer,
            serial_number,
        }
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<crate::ValidationPathOwned, crate::ValidationPathError> {
        let raw_trusted_roots = super::list_trusted_root_certificate_anchors()
            .await
            .context("Cannot list trusted roots")
            .map_err(crate::ValidationPathError::Internal)?;
        let raw_intermediates = super::list_intermediate_certificates()
            .await
            .context("Cannot list intermediates certificates")
            .map_err(crate::ValidationPathError::Internal)?;
        let leaf = self
            .get_der()
            .await
            .context("Cannot get certificate content")
            .map_err(crate::ValidationPathError::Internal)?;
        let end_cert = webpki::EndEntityCert::try_from(&leaf)
            .context("Invalid leaf certificate")
            .map_err(crate::ValidationPathError::Internal)?;
        let now = DateTime::now();
        let path =
            crate::verify_certificate(&end_cert, &raw_trusted_roots, &raw_intermediates, now)
                .inspect_err(|e| log::warn!("Failed to verify certificate: {e}"))
                .map_err(|_| crate::ValidationPathError::Untrusted)?;

        let intermediates = path
            .intermediate_certificates()
            .map(|cert| cert.der().into_owned())
            .collect();
        let root = path.anchor().to_owned();

        Ok(crate::ValidationPathOwned {
            root,
            intermediates,
            leaf,
        })
    }
}
