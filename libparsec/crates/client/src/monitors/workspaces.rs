// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};
use libparsec_types::prelude::*;

use super::Monitor;
use crate::{
    event_bus::{
        EventBus, EventMonitorCrashed, EventWorkspaceLocallyCreated, EventWorkspaceLocallyRenamed,
        EventWorkspaceRemotelyRenamed,
    },
    ClientEnsureWorkspacesBootstrappedError, ClientOps,
};

fn task_future_factory(
    event_bus: EventBus,
    client_ops: Arc<ClientOps>,
) -> impl Future<Output = ()> {
    enum Action {
        WorkspaceRemotelyRenamed,
        WorkspaceLocallyCreated,
        WorkspaceLocallyRenamed,
    }
    // Channel starts empty, hence the monitor will stay idle until the connection
    // monitor triggers its initial `EventWorkspaceRemotelyRenamed` event
    let (tx, rx) = channel::unbounded::<Action>();

    // Register events

    let events_connection_lifetime = (
        // TODO: finish this explanation !!!!!!
        // Unlike other monitors, we don't watch `EventMissedServerEvents` but instead
        // `EventWorkspaceRemotelyRenamed`. The reason is twofold:
        // - It minimizes the risk of overwriting realm name certificate in case a rename
        //   occured both remotely and in our client while offline.
        // - We refresh the cache
        {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventWorkspaceRemotelyRenamed| {
                let _ = tx.send(Action::WorkspaceRemotelyRenamed);
            })
        },
        {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventWorkspaceLocallyCreated| {
                let _ = tx.send(Action::WorkspaceLocallyCreated);
            })
        },
        {
            let tx = tx.clone();
            event_bus.connect(move |_: &EventWorkspaceLocallyRenamed| {
                let _ = tx.send(Action::WorkspaceLocallyRenamed);
            })
        },
    );

    // Actual background task

    let task_future = async move {
        // TODO: It is possible the user manifest is not up-to-date with the certificates
        //       present in local storage (e.g. the client was suddenly killed while
        //       processing the new certificates).
        //       So we must start by ensuring the cache is up-to-date.

        let _events_connection_lifetime = events_connection_lifetime;

        loop {
            let action = match rx.recv_async().await {
                Ok(action) => action,
                // Sender has left, time to shutdown !
                Err(_) => return,
            };

            loop {
                let outcome = match action {
                    Action::WorkspaceRemotelyRenamed
                    | Action::WorkspaceLocallyCreated
                    | Action::WorkspaceLocallyRenamed => {
                        client_ops.ensure_workspaces_minimal_sync().await
                    }
                };

                match outcome {
                    Ok(_) => (),
                    Err(ClientEnsureWorkspacesBootstrappedError::Offline) => {
                        event_bus.wait_server_online().await;
                        continue;
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::Stopped) => {
                        // Underlying client has stopped, time to shutdown !
                        return;
                    }
                    Err(
                        error @ ClientEnsureWorkspacesBootstrappedError::TimestampOutOfBallpark {
                            ..
                        },
                    ) => {
                        // Note `Client` is responsible for sending the
                        // bad timestamp event on the event bus
                        log::warn!("Client/server clock drift detected: {:?}", error);
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::InvalidCertificate(err)) => {
                        // Unexpected error occured, better stop the monitor
                        log::warn!("Certificate monitor has crashed: {}", err);
                        let event = EventMonitorCrashed {
                            monitor: "workspaces",
                            error: anyhow::anyhow!("Invalid certificate detected: {}", err).into(),
                        };
                        event_bus.send(&event);
                        return;
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::Internal(err)) => {
                        // Unexpected error occured, better stop the monitor
                        log::warn!("Certificate monitor has crashed: {}", err);
                        let event = EventMonitorCrashed {
                            monitor: "workspaces",
                            error: err.into(),
                        };
                        event_bus.send(&event);
                        return;
                    }
                }
                break;
            }
        }
    };

    task_future
}

pub(crate) async fn workspaces_monitor_factory(
    event_bus: EventBus,
    client_ops: Arc<ClientOps>,
) -> Monitor {
    let task_future = task_future_factory(event_bus, client_ops);
    let task_future = pretend_future_is_send_on_web(task_future);
    Monitor::start(task_future, None).await
}
