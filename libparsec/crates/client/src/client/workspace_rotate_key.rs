// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::Client;
use crate::certif::InvalidCertificateError;

#[derive(Debug, thiserror::Error)]
pub enum ClientRotateWorkspaceKeyError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn rotate_workspace_key(
    _client_ops: &Client,
    _realm_id: VlobID,
) -> Result<(), ClientRotateWorkspaceKeyError> {
    todo!();
}
