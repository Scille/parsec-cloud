// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventMissedServerEvents, EventRealmVlobUpdated},
    workspace::{WorkspaceOps, WorkspaceSyncError},
    EventMonitorCrashed,
};

const WORKSPACE_INBOUND_SYNC_MONITOR_NAME: &str = "workspace_inbound_sync";

pub(crate) async fn start_workspace_inbound_sync_monitor(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
) -> Monitor {
    let realm_id = workspace_ops.realm_id();
    let task_future = {
        let task_future = task_future_factory(workspace_ops, event_bus.clone());
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

fn task_future_factory(
    workspace_ops: Arc<WorkspaceOps>,
    event_bus: EventBus,
) -> impl Future<Output = ()> {
    enum Action {
        RemoteChange { entry_id: VlobID },
        MissedServerEvents,
    }
    // Channel starts empty, hence the monitor will stay idle until the connection
    // monitor triggers its initial `EventMissedServerEvents` event
    let (tx, rx) = channel::unbounded::<Action>();

    let events_connection_lifetime = (
        {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventMissedServerEvents| {
                let _ = tx.send(Action::MissedServerEvents);
            })
        },
        {
            let tx = tx.clone();
            let realm_id = workspace_ops.realm_id();
            event_bus.connect(move |e: &EventRealmVlobUpdated| {
                if e.realm_id == realm_id {
                    let _ = tx.send(Action::RemoteChange {
                        entry_id: e.vlob_id,
                    });
                }
            })
        },
    );

    async move {
        let _events_connection_lifetime = events_connection_lifetime;

        // TODO:
        // - Get the realm checkpoint
        // - For each inbound sync event, ensure it is for the next realm checkpoint
        // - If ok, do a inbound sync and provide the new realm checkpoint to be updated in storage
        // - If not ok, consider it is a missed server event

        loop {
            let action = match rx.recv_async().await {
                Ok(action) => action,
                // Sender has left, time to shutdown !
                // In theory this should never happen given `WorkspaceInboundSyncMonitor`
                // abort our coroutine on teardown instead.
                Err(_) => return,
            };

            let to_sync = match action {
                Action::MissedServerEvents => {
                    // TODO: error handling (typically offline is not handled here !)
                    workspace_ops
                        .refresh_realm_checkpoint()
                        .await
                        .expect("TODO: not expected at all !");
                    workspace_ops
                        .get_need_inbound_sync()
                        .await
                        .expect("TODO: not expected at all !")
                }
                Action::RemoteChange { entry_id } => {
                    // TODO: update realm checkpoint
                    // TODO: optionally provide the vlob in the event, so that the
                    // sync can be done with no other server request
                    vec![entry_id]
                }
            };

            // TODO: pretty poor implementation:
            // - the events keep piling up during the sync
            // - no detection of an already synced entry
            // - no parallelism in sync
            for entry_id in to_sync {
                // Need a loop here to retry the operation in case the server is not available
                loop {
                    let outcome = workspace_ops.inbound_sync(entry_id).await;
                    match outcome {
                        Ok(_) => break,
                        Err(err) => match err {
                            WorkspaceSyncError::Stopped => {
                                // Shouldn't occur in practice given the monitors are expected
                                // to be stopped before the opses. In anycase we have no
                                // choice but to also stop.
                                return;
                            }
                            WorkspaceSyncError::Offline => {
                                event_bus.wait_server_online().await;
                                continue;
                            }

                            err @ (
                                // We have lost read access to the workspace, the certificates
                                // ops should soon be notified and work accordingly (typically
                                // by stopping the workspace and its monitors).
                                WorkspaceSyncError::NotAllowed
                                // Other errors are unexpected ones
                                | WorkspaceSyncError::NoKey
                                | WorkspaceSyncError::NoRealm
                                | WorkspaceSyncError::InvalidKeysBundle(_)
                                | WorkspaceSyncError::InvalidCertificate(_)
                                | WorkspaceSyncError::TimestampOutOfBallpark { .. }
                            )
                            => {
                                log::warn!("Stopping inbound sync monitor due to unexpected outcome: {}", err);
                                return;
                            }

                            WorkspaceSyncError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::warn!("Certificate monitor has crashed: {}", err);
                                let event = EventMonitorCrashed {
                                    monitor: WORKSPACE_INBOUND_SYNC_MONITOR_NAME,
                                    workspace_id: Some(workspace_ops.realm_id()),
                                    error: Arc::new(err),
                                };
                                event_bus.send(&event);
                                return;
                            }
                        },
                    }
                }
            }
        }
    }
}
