// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    CertifDecryptOpaqueDataForRealmError, CertifGetLatestRealmKeyForEncryptionError,
    EncryptionUsage, InvalidCertificateError, InvalidKeysBundleError, workspace::WorkspaceOps,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceEncryptEditicsError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// The payloads of a batch travel as a single flat buffer, the i-th payload
// being `flat[offsets[i]..offsets[i+1]]` (so `offsets` has one more entry
// than the payload count). This is the shape used on the wire between the
// editor host page and its parent (see `oo-encrypt-batch`/`oo-decrypt-batch`
// in `client/editics/parent_host_api.ts`), so a batch crosses the whole
// stack without ever being split or re-flattened.
fn split_flat<'a>(flat: &'a [u8], offsets: &[u32]) -> anyhow::Result<Vec<&'a [u8]>> {
    if offsets.first() != Some(&0) {
        anyhow::bail!("Invalid editics batch offsets: must start at 0");
    }
    let mut payloads = Vec::with_capacity(offsets.len().saturating_sub(1));
    for window in offsets.windows(2) {
        let (start, end) = (window[0] as usize, window[1] as usize);
        if end < start {
            anyhow::bail!("Invalid editics batch offsets: must be non-decreasing");
        }
        let payload = flat.get(start..end).ok_or_else(|| {
            anyhow::anyhow!("Invalid editics batch offsets: out of the flat buffer bounds")
        })?;
        payloads.push(payload);
    }
    Ok(payloads)
}

// All the payloads of a batch are sealed with the same (latest) realm key,
// hence a single key index is returned: the other participants use it to
// resolve the key on their side.
pub(crate) async fn editics_encrypt_batch(
    ops: &WorkspaceOps,
    document_id: VlobID,
    flat_cleartexts: &[u8],
    offsets: &[u32],
) -> Result<(IndexInt, Vec<u8>, Vec<u32>), WorkspaceEncryptEditicsError> {
    let cleartexts = split_flat(flat_cleartexts, offsets)
        .map_err(|err| err.context("Cannot encrypt the editics payloads for realm"))?;

    let (key, key_index) = ops
        .certificates_ops
        .get_latest_realm_key_for_encryption(EncryptionUsage::Editics(document_id), ops.realm_id)
        .await
        .map_err(|err| match err {
            CertifGetLatestRealmKeyForEncryptionError::Stopped => {
                WorkspaceEncryptEditicsError::Stopped
            }
            CertifGetLatestRealmKeyForEncryptionError::Offline(err) => {
                WorkspaceEncryptEditicsError::Offline(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::NotAllowed => {
                WorkspaceEncryptEditicsError::NoRealmAccess
            }
            CertifGetLatestRealmKeyForEncryptionError::RealmDeleted => {
                WorkspaceEncryptEditicsError::RealmDeleted
            }
            CertifGetLatestRealmKeyForEncryptionError::NoKey => WorkspaceEncryptEditicsError::NoKey,
            CertifGetLatestRealmKeyForEncryptionError::InvalidKeysBundle(err) => {
                WorkspaceEncryptEditicsError::InvalidKeysBundle(err)
            }
            CertifGetLatestRealmKeyForEncryptionError::Internal(err) => err
                .context("Cannot encrypt the editics payload for realm")
                .into(),
        })?;

    let total_ciphertext_len = flat_cleartexts.len() + cleartexts.len() * SecretKey::OVERHEAD;
    if total_ciphertext_len > u32::MAX as usize {
        return Err(anyhow::anyhow!("The editics batch is too big").into());
    }

    let mut flat_ciphertexts = Vec::with_capacity(total_ciphertext_len);
    let mut ciphertext_offsets = Vec::with_capacity(cleartexts.len() + 1);
    ciphertext_offsets.push(0);
    let mut total_ciphertext_len = 0u32;
    for cleartext in cleartexts {
        let ciphertext = key.encrypt(cleartext);
        total_ciphertext_len += ciphertext.len() as u32;
        ciphertext_offsets.push(total_ciphertext_len);
        flat_ciphertexts.extend_from_slice(&ciphertext);
    }
    Ok((key_index, flat_ciphertexts, ciphertext_offsets))
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceDecryptEditicsError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("The referenced key doesn't exist yet in this realm")]
    KeyNotFound,
    #[error("The referenced key appears to be corrupted !")]
    CorruptedKey,
    #[error("Cannot decrypt the editics payload")]
    CorruptedData,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn editics_decrypt_batch(
    ops: &WorkspaceOps,
    document_id: VlobID,
    key_index: IndexInt,
    flat_ciphertexts: &[u8],
    offsets: &[u32],
) -> Result<(Vec<u8>, Vec<u32>), WorkspaceDecryptEditicsError> {
    let encrypteds = split_flat(flat_ciphertexts, offsets)
        .map_err(|err| err.context("Cannot decrypt the editics payloads for realm"))?;

    let cleartexts = ops
        .certificates_ops
        .decrypt_opaque_data_batch_for_realm(
            EncryptionUsage::Editics(document_id),
            ops.realm_id,
            key_index,
            &encrypteds,
        )
        .await
        .map_err(|err| match err {
            CertifDecryptOpaqueDataForRealmError::Stopped => WorkspaceDecryptEditicsError::Stopped,
            CertifDecryptOpaqueDataForRealmError::Offline(e) => {
                WorkspaceDecryptEditicsError::Offline(e)
            }
            CertifDecryptOpaqueDataForRealmError::NotAllowed => {
                WorkspaceDecryptEditicsError::NoRealmAccess
            }
            CertifDecryptOpaqueDataForRealmError::RealmDeleted => {
                WorkspaceDecryptEditicsError::RealmDeleted
            }
            CertifDecryptOpaqueDataForRealmError::KeyNotFound => {
                WorkspaceDecryptEditicsError::KeyNotFound
            }
            CertifDecryptOpaqueDataForRealmError::CorruptedKey => {
                WorkspaceDecryptEditicsError::CorruptedKey
            }
            CertifDecryptOpaqueDataForRealmError::CorruptedData => {
                WorkspaceDecryptEditicsError::CorruptedData
            }
            CertifDecryptOpaqueDataForRealmError::InvalidCertificate(err) => {
                WorkspaceDecryptEditicsError::InvalidCertificate(err)
            }
            CertifDecryptOpaqueDataForRealmError::InvalidKeysBundle(err) => {
                WorkspaceDecryptEditicsError::InvalidKeysBundle(err)
            }
            CertifDecryptOpaqueDataForRealmError::Internal(err) => err
                .context("Cannot decrypt the editics payload for realm")
                .into(),
        })?;

    // The cleartexts are smaller than the ciphertexts they come from (the
    // crypto overhead is removed), themselves bounded by the u32 input
    // offsets: the output offsets cannot overflow.
    let mut flat_cleartexts = Vec::with_capacity(flat_ciphertexts.len());
    let mut cleartext_offsets = Vec::with_capacity(cleartexts.len() + 1);
    cleartext_offsets.push(0);
    let mut total_cleartext_len = 0u32;
    for cleartext in cleartexts {
        total_cleartext_len += cleartext.len() as u32;
        cleartext_offsets.push(total_cleartext_len);
        flat_cleartexts.extend_from_slice(&cleartext);
    }
    Ok((flat_cleartexts, cleartext_offsets))
}
