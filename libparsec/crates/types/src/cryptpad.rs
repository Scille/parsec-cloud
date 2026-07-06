// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};

use libparsec_crypto::{SecretKey, SigningKey, VerifyKey};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DateTime, DeviceID, VlobID,
};
use crate::{impl_transparent_data_format_conversion, DataResult};

/*
 * CryptpadSessionKey
 */

parsec_data!("schema/cryptpad/cryptpad_session_key.json5");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "CryptpadSessionKeyData", try_from = "CryptpadSessionKeyData")]
pub struct CryptpadSessionKey {
    pub author: DeviceID,
    pub timestamp: DateTime,
    pub document_id: VlobID,
    pub can_edit: bool,
    pub key: String,
}

impl_transparent_data_format_conversion!(
    CryptpadSessionKey,
    CryptpadSessionKeyData,
    author,
    timestamp,
    document_id,
    can_edit,
    key,
);

impl CryptpadSessionKey {
    pub fn dump_sign_and_encrypt(&self, author_signkey: &SigningKey, key: &SecretKey) -> Vec<u8> {
        let serialized = format_v0_dump(&self);
        let signed = author_signkey.sign(&serialized);
        key.encrypt(&signed)
    }

    pub fn decrypt_verify_and_load(
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_document_id: VlobID,
    ) -> DataResult<Self> {
        let signed = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;

        Self::verify_and_load(
            &signed,
            author_verify_key,
            expected_author,
            expected_timestamp,
            expected_document_id,
        )
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        expected_document_id: VlobID,
    ) -> DataResult<Self> {
        let serialized = author_verify_key
            .verify(signed)
            .map_err(|_| DataError::Signature)?;

        let obj: Self = format_vx_load(serialized)?;

        if obj.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(obj.author),
            });
        }

        if obj.timestamp != expected_timestamp {
            return Err(DataError::UnexpectedTimestamp {
                expected: expected_timestamp,
                got: obj.timestamp,
            });
        }

        if obj.document_id != expected_document_id {
            return Err(DataError::UnexpectedId {
                expected: expected_document_id,
                got: obj.document_id,
            });
        }

        Ok(obj)
    }
}

#[cfg(test)]
#[path = "../tests/unit/cryptpad_session_key.rs"]
mod tests;
