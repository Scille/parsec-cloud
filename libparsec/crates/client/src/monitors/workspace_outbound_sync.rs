// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, future::Future, pin::pin, sync::Arc};

use libparsec_platform_async::{
    channel, event, pretend_future_is_send_on_web, select3_biased, spawn,
};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventMonitorCrashed, EventWorkspaceOpsOutboundSyncNeeded},
    workspace::{InboundSyncOutcome, OutboundSyncOutcome, WorkspaceOps, WorkspaceSyncError},
};

const WORKSPACE_OUTBOUND_SYNC_MONITOR_NAME: &str = "workspace_outbound_sync";
const MIN_SYNC_WAIT: Duration = match Duration::try_seconds(1) {
    Some(v) => v,
    None => panic!("Invalid duration"),
};
const MAX_SYNC_WAIT: Duration = match Duration::try_minutes(1) {
    Some(v) => v,
    None => panic!("Invalid duration"),
};

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

fn task_future_factory(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
    device: Arc<LocalDevice>,
) -> (impl Future<Output = ()>, Box<dyn FnOnce() + Send + 'static>) {
    let (tx, rx) = channel::unbounded::<VlobID>();

    let request_stop = event::Event::new();
    let stop_requested = request_stop.listen();

    let stop_cb = Box::new(move || {
        request_stop.notify(usize::MAX);
    });

    let events_connection_lifetime = ({
        let tx = tx.clone();
        let realm_id = workspace_ops.realm_id();
        event_bus.connect(move |e: &EventWorkspaceOpsOutboundSyncNeeded| {
            if e.realm_id == realm_id {
                let _ = tx.send(e.entry_id);
            }
        })
    },);

    let task_future = async move {
        let _events_connection_lifetime = events_connection_lifetime;

        // Start a sub-task that will do the actual synchronization work
        let (syncer_tx, syncer_rx) = channel::unbounded::<VlobID>();
        let syncer = spawn({
            let workspace_ops = workspace_ops.clone();
            let tx = tx.clone();
            async move {
                macro_rules! handle_workspace_sync_error {
                    ($err:expr) => {
                        match $err {
                                WorkspaceSyncError::Offline => {
                                    event_bus.wait_server_online().await;
                                }
                                WorkspaceSyncError::Stopped => {
                                    return;
                                }

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
                                | WorkspaceSyncError::TimestampOutOfBallpark { .. }
                            )
                            => {
                                log::warn!("Stopping outbound sync monitor due to unexpected outcome: {}", err);
                                return;
                            }

                            WorkspaceSyncError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::warn!("Certificate monitor has crashed: {}", err);
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
                    // TODO: handle errors ?
                    loop {
                        let outcome = workspace_ops.outbound_sync(entry_id).await;
                        match outcome {
                            Ok(OutboundSyncOutcome::Done) => break,
                            Ok(OutboundSyncOutcome::InboundSyncNeeded) => (),
                            Ok(OutboundSyncOutcome::EntryIsBusy) => {
                                // Re-enqueue to retry later
                                // Note the send may fail if the syncer sub task has crashed,
                                // in which case there is nothing we can do :(
                                let _ = tx.send(entry_id);
                                break;
                            }
                            Err(err) => handle_workspace_sync_error!(err),
                        }

                        match workspace_ops.inbound_sync(entry_id).await {
                            Ok(InboundSyncOutcome::EntryIsBusy) => {
                                // Re-enqueue to retry later
                                let _ = tx.send(entry_id);
                                break;
                            }
                            Ok(InboundSyncOutcome::Updated | InboundSyncOutcome::NoChange) => (),
                            Err(err) => handle_workspace_sync_error!(err),
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
            // TODO: error handling
            let to_sync = workspace_ops
                .get_need_outbound_sync(u32::MAX)
                .await
                .expect("TODO: not expected at all !")
                .into_iter()
                .map(|entry_id| (entry_id, DueTime { since, due_time }))
                .collect::<HashMap<VlobID, DueTime>>();
            let due_time = if to_sync.is_empty() {
                None
            } else {
                Some(due_time)
            };
            (to_sync, due_time)
        };

        enum Action {
            Stop,
            NewLocalChange { entry_id: VlobID },
            DueTimeReached,
        }

        let mut stop_requested = pin!(stop_requested);
        loop {
            let to_sleep = match due_time {
                None => Duration::max_value(),
                Some(due_time) => due_time - device.now(),
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
                        Ok(entry_id) => Action::NewLocalChange { entry_id },
                        Err(_) => Action::Stop,
                    }
                },
                _ = device.time_provider.sleep(to_sleep) => Action::DueTimeReached,
            );

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
                    let mut next_due_time = None;
                    to_sync.retain(|entry_id, time| {
                        if time.due_time < now {
                            // TODO: error handling ? what if syncer needs to stop by itself
                            // due to internal error ?
                            let _ = syncer_tx.send(*entry_id);
                            false
                        } else {
                            match next_due_time {
                                None => {
                                    next_due_time = Some(time.due_time);
                                }
                                Some(due_time) if due_time > time.due_time => {
                                    next_due_time = Some(time.due_time);
                                }
                                _ => (),
                            }
                            true
                        }
                    });
                    // Due time has been reached, don't forget to update it !
                    due_time = next_due_time;
                }

                Action::NewLocalChange { entry_id } => {
                    let now = device.now();
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
    };

    (task_future, stop_cb)
}
