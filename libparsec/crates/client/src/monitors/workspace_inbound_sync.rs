// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web, yield_now};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{Broadcastable, EventBus, EventMissedServerEvents, EventRealmVlobUpdated},
    workspace::{
        InboundSyncOutcome, WorkspaceGetNeedInboundSyncEntriesError, WorkspaceOps,
        WorkspaceSyncError,
    },
    EventBusConnectionLifetime, EventMonitorCrashed, EventWorkspaceOpsInboundSyncDone,
    EventWorkspaceOpsOutboundSyncNeeded,
};

const WORKSPACE_INBOUND_SYNC_MONITOR_NAME: &str = "workspace_inbound_sync";

pub(crate) async fn start_workspace_inbound_sync_monitor(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
) -> Monitor {
    let realm_id = workspace_ops.realm_id();
    let task_future = {
        let io = RealInboundSyncManagerIO::new(workspace_ops, event_bus.clone());
        let task_future = inbound_sync_monitor_loop(realm_id, io);
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(
        event_bus,
        WORKSPACE_INBOUND_SYNC_MONITOR_NAME,
        Some(realm_id),
        task_future,
        None,
    )
    .await
}

#[derive(Debug)]
enum IncomingEvent {
    RemoteChange { entry_id: VlobID },
    LocalEntryChange { entry_id: VlobID },
    MissedServerEvents,
}

#[derive(Debug)]
enum WaitForNextIncomingEventOutcome {
    NewEvent(IncomingEvent),
    Disconnected,
}

/// This internal trait is used to abstract all side effects `inbound_sync_monitor_loop` relies on.
/// This way tests can be done with a mocked version of `WorkspaceOps`&co.
///
/// Also note all the methods here are asynchronous (even if some are implemented by wrapping
/// synchronous call, e.g. `event_bus.send(...)`).
/// This is needed in the tests given aborting a task is a fire-and-forget call, and hence
/// a lot (or very few depending on the run) can occur if we don't have a way to detect the
/// abort has been called and then sleep forever waiting for our task to be actually aborted.
trait InboundSyncManagerIO: Send + Sync + 'static {
    async fn wait_for_next_incoming_event(&self) -> WaitForNextIncomingEventOutcome;
    async fn retry_later_busy_entry(&mut self, entry_id: VlobID);
    async fn event_bus_wait_server_reconnect(&self);
    async fn event_bus_send(&self, event: &impl Broadcastable);
    fn workspace_ops_refresh_realm_checkpoint(
        &self,
    ) -> impl std::future::Future<Output = Result<(), WorkspaceSyncError>> + Send;
    fn workspace_ops_get_need_inbound_sync(
        &self,
        limit: u32,
    ) -> impl std::future::Future<
        Output = Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError>,
    > + Send;
    fn workspace_ops_inbound_sync(
        &self,
        entry_id: VlobID,
    ) -> impl std::future::Future<Output = Result<InboundSyncOutcome, WorkspaceSyncError>> + Send;
}

/// This is the only implementation of `InboundSyncManagerIO` you should care about !
struct RealInboundSyncManagerIO {
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    incoming_events_tx: channel::Sender<IncomingEvent>,
    incoming_events_rx: channel::Receiver<IncomingEvent>,
    /// Lifetimes are never accessed, but must be kept around for the whole time we
    /// need to listen to the events.
    _incoming_events_connection_lifetimes: (
        EventBusConnectionLifetime<EventMissedServerEvents>,
        EventBusConnectionLifetime<EventRealmVlobUpdated>,
        EventBusConnectionLifetime<EventWorkspaceOpsOutboundSyncNeeded>,
        EventBusConnectionLifetime<EventWorkspaceOpsInboundSyncDone>,
    ),
}

impl RealInboundSyncManagerIO {
    fn new(workspace_ops: Arc<WorkspaceOps>, event_bus: EventBus) -> Self {
        let (tx, rx) = channel::unbounded::<IncomingEvent>();

        // A workspace (and its related monitors, including this one!) can be
        // started/stopped at any time.
        // This means we cannot rely on the connection monitor to send us a
        // `MissedServerEvents` as starting point (which is what does the
        // monitors that start with the client).
        // So instead we manually inject a this event to correctly start ourself.
        tx.send(IncomingEvent::MissedServerEvents)
            .expect("channel just created");

        let event_missed_server_events_lifetime = {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventMissedServerEvents| {
                let _ = tx.send(IncomingEvent::MissedServerEvents);
            })
        };

