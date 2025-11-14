// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::{StatusCode, Url};

use data_encoding::BASE64;
use libparsec_types::{prelude::*, uuid::Uuid};

#[derive(Debug)]
pub struct OpenBaoCmds {
    client: reqwest::Client,
    // Don't store the server URL as `url::Url` since we use as base URL for building
    // new ones, we also need to ensure that the URL is not a cannot-be-a-base.
    // Hence it is simpler to parse and do the cannot-be-a-base check together
    // where the new URL is needed (as `OpenBaoCmds::new` cannot fail).
    openbao_server_url: String,
    openbao_secret_mount_path: String,
    openbao_entity_id: String,
    openbao_auth_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoFetchOpaqueKeyError {
    #[error("Invalid OpenBao server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    NoServerResponse(reqwest::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoUploadOpaqueKeyError {
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
        openbao_entity_id: String,
        openbao_auth_token: String,
    ) -> Self {
        Self {
            client,
            openbao_server_url,
            openbao_secret_mount_path,
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
        let url = {
            let mut url = self
                .openbao_server_url
                .parse::<Url>()
                .map_err(|err| OpenBaoFetchOpaqueKeyError::BadURL(err.into()))?;
            {
                let mut path = url.path_segments_mut().map_err(|()| {
                    OpenBaoFetchOpaqueKeyError::BadURL(anyhow::anyhow!(
                        "URL should not be a cannot-be-a-base"
                    ))
                })?;
                path.pop_if_empty();
                path.push("v1");
                path.push(&self.openbao_secret_mount_path);
                path.push("data");
                path.push(openbao_ciphertext_key_path);
            }
            url
        };

        let rep = self
            .client
            .get(url)
            .header("x-vault-token", self.openbao_auth_token.clone())
            .send()
            .await
            .map_err(OpenBaoFetchOpaqueKeyError::NoServerResponse)?;

        if !matches!(rep.status(), StatusCode::OK) {
            return Err(OpenBaoFetchOpaqueKeyError::BadServerResponse(
                anyhow::anyhow!("Bad status code: {}", rep.status()),
            ));
        }

        // See https://openbao.org/api-docs/secret/kv/kv-v2/#read-secret-version
        let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
            OpenBaoFetchOpaqueKeyError::BadServerResponse(anyhow::anyhow!("Not JSON: {:?}", err))
        })?;

        let key = rep_json["data"]["data"]["opaque_key"]
            .as_str()
            .and_then(|b64_raw| {
                BASE64
                    .decode(b64_raw.as_bytes())
                    .ok()
                    .and_then(|raw| SecretKey::try_from(raw.as_slice()).ok())
            })
            .ok_or_else(|| {
                OpenBaoFetchOpaqueKeyError::BadServerResponse(anyhow::anyhow!(
                    "Missing or invalid `data/data/opaque_key` field"
                ))
            })?;

        Ok(key)
    }

    pub async fn upload_opaque_key(
        &self,
    ) -> Result<(String, SecretKey), OpenBaoUploadOpaqueKeyError> {
        let key = SecretKey::generate();
        let key_path = format!("{}/{}", self.openbao_entity_id, Uuid::new_v4().as_simple());

        let url = {
            let mut url = self
                .openbao_server_url
                .parse::<Url>()
                .map_err(|err| OpenBaoUploadOpaqueKeyError::BadURL(err.into()))?;
            {
                let mut path = url.path_segments_mut().map_err(|()| {
                    OpenBaoUploadOpaqueKeyError::BadURL(anyhow::anyhow!(
                        "URL should not be a cannot-be-a-base"
                    ))
                })?;
                path.pop_if_empty();
                path.push("v1");
                path.push(&self.openbao_secret_mount_path);
                path.push("data");
                path.push(&key_path);
            }
            url
        };

        let rep = self
            .client
            .post(url)
            .header("x-vault-token", self.openbao_auth_token.clone())
            .json(&serde_json::json!({
                "options": {
                    "cas": 0,  // Prevent overwriting existing secret
                },
                "data": {
                    "opaque_key": &BASE64.encode(key.as_ref()),
                },
            }))
            .send()
            .await
            .map_err(OpenBaoUploadOpaqueKeyError::NoServerResponse)?;

        if !matches!(rep.status(), StatusCode::OK) {
            return Err(OpenBaoUploadOpaqueKeyError::BadServerResponse(
                anyhow::anyhow!("Bad status code: {}", rep.status()),
            ));
        }

        Ok((key_path, key))
    }
}

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
mod tests;
