// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use reqwest::{StatusCode, Url};

use libparsec_types::prelude::*;

use crate::{OpenBaoCmds, OpenBaoListEntityEmailsError};

pub async fn list_emails(
    cmds: &OpenBaoCmds,
    openbao_entity_id: &str,
) -> Result<Vec<String>, OpenBaoListEntityEmailsError> {
    let url = {
        let mut url = cmds
            .openbao_server_url
            .parse::<Url>()
            .map_err(|err| OpenBaoListEntityEmailsError::BadURL(err.into()))?;
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                OpenBaoListEntityEmailsError::BadURL(anyhow::anyhow!(
                    "URL should not be a cannot-be-a-base"
                ))
            })?;
            path.pop_if_empty();
            path.extend(&["v1", "identity", "entity", "id", openbao_entity_id]);
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
        .map_err(OpenBaoListEntityEmailsError::NoServerResponse)?;

    let allowed_emails = match rep.status() {
        StatusCode::OK => {
            let rep_json = rep.json::<serde_json::Value>().await.map_err(|err| {
                OpenBaoListEntityEmailsError::BadServerResponse(anyhow::anyhow!(
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
                    OpenBaoListEntityEmailsError::BadServerResponse(anyhow::anyhow!(
                        "Get entity response has missing or invalid `data/aliases` field"
                    ))
                })?
        }

        bad_status => {
            return Err(OpenBaoListEntityEmailsError::BadServerResponse(
                anyhow::anyhow!("Get entity has failed, bad status code: {}", bad_status),
            ));
        }
    };

    Ok(allowed_emails)
}