        let event_realm_vlob_updated_lifetime = {
            let tx = tx.clone();
            let realm_id = workspace_ops.realm_id();
            event_bus.connect(move |e: &EventRealmVlobUpdated| {
                if e.realm_id == realm_id {
                    let _ = tx.send(IncomingEvent::RemoteChange {
                        entry_id: e.vlob_id,
                    });
                }
            })
        };

        let event_outbound_sync_needed_lifetime = {
            let tx = tx.clone();
            let realm_id = workspace_ops.realm_id();
            event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
                if e.realm_id == realm_id {
                    let _ = tx.send(IncomingEvent::LocalEntryChange {
                        entry_id: e.entry_id,
                    });
                }
            })
        };

        let event_inbound_sync_done_lifetime = {
            let tx = tx.clone();
            let realm_id = workspace_ops.realm_id();
            event_bus.connect(move |e: &EventWorkspaceOpsInboundSyncDone| {
                if e.realm_id == realm_id {
                    let _ = tx.send(IncomingEvent::LocalEntryChange {
                        entry_id: e.entry_id,
                    });
                }
            })
        };

        Self {
            workspace_ops,
            event_bus,
            incoming_events_tx: tx,
            incoming_events_rx: rx,
            _incoming_events_connection_lifetimes: (
                event_missed_server_events_lifetime,
                event_realm_vlob_updated_lifetime,
                event_outbound_sync_needed_lifetime,
                event_inbound_sync_done_lifetime,
            ),
        }
    }
}

impl InboundSyncManagerIO for RealInboundSyncManagerIO {
    async fn wait_for_next_incoming_event(&self) -> WaitForNextIncomingEventOutcome {
        let fut = self.incoming_events_rx.recv_async();
        match pretend_future_is_send_on_web(fut).await {
            Ok(event) => WaitForNextIncomingEventOutcome::NewEvent(event),
            Err(channel::RecvError::Disconnected) => WaitForNextIncomingEventOutcome::Disconnected,
        }
    }

    async fn retry_later_busy_entry(&mut self, entry_id: VlobID) {
        let _ = self
            .incoming_events_tx
            .send(IncomingEvent::RemoteChange { entry_id });
    }

    async fn event_bus_wait_server_reconnect(&self) {
        let fut = self.event_bus.wait_server_reconnect();
        pretend_future_is_send_on_web(fut).await
    }

    async fn event_bus_send(&self, event: &impl Broadcastable) {
        self.event_bus.send(event)
    }

    async fn workspace_ops_refresh_realm_checkpoint(&self) -> Result<(), WorkspaceSyncError> {
        let fut = self.workspace_ops.refresh_realm_checkpoint();
        pretend_future_is_send_on_web(fut).await
    }

    async fn workspace_ops_get_need_inbound_sync(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError> {
        let fut = self.workspace_ops.get_need_inbound_sync(limit);
        pretend_future_is_send_on_web(fut).await
    }

    async fn workspace_ops_inbound_sync(
        &self,
        entry_id: VlobID,
    ) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
        let fut = self.workspace_ops.inbound_sync(entry_id);
        pretend_future_is_send_on_web(fut).await
    }
}

#[derive(Default, Debug)]
struct ConfinedEntriesTracker {
    confinement_member_to_confined_entries: HashMap<VlobID, HashSet<VlobID>>,
    confined_entry_to_confinement_members: HashMap<VlobID, HashSet<VlobID>>,
}

impl ConfinedEntriesTracker {
    fn register_confined_entry(
        &mut self,
        confined_entry: VlobID,
        confinement_point: VlobID,
        entry_chain: &[VlobID],
    ) {
        // The confinement members are all the entries between the confinement point and the entry
        let confinement_members: HashSet<_> = entry_chain
            .iter()
            .skip_while(|&x| x != &confinement_point)
            .filter(|&x| x != &confined_entry)
            .copied()
            .collect();
        for confinement_member in confinement_members.iter() {
            self.confinement_member_to_confined_entries
                .entry(*confinement_member)
                .or_default()
                .insert(confined_entry);
        }
        self.confined_entry_to_confinement_members
            .insert(confined_entry, confinement_members);
    }

