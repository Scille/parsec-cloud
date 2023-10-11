// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::{channel, spawn, JoinHandle};
use libparsec_types::prelude::*;

use crate::{
    event_bus::{
        EventBus, EventMessageReceived, EventMessagesMonitorCrashed, EventMissedServerEvents,
    },
    user_ops::{ProcessLastMessagesError, UserOps},
};

pub struct MessagesMonitor {
    worker: JoinHandle<()>,
}

impl MessagesMonitor {
    pub async fn start(user_ops: Arc<UserOps>, event_bus: EventBus) -> Self {
        enum Action {
            MessageReceived(IndexInt),
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
            event_bus.connect(move |e: &EventMessageReceived| {
                let _ = tx.send(Action::MessageReceived(e.index));
            }),
        );

        let worker = spawn(async move {
            let _events_connection_lifetime = events_connection_lifetime;

            loop {
                let latest_known_index = match rx.recv_async().await {
                    Ok(Action::MessageReceived(latest_known_index)) => Some(latest_known_index),
                    Ok(Action::MissedServerEvents) => None,
                    // Sender has left, time to shutdown !
                    // In theory this should never happen given `MessagesMonitor`
                    // abort our coroutine on teardown instead.
                    Err(_) => return,
                };
                // Need a loop here to retry the operation in case the server is not available
                loop {
                    if let Err(err) = user_ops.process_last_messages(latest_known_index).await {
                        match err {
                            ProcessLastMessagesError::Offline => {
                                event_bus.wait_server_online().await;
                                continue;
                            }
                            ProcessLastMessagesError::InvalidCertificate(error) => {
                                // Note `UserOps` is responsible for sending the
                                // invalid certificate event on the event bus
                                log::warn!("Invalid certificate detected: {}", error);
                            }
                            ProcessLastMessagesError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::warn!("Certificate monitor has crashed: {}", err);
                                let event = EventMessagesMonitorCrashed(err);
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

impl Drop for MessagesMonitor {
    fn drop(&mut self) {
        // TODO: aborting the coroutine might be too violent here !
        self.worker.abort()
    }
}
