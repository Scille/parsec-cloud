// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::{StatusCode, Url};

use data_encoding::BASE64;
use libparsec_types::prelude::*;

use crate::{OpenBaoCmds, OpenBaoSignError, OpenBaoVerifyError};

enum SignOutcome {
    Done { signature: String },
    KeyUploadNeeded,
}

fn generate_sign_key_name(openbao_entity_id: &str) -> String {
    format!("user-{}", openbao_entity_id)
}

pub async fn sign(cmds: &OpenBaoCmds, payload: &[u8]) -> Result<String, OpenBaoSignError> {
    let key_name = generate_sign_key_name(&cmds.openbao_entity_id);

    match do_sign(cmds, &key_name, payload).await? {
        // Main case: the key already exist and we can use it just fine
        SignOutcome::Done { signature } => return Ok(signature),

        // Initial special case: need to first create the signing key !
        SignOutcome::KeyUploadNeeded => (),
    }

    do_create_signing_key(cmds, &key_name).await?;

    match do_sign(cmds, &key_name, payload).await? {
        SignOutcome::Done { signature } => Ok(signature),
        SignOutcome::KeyUploadNeeded => {
            // This is unexpected !
            Err(OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                "Signing key creation succeeded, but cannot use it!"
            )))
        }
    }
}

async fn do_create_signing_key(cmds: &OpenBaoCmds, key_name: &str) -> Result<(), OpenBaoSignError> {
    let url = {
        let mut url = cmds
            .openbao_server_url
            .parse::<Url>()
            .map_err(|err| OpenBaoSignError::BadURL(err.into()))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                OpenBaoSignError::BadURL(anyhow::anyhow!("URL should not be a cannot-be-a-base"))
            })?;
            path.pop_if_empty();
            path.push("v1");
            path.push(&cmds.openbao_transit_mount_path);
            path.push("keys");
            path.push(key_name);
        }
        url
    };

    // See https://openbao.org/api-docs/secret/transit/#create-key

    let rep = cmds
        .client
        .post(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
        .json(&serde_json::json!({
            "type": "ed25519",
        }))
        .send()
        .await
        .map_err(OpenBaoSignError::NoServerResponse)?;

    match rep.status() {
        // Note key creation is idempotent, so there is no special status code if the key already exists
        StatusCode::OK => Ok(()),
        bad_status => Err(OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
            "Signing key creation has failed, bad status code: {}",
            bad_status
        ))),
    }
}

async fn do_sign(
    cmds: &OpenBaoCmds,
    key_name: &str,
    payload: &[u8],
) -> Result<SignOutcome, OpenBaoSignError> {
    let url = {
        let mut url = cmds
            .openbao_server_url
            .parse::<Url>()
            .map_err(|err| OpenBaoSignError::BadURL(err.into()))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                OpenBaoSignError::BadURL(anyhow::anyhow!("URL should not be a cannot-be-a-base"))
            })?;
            path.pop_if_empty();
            path.push("v1");
            path.push(&cmds.openbao_transit_mount_path);
            path.push("sign");
            path.push(key_name);
        }
        url
    };

    // See https://openbao.org/api-docs/secret/transit/#sign-data

    let rep = cmds
        .client
        .post(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
        .json(&serde_json::json!({
            "input": &BASE64.encode(payload.as_ref()),
        }))
        .send()
        .await
        .map_err(OpenBaoSignError::NoServerResponse)?;

    match rep.status() {
        StatusCode::OK => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                    "Sign response is not JSON: {:?}",
                    err
                ))
            })?;

            // Note this won't panic field is missing: instead a `serde_json::Value` object
            // representing `undefined` is returned that will itself return another
            // `undefined` if another field is looked into it.
            let signature = rep_json["data"]["signature"]
                .as_str()
                .ok_or_else(|| {
                    OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                        "Sign response has missing or invalid `data/signature` field"
                    ))
                })?
                .to_owned();

            Ok(SignOutcome::Done { signature })
        }

        StatusCode::BAD_REQUEST => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                    "Sign response is a 400, but body is not JSON: {:?}",
                    err
                ))
            })?;

            let errors = rep_json["errors"].as_array().ok_or_else(|| {
                OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                    "Sign response is a 400, but with missing or invalid `errors` field"
                ))
            })?;
            let key_not_found = errors
                .iter()
                .any(|x| x.as_str() == Some("signing key not found"));

            if key_not_found {
                Ok(SignOutcome::KeyUploadNeeded)
            } else {
                Err(OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
                    "Sign response is a 400: {}",
                    rep_json
                )))
            }
        }

        bad_status => Err(OpenBaoSignError::BadServerResponse(anyhow::anyhow!(
            "Sign has failed, bad status code: {}",
            bad_status
        ))),
    }
}

pub async fn verify(
    cmds: &OpenBaoCmds,
    author_openbao_entity_id: &str,
    signature: &str,
    payload: &[u8],
    expected_author: Option<&EmailAddress>,
) -> Result<(), OpenBaoVerifyError> {
    do_verify(cmds, author_openbao_entity_id, signature, payload).await?;

    if let Some(expected_author) = expected_author {
        // Signature is valid... but now we must make sure the author is who we think it is!
        do_check_author(cmds, author_openbao_entity_id, expected_author).await?;
    }

    Ok(())
}