    fn unregister_confined_entry(&mut self, confined_entry: VlobID) {
        if let Some(confinement_members) = self
            .confined_entry_to_confinement_members
            .remove(&confined_entry)
        {
            for confinement_member in confinement_members {
                if let Some(confined_entries) = self
                    .confinement_member_to_confined_entries
                    .get_mut(&confinement_member)
                {
                    confined_entries.remove(&confined_entry);
                    if confined_entries.is_empty() {
                        self.confinement_member_to_confined_entries
                            .remove(&confinement_member);
                    }
                }
            }
        }
    }

    fn get_confined_entries(&self, confinement_member: VlobID) -> Vec<VlobID> {
        self.confinement_member_to_confined_entries
            .get(&confinement_member)
            .map(|confined_entries| confined_entries.iter().copied().collect())
            .unwrap_or_default()
    }
}

async fn inbound_sync_monitor_loop(realm_id: VlobID, mut io: impl InboundSyncManagerIO) {
    // TODO:
    // - Get the realm checkpoint
    // - For each inbound sync event, ensure it is for the next realm checkpoint
    // - If ok, do a inbound sync and provide the new realm checkpoint to be updated in storage
    // - If not ok, consider it is a missed server event

    let mut confined_entries_tracker = ConfinedEntriesTracker::default();

    loop {
        let incoming_event = match io.wait_for_next_incoming_event().await {
            WaitForNextIncomingEventOutcome::NewEvent(incoming_event) => incoming_event,
            // Sender has left, time to shutdown !
            // In theory this should never happen given `WorkspaceInboundSyncMonitor`
            // abort our coroutine on teardown instead.
            WaitForNextIncomingEventOutcome::Disconnected => return,
        };
        log::debug!("Workspace {realm_id}: incoming event {incoming_event:?}");

        let to_sync = match incoming_event {
            IncomingEvent::MissedServerEvents => {
                // Need a loop here to retry the operation in case the server is not available
                loop {
                    let outcome = io.workspace_ops_refresh_realm_checkpoint().await;
                    log::debug!(
                        "Workspace {realm_id}: refresh realm checkpoint, outcome: {outcome:?}"
                    );
                    match outcome {
                        Ok(()) => break,
                        Err(err) => match err {
                            WorkspaceSyncError::Offline(_) | WorkspaceSyncError::ServerBlockstoreUnavailable => {
                                io.event_bus_wait_server_reconnect().await;
                                continue;
                            },
                            WorkspaceSyncError::Stopped => {
                                // Shouldn't occur in practice given the monitors are expected
                                // to be stopped before the opses. In any case we have no
                                // choice but to also stop.
                                log::error!("Workspace {realm_id}: stopping due to unexpected WorkspaceOps stop");
                                return;
                            },
                            err @ (
                                // We have lost read access to the workspace, the certificates
                                // ops should soon be notified and work accordingly (typically
                                // by stopping the workspace and its monitors).
                                WorkspaceSyncError::NotAllowed
                                // Other errors are unexpected ones
                                | WorkspaceSyncError::NoKey
                                | WorkspaceSyncError::NoRealm
                                | WorkspaceSyncError::InvalidManifest(_)
                                | WorkspaceSyncError::InvalidBlockAccess(_)
                                | WorkspaceSyncError::InvalidKeysBundle(_)
                                | WorkspaceSyncError::InvalidCertificate(_)
                                | WorkspaceSyncError::Internal(_)
                                | WorkspaceSyncError::TimestampOutOfBallpark { .. }
                            ) => {
                                log::error!("Workspace {realm_id}: stopping due to unexpected error: {err:?}");
                                return;
                            },
                        },
                    }
                }

                let outcome = io.workspace_ops_get_need_inbound_sync(u32::MAX).await;
                log::debug!("Workspace {realm_id}: get need inbound sync, outcome: {outcome:?}");
                match outcome {
                    Ok(to_sync) => to_sync,
                    Err(err) => {
                        match err {
                            WorkspaceGetNeedInboundSyncEntriesError::Stopped => {
                                // Shouldn't occur in practice given the monitors are expected
                                // to be stopped before the opses. In any case we have no
                                // choice but to also stop.
                                log::error!("Workspace {realm_id}: stopping due to unexpected WorkspaceOps stop");
                                return;
                            }
                            WorkspaceGetNeedInboundSyncEntriesError::Internal(err) => {
                                log::error!("Workspace {realm_id}: stopping due to unexpected error: {err:?}");
                                return;
                            }
                        }
                    }
                }
            }
            IncomingEvent::RemoteChange { entry_id } => {
                // TODO: update realm checkpoint
                // TODO: optionally provide the vlob in the event, so that the
                // sync can be done with no other server request
                vec![entry_id]
            }
            IncomingEvent::LocalEntryChange { entry_id } => {
                // Reschedule the confined entries that might depend on this one
                confined_entries_tracker.get_confined_entries(entry_id)
            }
        };

        // TODO: pretty poor implementation:
        // - the events keep piling up during the sync
        // - no detection of an already synced entry
        // - no parallelism in sync
        // - no exponential backoff when retrying sync on busy entry
        for entry_id in to_sync {
            // Need a loop here to retry the operation in case the server is not available
            loop {
                let outcome = io.workspace_ops_inbound_sync(entry_id).await;
                log::debug!("Workspace {realm_id}: inbound sync {entry_id}, outcome: {outcome:?}");
                match outcome {
                    Ok(InboundSyncOutcome::NoChange | InboundSyncOutcome::Updated) => {
                        // Unregister the entry from the list of confined entries if it was there
                        confined_entries_tracker.unregister_confined_entry(entry_id);
                        break;
                    }
                    Ok(InboundSyncOutcome::EntryIsConfined {
                        confinement_point,
                        entry_chain,
                    }) => {
                        // Add the entry to the list of confined entries
                        confined_entries_tracker.register_confined_entry(
                            entry_id,
                            confinement_point,
                            &entry_chain,
                        );
                        break;
                    }
                    Ok(InboundSyncOutcome::EntryIsBusy) => {
                        // Re-enqueue to retry later

                        // We are playing a dangerous game here: `flume::Receiver::recv_async`
                        // doesn't yield to the executor if it contains an item, and we send
                        // the event from the very same coroutine that will receive it.
                        // This means we can end up in a busy-loop if another coroutine locks
                        // the entry and our executor is mono-threaded !
                        //
                        // The solution to avoid this is simple: we just explicitly yield
                        // before re-enqueueing.
                        yield_now().await;

                        io.retry_later_busy_entry(entry_id).await;

                        break;
                    }
                    Err(err) => match err {
                        WorkspaceSyncError::Offline(_) | WorkspaceSyncError::ServerBlockstoreUnavailable => {
                            io.event_bus_wait_server_reconnect().await;
                            continue;
                        }
                        // We have lost read access to the workspace, the certificates
                        // ops should soon be notified and work accordingly (typically
                        // by stopping the workspace and its monitors).
                        WorkspaceSyncError::NotAllowed => {
                            log::info!("Workspace {realm_id}: stopping as we no longer allowed to access this realm");
                            return;
                        }
                        WorkspaceSyncError::Stopped => {
                            // Shouldn't occur in practice given the monitors are expected
                            // to be stopped before the opses. In any case we have no
                            // choice but to also stop.
                            log::error!("Workspace {realm_id}: stopping due to unexpected WorkspaceOps stop");
                            return;
                        }
                        err @ (
                            // Other errors are unexpected ones
                            | WorkspaceSyncError::NoKey
                            | WorkspaceSyncError::NoRealm
                            | WorkspaceSyncError::InvalidManifest(_)
                            | WorkspaceSyncError::InvalidBlockAccess(_)
                            | WorkspaceSyncError::InvalidKeysBundle(_)
                            | WorkspaceSyncError::InvalidCertificate(_)
                            | WorkspaceSyncError::TimestampOutOfBallpark { .. }
                        )
                        => {
                            log::error!("Workspace {realm_id}: stopping due to unexpected error: {err:?}");
                            return;
                        }

                        WorkspaceSyncError::Internal(err) => {
                            // Unexpected error occurred, better stop the monitor
                            log::error!("Workspace {realm_id}: stopping due to unexpected error: {err:?}");
                            let event = EventMonitorCrashed {
                                monitor: WORKSPACE_INBOUND_SYNC_MONITOR_NAME,
                                workspace_id: Some(realm_id),
                                error: Arc::new(err),
                            };
                            io.event_bus_send(&event).await;
                            return;
                        }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace_inbound_sync_monitor.rs"]
mod tests;
