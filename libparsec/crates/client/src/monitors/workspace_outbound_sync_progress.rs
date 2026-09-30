// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    sync::Arc,
};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web, select2_biased};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    EventWorkspaceOutboundSyncBacklog,
    event_bus::{
        EventBus, EventWorkspaceOpsOutboundSyncDone, EventWorkspaceOpsOutboundSyncNeeded,
        EventWorkspaceOpsOutboundSyncProgress,
    },
    workspace::{
        PathConfinementPoint, RetrievePathFromIDEntry, RetrievePathFromIDError,
        WorkspaceGetNeedOutboundSyncEntriesError, WorkspaceOps,
    },
};

const WORKSPACE_OUTBOUND_SYNC_PROGRESS_MONITOR_NAME: &str = "workspace_outbound_sync_progress";

const BACKLOG_TRACKER_WAIT: Duration = cfg_select! {
    // 10 times faster than production
    test => Duration::milliseconds(100),
    _ => Duration::seconds(1)
};

#[derive(Debug)]
struct PendingFile {
    /// Actual size of the file, as provided by its manifest
    size: u64,
    /// Bytes already uploaded, as provided by `EventWorkspaceOpsOutboundSyncProgress`
    uploaded: u64,
}

impl PendingFile {
    fn remaining(&self) -> u64 {
        // Saturating in case the file has been shrunk since the upload started
        self.size.saturating_sub(self.uploaded)
    }
}

#[derive(Debug)]
struct ProgressTracker {
    /// Files left to sync (confined files and folders are not tracked)
    files_to_sync: HashMap<VlobID, PendingFile>,
    /// Set when the tracker may be out of sync with the local storage
    reconcile_needed: bool,
}

impl ProgressTracker {
    fn new() -> Self {
        Self {
            files_to_sync: HashMap::new(),
            // Entries modified before the monitor started don't produce any event,
            // so the tracker must first be populated from the local storage
            reconcile_needed: true,
        }
    }

    /// Track the file with its new size, keeping its upload progress if it was
    /// already tracked.
    fn track(&mut self, entry_id: VlobID, size: u64) {
        self.files_to_sync
            .entry(entry_id)
            .and_modify(|file| file.size = size)
            .or_insert(PendingFile { size, uploaded: 0 });
    }

    /// Bring the tracker in line with what is left to sync in the local storage.
    ///
    /// This catches the changes we didn't think about and makes sure that our
    /// progress stays synchronized.
    async fn reconcile(
        &mut self,
        ops: &WorkspaceOps,
    ) -> Result<(), WorkspaceGetNeedOutboundSyncEntriesError> {
        self.reconcile_needed = false;

        let need_sync = ops.get_need_outbound_sync(u32::MAX).await?;

        for entry_id in need_sync.iter().copied() {
            match stat_pending_file(ops, entry_id).await {
                PendingFileStat::Track { size } => self.track(entry_id, size),
                PendingFileStat::Untrack => {
                    self.files_to_sync.remove(&entry_id);
                }
                PendingFileStat::Unknown => (),
                PendingFileStat::Stopped => {
                    return Err(WorkspaceGetNeedOutboundSyncEntriesError::Stopped);
                }
            }
        }

        // Entries no longer needing sync have been synced in the meantime
        let need_sync: HashSet<_> = need_sync.into_iter().collect();
        self.files_to_sync
            .retain(|entry_id, _| need_sync.contains(entry_id));

        Ok(())
    }
}

enum PendingFileStat {
    /// Non-confined file, it should be counted in the backlog
    Track {
        size: u64,
    },
    /// Folder, confined entry or entry that is no longer part of the workspace
    Untrack,
    /// The stat failed (and the error has been logged), we cannot tell
    Unknown,
    Stopped,
}

async fn stat_pending_file(ops: &WorkspaceOps, entry_id: VlobID) -> PendingFileStat {
    // Note we don't use `stat_entry_by_id` here since it considers unreachable entries
    // as regular ones. However those are typically removed entries: they stay marked as
    // need sync but are never synchronized, so they should not be counted.
    match ops.retrieve_path_from_id(entry_id).await {
        Ok(RetrievePathFromIDEntry::Reachable {
            manifest: ArcLocalChildManifest::File(manifest),
            confinement_point: PathConfinementPoint::NotConfined,
            ..
        }) => PendingFileStat::Track {
            size: manifest.size,
        },
        // Folder, confined, removed or not in local storage
        Ok(
            RetrievePathFromIDEntry::Reachable { .. }
            | RetrievePathFromIDEntry::Unreachable { .. }
            | RetrievePathFromIDEntry::Missing,
        ) => PendingFileStat::Untrack,
        Err(RetrievePathFromIDError::Stopped) => PendingFileStat::Stopped,
        Err(err) => {
            log::warn!(
                "Workspace {}: cannot stat {entry_id} to track its outbound sync: {err:?}",
                ops.realm_id()
            );
            PendingFileStat::Unknown
        }
    }
}

pub(crate) async fn start_workspace_outbound_sync_progress_monitor(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    device: Arc<LocalDevice>,
) -> Monitor {
    let realm_id = workspace_ops.realm_id();

    let task_future = task_future_factory(workspace_ops, event_bus.clone(), device);
    let task_future = pretend_future_is_send_on_web(task_future);
    Monitor::start(
        event_bus,
        WORKSPACE_OUTBOUND_SYNC_PROGRESS_MONITOR_NAME,
        Some(realm_id),
        task_future,
        None,
    )
    .await
}

