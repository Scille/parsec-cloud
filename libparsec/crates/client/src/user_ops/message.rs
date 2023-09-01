// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{merge::merge_workspace_entry, UserOps};
use crate::certificates_ops::{InvalidCertificateError, ValidateMessageError};
use crate::event_bus::EventPing;

#[derive(Debug, thiserror::Error)]
pub enum ProcessLastMessagesError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Cannot process user messages: {0}")]
    InvalidCertificate(InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for ProcessLastMessagesError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(super) async fn process_last_messages(
    ops: &UserOps,
    latest_known_index: Option<IndexInt>,
) -> Result<(), ProcessLastMessagesError> {
    // Concurrent message processing is totally pointless
    let _guard = ops.process_messages_lock.lock().await;

    let initial_last_index = {
        let user_manifest = ops.storage.get_user_manifest();
        user_manifest.last_processed_message
    };

    // `latest_known_index` is useful to detect outdated `MessageReceived`
    // events given the server has already been polled in the meantime.
    if let Some(latest_known_index) = latest_known_index {
        if initial_last_index >= latest_known_index {
            return Ok(());
        }
    }

    // Ask server for the new messages

    use authenticated_cmds::latest::message_get::{Rep, Req};

    // Message index starts at 1, so can be used as-is as offset
    let req = Req {
        offset: initial_last_index,
    };
    let rep = ops.cmds.send(req).await?;
    let messages = match rep {
        Rep::Ok { messages } => messages,
        bad_rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
        }
    };

    // Process each new message

    let mut new_last_index = initial_last_index;
    for msg in messages {
        let content = match ops
            .certificates_ops
            .validate_message(
                msg.certificate_index,
                msg.index,
                &msg.sender,
                msg.timestamp,
                &msg.body,
            )
            .await
        {
            Ok(content) => content,
            // If the message is invalid, we have no choice but to ignore it.
            // Note CertificateOps is already responsible to dispatch an event
            // in order to inform the user about this issue.
            Err(ValidateMessageError::InvalidMessage(err)) => {
                // TODO: is warning needed here ? or should we only rely on the event
                // triggered by CertificatesOps's validation ?
                log::warn!(
                    "Ignoring invalid user message #{} from {}: {}",
                    msg.index,
                    msg.sender,
                    err
                );
                continue;
            }
            // We couldn't validate the message due to an invalid certificate provided
            // by the server. Hence there is nothing more we can do :(
            Err(ValidateMessageError::InvalidCertificate(err)) => {
                return Err(ProcessLastMessagesError::InvalidCertificate(err));
            }
            Err(err @ ValidateMessageError::Internal(_)) => {
                return Err(ProcessLastMessagesError::Internal(err.into()))
            }
            Err(ValidateMessageError::Offline) => return Err(ProcessLastMessagesError::Offline),
        };

        // CertificateOps is responsible for ensuring the message is valid and
        // consistent with the rest of the system.
        // So from now on we can 100% trust the content of the message !

        match content {
            MessageContent::SharingGranted {
                name,
                id,
                encryption_revision,
                encrypted_on,
                key,
                ..
            }
            | MessageContent::SharingReencrypted {
                name,
                id,
                encryption_revision,
                encrypted_on,
                key,
                ..
            } => {
                process_message_sharing_granted(
                    ops,
                    name,
                    id,
                    encryption_revision,
                    encrypted_on,
                    key,
                )
                .await?;
            }
            MessageContent::SharingRevoked { .. } => {
                // We used to have to update user manifest's `WorkspaceEntry`'s
                // `role`/`role_cached_on` fields. However this is no longer needed
                // given certificates are now eagerly fetched by the client.
                //
                // The corollary is `MessageSharingRevoked` is now useless given we are
                // notified of the role revocation when we receive the corresponding
                // realm role certificate (but this message type should still be provided
                // for compatibility with Parsec < 3)

                // TODO: this comment is invalid if, in the end, we choose to have
                // `certificate_get` only providing realm role certifs the user is part of
            }
            MessageContent::Ping { ping, .. } => {
                let event = EventPing { ping };
                ops.event_bus.send(&event);
            }
        }

        // In theory we could just take the index of the last message, but better be
        // extra safe by doing it this way
        new_last_index = std::cmp::max(new_last_index, msg.index);
    }

    // Finally, update message offset in user manifest

    // Up to this point, the user manifest could have been concurrently modified
    // from when we retrieved `initial_last_index` (typically because we processed
    // sharing revoked messages)
    let (updater, mut user_manifest) = ops.storage.for_update().await;

    if user_manifest.last_processed_message < new_last_index {
        // `Arc::make_mut` clones user manifest before we modify it
        Arc::make_mut(&mut user_manifest).evolve_last_processed_message_and_mark_updated(
            new_last_index,
            ops.device.time_provider.now(),
        );
        updater.set_user_manifest(user_manifest).await?;
        // TODO: event
        // ops.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=ops.user_manifest_id)
    }

    Ok(())
}

async fn process_message_sharing_granted(
    ops: &UserOps,
    name: EntryName,
    id: RealmID,
    encryption_revision: IndexInt,
    encrypted_on: DateTime,
    key: SecretKey,
) -> Result<(), ProcessLastMessagesError> {
    let (updater, mut user_manifest) = ops.storage.for_update().await;

    let timestamp = ops.device.time_provider.now();
    let workspace_entry = WorkspaceEntry {
        id,
        name,
        key,
        encryption_revision,
        encrypted_on,
        // For backward compatibility we still have to provide the legacy cache on role
        // even if we don't use it anymore.
        // The good news is it's easy to provide an always valid value: at epoch 0
        // Parsec didn't exist, hence the user couldn't have access to this workspace !
        legacy_role_cache_timestamp: DateTime::from_f64_with_us_precision(0.0),
        legacy_role_cache_value: None,
    };

    // Check if we already know this workspace
    let workspace_entry = match user_manifest.get_workspace_entry(workspace_entry.id) {
        None => workspace_entry,
        Some(already_existing_entry) => {
            // Merge with existing as target to keep possible workspace rename
            merge_workspace_entry(None, &workspace_entry, already_existing_entry)
        }
    };
    // `Arc::make_mut` clones user manifest before we modify it
    Arc::make_mut(&mut user_manifest)
        .evolve_workspaces_and_mark_updated(workspace_entry, timestamp);

    updater.set_user_manifest(user_manifest).await?;
    // TODO: events
    // ops.event_bus.send(CoreEvent.USERFS_UPDATED)
    //
    // if not already_existing_entry:
    //     # TODO: remove this event ?
    //     ops.event_bus.send(CoreEvent.FS_ENTRY_SYNCED, id=workspace_entry.id)
    //
    // ops.event_bus.send(
    //     CoreEvent.SHARING_UPDATED,
    //     new_entry=workspace_entry,
    //     previous_entry=already_existing_entry,
    // )
    Ok(())
}
