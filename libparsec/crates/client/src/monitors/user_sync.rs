// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};

use super::Monitor;
use crate::{
    event_bus::{
        EventBus, EventMissedServerEvents, EventRealmVlobUpdated, EventUserOpsOutboundSyncNeeded,
    },
    user::{UserOps, UserSyncError},
    EventMonitorCrashed,
};

const USER_SYNC_MONITOR_NAME: &str = "user_sync";

// Local user manifest currently doesn't need to be synchronized, hence the
// user sync monitor is never used (especially since it current implementation
// is now incompatible with OUTSIDER users now that they can created realm).
#[allow(unused)]
pub(crate) async fn start_user_sync_monitor(
    user_ops: Arc<UserOps>,
    event_bus: EventBus,
) -> Monitor {
    let future = {
        let task_future = task_future_factory(user_ops, event_bus.clone());
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(event_bus, USER_SYNC_MONITOR_NAME, None, future, None).await
}

fn task_future_factory(user_ops: Arc<UserOps>, event_bus: EventBus) -> impl Future<Output = ()> {
    enum Action {
        RemoteChange,
        LocalChange,
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
            let realm_id = user_ops.realm_id();
            event_bus.connect(move |e: &EventRealmVlobUpdated| {
                if e.realm_id == realm_id {
                    // TODO: use the event's blob to avoid a round-trip to the server
                    let _ = tx.send(Action::RemoteChange);
                }
            })
        },
        event_bus.connect(move |_: &EventUserOpsOutboundSyncNeeded| {
            let _ = tx.send(Action::LocalChange);
        }),
    );

    async move {
        let _events_connection_lifetime = events_connection_lifetime;

        loop {
            match rx.recv_async().await {
                // Sync is needed, we don't bother determining inbound/outbound and
                // dealing with realm checkpoint given it's always the same single
                // vlob that we have to sync
                Ok(_) => (),
                // Sender has left, time to shutdown !
                // In theory this should never happen given `UserSyncMonitor`
                // abort our coroutine on teardown instead.
                Err(_) => return,
            };
            // Need a loop here to retry the operation in case the server is not available
            loop {
                let outcome = user_ops.sync().await;
                match outcome {
                    Ok(_) => break,
                    Err(err) => match err {
                        UserSyncError::Stopped => {
                            // Shouldn't occur in practice given the monitors are expected
                            // to be stopped before the opses. In any case we have no
                            // choice but to also stop.
                            return;
                        }
                        UserSyncError::Offline(_) => {
                            event_bus.wait_server_reconnect().await;
                            continue;
                        }
                        UserSyncError::AuthorNotAllowed => {
                            // OUTSIDER user are not allowed to create realm in Parsec v3.
                            // In practice this is not an issue given we don't need to sync
                            // user manifest for the moment.
                            log::warn!("Rejected upload: OUTSIDER is not allowed to create realm",);
                            break;
                        }
                        UserSyncError::RejectedBySequesterService { service_id, reason } => {
                            // Note `UserOps` is responsible for sending the
                            // reject by sequester event on the event bus
                            log::warn!(
                                "Rejected upload by sequester service {}: {}",
                                service_id,
                                reason.as_ref().map_or("<no reason>", |r| r.as_str())
                            );
                            break;
                        }
                        UserSyncError::InvalidCertificate(error) => {
                            // Note `UserOps` is responsible for sending the
                            // invalid certificate event on the event bus
                            log::warn!("Invalid certificate detected: {error}");
                            break;
                        }
                        error @ UserSyncError::TimestampOutOfBallpark { .. } => {
                            // Note `UserOps` is responsible for sending the
                            // bad timestamp event on the event bus
                            log::warn!("Client/server clock drift detected: {error:?}");
                            break;
                        }
                        UserSyncError::Internal(err) => {
                            // Unexpected error occurred, better stop the monitor
                            log::error!("Certificate monitor has crashed: {err}");
                            let event = EventMonitorCrashed {
                                monitor: USER_SYNC_MONITOR_NAME,
                                workspace_id: None,
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
