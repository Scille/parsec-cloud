// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

mod identity;
mod opaque_key;
mod sign;

#[derive(Debug)]
pub struct OpenBaoCmds {
    client: reqwest::Client,
    // Don't store the server URL as `url::Url` since we use as base URL for building
    // new ones, we also need to ensure that the URL is not a cannot-be-a-base.
    // Hence it is simpler to parse and do the cannot-be-a-base check together
    // where the new URL is needed (as `OpenBaoCmds::new` cannot fail).
    openbao_server_url: String,
    openbao_secret_mount_path: String,
    openbao_transit_mount_path: String,
    openbao_entity_id: String,
    openbao_auth_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoOperationError {
    #[error("Invalid OpenBao server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    NoServerResponse(reqwest::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

pub type OpenBaoFetchOpaqueKeyError = OpenBaoOperationError;
pub type OpenBaoUploadOpaqueKeyError = OpenBaoOperationError;
pub type OpenBaoSignError = OpenBaoOperationError;
pub type OpenBaoListEntityEmailsError = OpenBaoOperationError;

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoVerifyError {
    #[error("Bad signature")]
    BadSignature,
    #[error("Signature is valid, but doesn't come from the expected author!")]
    UnexpectedAuthor,
    #[error("Invalid OpenBao server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    NoServerResponse(reqwest::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

impl OpenBaoCmds {
    pub fn new(
        client: reqwest::Client,
        openbao_server_url: String,
        openbao_secret_mount_path: String,
        openbao_transit_mount_path: String,
        openbao_entity_id: String,
        openbao_auth_token: String,
    ) -> Self {
        Self {
            client,
            openbao_server_url,
            openbao_secret_mount_path,
            openbao_transit_mount_path,
            openbao_entity_id,
            openbao_auth_token,
        }
    }

    pub fn openbao_entity_id(&self) -> &str {
        &self.openbao_entity_id
    }

    pub async fn fetch_opaque_key(
        &self,
        openbao_ciphertext_key_path: &str,
    ) -> Result<SecretKey, OpenBaoFetchOpaqueKeyError> {
        opaque_key::fetch_opaque_key(self, openbao_ciphertext_key_path).await
    }

    pub async fn upload_opaque_key(
        &self,
    ) -> Result<(String, SecretKey), OpenBaoUploadOpaqueKeyError> {
        opaque_key::upload_opaque_key(self).await
    }

    /// This signing system relies on the fact OpenBao is configured to only
    /// allow POST `/transit/sign/entity-{entity_id}` (i.e. the signing API) to
    /// the user referenced in OpenBao by this entity ID.
    ///
    /// This way the verify operation knows the entity ID of the author, and can
    /// then request OpenBao about it to obtain its emails.
    pub async fn sign(&self, payload: &[u8]) -> Result<String, OpenBaoSignError> {
        sign::sign(self, payload).await
    }

    pub async fn verify(
        &self,
        author_openbao_entity_id: &str,
        signature: &str,
        payload: &[u8],
        expected_author: Option<&EmailAddress>,
    ) -> Result<(), OpenBaoVerifyError> {
        sign::verify(
            self,
            author_openbao_entity_id,
            signature,
            payload,
            expected_author,
        )
        .await
    }

    /// Note we return the emails as `String` and not as `EmailAddress`!
    /// This is because `EmailADdress` only allow a subset of all the valid emails
    /// (typically emails containing unicode are not allowed), while OpenBAO has
    /// no such limitation.
    pub async fn list_self_emails(&self) -> Result<Vec<String>, OpenBaoListEntityEmailsError> {
        identity::list_emails(self, &self.openbao_entity_id).await
    }
}

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
mod tests;
