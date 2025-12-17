// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
mod shared;
#[cfg(any(test, feature = "test-fixture"))]
pub mod test_fixture;
#[cfg(target_os = "windows")]
mod windows;
pub mod x509;
#[cfg(any(test, feature = "test-fixture"))]
pub use test_fixture::*;

#[cfg(test)]
#[path = "../tests/units/mod.rs"]
mod test;

use libparsec_types::{PKIEncryptionAlgorithm, PkiSignatureAlgorithm, X509CertificateReference};

use bytes::Bytes;

#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

// Mock module for unsupported platform
#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::{
        errors::ListTrustedRootCertificatesError, CertificateDer, DecryptMessageError,
        DecryptedMessage, EncryptMessageError, EncryptedMessage, GetDerEncodedCertificateError,
        ListIntermediateCertificatesError, ShowCertificateSelectionDialogError, SignMessageError,
        SignedMessageFromPki,
    };
    use libparsec_types::{PKIEncryptionAlgorithm, X509CertificateReference};

    pub fn get_der_encoded_certificate(
        certificate_ref: &X509CertificateReference,
    ) -> Result<CertificateDer, GetDerEncodedCertificateError> {
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub fn list_trusted_root_certificate_anchors(
    ) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
        unimplemented!("platform not supported")
    }

    pub fn list_intermediate_certificates(
    ) -> Result<Vec<rustls_pki_types::CertificateDer<'static>>, ListIntermediateCertificatesError>
    {
        unimplemented!("platform not supported")
    }

    pub fn sign_message(
        message: &[u8],
        certificate_ref: &X509CertificateReference,
    ) -> Result<SignedMessageFromPki, SignMessageError> {
        let _ = message;
        let _ = certificate_ref;
        unimplemented!("platform not supported")
    }

    pub fn encrypt_message(
        message: &[u8],
        certificate_ref: &X509CertificateReference,
    ) -> Result<EncryptedMessage, EncryptMessageError> {
        let _ = (message, certificate_ref);
        unimplemented!("platform not supported")
    }

    pub fn decrypt_message(
        algo: PKIEncryptionAlgorithm,
        encrypted_message: &[u8],
        certificate_ref: &X509CertificateReference,
    ) -> Result<DecryptedMessage, DecryptMessageError> {
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
}

#[derive(Debug)]
pub enum ShowCertificateSelectionDialogError {
    CannotOpenStore(std::io::Error),
    CannotGetCertificateInfo(std::io::Error),
}

impl std::fmt::Display for ShowCertificateSelectionDialogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShowCertificateSelectionDialogError::CannotOpenStore(e) => {
                write!(f, "Cannot open certificate store: {e}")
            }
            ShowCertificateSelectionDialogError::CannotGetCertificateInfo(e) => {
                write!(f, "Cannot get certificate info: {e}")
            }
        }
    }
}

impl std::error::Error for ShowCertificateSelectionDialogError {}

// TODO: https://github.com/Scille/parsec-cloud/issues/11215
// This is specific to windows, it cannot be replicated on other platform.
// Instead, we likely need to go the manual way and show a custom dialog on the client side with a
// list of certificate that we retrieve from the platform certstore.
pub use platform::show_certificate_selection_dialog_windows_only;

pub use errors::GetDerEncodedCertificateError;
pub use platform::get_der_encoded_certificate;

pub struct CertificateDer {
    pub cert_ref: X509CertificateReference,
    pub der_content: Bytes,
}

pub use errors::ListTrustedRootCertificatesError;
pub use platform::list_trusted_root_certificate_anchors;

pub use errors::ListIntermediateCertificatesError;
pub use platform::list_intermediate_certificates;

pub struct SignedMessageFromPki {
    pub algo: PkiSignatureAlgorithm,
    pub cert_ref: X509CertificateReference,
    pub signature: Bytes,
}

pub use errors::SignMessageError;
pub use platform::sign_message;

pub use shared::{verify_message, Certificate, SignedMessage, X509EndCertificate};

pub struct EncryptedMessage {
    pub algo: PKIEncryptionAlgorithm,
    pub cert_ref: X509CertificateReference,
    pub ciphered: Bytes,
}

pub use errors::EncryptMessageError;
pub use platform::encrypt_message;

pub struct DecryptedMessage {
    pub cert_ref: X509CertificateReference,
    pub data: Bytes,
}

pub use errors::DecryptMessageError;
pub use platform::decrypt_message;

pub use errors::CreateLocalPendingError;
pub use shared::create_local_pending;

pub use platform::is_available;

pub use errors::VerifyCertificateError;
pub use shared::verify_certificate;
pub use webpki::KeyUsage;

pub use errors::LoadSubmitPayloadError;
pub use shared::load_submit_payload;

pub use errors::LoadAnswerPayloadError;
pub use shared::load_answer_payload;

pub use errors::GetIntermediatesCertsForCertError;
pub use shared::get_intermediate_certs_for_cert;
