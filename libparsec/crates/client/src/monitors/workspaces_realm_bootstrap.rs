// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{
        EventBus, EventMissedServerEvents, EventMonitorCrashed, EventWorkspaceLocallyCreated,
        EventWorkspaceLocallyRenamed,
    },
    user::{EnsureWorkspacesRealmsBootstrappedError, UserOps},
};

fn task_future_factory(event_bus: EventBus, user_ops: Arc<UserOps>) -> impl Future<Output = ()> {
    enum Action {
        WorkspaceChanged,
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
            event_bus.connect(move |_: &EventWorkspaceLocallyCreated| {
                let _ = tx.send(Action::WorkspaceChanged);
            })
        },
        {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventWorkspaceLocallyRenamed| {
                let _ = tx.send(Action::WorkspaceChanged);
            })
        },
    );

    async move {
        let _events_connection_lifetime = events_connection_lifetime;

        loop {
            let action = match rx.recv_async().await {
                Ok(action) => action,
                // Sender has left, time to shutdown !
                Err(_) => return,
            };

            loop {
                let outcome = match action {
                    Action::MissedServerEvents | Action::WorkspaceChanged => {
                        user_ops
                            .ensure_workspaces_realms_created_with_key_and_name()
                            .await
                    }
                };

                match outcome {
                    Ok(_) => (),
                    Err(EnsureWorkspacesRealmsBootstrappedError::Offline) => {
                        event_bus.wait_server_online().await;
                        continue;
                    }
                    Err(EnsureWorkspacesRealmsBootstrappedError::Stopped) => {
                        // Underlying `UserOps` has been stopped, time to shutdown !
                        return;
                    }
                    Err(
                        error @ EnsureWorkspacesRealmsBootstrappedError::TimestampOutOfBallpark {
                            ..
                        },
                    ) => {
                        // Note `WorkspaceOps` is responsible for sending the
                        // bad timestamp event on the event bus
                        log::warn!("Client/server clock drift detected: {:?}", error);
                    }
                    Err(EnsureWorkspacesRealmsBootstrappedError::InvalidCertificate(err)) => {
                        // Unexpected error occured, better stop the monitor
                        log::warn!("Certificate monitor has crashed: {}", err);
                        let event = EventMonitorCrashed {
                            monitor: "workspaces_realm_bootstrap",
                            error: anyhow::anyhow!("Invalid certificate detected: {}", err).into(),
                        };
                        event_bus.send(&event);
                        return;
                    }
                    Err(EnsureWorkspacesRealmsBootstrappedError::Internal(err)) => {
                        // Unexpected error occured, better stop the monitor
                        log::warn!("Certificate monitor has crashed: {}", err);
                        let event = EventMonitorCrashed {
                            monitor: "workspaces_realm_bootstrap",
                            error: err.into(),
                        };
                        event_bus.send(&event);
                        return;
                    }
                }
                break;
            }
        }
    }
}

pub(crate) async fn workspaces_realm_boostrap_factory(
    event_bus: EventBus,
    user_ops: Arc<UserOps>,
) -> Monitor {
    let task_future = {
        let event_bus = event_bus.clone();
        task_future_factory(event_bus, user_ops)
    };
    let task_future = pretend_future_is_send_on_web(task_future);
    Monitor::start(event_bus, task_future, None).await
}