#[derive(Debug)]
enum IncomingOutboundSyncEvent {
    Needed {
        entry_id: VlobID,
    },
    Progress {
        entry_id: VlobID,
        block_index: IndexInt,
        blocksize: SizeInt,
    },
    Done {
        entry_id: VlobID,
    },
}

fn task_future_factory(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    device: Arc<LocalDevice>,
) -> impl Future<Output = ()> {
    let realm_id = workspace_ops.realm_id();
    let (tx, rx) = channel::unbounded::<IncomingOutboundSyncEvent>();

    let outbound_sync_needed_events_connection_lifetime = {
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Needed {
                    entry_id: e.entry_id,
                });
            }
        })
    };

    let outbound_sync_progress_events_connection_lifetime = {
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncProgress| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Progress {
                    entry_id: e.entry_id,
                    block_index: e.block_index,
                    blocksize: e.blocksize,
                });
            }
        })
    };

    // Note `EventWorkspaceOpsOutboundSyncAborted` is not listened to: it is sent whenever
    // the outbound sync could not be completed (entry busy, inbound sync needed first,
    // offline etc.), but the file will be retried later so it is still left to sync.

    let outbound_sync_done_events_connection_lifetime = {
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncDone| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Done {
                    entry_id: e.entry_id,
                });
            }
        })
    };

    async move {
        let mut progress_tracker = ProgressTracker::new();
        let _outbound_sync_needed_events_connection_lifetime =
            outbound_sync_needed_events_connection_lifetime;
        let _outbound_sync_progress_events_connection_lifetime =
            outbound_sync_progress_events_connection_lifetime;
        let _outbound_sync_done_events_connection_lifetime =
            outbound_sync_done_events_connection_lifetime;
        // Start with an immediate send so the initial state gets reported.
        let mut due_time = Some(device.time_provider.now());
        loop {
            let to_sleep = match due_time {
                None => Duration::MAX,
                Some(due_time) => due_time - device.time_provider.now(),
            };

            select2_biased!(
                _ = device.time_provider.sleep(to_sleep) => {
                    if progress_tracker.reconcile_needed {
                        match progress_tracker.reconcile(&workspace_ops).await {
                            Ok(()) => (),
                            Err(WorkspaceGetNeedOutboundSyncEntriesError::Stopped) => return,
                            Err(WorkspaceGetNeedOutboundSyncEntriesError::Internal(err)) => {
                                log::warn!("Workspace {realm_id}: cannot reconcile outbound sync progress: {err:?}");
                            }
                        }
                    }

                    // Aggregate currently known data and send it
                    let (number_of_files_to_sync, size_to_sync) = progress_tracker
                        .files_to_sync
                        .values()
                        .fold((0, 0), |(acc_files, acc_bytes), file| (acc_files + 1, acc_bytes + file.remaining()));

                    event_bus.send(&EventWorkspaceOutboundSyncBacklog {
                        number_of_files_to_sync,
                        size_to_sync,
                        realm_id,
                    });

                    due_time = None;
                },
                event = rx.recv_async() => {
                    let Ok(event) = event else {
                        return;
                    };

                    // Schedule a send if none is pending. If one is already pending, keep
                    // its due time, otherwise a steady stream of events would postpone it forever.
                    if due_time.is_none() {
                        due_time = Some(device.time_provider.now() + BACKLOG_TRACKER_WAIT);
                    }

                    match event {
                        IncomingOutboundSyncEvent::Needed { entry_id } => {
                            match stat_pending_file(&workspace_ops, entry_id).await {
                                PendingFileStat::Track { size } => progress_tracker.track(entry_id, size),
                                PendingFileStat::Untrack => {
                                    // The entry may have been tracked before (e.g. renamed into
                                    // a confined name), so make sure it is no longer counted
                                    progress_tracker.files_to_sync.remove(&entry_id);
                                    // This is typically a folder whose children changed, which
                                    // may have made some tracked files unreachable (e.g. removed)
                                    progress_tracker.reconcile_needed = true;
                                }
                                PendingFileStat::Unknown => (),
                                PendingFileStat::Stopped => return,
                            }
                        }
                        IncomingOutboundSyncEvent::Progress { entry_id, block_index, blocksize } => {
                            // The event is sent right before uploading the block, so all the
                            // blocks before it are uploaded. Blocks all have the same size
                            // (except the last one).
                            let uploaded = block_index * blocksize;

                            if let Some(file) = progress_tracker.files_to_sync.get_mut(&entry_id) {
                                file.uploaded = uploaded;
                            } else {
                                // Not tracked yet (e.g. the stat failed earlier), but if it is
                                // being uploaded it is definitely left to sync
                                match stat_pending_file(&workspace_ops, entry_id).await {
                                    PendingFileStat::Track { size } => {
                                        progress_tracker
                                            .files_to_sync
                                            .insert(entry_id, PendingFile { size, uploaded });
                                    }
                                    PendingFileStat::Untrack | PendingFileStat::Unknown => (),
                                    PendingFileStat::Stopped => return,
                                }
                            }
                        }
                        IncomingOutboundSyncEvent::Done { entry_id } => {
                            progress_tracker.files_to_sync.remove(&entry_id);
                        }
                    }
                },
            )
        }
    }
}
