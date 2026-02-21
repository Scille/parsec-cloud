// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Mutex;

pub use libparsec_platform_async::PinBoxFutureResult;
pub use libparsec_types::prelude::*;

pub use crate::{
    AccountVaultOperations, AccountVaultOperationsFetchOpaqueKeyError,
    AccountVaultOperationsUploadOpaqueKeyError, OpenBaoDeviceOperations,
    OpenBaoOperationsFetchOpaqueKeyError, OpenBaoOperationsUploadOpaqueKeyError,
};

use libparsec_platform_filesystem::{load_file, save_content, LoadFileError};

use crate::get_device_archive_path;
use std::path::Path;

pub async fn create_device_file(path: &Path, content: &[u8]) {
    save_content(path, content).await.unwrap()
}

pub async fn key_present_in_system(path: &Path) -> bool {
    match load_file(path).await {
        Ok(_) => true,
        Err(LoadFileError::NotFound) => false,
        Err(_) => panic!("Cannot get item in storage"),
    }
}

pub async fn key_is_archived(path: &Path) -> bool {
    let archived_path = get_device_archive_path(path);

    !key_present_in_system(path).await && key_present_in_system(&archived_path).await
}

/*
 * MockedAccountVaultOperations
 */

#[derive(Debug)]
pub(super) struct MockedAccountVaultOperations {
    account_email: EmailAddress,
    next_error_fetch_opaque_key: Mutex<Option<AccountVaultOperationsFetchOpaqueKeyError>>,
    next_error_upload_opaque_key: Mutex<Option<AccountVaultOperationsUploadOpaqueKeyError>>,
}

impl MockedAccountVaultOperations {
    pub fn new(account_email: EmailAddress) -> Self {
        Self {
            account_email,
            next_error_fetch_opaque_key: Mutex::new(None),
            next_error_upload_opaque_key: Mutex::new(None),
        }
    }

    pub fn inject_next_error_fetch_opaque_key(
        &self,
        err: AccountVaultOperationsFetchOpaqueKeyError,
    ) {
        let mut guard = self
            .next_error_fetch_opaque_key
            .lock()
            .expect("Mutex is poisoned");
        assert!((*guard).is_none(), "Next error already configured");
        *guard = Some(err);
    }

    pub fn inject_next_error_upload_opaque_key(
        &self,
        err: AccountVaultOperationsUploadOpaqueKeyError,
    ) {
        let mut guard = self
            .next_error_upload_opaque_key
            .lock()
            .expect("Mutex is poisoned");
        assert!((*guard).is_none(), "Next error already configured");
        *guard = Some(err);
    }
}

fn hash_email(account_email: &EmailAddress) -> [u8; 4] {
    let raw_email = account_email.to_string();
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(raw_email.as_bytes());
    let hash = hasher.finalize();
    hash.to_le_bytes()
}

fn key_from_account_vault_ciphertext_key_id(key_id: AccountVaultItemOpaqueKeyID) -> SecretKey {
    let mut raw_key = [0; 32];
    raw_key[0..16].copy_from_slice(key_id.as_bytes());
    raw_key[16..32].copy_from_slice(key_id.as_bytes());
    SecretKey::from(raw_key)
}

impl AccountVaultOperations for MockedAccountVaultOperations {
    fn account_email(&self) -> &EmailAddress {
        &self.account_email
    }

    fn fetch_opaque_key(
        &self,
        ciphertext_key_id: AccountVaultItemOpaqueKeyID,
    ) -> PinBoxFutureResult<SecretKey, AccountVaultOperationsFetchOpaqueKeyError> {
        {
            let mut guard = self
                .next_error_fetch_opaque_key
                .lock()
                .expect("Mutex is poisoned");
            if let Some(err) = guard.take() {
                return Box::pin(async move { Err(err) });
            }
        }

        let outcome = if ciphertext_key_id.as_bytes()[0..4] != hash_email(&self.account_email) {
            Err(AccountVaultOperationsFetchOpaqueKeyError::UnknownOpaqueKey)
        } else {
            let key = key_from_account_vault_ciphertext_key_id(ciphertext_key_id);
            Ok(key)
        };

        Box::pin(async move { outcome })
    }

