// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::{StatusCode, Url};

use data_encoding::BASE64;
use libparsec_types::{prelude::*, uuid::Uuid};

use crate::{OpenBaoCmds, OpenBaoFetchOpaqueKeyError, OpenBaoUploadOpaqueKeyError};

pub async fn fetch_opaque_key(
    cmds: &OpenBaoCmds,
    openbao_ciphertext_key_path: &str,
) -> Result<SecretKey, OpenBaoFetchOpaqueKeyError> {
    let url = {
        let mut url = cmds
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
            path.push(&cmds.openbao_secret_mount_path);
            path.push("data");
            path.push(openbao_ciphertext_key_path);
        }
        url
    };

    let rep = cmds
        .client
        .get(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
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

    // Note this won't panic field is missing: instead a `serde_json::Value` object
    // representing `undefined` is returned that will itself return another
    // `undefined` if another field is looked into it.
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
    cmds: &OpenBaoCmds,
) -> Result<(String, SecretKey), OpenBaoUploadOpaqueKeyError> {
    let key = SecretKey::generate();
    let key_path = format!("{}/{}", cmds.openbao_entity_id, Uuid::new_v4().as_simple());

    let url = {
        let mut url = cmds
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
            path.push(&cmds.openbao_secret_mount_path);
            path.push("data");
            path.push(&key_path);
        }
        url
    };

    let rep = cmds
        .client
        .post(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
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
