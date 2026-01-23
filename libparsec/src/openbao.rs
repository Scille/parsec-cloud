// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_openbao::{OpenBaoCmds, OpenBaoListEntityEmailsError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum OpenBaoListSelfEmailsError {
    #[error("Invalid OpenBao server URL: {0}")]
    BadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    NoServerResponse(anyhow::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn openbao_list_self_emails(
    openbao_server_url: String,
    openbao_secret_mount_path: String,
    openbao_transit_mount_path: String,
    openbao_entity_id: String,
    openbao_auth_token: String,
) -> Result<Vec<String>, OpenBaoListSelfEmailsError> {
    let client = libparsec_client_connection::build_client()?;
    let cmds = Arc::new(OpenBaoCmds::new(
        client,
        openbao_server_url,
        openbao_secret_mount_path,
        openbao_transit_mount_path,
        openbao_entity_id,
        openbao_auth_token,
    ));
    cmds.list_self_emails().await.map_err(|err| match err {
        OpenBaoListEntityEmailsError::BadURL(err) => OpenBaoListSelfEmailsError::BadURL(err),
        OpenBaoListEntityEmailsError::NoServerResponse(err) => {
            OpenBaoListSelfEmailsError::NoServerResponse(err.into())
        }
        OpenBaoListEntityEmailsError::BadServerResponse(err) => {
            OpenBaoListSelfEmailsError::BadServerResponse(err)
        }
    })
}