pub async fn do_verify(
    cmds: &OpenBaoCmds,
    author_openbao_entity_id: &str,
    signature: &str,
    payload: &[u8],
) -> Result<(), OpenBaoVerifyError> {
    let key_name = generate_sign_key_name(author_openbao_entity_id);

    let url = {
        let mut url = cmds
            .openbao_server_url
            .parse::<Url>()
            .map_err(|err| OpenBaoVerifyError::BadURL(err.into()))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                OpenBaoVerifyError::BadURL(anyhow::anyhow!("URL should not be a cannot-be-a-base"))
            })?;
            path.pop_if_empty();
            path.push("v1");
            path.push(&cmds.openbao_transit_mount_path);
            path.push("verify");
            path.push(&key_name);
        }
        url
    };

    // See https://openbao.org/api-docs/secret/transit/#verify-signed-data

    let rep = cmds
        .client
        .post(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
        .json(&serde_json::json!({
            "input": &BASE64.encode(payload.as_ref()),
            "signature": signature,
        }))
        .send()
        .await
        .map_err(OpenBaoVerifyError::NoServerResponse)?;

    match rep.status() {
        StatusCode::OK => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                    "Verify response is not JSON: {:?}",
                    err
                ))
            })?;

            // Note this won't panic field is missing: instead a `serde_json::Value` object
            // representing `undefined` is returned that will itself return another
            // `undefined` if another field is looked into it.
            let is_valid = rep_json["data"]["valid"]
                .as_bool()
                .ok_or_else(|| {
                    OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                        "Verify response has missing or invalid `data/valid` field"
                    ))
                })?
                .to_owned();

            if !is_valid {
                return Err(OpenBaoVerifyError::BadSignature);
            }
        }

        StatusCode::BAD_REQUEST => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                    "Verify response is a 400, but body is not JSON: {:?}",
                    err
                ))
            })?;

            let errors = rep_json["errors"].as_array().ok_or_else(|| {
                OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                    "Verify response is a 400, but with missing or invalid `errors` field"
                ))
            })?;
            let key_not_found = errors
                .iter()
                .any(|x| x.as_str() == Some("signature verification key not found"));

            if key_not_found {
                return Err(OpenBaoVerifyError::BadSignature);
            } else {
                return Err(OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                    "Verify response is a 400: {}",
                    rep_json
                )));
            }
        }

        bad_status => {
            return Err(OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                "Verify has failed, bad status code: {}",
                bad_status
            )));
        }
    }

    Ok(())
}

pub async fn do_check_author(
    cmds: &OpenBaoCmds,
    author_openbao_entity_id: &str,
    expected_author: &EmailAddress,
) -> Result<(), OpenBaoVerifyError> {
    let url = {
        let mut url = cmds
            .openbao_server_url
            .parse::<Url>()
            .map_err(|err| OpenBaoVerifyError::BadURL(err.into()))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                OpenBaoVerifyError::BadURL(anyhow::anyhow!("URL should not be a cannot-be-a-base"))
            })?;
            path.pop_if_empty();
            path.push("v1");
            path.push("identity");
            path.push("entity");
            path.push("id");
            path.push(author_openbao_entity_id);
        }
        url
    };

    // See https://openbao.org/api-docs/secret/identity/entity/#read-entity-by-id

    let rep = cmds
        .client
        .get(url)
        .header("x-vault-token", cmds.openbao_auth_token.clone())
        .send()
        .await
        .map_err(OpenBaoVerifyError::NoServerResponse)?;

    let allowed_emails = match rep.status() {
        StatusCode::OK => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                    "Get entity response is not JSON: {:?}",
                    err
                ))
            })?;

            // Note this won't panic field is missing: instead a `serde_json::Value` object
            // representing `undefined` is returned that will itself return another
            // `undefined` if another field is looked into it.
            rep_json["data"]["aliases"]
                .as_array()
                .and_then(|aliases| {
                    let mut allowed_emails = vec![];
                    for alias in aliases {
                        match alias["name"].as_str() {
                            None => return None, // Invalid field
                            Some(name) => allowed_emails.push(name.to_string()),
                        }
                    }
                    // Note we don't try to validate this `name` field to make sure it
                    // corresponds to a valid email. This is for three reasons:
                    // - Those names are only going to be compared with a valid email,
                    //   hence any non-email value will just fail the comparison.
                    // - The `name` field is not guaranteed to contain an email: it is
                    //   only a configuration in the OpenBao server that requests it.
                    // - In Parsec we only accept a subset of all possible emails (i.e.
                    //   ASCII-only emails), so otherwise valid emails may fail our
                    //   validation nevertheless.
                    Some(allowed_emails)
                })
                .ok_or_else(|| {
                    OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                        "Get entity response has missing or invalid `data/aliases` field"
                    ))
                })?
        }

        bad_status => {
            return Err(OpenBaoVerifyError::BadServerResponse(anyhow::anyhow!(
                "Get entity has failed, bad status code: {}",
                bad_status
            )));
        }
    };

    let expected_author = expected_author.to_string();
    if !allowed_emails.contains(&expected_author) {
        return Err(OpenBaoVerifyError::UnexpectedAuthor);
    }

    Ok(())
}
