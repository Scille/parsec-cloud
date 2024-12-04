// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::{
    workspace::{store::ResolvePathError, WorkspaceOps},
    EventBusConnectionLifetime, EventWorkspaceOpsInboundSyncDone,
    EventWorkspaceOpsOutboundSyncNeeded, EventWorkspaceWatchedEntryChanged,
    InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};

pub(crate) struct EntryWatcher {
    pub id: u32,
    // Never accessed, but we need to keep it alive
    #[allow(dead_code)]
    pub lifetimes: (
        // Event triggered when a local change occurred
        EventBusConnectionLifetime<EventWorkspaceOpsOutboundSyncNeeded>,
        // Event triggered when a remote change occurred
        EventBusConnectionLifetime<EventWorkspaceOpsInboundSyncDone>,
    ),
}

impl std::fmt::Debug for EntryWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntryWatcher")
            .field("id", &self.id)
            .finish()
    }
}

#[derive(Default, Debug)]
pub(crate) struct EntryWatchers {
    pub last_id: u32,
    pub watchers: Vec<EntryWatcher>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceWatchEntryOneShotError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn watch_entry_oneshot(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<VlobID, WorkspaceWatchEntryOneShotError> {
    // 1) Resolve the path to get the entry id

    let entry_id = ops
        .store
        .resolve_path(path)
        .await
        .map(|(manifest, _)| manifest.id())
        .map_err(|err| match err {
            ResolvePathError::Offline => WorkspaceWatchEntryOneShotError::Offline,
            ResolvePathError::Stopped => WorkspaceWatchEntryOneShotError::Stopped,
            ResolvePathError::EntryNotFound => WorkspaceWatchEntryOneShotError::EntryNotFound,
            ResolvePathError::NoRealmAccess => WorkspaceWatchEntryOneShotError::NoRealmAccess,
            ResolvePathError::InvalidKeysBundle(err) => {
                WorkspaceWatchEntryOneShotError::InvalidKeysBundle(err)
            }
            ResolvePathError::InvalidCertificate(err) => {
                WorkspaceWatchEntryOneShotError::InvalidCertificate(err)
            }
            ResolvePathError::InvalidManifest(err) => {
                WorkspaceWatchEntryOneShotError::InvalidManifest(err)
            }
            ResolvePathError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    // 2) Connect the entry watcher to the event bus

    // Note step 1 and 2 are not atomic, hence a modification occurring between the two
    // steps will not be caught by the watcher. This should be okay though, as step 1
    // is only used to get the entry ID from the path, so we can just pretend the
    // modification occured just before step 1 (it would be different if we were
    // returning the stat obtained from step 1 to the caller).

    {
        let mut entry_watchers_guard = ops.entry_watchers.lock().expect("Mutex is poisoned");
        entry_watchers_guard.last_id += 1;
        let entry_watcher_id = entry_watchers_guard.last_id;

        let on_event_triggered = {
            let realm_id = ops.realm_id();
            let entry_watchers = ops.entry_watchers.clone();
            let event_bus = ops.event_bus.clone();

            move |candidate_realm_id: VlobID, candidate_entry_id: VlobID| {
                if candidate_realm_id != realm_id || candidate_entry_id != entry_id {
                    return;
                }

                // Disconnect the entry watcher since it is a one-shot

                {
                    let mut entry_watchers_guard =
                        entry_watchers.lock().expect("Mutex is poisoned");
                    let index = entry_watchers_guard
                        .watchers
                        .iter()
                        .position(|x| x.id == entry_watcher_id);
                    match index {
                        // The watcher is already disconnected, we should not have received the event !
                        None => return,
                        Some(index) => {
                            let watcher = entry_watchers_guard.watchers.swap_remove(index);
                            // The watcher object contains an event bus connection lifetime
                            // about the event type we are currently handling.
                            // Given each event type has its own dedicated lock, dropping
                            // the lifetime here would created a deadlock !
                            // So instead we spawn a task to do the drop later.
                            libparsec_platform_async::spawn(async move {
                                drop(watcher);
                            });
                        }
                    }
                }

                // Dispatch the watcher event

                let event = EventWorkspaceWatchedEntryChanged { realm_id, entry_id };
                // Sending a event from within an event handler :/
                // This is fine as long as the event currently handled and
                // the one being send are of a different type (as each event
                // type has it own dedicated lock).
                event_bus.send(&event);
            }
        };

        let on_local_change_lifetime = ops.event_bus.connect({
            let on_event_triggered = on_event_triggered.clone();
            move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
                on_event_triggered(e.realm_id, e.entry_id)
            }
        });

        let on_remote_change_lifetime = ops.event_bus.connect({
            move |e: &EventWorkspaceOpsInboundSyncDone| on_event_triggered(e.realm_id, e.entry_id)
        });

        entry_watchers_guard.watchers.push(EntryWatcher {
            id: entry_watcher_id,
            lifetimes: (on_local_change_lifetime, on_remote_change_lifetime),
        });
    }

    Ok(entry_id)
}
