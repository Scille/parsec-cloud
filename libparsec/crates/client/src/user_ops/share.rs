// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use crate::{
    certificates_ops::{CertificatesOps, User},
    event_bus::{EventBus, EventTooMuchDriftWithServerClock},
};

#[derive(Debug, thiserror::Error)]
pub enum UserOpsWorkspaceShareError {
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
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for UserOpsWorkspaceShareError {
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

impl super::UserOps {
    pub async fn workspace_share(
        &self,
        workspace_id: &EntryID,
        recipient: &UserID,
        role: Option<RealmRole>,
    ) -> Result<(), UserOpsWorkspaceShareError> {
        if self.device.device_id.user_id() == recipient {
            return Err(UserOpsWorkspaceShareError::ShareToSelf);
        }

        let um = self.storage.get_user_manifest();

        let workspace_entry = um
            .get_workspace_entry(workspace_id)
            .ok_or_else(|| UserOpsWorkspaceShareError::UnknownWorkspace)?;

        // Make sure the workspace is not a placeholder
        // TODO !
        // await self._workspace_minimal_sync(workspace_entry)

        // Retrieve the user

        let recipient = match self.certificates_ops.get_user(recipient).await? {
            None => return Err(UserOpsWorkspaceShareError::UnknownRecipient),
            Some(user) => user,
        };

        // Workspace sharing involves multiple checks:
        // - recipient should not be revoked
        // - OWNER/MANAGER role cannot be given to an OUTSIDER recipient
        // - The current user role determine which (or even if !) role he can
        //   given to the recipient
        //
        // In theory we should not trust the server on those checks, hand hence we should:
        // 1) do the checks in local
        // 2) if ok, then send the command to the server
        // 3) if the server returned us an error, load the newers certificates from the
        //    server and re-do the checks in local.
        //
        // However this is cumbersome, so here instead we rely entirely on the server for
        // the checks.
        // Of course this also gives room for the server to lie to us (e.g. it can pretend
        // the recipient has been revoked).
        // This is considered ok-enough given the only thing the server can do here is to
        // prevent us from sharing a workspace (it cannot trick us to alter the realm role
        // certificate), which it can also achieve by just pretending to be not available.

        let mut timestamp = self.device.time_provider.now();
        loop {
            match self
                .workspace_share_do_server_command(workspace_entry, &recipient, role, timestamp)
                .await?
            {
                WorkspaceShareDoServerCommandOutcome::Done => break,
                WorkspaceShareDoServerCommandOutcome::RequireGreaterTimestamp(
                    strictly_greater_than,
                ) => {
                    timestamp =
                        std::cmp::max(self.device.time_provider.now(), strictly_greater_than);
                }
            }
        }

        Ok(())
    }

    async fn workspace_share_do_server_command(
        &self,
        workspace_entry: &WorkspaceEntry,
        recipient: &User,
        role: Option<RealmRole>,
        timestamp: DateTime,
    ) -> Result<WorkspaceShareDoServerCommandOutcome, UserOpsWorkspaceShareError> {
        // Build the sharing message

        let recipient_message = if role.is_some() {
            MessageContent::SharingGranted {
                author: self.device.device_id.clone(),
                timestamp: timestamp.clone(),
                name: workspace_entry.name.clone(),
                id: workspace_entry.id.clone(),
                encryption_revision: workspace_entry.encryption_revision,
                encrypted_on: workspace_entry.encrypted_on.clone(),
                key: workspace_entry.key.clone(),
            }
        } else {
            MessageContent::SharingRevoked {
                author: self.device.device_id.clone(),
                timestamp: timestamp.clone(),
                id: workspace_entry.id.clone(),
            }
        };

        let dumped_recipient_message = recipient_message
            .dump_sign_and_encrypt_for(&self.device.signing_key, recipient.pubkey());

        // Build role certificate

        let role_certificate = RealmRoleCertificate {
            author: CertificateSignerOwned::User(self.device.device_id.clone()),
            timestamp,
            realm_id: RealmID::from(workspace_entry.id.as_ref().clone()),
            user_id: recipient.user_id().to_owned(),
            role,
        };
        let dumped_role_certificate = role_certificate.dump_and_sign(&self.device.signing_key);

        // Actually send the command to the backend

        use authenticated_cmds::latest::realm_update_roles::{Rep, Req};

        let req = Req {
            recipient_message: Some(dumped_recipient_message.into()),
            role_certificate: dumped_role_certificate.into(),
        };
        let rep = self.cmds.send(req).await?;
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
            Rep::InMaintenance => Err(UserOpsWorkspaceShareError::WorkspaceInMaintenance),
            Rep::NotAllowed { .. } => Err(UserOpsWorkspaceShareError::NotAllowed),
            ref bad_rep @ Rep::BadTimestamp {
                backend_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
                ..
            } => {
                let err = anyhow::anyhow!("Unexpected server response: {:?}", bad_rep);
                let event = EventTooMuchDriftWithServerClock {
                    backend_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                };
                self.event_bus.send(&event);
                Err(err.into())
            }
            Rep::IncompatibleProfile { .. } => {
                Err(UserOpsWorkspaceShareError::OutsiderCannotBeManagerOrOwner)
            }
            Rep::UserRevoked => Err(UserOpsWorkspaceShareError::RevokedRecipient),
            Rep::NotFound { .. } => {
                // TODO: replace this generic `not_found` status by something more precise
                // to know what is not found !

                // This should never occur given we have already retrieve the user on our side,
                // and we have made sure the workspace exists in the server.
                Err(UserOpsWorkspaceShareError::UnknownRecipientOrWorkspace)
            }
            bad_rep @ (Rep::InvalidCertification { .. }
            | Rep::InvalidData { .. }
            | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    }
}
