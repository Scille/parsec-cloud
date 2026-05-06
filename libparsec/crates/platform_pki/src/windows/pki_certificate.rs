// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::PlatformPkiPrivateKey;
use crate::{
    verify_certificate, PkiCertificateGetDerError, PkiCertificateGetValidationPathError,
    PkiCertificateRequestPrivateKeyError, PkiCertificateToReferenceError, PkiPrivateKey,
    X509CertificateDer, X509EndCertificate, X509ValidationPathOwned,
};

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
    pub async fn get_der(&self) -> Result<X509CertificateDer<'static>, PkiCertificateGetDerError> {
        Ok(X509CertificateDer::from_slice(self.0.to_der()).into_owned())
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<PkiPrivateKey, PkiCertificateRequestPrivateKeyError> {
        self.0
            .private_key()
            .compare_key(true)
            .acquire()
            .map(Into::into)
            .map(wrap_platform_private_key)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => PkiCertificateRequestPrivateKeyError::NotFound,
                _ => PkiCertificateRequestPrivateKeyError::Internal(e.into()),
            })
    }

    pub async fn to_reference(
        &self,
    ) -> Result<X509CertificateReference, PkiCertificateToReferenceError> {
        crate::shared::uri::get_certificate_ref(
            None,
            self.0.friendly_name().map(|s| s.as_bytes().into()).ok(),
            self.0.to_der(),
        )
        .map(|(cert_ref, _)| cert_ref)
        .map_err(|(_, e)| e)
        .context("Cannot get reference")
        .map_err(PkiCertificateToReferenceError::Internal)
    }

    pub async fn get_validation_path(
        &self,
    ) -> Result<X509ValidationPathOwned, PkiCertificateGetValidationPathError> {
        let certificate_revocation_lists = super::list_x509_certificate_revocation_lists()
            .await
            .context("Cannot list certificate revocation lists")
            .map_err(PkiCertificateGetValidationPathError::Internal)?;
        let raw_trusted_roots = super::list_x509_trust_anchors()
            .await
            .context("Cannot list trusted roots")
            .map_err(PkiCertificateGetValidationPathError::Internal)?;
        let raw_intermediates = super::list_intermediate_x509_certificates()
            .await
            .context("Cannot list intermediates certificates")
            .map_err(PkiCertificateGetValidationPathError::Internal)?;
        let leaf = self
            .get_der()
            .await
            .context("Cannot get certificate content")
            .map_err(PkiCertificateGetValidationPathError::Internal)?;
        let end_cert = X509EndCertificate::try_from(&leaf)
            .context("Invalid leaf certificate")
            .map_err(PkiCertificateGetValidationPathError::Internal)?;
        let now = DateTime::now();
        let path = verify_certificate(
            &end_cert,
            &raw_intermediates,
            &raw_trusted_roots,
            &certificate_revocation_lists,
            now,
        )
        .inspect_err(|e| log::warn!("Failed to verify certificate: {e}"))
        .map_err(|_| PkiCertificateGetValidationPathError::Untrusted)?;

        let intermediates = path
            .intermediate_certificates()
            .map(|cert| cert.der().into_owned())
            .collect();
        let root = path.anchor().to_owned();

        Ok(X509ValidationPathOwned {
            root,
            intermediates,
            leaf,
        })
    }

    pub async fn get_issuer_and_serial(&self) -> (Vec<u8>, Vec<u8>) {
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
        let mut serial_number = unsafe {
            std::slice::from_raw_parts(
                cert_info.SerialNumber.pbData,
                cert_info.SerialNumber.cbData as usize,
            )
        }
        .to_vec();

        // Windows provide the serial_number as little-endian, for consistency, we convert it to
        // big-endian by reversing bytes order
        use std::ops::DerefMut;
        serial_number.deref_mut().reverse();
        (issuer, serial_number)
    }
}

#[cfg(not(feature = "test-with-testbed"))]
fn wrap_platform_private_key(platform: PlatformPkiPrivateKey) -> PkiPrivateKey {
    PkiPrivateKey { platform }
}

#[cfg(feature = "test-with-testbed")]
fn wrap_platform_private_key(platform: PlatformPkiPrivateKey) -> PkiPrivateKey {
    PkiPrivateKey {
        platform: crate::testbed::MaybeWithTestbed::WithPlatform(platform),
    }
}
