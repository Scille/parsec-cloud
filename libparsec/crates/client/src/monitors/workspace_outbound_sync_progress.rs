// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::{collections::HashMap, future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web, select2_biased};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    EventWorkspaceOutboundSyncBacklog,
    event_bus::{
        EventBus, EventWorkspaceOpsOutboundSyncAborted, EventWorkspaceOpsOutboundSyncDone,
        EventWorkspaceOpsOutboundSyncNeeded, EventWorkspaceOpsOutboundSyncProgress,
    },
    workspace::{WorkspaceGetNeedOutboundSyncEntriesError, WorkspaceOps, WorkspaceStatEntryError},
};

const WORKSPACE_OUTBOUND_SYNC_PROGRESS_MONITOR_NAME: &str = "workspace_outbound_sync_progress";

// 10 times faster than production
#[cfg(test)]
const BACKLOG_TRACKER_WAIT: Duration = Duration::milliseconds(100);

#[cfg(not(test))]
const BACKLOG_TRACKER_WAIT: Duration = Duration::seconds(1);

#[derive(Debug)]
struct ProgressTracker {
    /// Keep track of how much is left to sync by file
    /// No confined file is tracked
    /// Size in bytes starts at the size provided in EntryStat
    /// and decreases accordingly to EventOutboundSyncProgress
    /// It has the precision of blocksize
    /// so it can be inaccurate when the last block is being synced
    files_to_sync: HashMap<VlobID, u64>,
    /// Currently one file is synced at a time
    /// So we can send that data to the GUI
    last_updated_file: Option<VlobID>,
    /// If the event that would be sent is the same
    /// as before we don't want to send it
    last_sent_event: EventWorkspaceOutboundSyncBacklog,
}

