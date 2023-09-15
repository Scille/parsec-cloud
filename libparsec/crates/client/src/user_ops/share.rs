// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::UserOps;
use crate::EventTooMuchDriftWithServerClock;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceShareError {
    #[error("Cannot share with oneself")]
    ShareToSelf,
    #[error("Unknown workspace")]
    UnknownWorkspace,
    #[error("Unknown recipient")]
    UnknownRecipient,
    #[error("Unknown recipient or workspace")]
    // TODO: remove me and replace by UnknownRecipient/UnknownWorkspace
    UnknownRecipientOrWorkspace,
    #[error("Recipient user exists, but is revoked")]
    RevokedRecipient,
    #[error("Workspace is under maintenance, cannot share it for the moment")]
    WorkspaceInMaintenance,
    #[error("You don't have the right to share workspace")]
    NotAllowed,
    #[error("As a user with profile OUTSIDER, recipient cannot be granted the role MANAGER/OWNER in a workspace")]
    OutsiderCannotBeManagerOrOwner,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceShareError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

enum WorkspaceShareDoServerCommandOutcome {
    Done,
    RequireGreaterTimestamp(DateTime),
}

pub async fn workspace_share(
    ops: &UserOps,
    realm_id: VlobID,
    recipient: &UserID,
    role: Option<RealmRole>,
) -> Result<(), WorkspaceShareError> {
    if ops.device.device_id.user_id() == recipient {
        return Err(WorkspaceShareError::ShareToSelf);
    }

    let um = ops.storage.get_user_manifest();

    let workspace_entry = um
        .get_workspace_entry(realm_id)
        .ok_or_else(|| WorkspaceShareError::UnknownWorkspace)?;

    // Make sure the workspace is not a placeholder
    // TODO !
    // await ops._workspace_minimal_sync(workspace_entry)

    // Workspace sharing involves multiple checks:
    // - recipient should not be revoked
    // - OWNER/MANAGER role cannot be given to an OUTSIDER recipient
    // - The current user role determine which role (or even if !) he can
    //   give to the recipient
    //
    // In theory we should not trust the server on those checks, hand hence we should:
    // 1) do the checks in local
    // 2) if ok, then send the command to the server
    // 3) if the server returned us an error, load the newer certificates from the
    //    server and re-do the checks in local.
    //
    // However this is cumbersome, so here instead we rely entirely on the server for
    // the checks.
    // Of course this also gives room for the server to lie to us (e.g. it can pretend
    // the recipient has been revoked).
    // This is considered ok-enough given the only thing the server can do here is to
    // prevent us from sharing a workspace (it cannot trick us to alter the realm role
    // certificate), which it can also achieve by just pretending to be not available.

    let mut timestamp = ops.device.time_provider.now();
    loop {
        match workspace_share_do_server_command(ops, workspace_entry, recipient, role, timestamp)
            .await?
        {
            WorkspaceShareDoServerCommandOutcome::Done => break,
            WorkspaceShareDoServerCommandOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ) => {
                timestamp = std::cmp::max(ops.device.time_provider.now(), strictly_greater_than);
            }
        }
    }

    Ok(())
}

async fn workspace_share_do_server_command(
    ops: &UserOps,
    workspace_entry: &WorkspaceEntry,
    recipient: &UserID,
    role: Option<RealmRole>,
    timestamp: DateTime,
) -> Result<WorkspaceShareDoServerCommandOutcome, WorkspaceShareError> {
    // 1) Build the sharing message for recipient

    let encrypted_message = {
        let signed_message = if role.is_some() {
            MessageContent::SharingGranted {
                author: ops.device.device_id.clone(),
                timestamp,
                name: workspace_entry.name.clone(),
                id: workspace_entry.id,
                encryption_revision: workspace_entry.encryption_revision,
                encrypted_on: workspace_entry.encrypted_on,
                key: workspace_entry.key.clone(),
            }
        } else {
            MessageContent::SharingRevoked {
                author: ops.device.device_id.clone(),
                timestamp,
                id: workspace_entry.id,
            }
        }
        .dump_and_sign(&ops.device.signing_key);

        match ops
            .certificates_ops
            .encrypt_for_user(recipient, &signed_message)
            .await?
        {
            None => return Err(WorkspaceShareError::UnknownRecipient),
            Some(encrypted) => encrypted,
        }
    };

    // 2) Build role certificate

    let signed_certificate = RealmRoleCertificate {
        author: CertificateSignerOwned::User(ops.device.device_id.clone()),
        timestamp,
        realm_id: workspace_entry.id,
        user_id: recipient.to_owned(),
        role,
    }
    .dump_and_sign(&ops.device.signing_key);

    // 3) Actually send the command to the server

    use authenticated_cmds::latest::realm_update_roles::{Rep, Req};

    let req = Req {
        recipient_message: Some(encrypted_message.into()),
        role_certificate: signed_certificate.into(),
    };
    let rep = ops.cmds.send(req).await?;
    match rep {
        // Handle already granted in an idempotent way
        Rep::Ok | Rep::AlreadyGranted => Ok(WorkspaceShareDoServerCommandOutcome::Done),
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // TODO: handle `strictly_greater_than` out of the client ballpark by
            // returning an error
            // The retry is handled by the caller
            Ok(
                WorkspaceShareDoServerCommandOutcome::RequireGreaterTimestamp(
                    strictly_greater_than,
                ),
            )
        }
        Rep::InMaintenance => Err(WorkspaceShareError::WorkspaceInMaintenance),
        Rep::NotAllowed { .. } => Err(WorkspaceShareError::NotAllowed),
        Rep::IncompatibleProfile { .. } => Err(WorkspaceShareError::OutsiderCannotBeManagerOrOwner),
        Rep::UserRevoked => Err(WorkspaceShareError::RevokedRecipient),
        Rep::NotFound { .. } => {
            // TODO: replace this generic `not_found` status by something more precise
            // to know what is not found !

            // This should never occur given we have already retrieve the user on our side,
            // and we have made sure the workspace exists in the server.
            Err(WorkspaceShareError::UnknownRecipientOrWorkspace)
        }
        Rep::BadTimestamp {
            backend_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let event = EventTooMuchDriftWithServerClock {
                backend_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            ops.event_bus.send(&event);

            Err(WorkspaceShareError::BadTimestamp {
                server_timestamp: backend_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        bad_rep @ (Rep::InvalidCertification { .. }
        | Rep::InvalidData { .. }
        | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
