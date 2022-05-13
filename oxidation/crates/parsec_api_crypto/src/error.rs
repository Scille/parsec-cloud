// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CryptoError {
    #[error("Invalid signature")]
    Signature,
    #[error("Signature was forged or corrupt")]
    SignatureVerification,
    #[error("Invalid data size")]
    DataSize,
    #[error("Decryption error")]
    Decryption,
    #[error("The nonce must be exactly 24 bytes long")]
    Nonce,
}
