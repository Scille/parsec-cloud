// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventMissedServerEvents, EventMonitorCrashed},
    Client, ClientProcessWorkspacesNeedsError, EventNewCertificates,
};

const WORKSPACES_PROCESS_NEEDS_MONITOR_NAME: &str = "workspaces_process_needs";

pub(crate) async fn start_workspaces_process_needs_monitor(
    event_bus: EventBus,
    client: Arc<Client>,
) -> Monitor {
    let future = {
        let task_future = task_future_factory(event_bus.clone(), client);
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(
        event_bus,
        WORKSPACES_PROCESS_NEEDS_MONITOR_NAME,
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
            // TODO: be more clever to avoid full refresh while it is most
            //       likely nothing has changed !
            event_bus.connect(move |_: &EventMissedServerEvents| {
                let _ = tx.send(Action::MissedServerEvents);
            })
        },
        {
            let tx = tx.clone();
            // TODO: pretty broad event, it would be better to only refresh
            //       the realm of the workspace that has been updated, and to
            //       detect the most common case where we already know the name
            //       and the new certificate hasn't changed it.
            event_bus.connect(move |_: &EventNewCertificates| {
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
                        client.process_workspaces_needs().await
                    }
                };
                match outcome {
                    Ok(_) => break,
                    Err(err) => match err {
                        ClientProcessWorkspacesNeedsError::Offline(_) => {
                            event_bus.wait_server_reconnect().await;
                            continue;
                        }
                        ClientProcessWorkspacesNeedsError::Stopped => {
                            // Client has been stopped, time to shutdown !
                            return;
                        }
                        err @ (ClientProcessWorkspacesNeedsError::TimestampOutOfBallpark {
                            ..
                        }
                        | ClientProcessWorkspacesNeedsError::InvalidCertificate(_)) => {
                            log::error!("Stopping workspaces process needs monitor due to unexpected outcome: {}", err);
                            return;
                        }
                        ClientProcessWorkspacesNeedsError::Internal(err) => {
                            // Unexpected error occured, better stop the monitor
                            log::error!("Workspaces bootstrap monitor has crashed: {}", err);
                            let event = EventMonitorCrashed {
                                monitor: WORKSPACES_PROCESS_NEEDS_MONITOR_NAME,
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
