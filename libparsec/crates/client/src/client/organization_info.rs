// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds::latest::organization_info::{Rep, Req};
use libparsec_types::{anyhow, DateTime};

use crate::{CertifPollServerError, CertifStoreError, InvalidCertificateError};

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, thiserror::Error)]
pub enum ClientGetOrganizationBootstrapDateError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Component has stopped")]
    Stopped,
    #[error("Bootstrap date not found")]
    BootstrapDateNotFound,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("A certificate provided by the server is invalid: {0}")]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
}

impl From<CertifStoreError> for ClientGetOrganizationBootstrapDateError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

impl From<CertifPollServerError> for ClientGetOrganizationBootstrapDateError {
    fn from(value: CertifPollServerError) -> Self {
        match value {
            CertifPollServerError::Stopped => ClientGetOrganizationBootstrapDateError::Stopped,
            CertifPollServerError::Offline(e) => {
                ClientGetOrganizationBootstrapDateError::Offline(e)
            }
            CertifPollServerError::InvalidCertificate(err) => {
                ClientGetOrganizationBootstrapDateError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        }
    }
}

pub(super) async fn get_organization_bootstrap_date(
    client: &super::Client,
) -> Result<DateTime, ClientGetOrganizationBootstrapDateError> {
    // The organization bootstrap date coincides with the date the first user was created.
    // Since the first user is the only one not invited by another user, the strategy here is to
    // look for this user and get its creation date.

    let users = client
        .certificates_ops
        .list_users(false, None, None)
        .await?;
    let users = if users.is_empty() {
        // if no users found, poll for new certificates
        // and list users again
        // Should only happen during first startup
        // as there is no certificates available yet
        client.poll_server_for_new_certificates().await?;
        client
            .certificates_ops
            .list_users(false, None, None)
            .await?
    } else {
        users
    };

    users
        .into_iter()
        .find(|info| info.created_by.is_none()) // User was created by root verify key)
        .ok_or(ClientGetOrganizationBootstrapDateError::BootstrapDateNotFound)
        .map(|info| info.created_on)
}
