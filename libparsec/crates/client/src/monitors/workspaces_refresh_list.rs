// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventMissedServerEvents, EventMonitorCrashed},
    Client, EventNewCertificates, RefreshWorkspacesListError,
};

const WORKSPACES_REFRESH_LIST_MONITOR_NAME: &str = "workspaces_refresh_list";

pub(crate) async fn start_workspaces_refresh_list_monitor(
    event_bus: EventBus,
    client: Arc<Client>,
) -> Monitor {
    let future = {
        let task_future = task_future_factory(event_bus.clone(), client);
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(
        event_bus,
        WORKSPACES_REFRESH_LIST_MONITOR_NAME,
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
                        client.refresh_workspaces_list().await
                    }
                };
                match outcome {
                    Ok(_) => break,
                    Err(err) => match err {
                        RefreshWorkspacesListError::Offline => {
                            event_bus.wait_server_online().await;
                            continue;
                        }
                        RefreshWorkspacesListError::Stopped => {
                            // Client has been stopped, time to shutdown !
                            return;
                        }

                        err @ (RefreshWorkspacesListError::InvalidEncryptedRealmName(_)
                        | RefreshWorkspacesListError::InvalidKeysBundle(_)) => {
                            log::warn!("Stopping workspaces refresh list monitor due to unexpected outcome: {}", err);
                            return;
                        }

                        RefreshWorkspacesListError::Internal(err) => {
                            // Unexpected error occured, better stop the monitor
                            log::warn!("Workspaces bootstrap monitor has crashed: {}", err);
                            let event = EventMonitorCrashed {
                                monitor: WORKSPACES_REFRESH_LIST_MONITOR_NAME,
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
