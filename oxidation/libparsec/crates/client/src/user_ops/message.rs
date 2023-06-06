// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::merge::merge_workspace_entry;
use crate::certificates_ops::ValidateMessageError;
use crate::event_bus::EventPing;

#[derive(Debug, thiserror::Error)]
pub enum ProcessLastMessagesError {
    #[error("Cannot reach the server")]
    Offline,
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

impl super::UserOps {
    pub async fn process_last_messages(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<(), ProcessLastMessagesError> {
        // Concurrent message processing is totally pointless
        let _guard = self.process_messages_lock.lock().await;

        let initial_last_index = {
            let user_manifest = self.storage.get_user_manifest();
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
        let rep = self.cmds.send(req).await?;
        let messages = match rep {
            Rep::Ok { messages } => messages,
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        };

        // Process each new message

        let mut new_last_index = initial_last_index;
        for msg in messages {
            let content = match self
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
                    log::warn!(
                        "Ignoring invalid user message #{} from {}: {}",
                        msg.index,
                        msg.sender,
                        err
                    );
                    continue;
                }
                Err(
                    err @ (ValidateMessageError::Internal(_)
                    | ValidateMessageError::PollServerError(_)),
                ) => return Err(anyhow::anyhow!(err).into()),
                Err(ValidateMessageError::Offline) => {
                    return Err(ProcessLastMessagesError::Offline)
                }
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
                    self.process_message_sharing_granted(
                        name,
                        id,
                        encryption_revision,
                        encrypted_on,
                        key,
                    )
                    .await?;
                }
                MessageContent::SharingRevoked { .. } => {
                    // We used to have to update user manifest's `WorkspaceEntry.role/role_cached_on`
                    // fields. However this is no longer needed given certificates are now eagerly
                    // fetched by the client.
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
                    self.event_bus.send(&event);
                }
            }

            // In theory we could just take the index of the last message, but better be
            // extra safe by doing it this way
            new_last_index = std::cmp::max(new_last_index, msg.index);
        }

        // Finally, update message offset in user manifest

        // Up to this point, the user manifest could have been concurrently modified
        // from when we retreived `initial_last_index` (typically because we processed
        // sharing revoked messages)
        let _update_user_manifest_guard = self.update_user_manifest_lock.lock().await;
        let mut user_manifest = self.storage.get_user_manifest();

        if user_manifest.last_processed_message < new_last_index {
            // `Arc::make_mut` clones user manifest before we modify it
            Arc::make_mut(&mut user_manifest).evolve_last_processed_message_and_mark_updated(
                new_last_index,
                self.device.time_provider.now(),
            );
            self.storage.set_user_manifest(user_manifest).await?;
            // TODO: event
            // self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)
        }

        Ok(())
    }

    async fn process_message_sharing_granted(
        &self,
        name: EntryName,
        id: RealmID,
        encryption_revision: IndexInt,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> Result<(), ProcessLastMessagesError> {
        let _update_user_manifest_guard = self.update_user_manifest_lock.lock().await;

        // Given we have taken `update_user_manifest_lock` we are guaranteed the user manifest
        // cannot change in our back
        let mut user_manifest = self.storage.get_user_manifest();

        let timestamp = self.device.time_provider.now();
        let workspace_entry = WorkspaceEntry {
            id: EntryID::from(*id),
            name,
            key,
            encryption_revision,
            encrypted_on,
            // As they name suggest, `role`/`role_cached_on` are only cache information.
            // However they are no longer needed given certificates are now eagerly
            // fetched by the client.
            // We still have to provide them for compatibility reason. So we choose
            // an always true value: at epoch 0 Parsec didn't exist, hence the user
            // couldn't have access to this workspace !
            role_cached_on: DateTime::from_f64_with_us_precision(0.0),
            role: None,
        };

        // Check if we already know this workspace
        let workspace_entry = match user_manifest.get_workspace_entry(&workspace_entry.id) {
            None => workspace_entry,
            Some(already_existing_entry) => {
                // Merge with existing as target to keep possible workspace rename
                merge_workspace_entry(None, &workspace_entry, already_existing_entry)
            }
        };
        // `Arc::make_mut` clones user manifest before we modify it
        Arc::make_mut(&mut user_manifest)
            .evolve_workspaces_and_mark_updated(workspace_entry, timestamp);

        self.storage.set_user_manifest(user_manifest).await?;
        // TODO: events
        // self.event_bus.send(CoreEvent.USERFS_UPDATED)
        //
        // if not already_existing_entry:
        //     # TODO: remove this event ?
        //     self.event_bus.send(CoreEvent.FS_ENTRY_SYNCED, id=workspace_entry.id)
        //
        // self.event_bus.send(
        //     CoreEvent.SHARING_UPDATED,
        //     new_entry=workspace_entry,
        //     previous_entry=already_existing_entry,
        // )
        Ok(())
    }
}
