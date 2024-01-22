// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};

use super::Monitor;
use crate::{
    event_bus::{
        EventBus, EventMissedServerEvents, EventMonitorCrashed, EventWorkspaceLocallyCreated,
    },
    Client, ClientEnsureWorkspacesBootstrappedError,
};

const WORKSPACES_BOOTSTRAP_MONITOR_NAME: &str = "workspaces_bootstrap";

pub(crate) async fn start_workspaces_boostrap_monitor(
    event_bus: EventBus,
    client: Arc<Client>,
) -> Monitor {
    let future = {
        let task_future = task_future_factory(event_bus.clone(), client);
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(
        event_bus,
        WORKSPACES_BOOTSTRAP_MONITOR_NAME,
        None,
        future,
        None,
    )
    .await
}

fn task_future_factory(event_bus: EventBus, client: Arc<Client>) -> impl Future<Output = ()> {
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
                        client.ensure_workspaces_bootstrapped().await
                    }
                };

                match outcome {
                    Ok(_) => (),
                    Err(ClientEnsureWorkspacesBootstrappedError::Offline) => {
                        event_bus.wait_server_online().await;
                        continue;
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::Stopped) => {
                        // Client has been stopped, time to shutdown !
                        return;
                    }
                    Err(
                        error @ ClientEnsureWorkspacesBootstrappedError::TimestampOutOfBallpark {
                            ..
                        },
                    ) => {
                        // Note ops components are responsible for sending the
                        // bad timestamp event on the event bus
                        log::warn!("Client/server clock drift detected: {:?}", error);
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::InvalidKeysBundle(error)) => {
                        // Note ops components are responsible for sending the
                        // invalid certificate event on the event bus
                        log::warn!("Invalid keys bundle detected: {}", error);
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::InvalidCertificate(error)) => {
                        // Note ops components are responsible for sending the
                        // invalid certificate event on the event bus
                        log::warn!("Invalid certificate detected: {}", error);
                    }
                    Err(ClientEnsureWorkspacesBootstrappedError::Internal(err)) => {
                        // Unexpected error occured, better stop the monitor
                        log::warn!("Workspaces bootstrap monitor has crashed: {}", err);
                        let event = EventMonitorCrashed {
                            monitor: WORKSPACES_BOOTSTRAP_MONITOR_NAME,
                            workspace_id: None,
                            error: Arc::new(err),
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