#[derive(Debug, thiserror::Error)]
enum WorkspaceOutboundSyncProgressMonitorInitError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    UnableToStatEntry(#[from] WorkspaceStatEntryError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<WorkspaceGetNeedOutboundSyncEntriesError>
    for WorkspaceOutboundSyncProgressMonitorInitError
{
    fn from(value: WorkspaceGetNeedOutboundSyncEntriesError) -> Self {
        match value {
            WorkspaceGetNeedOutboundSyncEntriesError::Stopped => {
                WorkspaceOutboundSyncProgressMonitorInitError::Stopped
            }
            WorkspaceGetNeedOutboundSyncEntriesError::Internal(e) => {
                WorkspaceOutboundSyncProgressMonitorInitError::Internal(e)
            }
        }
    }
}

impl ProgressTracker {
    fn new(realm_id: VlobID) -> Self {
        Self {
            files_to_sync: HashMap::new(),
            last_updated_file: None,
            last_sent_event: EventWorkspaceOutboundSyncBacklog {
                number_of_files_to_sync: 0,
                size_to_sync: 0,
                realm_id,
                entry_currently_being_synced: None,
            },
        }
    }

    async fn init(
        workspace_ops: Arc<WorkspaceOps>,
    ) -> Result<Self, WorkspaceOutboundSyncProgressMonitorInitError> {
        // what is left to be sync in DB at startup
        let mut files_to_sync = HashMap::new();
        for entry in workspace_ops.get_need_outbound_sync(u32::MAX).await? {
            let stat = workspace_ops.stat_entry_by_id(entry).await?;
            if !stat.is_confined() && stat.is_file() {
                files_to_sync.insert(entry, stat.size());
            }
        }

        Ok(Self {
            files_to_sync,
            ..Self::new(workspace_ops.realm_id())
        })
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
        blocks: IndexInt,
        block_index: IndexInt,
        blocksize: SizeInt,
    },
    Aborted {
        entry_id: VlobID,
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

    let outbound_sync_needed_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Needed {
                    entry_id: e.entry_id,
                });
            }
        })
    },);

    let outbound_sync_progress_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncProgress| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Progress {
                    entry_id: e.entry_id,
                    block_index: e.block_index,
                    blocks: e.blocks,
                    blocksize: e.blocksize,
                });
            }
        })
    },);

    let outbound_sync_aborted_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncAborted| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Aborted {
                    entry_id: e.entry_id,
                });
            }
        })
    },);

    let outbound_sync_done_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncDone| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingOutboundSyncEvent::Done {
                    entry_id: e.entry_id,
                });
            }
        })
    },);

    async move {
        //  init progress tracker with DB content
        let mut progress_tracker = match ProgressTracker::init(workspace_ops.clone()).await {
            Ok(p) => p,
            Err(e) => {
                log::info!(
                    "Unexpected error while initializing workspace outbound sync progress tracker {e:?}"
                );
                return;
            }
        };
        let _outbound_sync_needed_events_connection_lifetime =
            outbound_sync_needed_events_connection_lifetime;
        let _outbound_sync_progress_events_connection_lifetime =
            outbound_sync_progress_events_connection_lifetime;
        let _outbound_sync_aborted_events_connection_lifetime =
            outbound_sync_aborted_events_connection_lifetime;
        let _outbound_sync_done_events_connection_lifetime =
            outbound_sync_done_events_connection_lifetime;
        let mut due_time = device.time_provider.now() + BACKLOG_TRACKER_WAIT;
        loop {
            select2_biased!(
                res = device.time_provider.sleep_until(due_time) => {
                    // wake up and send progress
                    // aggregate currently known data

                    let (number_of_files_to_sync, size_to_sync) = progress_tracker
                        .files_to_sync
                        .iter()
                        .fold((0,0), |(acc_files, acc_bytes), (_,size)| (acc_files + 1, acc_bytes + size));

                    // check that it's not the same as previous event
                    if !(number_of_files_to_sync == progress_tracker.last_sent_event.number_of_files_to_sync
                        && size_to_sync == progress_tracker.last_sent_event.size_to_sync) {
                            // send event
                            let new_event = EventWorkspaceOutboundSyncBacklog {
                                number_of_files_to_sync,
                                size_to_sync,
                                realm_id,
                                entry_currently_being_synced: progress_tracker.last_updated_file
                            };
                            event_bus.send(dbg!(&new_event));
                            progress_tracker.last_sent_event = new_event;
                        }
                    due_time = device.time_provider.now()+BACKLOG_TRACKER_WAIT;


                },
                event = rx.recv_async() => {
                    // monitor progress

                    match event{
                        Ok(IncomingOutboundSyncEvent::Needed { entry_id }) => {
                             match workspace_ops.stat_entry_by_id(entry_id).await {
                                Ok(stat) => {
                                    if !stat.is_confined() && stat.is_file(){
                                        progress_tracker.files_to_sync.insert(entry_id, stat.size());

                            }
                                },
                                Err(e) => {
                                    // log and ignore
                                    log::error!("unable to stat {entry_id} because: {e:?}");

                                },
                            };

                        }
                        Ok(IncomingOutboundSyncEvent::Progress { entry_id, blocks, block_index, blocksize }) => {
                            // we are expecting that on the last block it may overflow, as the value that we
                            // used as reference is the real size, whereas here we don't see smaller than a block
                            // Note that the event was sent before the upload was started
                            progress_tracker
                                .files_to_sync
                                .insert(
                                    entry_id,
                                    blocks
                                        .saturating_mul(blocksize)
                                        .saturating_sub(
                                            block_index
                                                .saturating_mul(blocksize)
                                            )
                                    );
                        },
                        Ok(IncomingOutboundSyncEvent::Aborted { entry_id }) => {
                            // This event is sent when outbound_sync_child is not able to do the outbound sync
                            // this can happen when the entry is busy, unreachable or confined or when an
                            // inbound sync is needed first.
                            progress_tracker
                                .files_to_sync
                                .remove(&entry_id);
                            },
                        Ok(IncomingOutboundSyncEvent::Done { entry_id }) => {
                            progress_tracker
                                .files_to_sync
                                .remove(&entry_id);
                            },
                        Err(e) => log::error!("error while monitoring outbound sync progress : {e:?}"),
                    }

                },

            )
        }
    }
}
