// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
mod shared;
#[cfg(any(test, feature = "test-fixture"))]
pub mod test_fixture;
pub mod x509;
#[cfg(any(test, feature = "test-fixture"))]
pub use test_fixture::*;

#[cfg(test)]
#[path = "../tests/units/mod.rs"]
mod test;

use libparsec_types::prelude::*;

// Mock module for unsupported platform
#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::{
        errors::ListTrustedRootCertificatesError, DecryptMessageError,
        GetDerEncodedCertificateError, ListIntermediateCertificatesError,
        ShowCertificateSelectionDialogError, SignMessageError,
    };
    use libparsec_types::prelude::*;

    pub async fn get_der_encoded_certificate(
        certificate_ref: &X509CertificateReference,
    ) -> Result<Bytes, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub async fn list_trusted_root_certificate_anchors(
    ) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
        unimplemented!("platform not supported")
    }

    pub async fn list_intermediate_certificates(
    ) -> Result<Vec<rustls_pki_types::CertificateDer<'static>>, ListIntermediateCertificatesError>
    {
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

    pub struct PkiSystem;

    impl PkiSystem {
        /// Function to initialize the PKI System.
        pub async fn init(_config: crate::PkiConfig<'_>) -> anyhow::Result<Self> {
            anyhow::bail!("Platform not supported")
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

pub use shared::{verify_message, Certificate, SignedMessage, X509EndCertificate};

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

/// Configuration that may be useful for initializing a PKI system
pub struct PkiConfig<'a> {
    pub config_dir: &'a std::path::Path,
    pub addr: &'a libparsec_types::ParsecAddr,
    pub proxy: &'a libparsec_platform_http_proxy::ProxyConfig,
}

pub use platform::PkiSystem;
