// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::pin,
    sync::Arc,
};

use libparsec_platform_async::{
    channel, event, pretend_future_is_send_on_web, select2_biased, select3_biased, spawn,
};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventMonitorCrashed, EventWorkspaceOpsOutboundSyncNeeded},
    workspace::{
        InboundSyncOutcome, OutboundSyncOutcome, WorkspaceGetNeedOutboundSyncEntriesError,
        WorkspaceOps, WorkspaceSyncError,
    },
    EventWorkspaceOpsInboundSyncDone,
};

const WORKSPACE_OUTBOUND_SYNC_MONITOR_NAME: &str = "workspace_outbound_sync";

// Test constants are 10 times faster than production ones
#[cfg(test)]
const MIN_SYNC_WAIT: Duration = Duration::milliseconds(100);
#[cfg(test)]
const MAX_SYNC_WAIT: Duration = Duration::seconds(6);
#[cfg(test)]
const SERVER_STORE_UNAVAILABLE_WAIT: Duration = Duration::seconds(6);

#[cfg(not(test))]
const MIN_SYNC_WAIT: Duration = Duration::seconds(1);
#[cfg(not(test))]
const MAX_SYNC_WAIT: Duration = Duration::minutes(1);
#[cfg(not(test))]
const SERVER_STORE_UNAVAILABLE_WAIT: Duration = Duration::minutes(1);

pub(crate) async fn start_workspace_outbound_sync_monitor(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    device: Arc<LocalDevice>,
) -> Monitor {
    let realm_id = workspace_ops.realm_id();
    let (task_future, stop_cb) = task_future_factory(workspace_ops, event_bus.clone(), device);
    let task_future = pretend_future_is_send_on_web(task_future);
    Monitor::start(
        event_bus,
        WORKSPACE_OUTBOUND_SYNC_MONITOR_NAME,
        Some(realm_id),
        task_future,
        Some(stop_cb),
    )
    .await
}

#[derive(Default, Debug)]
struct ConfinedEntriesTracker {
    confinement_point_to_confined_entries: HashMap<VlobID, HashSet<VlobID>>,
    confined_entry_to_confinement_point: HashMap<VlobID, VlobID>,
    recording: Option<HashSet<VlobID>>,
}

impl ConfinedEntriesTracker {
    fn record_local_changes(&mut self) {
        self.recording = Some(HashSet::new());
    }

    fn stop_recording_local_changes(&mut self) -> HashSet<VlobID> {
        self.recording.take().unwrap_or_default()
    }

    fn get_confined_entries_after_local_change(
        &mut self,
        confinement_point: VlobID,
    ) -> Vec<VlobID> {
        if let Some(recording) = self.recording.as_mut() {
            recording.insert(confinement_point);
        }
        self.confinement_point_to_confined_entries
            .get(&confinement_point)
            .map(|confined_entries| confined_entries.iter().copied().collect())
            .unwrap_or_default()
    }

    fn register_confined_entry(
        &mut self,
        confinement_point: VlobID,
        _entry_chain: &[VlobID],
        confined_entry: VlobID,
    ) {
        self.confinement_point_to_confined_entries
            .entry(confinement_point)
            .or_default()
            .insert(confined_entry);
        self.confined_entry_to_confinement_point
            .insert(confined_entry, confinement_point);
    }

    fn unregister_confined_entry(&mut self, confined_entry: VlobID) {
        if let Some(confinement_point) = self
            .confined_entry_to_confinement_point
            .remove(&confined_entry)
        {
            if let Some(confined_entries) = self
                .confinement_point_to_confined_entries
                .get_mut(&confinement_point)
            {
                confined_entries.remove(&confined_entry);
                if confined_entries.is_empty() {
                    self.confinement_point_to_confined_entries
                        .remove(&confinement_point);
                }
            }
        }
    }
}

#[derive(Debug)]
enum IncomingEvent {
    OutboundSyncNeeded { entry_id: VlobID },
    InboundSyncDone { entry_id: VlobID },
}

