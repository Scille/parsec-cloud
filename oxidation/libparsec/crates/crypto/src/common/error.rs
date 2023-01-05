// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Unsupported algorithm: {0}")]
    Algorithm(String),
    #[error("Invalid signature")]
    Signature,
    #[error("Signature was forged or corrupt")]
    SignatureVerification,
    #[error("Invalid data size")]
    DataSize,
    #[error("Decryption error")]
    Decryption,
    #[error("Invalid key size: expected {expected} bytes, got {got} bytes")]
    KeySize { expected: usize, got: usize },
    #[error("The nonce must be exactly 24 bytes long")]
    Nonce,
    #[error("Invalid SequesterPrivateKeyDer {0}")]
    SequesterPrivateKeyDer(String),
    #[error("Invalid SequesterPublicKeyDer {0}")]
    SequesterPublicKeyDer(String),
}

pub type CryptoResult<T> = Result<T, CryptoError>;
