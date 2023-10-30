// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::{channel, spawn, JoinHandle};
use libparsec_types::prelude::*;

use crate::{
    event_bus::{
        EventBus, EventMissedServerEvents, EventRealmVlobsUpdated, EventUserOpsNeedSync,
        EventUserSyncMonitorCrashed,
    },
    user_ops::{SyncError, UserOps},
};

pub struct UserSyncMonitor {
    worker: JoinHandle<()>,
}

impl UserSyncMonitor {
    pub async fn start(user_ops: Arc<UserOps>, event_bus: EventBus) -> Self {
        enum Action {
            RemoteChange(IndexInt),
            LocalChange,
            MissedServerEvents,
        }
        // Channel starts empty, hence the monitor will stay idle until the connection
        // monitor triggers it initial `EventMissedServerEvents` event
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
                event_bus.connect(move |e: &EventRealmVlobsUpdated| {
                    if e.realm_id == realm_id {
                        let _ = tx.send(Action::RemoteChange(e.checkpoint));
                    }
                })
            },
            event_bus.connect(move |_: &EventUserOpsNeedSync| {
                let _ = tx.send(Action::LocalChange);
            }),
        );

        let worker = spawn(async move {
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
                    // TODO: should use realm checkpoint system ! Currently we always start
                    // by trying a sync of the user manifest
                    if let Err(err) = user_ops.sync().await {
                        match err {
                            SyncError::Offline => {
                                event_bus.wait_server_online().await;
                                continue;
                            }
                            // TODO
                            SyncError::InMaintenance => todo!(),
                            SyncError::InvalidCertificate(error) => {
                                // Note `UserOps` is responsible for sending the
                                // invalid certificate event on the event bus
                                log::warn!("Invalid certificate detected: {}", error);
                            }
                            error @ SyncError::BadTimestamp { .. } => {
                                // Note `UserOps` is responsible for sending the
                                // bad timestamp event on the event bus
                                log::warn!("Client/server clock drift detected: {:?}", error);
                            }
                            SyncError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::warn!("Certificate monitor has crashed: {}", err);
                                let event = EventUserSyncMonitorCrashed(err);
                                event_bus.send(&event);
                                return;
                            }
                        }
                    }
                    break;
                }
            }
        });

        Self { worker }
    }
}

impl Drop for UserSyncMonitor {
    fn drop(&mut self) {
        // TODO: aborting the coroutine might be too violent here !
        self.worker.abort()
    }
}
