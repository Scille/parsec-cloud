// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ShowCertificateSelectionDialogError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot get certificate info: {0}")]
    CannotGetCertificateInfo(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetDerEncodedCertificateError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum ListCertificatesError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
}
pub type ListTrustedRootCertificatesError = ListCertificatesError;
pub type ListIntermediateCertificatesError = ListCertificatesError;

#[derive(Debug, thiserror::Error)]
pub enum SignMessageError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
    NotFound,
    #[error("Cannot acquire keypair related to certificate: {0}")]
    CannotAcquireKeypair(std::io::Error),
    #[error("Cannot sign message: {0}")]
    CannotSign(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyMessageError {
    #[error("X509 certificate cannot be trusted: {0}")]
    X509CertificateUntrusted(webpki::Error),
    #[error("Invalid signature: {0}")]
    InvalidSignature(webpki::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum VerifySignatureError {
    #[error("Invalid signature: {0}")]
    InvalidSignature(webpki::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptMessageError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
    NotFound,
    #[error("Cannot acquire keypair related to certificate: {0}")]
    CannotAcquireKeypair(std::io::Error),
    #[error("Cannot decrypt message: {0}")]
    CannotDecrypt(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetValidationPathForCertError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot find certificate")]
    NotFound,
    #[error("Invalid certificate: invalid DER format: {0}")]
    InvalidCertificateDer(webpki::Error),
    #[error("Invalid certificate: time out of valid range: {0}")]
    InvalidCertificateDateTimeOutOfRange(chrono::OutOfRangeError),
    #[error("Invalid certificate: cannot be trusted: {0}")]
    InvalidCertificateUntrusted(webpki::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetRootCertificateInfoFromTrustchainError {
    #[error("Invalid certificate: invalid DER format: {0}")]
    InvalidCertificateDer(anyhow::Error),
    #[error("Invalid certificate: missing common name")]
    InvalidCertificateNoCommonName,
}

error_set::error_set! {
    ValidatePayloadError := {
            #[display("Invalid certificate: {0}")]
            InvalidCertificateDer(webpki::Error),
            #[display("Invalid signature for the given message and certificate: {0}")]
            InvalidSignature(webpki::Error),
            #[display("Invalid certificate: cannot be trusted: {0}")]
            Untrusted(webpki::Error),
        }
    DataError := {
        DataError(libparsec_types::DataError)
    }
    LoadSubmitPayloadError := ValidatePayloadError || DataError
    LoadAnswerPayloadError := ValidatePayloadError || DataError
}
