// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Client;
pub use crate::invite::{
    list_invitations, DeviceGreetInitialCtx, InviteListItem, ListInvitationsError,
    ShamirRecoveryGreetInitialCtx, UserGreetInitialCtx,
};
use crate::{certif::InvalidCertificateError, CertifGetShamirRecoveryShareDataError};

pub fn start_user_invitation_greet(client: &Client, token: AccessToken) -> UserGreetInitialCtx {
    UserGreetInitialCtx::new(
        client.device.clone(),
        client.cmds.clone(),
        client.event_bus.clone(),
        token,
    )
}

pub fn start_device_invitation_greet(client: &Client, token: AccessToken) -> DeviceGreetInitialCtx {
    DeviceGreetInitialCtx::new(
        client.device.clone(),
        client.cmds.clone(),
        client.event_bus.clone(),
        token,
    )
}

#[derive(Debug, thiserror::Error)]
pub enum ClientStartShamirRecoveryInvitationGreetError {
    #[error("Invitation not found")]
    InvitationNotFound,
    #[error("Shamir recovery not found, no shamir certificates found")]
    ShamirRecoveryNotFound,
    #[error("Shamir recovery has been deleted")]
    ShamirRecoveryDeleted,
    #[error("Shamir recovery is unusable due to revoked recipients")]
    ShamirRecoveryUnusable,
    #[error(transparent)]
    CorruptedShareData(DataError),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ListInvitationsError> for ClientStartShamirRecoveryInvitationGreetError {
    fn from(value: ListInvitationsError) -> Self {
        match value {
            ListInvitationsError::Offline(e) => {
                ClientStartShamirRecoveryInvitationGreetError::Offline(e)
            }
            ListInvitationsError::Internal(e) => {
                ClientStartShamirRecoveryInvitationGreetError::Internal(e)
            }
        }
    }
}

impl From<CertifGetShamirRecoveryShareDataError> for ClientStartShamirRecoveryInvitationGreetError {
    fn from(value: CertifGetShamirRecoveryShareDataError) -> Self {
        match value {
            CertifGetShamirRecoveryShareDataError::ShamirRecoveryBriefCertificateNotFound => {
                ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryNotFound
            }
            CertifGetShamirRecoveryShareDataError::ShamirRecoveryShareCertificateNotFound => {
                ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryNotFound
            }
            CertifGetShamirRecoveryShareDataError::ShamirRecoveryDeleted => {
                ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryDeleted
            }
            CertifGetShamirRecoveryShareDataError::ShamirRecoveryUnusable => {
                ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryUnusable
            }
            CertifGetShamirRecoveryShareDataError::CorruptedShareData(e) => {
                ClientStartShamirRecoveryInvitationGreetError::CorruptedShareData(e)
            }
            CertifGetShamirRecoveryShareDataError::Offline(e) => {
                ClientStartShamirRecoveryInvitationGreetError::Offline(e)
            }
            CertifGetShamirRecoveryShareDataError::Stopped => {
                ClientStartShamirRecoveryInvitationGreetError::Stopped
            }
            CertifGetShamirRecoveryShareDataError::InvalidCertificate(e) => {
                ClientStartShamirRecoveryInvitationGreetError::InvalidCertificate(e)
            }
            CertifGetShamirRecoveryShareDataError::InvalidRequirements => {
                // This shouldn't occur since the requirements timestamp is provided by the server along with the invitation list
                // (and the server is expected to only provide us with valid requirements !).
                ClientStartShamirRecoveryInvitationGreetError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifGetShamirRecoveryShareDataError::Internal(e) => {
                ClientStartShamirRecoveryInvitationGreetError::Internal(e)
            }
        }
    }
}

pub async fn start_shamir_recovery_invitation_greet(
    client: &Client,
    token: AccessToken,
) -> Result<ShamirRecoveryGreetInitialCtx, ClientStartShamirRecoveryInvitationGreetError> {
    // Retrieve the invitation
    let invitations = list_invitations(&client.cmds).await?;
    let (claimer_user_id, shamir_recovery_created_on) = invitations
        .into_iter()
        .find_map(|i| match i {
            InviteListItem::ShamirRecovery {
                token: t,
                claimer_user_id,
                shamir_recovery_created_on,
                ..
            } if token == t => Some((claimer_user_id, shamir_recovery_created_on)),
            _ => None,
        })
        .ok_or(ClientStartShamirRecoveryInvitationGreetError::InvitationNotFound)?;

    // Retrieve the share data
    let share_data = client
        .certificates_ops
        .get_shamir_recovery_share_data(claimer_user_id, shamir_recovery_created_on)
        .await?;

    // Return the context
    Ok(ShamirRecoveryGreetInitialCtx::new(
        client.device.clone(),
        client.cmds.clone(),
        client.event_bus.clone(),
        token,
        share_data,
    ))
}