fn task_future_factory(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    device: Arc<LocalDevice>,
) -> (impl Future<Output = ()>, Box<dyn FnOnce() + Send + 'static>) {
    let realm_id = workspace_ops.realm_id();
    let (tx, rx) = channel::unbounded::<IncomingEvent>();

    let request_stop = event::Event::new();
    let stop_requested = request_stop.listen();
    let syncer_stop_requested = request_stop.listen();

    let stop_cb = Box::new(move || {
        request_stop.notify(usize::MAX);
    });

    let outbound_sync_needed_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingEvent::OutboundSyncNeeded {
                    entry_id: e.entry_id,
                });
            }
        })
    },);

    let inbound_sync_needed_events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsInboundSyncDone| {
            if e.realm_id == realm_id {
                let _ = tx.send(IncomingEvent::InboundSyncDone {
                    entry_id: e.entry_id,
                });
            }
        })
    },);

    let task_future = async move {
        let _outbound_sync_needed_events_connection_lifetime =
            outbound_sync_needed_events_connection_lifetime;
        let _inbound_sync_needed_events_connection_lifetime =
            inbound_sync_needed_events_connection_lifetime;
        let confined_entries_tracker =
            Arc::new(std::sync::Mutex::new(ConfinedEntriesTracker::default()));

        // Start a sub-task that will do the actual synchronization work

        // Note the rendez-vous channel (i.e. bound with zero capacity) used for
        // communication.
        // This is because we want the entry waiting for sync to stay in the
        // `to_sync` list, so that the due time can be updated if additional
        // modification occurs.
        let (syncer_tx, syncer_rx) = channel::bounded::<VlobID>(0);
        let syncer = spawn({
            let workspace_ops = workspace_ops.clone();
            let tx = tx.clone();
            let device = device.clone();
            let confined_entries_tracker = confined_entries_tracker.clone();
            async move {
                let mut syncer_stop_requested = pin!(syncer_stop_requested);
                macro_rules! handle_workspace_sync_error {
                    ($err:expr, $entry_id:expr) => {
                        match $err {
                            WorkspaceSyncError::Offline(_) => {
                                // Make sure we do not block the stopping of the monitor here
                                select2_biased!(
                                    _ = event_bus.wait_server_reconnect() => {},
                                    // `syncer_stop_requested` doesn't need to be fused (i.e. won't be
                                    // polled once completed) given we return right away here.
                                    _ = &mut syncer_stop_requested => return,
                                )
                            }
                            // We have lost read access to the workspace, the certificates
                            // ops should soon be notified and work accordingly (typically
                            // by stopping the workspace and its monitors).
                            WorkspaceSyncError::NotAllowed => {
                                log::info!("Workspace {realm_id}: stopping as we no longer allowed to access this realm");
                                return
                            },
                            WorkspaceSyncError::Stopped => {
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
                            WorkspaceSyncError::ServerBlockstoreUnavailable => {
                                // Re-enqueue to retry later
                                let entry_id = $entry_id;
                                log::info!("Workspace {realm_id}: {entry_id} sync failed due to server block store unavailable, aborting sync and waiting {}s", SERVER_STORE_UNAVAILABLE_WAIT.num_seconds());
                                let _ = tx.send(IncomingEvent::OutboundSyncNeeded { entry_id });
                                device
                                    .time_provider
                                    .sleep(SERVER_STORE_UNAVAILABLE_WAIT)
                                    .await;
                                break;
                            }
                            WorkspaceSyncError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::error!("Workspace {realm_id}: stopping due to unexpected error: {err:?}");
                                let event = EventMonitorCrashed {
                                    monitor: WORKSPACE_OUTBOUND_SYNC_MONITOR_NAME,
                                    workspace_id: Some(workspace_ops.realm_id()),
                                    error: Arc::new(err),
                                };
                                event_bus.send(&event);
                                return;
                            }
                        }
                    };
                }
                loop {
                    let entry_id = match syncer_rx.recv_async().await {
                        Ok(entry_id) => entry_id,
                        Err(_) => return,
                    };
                    loop {
                        // Sleep a bit to avoid busy loops
                        // TODO: investigate why this fixes issue #9752
                        #[cfg(test)]
                        device.time_provider.sleep(Duration::milliseconds(1)).await;
                        confined_entries_tracker
                            .lock()
                            .expect("Mutex is poisoned")
                            .record_local_changes();
                        let outcome = workspace_ops.outbound_sync(entry_id).await;
                        let recorded_local_changes = confined_entries_tracker
                            .lock()
                            .expect("Mutex is poisoned")
                            .stop_recording_local_changes();
                        log::debug!(
                            "Workspace {realm_id}: outbound sync {entry_id}, outcome: {outcome:?}"
                        );
                        match outcome {
                            Ok(
                                // If the entry is unreachable in the path-space, it seems better to not
                                // synchronize it since we don't even know what it is. Just treat it as
                                // if it was done, it will likely come back since it tagged as `need_sync`
                                // but we will ignore it again.
                                OutboundSyncOutcome::Done | OutboundSyncOutcome::EntryIsUnreachable,
                            ) => {
                                // Unregister the confinement point
                                confined_entries_tracker
                                    .lock()
                                    .expect("Mutex is poisoned")
                                    .unregister_confined_entry(entry_id);
                                break;
                            }
                            Ok(OutboundSyncOutcome::EntryIsConfined {
                                confinement_point,
                                entry_chain,
                            }) => {
                                // The confinement point has changed while we were syncing, try again
                                if recorded_local_changes.contains(&confinement_point) {
                                    continue;
                                }
                                // Register the confinement point
                                confined_entries_tracker
                                    .lock()
                                    .expect("Mutex is poisoned")
                                    .register_confined_entry(
                                        confinement_point,
                                        &entry_chain,
                                        entry_id,
                                    );
                                break;
                            }
                            Ok(OutboundSyncOutcome::InboundSyncNeeded) => (),
                            Ok(OutboundSyncOutcome::EntryIsBusy) => {
                                // Re-enqueue to retry later
                                // Note the send may fail if the syncer sub task has crashed,
                                // in which case there is nothing we can do :(
                                log::info!("Workspace {realm_id}: {entry_id} is busy so aborting sync to retry later");
                                let _ = tx.send(IncomingEvent::OutboundSyncNeeded { entry_id });
                                break;
                            }
                            Err(err) => handle_workspace_sync_error!(err, entry_id),
                        }

                        let outcome = workspace_ops.inbound_sync(entry_id).await;
                        log::debug!(
                            "Workspace {realm_id}: inbound sync {entry_id}, outcome: {outcome:?}"
                        );
                        match outcome {
                            Ok(InboundSyncOutcome::EntryIsBusy) => {
                                // Re-enqueue to retry later
                                log::info!("Workspace {realm_id}: {entry_id} is busy so aborting sync to retry later");
                                let _ = tx.send(IncomingEvent::OutboundSyncNeeded { entry_id });
                                break;
                            }
                            Ok(
                                InboundSyncOutcome::Updated
                                | InboundSyncOutcome::NoChange
                                | InboundSyncOutcome::EntryIsConfined { .. },
                            ) => (),
                            Err(err) => handle_workspace_sync_error!(err, entry_id),
                        }
                    }
                }
            }
        });

        struct DueTime {
            since: DateTime,
            due_time: DateTime,
        }
        let (mut to_sync, mut due_time) = {
            let since = device.now();
            let due_time = since + MIN_SYNC_WAIT;

            let outcome = workspace_ops.get_need_outbound_sync(u32::MAX).await;
            log::debug!("Workspace {realm_id}: get need outbound sync, outcome: {outcome:?}");
            let to_sync = match outcome {
                Ok(to_sync) => to_sync
                    .into_iter()
                    .map(|entry_id| (entry_id, DueTime { since, due_time }))
                    .collect::<HashMap<VlobID, DueTime>>(),
                Err(err) => {
                    match err {
                        WorkspaceGetNeedOutboundSyncEntriesError::Stopped => {
                            // Shouldn't occur in practice given the monitors are expected
                            // to be stopped before the opses. In any case we have no
                            // choice but to also stop.
                            log::error!("Workspace {realm_id}: stopping due to unexpected WorkspaceOps stop");
                            return;
                        }
                        WorkspaceGetNeedOutboundSyncEntriesError::Internal(err) => {
                            log::error!(
                                "Workspace {realm_id}: stopping due to unexpected error: {err:?}"
                            );
                            return;
                        }
                    }
                }
            };

            let due_time = if to_sync.is_empty() {
                None
            } else {
                Some(due_time)
            };
            (to_sync, due_time)
        };

        #[derive(Debug)]
        enum Action {
            Stop,
            NewLocalChange { entry_id: VlobID },
            NewRemoteChange { entry_id: VlobID },
            DueTimeReached,
        }

        let mut stop_requested = pin!(stop_requested);
        loop {
            let to_sleep = match due_time {
                None => {
                    log::debug!("Workspace {realm_id}: sleeping forever");
                    Duration::MAX
                }
                Some(due_time) => {
                    let duration = due_time - device.now();
                    log::debug!(
                        "Workspace {realm_id}: sleeping for {}ms",
                        duration.num_milliseconds()
                    );
                    duration
                }
            };

            let action = select3_biased!(
                // The select macro is biased so the first future has priority.
                // Hence we first check for stop to avoid famine if `rx` contains
                // a lot of items.
                //
                // Also, `stop_requested` doesn't need to be fused (i.e. won't be
                // polled once completed) given it signals it's time for shutdown.
                _ = &mut stop_requested => Action::Stop,
                outcome = rx.recv_async() => {
                    match outcome {
                        Ok(IncomingEvent::InboundSyncDone { entry_id }) => Action::NewRemoteChange { entry_id },
                        Ok(IncomingEvent::OutboundSyncNeeded { entry_id }) => Action::NewLocalChange { entry_id },
                        Err(_) => Action::Stop,
                    }
                },
                // We also keep the due time last, as a new local change on a file
                // that has reach it due time means we should keep waiting instead
                // of synchronizing it right away.
                _ = device.time_provider.sleep(to_sleep) => Action::DueTimeReached,
            );
            log::debug!("Workspace {realm_id}: incoming action {action:?}");

            match action {
                Action::Stop => {
                    // Shutdown the sub-task before leaving
                    drop(syncer_tx);
                    let _ = syncer.await;
                    // This return is why `stop_requested` doesn't need to be fused
                    return;
                }

                Action::DueTimeReached => {
                    let now = device.now();

                    let due = to_sync
                        .iter()
                        .filter_map(|(entry_id, time)| {
                            if time.due_time < now {
                                Some(*entry_id)
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some(entry_id) = due {
                        log::debug!(
                            "Workspace {realm_id}: sending {entry_id} for sub-task to sync"
                        );
                        to_sync.remove(&entry_id);
                        // Block until the sub-task is ready to sync our entry.
                        match syncer_tx.send_async(entry_id).await {
                            Ok(_) => (),
                            Err(_) => {
                                log::warn!("Workspace {realm_id}: stopping due to sub-task unexpectedly stopped");
                                return;
                            }
                        }
                    }

                    // Due time has been reached, don't forget to update it !
                    //
                    // Note there may be multiple entries that have reached their due time,
                    // but we only sync a single one here.
                    //
                    // This is because during the time it took to sync the first entry,
                    // some further change may have modified the others due entries, which
                    // hence should get there due time recomputed.
                    due_time = to_sync.values().map(|time| time.due_time).min();
                }

                Action::NewLocalChange { entry_id } | Action::NewRemoteChange { entry_id } => {
                    // For new local change, schedule the entry for sync.
                    let mut entries_to_schedule = vec![];
                    if let Action::NewLocalChange { .. } = action {
                        entries_to_schedule.push(entry_id);
                    }
                    // In both cases, we also need to schedule the confined entries
                    entries_to_schedule.extend(
                        confined_entries_tracker
                            .lock()
                            .expect("Mutex is poisoned")
                            .get_confined_entries_after_local_change(entry_id),
                    );
                    let now = device.now();
                    for entry_id in entries_to_schedule {
                        let potential_due_time = match to_sync.entry(entry_id) {
                            std::collections::hash_map::Entry::Vacant(entry) => {
                                let due_time = now + MIN_SYNC_WAIT;
                                entry.insert(DueTime {
                                    since: now,
                                    due_time,
                                });
                                due_time
                            }
                            std::collections::hash_map::Entry::Occupied(mut entry) => {
                                let e = entry.get_mut();
                                let new_due_time = now + MIN_SYNC_WAIT;
                                if new_due_time - e.since < MAX_SYNC_WAIT {
                                    e.due_time = new_due_time;
                                }
                                e.due_time
                            }
                        };
                        match due_time {
                            None => due_time = Some(potential_due_time),
                            Some(current_due_time) if current_due_time > potential_due_time => {
                                due_time = Some(potential_due_time)
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    };

    (task_future, stop_cb)
}
