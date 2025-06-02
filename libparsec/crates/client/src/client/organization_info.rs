// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds::latest::organization_info::{Rep, Req};
use libparsec_types::anyhow;

pub struct OrganizationInfo {
    pub total_block_bytes: u64,
    pub total_metadata_bytes: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientOrganizationInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn organization_info(
    client: &super::Client,
) -> Result<OrganizationInfo, ClientOrganizationInfoError> {
    match client.cmds.send(Req).await? {
        Rep::Ok {
            total_block_bytes,
            total_metadata_bytes,
        } => Ok(OrganizationInfo {
            total_block_bytes,
            total_metadata_bytes,
        }),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {bad_rep:?}").into())
        }
    }
}
