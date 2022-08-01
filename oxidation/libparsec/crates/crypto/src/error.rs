// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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
