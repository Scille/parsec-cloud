// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
mod shared;
pub mod x509;

#[cfg(test)]
#[path = "../tests/units/mod.rs"]
mod test;

use libparsec_types::prelude::*;

pub type X509CertificateDer<'a> = rustls_pki_types::CertificateDer<'a>;

// Mock module for unsupported platform
#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::{
        errors::{ListTrustedRootCertificatesError, ListUserCertificatesError},
        DecryptMessageError, DerCertificate, GetDerEncodedCertificateError,
        ListIntermediateCertificatesError, ShowCertificateSelectionDialogError, SignMessageError,
        ValidationPathOwned, X509CertificateDer,
    };
    use libparsec_types::prelude::*;
    use rustls_pki_types::CertificateDer;

    pub async fn get_der_encoded_certificate(
        certificate_ref: &X509CertificateReference,
    ) -> Result<X509CertificateDer<'static>, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub async fn list_trusted_root_certificate_anchors(
    ) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
        unimplemented!("platform not supported")
    }

    pub async fn list_intermediate_certificates(
    ) -> Result<Vec<X509CertificateDer<'static>>, ListIntermediateCertificatesError> {
        unimplemented!("platform not supported")
    }

    pub async fn sign_message(
        message: &[u8],
        certificate_ref: &X509CertificateReference,
    ) -> Result<(PkiSignatureAlgorithm, Bytes), SignMessageError> {
        let _ = message;
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub async fn decrypt_message(
        algo: PKIEncryptionAlgorithm,
        encrypted_message: &[u8],
        certificate_ref: &X509CertificateReference,
    ) -> Result<Bytes, DecryptMessageError> {
        let _ = (algo, encrypted_message, certificate_ref);
        unimplemented!("platform not supported")
    }

    pub fn show_certificate_selection_dialog_windows_only(
    ) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
        unimplemented!("platform not supported")
    }

    pub fn is_available() -> bool {
        false
    }

    pub async fn list_user_certificates_der(
    ) -> Result<Vec<DerCertificate<'static>>, ListUserCertificatesError> {
        unimplemented!("platform not supported")
    }

    pub struct PkiSystem;

    impl PkiSystem {
        /// Function to initialize the PKI System.
        pub async fn init(_config: crate::PkiConfig<'_>) -> anyhow::Result<Self> {
            anyhow::bail!("Platform not supported")
        }

        pub async fn find_certificate(
            &self,
            #[expect(unused_variables)] cert_ref: &X509CertificateReference,
        ) -> Result<Option<Certificate>, crate::FindCertificateError> {
            unimplemented!("platform not supported")
        }

        pub async fn list_user_certificates<'a>(
            &'a self,
        ) -> Result<impl Iterator<Item = Certificate> + use<'a>, crate::ListUserCertificateError>
        {
            unimplemented!("platform not supported");
            #[expect(
                unreachable_code,
                reason = "This return value is here to satisfy the `impl Iterator...`"
            )]
            Ok(Vec::new().into_iter())
        }
    }

    pub struct Certificate;

    impl Certificate {
        pub async fn get_der(
            &self,
        ) -> Result<CertificateDer<'static>, crate::GetCertificateDerError> {
            unimplemented!("platform not supported")
        }

        pub async fn request_private_key(
            &self,
        ) -> Result<X509PrivateKey, crate::RequestPrivateKeyError> {
            unimplemented!("platform not supported")
        }

        pub async fn to_reference(
            &self,
        ) -> Result<X509CertificateReference, crate::GetCertificateReferenceError> {
            unimplemented!("platform not supported")
        }

        pub async fn get_validation_path(
            &self,
        ) -> Result<ValidationPathOwned, crate::ValidationPathError> {
            unimplemented!("platform not supported")
        }
    }

    pub struct X509PrivateKey;

    impl X509PrivateKey {
        pub async fn sign(
            &self,
            #[expect(unused_variables)] message: &[u8],
        ) -> Result<(PkiSignatureAlgorithm, Bytes), crate::SignError> {
            unimplemented!("platform not supported")
        }

        pub async fn decrypt(
            &self,
            #[expect(unused_variables)] algorithm: PKIEncryptionAlgorithm,
            #[expect(unused_variables)] ciphertext: &[u8],
        ) -> Result<Bytes, crate::DecryptError> {
            unimplemented!("platform not supported")
        }
    }
}

// TODO: https://github.com/Scille/parsec-cloud/issues/11215
// This is specific to windows, it cannot be replicated on other platform.
// Instead, we likely need to go the manual way and show a custom dialog on the client side with a
// list of certificate that we retrieve from the platform certstore.
pub use errors::ShowCertificateSelectionDialogError;
pub use platform::show_certificate_selection_dialog_windows_only;

pub use errors::GetDerEncodedCertificateError;
pub use platform::get_der_encoded_certificate;

pub use errors::ListTrustedRootCertificatesError;
pub use platform::list_trusted_root_certificate_anchors;

pub use errors::ListIntermediateCertificatesError;
pub use platform::list_intermediate_certificates;

pub use errors::SignMessageError;
pub use platform::sign_message;

pub use shared::{verify_message, DerCertificate, SignedMessage, X509EndCertificate};

pub use shared::{encrypt_message, EncryptMessageError};

pub use errors::DecryptMessageError;
pub use platform::decrypt_message;

pub use platform::is_available;

pub use shared::verify_certificate;
pub use webpki::KeyUsage;

pub use errors::VerifyMessageError;
pub use shared::verify_message2;

pub use errors::GetValidationPathForCertError;
pub use shared::{get_validation_path_for_cert, ValidationPathOwned};

pub use shared::get_root_certificate_info_from_trustchain;

pub use errors::ListUserCertificatesError;
pub use shared::{
    list_user_certificates_with_details, CertificateDetails, CertificateWithDetails,
    InvalidCertificateReason,
};
pub use x509::DistinguishedNameValue;

/// Configuration that may be useful for initializing a PKI system
pub struct PkiConfig<'a> {
    pub config_dir: &'a std::path::Path,
    pub addr: &'a libparsec_types::ParsecAddr,
    pub proxy: &'a libparsec_platform_http_proxy::ProxyConfig,
}

pub use platform::{Certificate, PkiSystem};

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateDerError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum FindCertificateError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListUserCertificateError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RequestPrivateKeyError {
    #[error("private key not found")]
    NotFound,
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("unsupported signature algorithm")]
    UnsupportedAlgorithm,
    #[error("error during signature: {0}")]
    Sign(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptError {
    #[error("unsupported encryption algorithm")]
    UnsupportedAlgorithm,
    #[error("error during decryption: {0}")]
    Decrypt(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateReferenceError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationPathError {
    #[error("certificate not trusted")]
    Untrusted,
    #[error(transparent)]
    Internal(anyhow::Error),
}
