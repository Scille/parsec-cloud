// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod accept;
mod list;
mod reject;
mod submit;
mod submitter_cancel;
mod submitter_finalize;
mod submitter_info;
mod submitter_list_local;

pub use accept::*;
use libparsec_platform_pki::{
    EncryptMessageError, PkiCertificateGetDerError, PkiCertificateGetValidationPathError,
    PkiCertificateToReferenceError, PkiPrivateKeyDecryptError, PkiPrivateKeySignError,
    PkiSystemGetCertificateRevocationListsError,
};
use libparsec_types::anyhow;
pub use list::*;
pub use reject::*;
pub use submit::*;
pub use submitter_cancel::*;
pub use submitter_finalize::*;
pub use submitter_info::*;
pub use submitter_list_local::*;

#[derive(Debug, thiserror::Error)]
pub enum PkiErrorDetail {
    #[error("unsupported algorithm")]
    UnsupportedAlgorithm,
    #[error("error during decryption: {0}")]
    Decrypt(anyhow::Error),
    #[error("untrusted certificate")]
    Untrusted,
    #[error("error during signature: {0}")]
    Sign(anyhow::Error),
    #[error("Cannot acquire public key: {0}")]
    CannotAcquirePubkey(anyhow::Error),
    #[error("Cannot encrypt message: {0}")]
    CannotEncrypt(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<PkiPrivateKeyDecryptError> for PkiErrorDetail {
    fn from(value: PkiPrivateKeyDecryptError) -> Self {
        match value {
            PkiPrivateKeyDecryptError::UnsupportedAlgorithm => PkiErrorDetail::UnsupportedAlgorithm,
            PkiPrivateKeyDecryptError::Decrypt(error) => PkiErrorDetail::Decrypt(error),
            PkiPrivateKeyDecryptError::Internal(error) => PkiErrorDetail::Internal(error),
        }
    }
}

impl From<PkiCertificateGetValidationPathError> for PkiErrorDetail {
    fn from(value: PkiCertificateGetValidationPathError) -> Self {
        match value {
            PkiCertificateGetValidationPathError::Untrusted => PkiErrorDetail::Untrusted,
            PkiCertificateGetValidationPathError::Internal(error) => {
                PkiErrorDetail::Internal(error)
            }
        }
    }
}

impl From<PkiPrivateKeySignError> for PkiErrorDetail {
    fn from(value: PkiPrivateKeySignError) -> Self {
        match value {
            PkiPrivateKeySignError::UnsupportedAlgorithm => PkiErrorDetail::UnsupportedAlgorithm,
            PkiPrivateKeySignError::Sign(error) => PkiErrorDetail::Sign(error),
            PkiPrivateKeySignError::Internal(error) => PkiErrorDetail::Internal(error),
        }
    }
}

impl From<PkiCertificateToReferenceError> for PkiErrorDetail {
    fn from(value: PkiCertificateToReferenceError) -> Self {
        match value {
            PkiCertificateToReferenceError::Internal(error) => PkiErrorDetail::Internal(error),
        }
    }
}

impl From<PkiSystemGetCertificateRevocationListsError> for PkiErrorDetail {
    fn from(value: PkiSystemGetCertificateRevocationListsError) -> Self {
        match value {
            PkiSystemGetCertificateRevocationListsError::Internal(error) => {
                PkiErrorDetail::Internal(error)
            }
        }
    }
}
impl From<PkiCertificateGetDerError> for PkiErrorDetail {
    fn from(value: PkiCertificateGetDerError) -> Self {
        match value {
            PkiCertificateGetDerError::Internal(error) => PkiErrorDetail::Internal(error),
        }
    }
}

impl From<EncryptMessageError> for PkiErrorDetail {
    fn from(value: EncryptMessageError) -> Self {
        match value {
            EncryptMessageError::CannotAcquirePubkey(get_certificate_public_key_error) => {
                PkiErrorDetail::CannotAcquirePubkey(get_certificate_public_key_error.into())
            }
            EncryptMessageError::CannotEncrypt(error) => {
                PkiErrorDetail::CannotEncrypt(error.into())
            }
        }
    }
}