    fn upload_opaque_key(
        &self,
    ) -> PinBoxFutureResult<
        (AccountVaultItemOpaqueKeyID, SecretKey),
        AccountVaultOperationsUploadOpaqueKeyError,
    > {
        {
            let mut guard = self
                .next_error_upload_opaque_key
                .lock()
                .expect("Mutex is poisoned");
            if let Some(err) = guard.take() {
                return Box::pin(async move { Err(err) });
            }
        }

        // Key ID is a UUID, so 16 bytes long.
        // We use the first 4 bytes to hash the account email, this way we can easily
        // tell if an account is supposed to know about a key ID or should return an error.

        let key_id = {
            let mut raw_id = AccountVaultItemOpaqueKeyID::default().into_bytes();

            raw_id[0..4].copy_from_slice(&hash_email(&self.account_email));

            AccountVaultItemOpaqueKeyID::from(raw_id)
        };

        let key = key_from_account_vault_ciphertext_key_id(key_id);

        Box::pin(async move { Ok((key_id, key)) })
    }
}

/*
 * MockedOpenBaoOperations
 */

#[derive(Debug, Clone)]
pub(super) struct MockedOpenBaoOperations {
    openbao_entity_id: String,
}

impl MockedOpenBaoOperations {
    pub fn new(account_email: EmailAddress) -> Self {
        Self {
            openbao_entity_id: openbao_entity_id_from_email(&account_email),
        }
    }
}

fn key_from_openbao_ciphertext_key_path(key_path: &str) -> SecretKey {
    let bytes = {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(key_path.as_bytes());
        let hash = hasher.finalize();
        hash.to_le_bytes()
    };

    let mut raw_key = [0; 32];
    raw_key[0..4].copy_from_slice(&bytes);
    SecretKey::from(raw_key)
}

fn openbao_entity_id_from_email(email: &EmailAddress) -> String {
    let hash = hash_email(email);
    // OpenBao identity ID is an UUID (e.g `65732d02-bb5f-7ce7-eae4-69067383b61d`)
    let openbao_entity_id = format!(
        "{:x}{:x}{:x}{:x}-0000-0000-0000-000000000000",
        hash[0], hash[1], hash[2], hash[3]
    );
    assert_eq!(openbao_entity_id.len(), 36); // 36 characters: four dashes + 128bits UUID as hex
    openbao_entity_id
}

impl OpenBaoDeviceOperations for MockedOpenBaoOperations {
    fn openbao_entity_id(&self) -> &str {
        &self.openbao_entity_id
    }

    fn openbao_preferred_auth_id(&self) -> &str {
        "HEXAGONE"
    }

    fn upload_opaque_key(
        &self,
    ) -> PinBoxFutureResult<(String, SecretKey), OpenBaoOperationsUploadOpaqueKeyError> {
        let key_path = format!(
            "{}/{}",
            &self.openbao_entity_id,
            uuid::Uuid::new_v4().as_simple()
        );
        let key = key_from_openbao_ciphertext_key_path(&key_path);

        Box::pin(async move { Ok((key_path, key)) })
    }

    fn fetch_opaque_key(
        &self,
        ciphertext_key_path: String,
    ) -> PinBoxFutureResult<SecretKey, OpenBaoOperationsFetchOpaqueKeyError> {
        let outcome = if !ciphertext_key_path.starts_with(&self.openbao_entity_id) {
            Err(OpenBaoOperationsFetchOpaqueKeyError::BadServerResponse(
                anyhow::anyhow!("Secret not found"),
            ))
        } else {
            let key = key_from_openbao_ciphertext_key_path(&ciphertext_key_path);
            Ok(key)
        };

        Box::pin(async move { outcome })
    }
}
