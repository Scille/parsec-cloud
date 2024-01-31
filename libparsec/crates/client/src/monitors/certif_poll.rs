// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{channel, pretend_future_is_send_on_web};
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;

use crate::{
    certif::{CertifOps, CertifPollServerError},
    event_bus::*,
};

use super::Monitor;
use crate::event_bus::{EventBus, EventMissedServerEvents};

const CERTIF_POLL_MONITOR_NAME: &str = "certif_poll";

pub(crate) async fn start_certif_poll_monitor(
    certif_ops: Arc<CertifOps>,
    event_bus: EventBus,
) -> Monitor {
    let task_future = {
        let task_future = task_future_factory(certif_ops, event_bus.clone());
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(event_bus, CERTIF_POLL_MONITOR_NAME, None, task_future, None).await
}

fn task_future_factory(
    certif_ops: Arc<CertifOps>,
    event_bus: EventBus,
) -> impl Future<Output = ()> {
    enum Action {
        NewCertificate(PerTopicLastTimestamps),
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
        event_bus.connect(move |e: &EventCertificatesUpdated| {
            let _ = tx.send(Action::NewCertificate(e.last_timestamps.clone()));
        }),
    );

    async move {
        let _events_connection_lifetime = events_connection_lifetime;

        loop {
            let noop_if_newer_than = match rx.recv_async().await {
                Ok(Action::MissedServerEvents) => None,
                Ok(Action::NewCertificate(per_topic_last_timestamps)) => {
                    Some(per_topic_last_timestamps)
                }
                // Sender has left, time to shutdown !
                // In theory this should never happen given `CertificatesMonitor`
                // abort our coroutine on teardown instead.
                Err(_) => return,
            };
            // Need a loop here to retry the operation in case the server is not available
            loop {
                if let Err(err) = certif_ops
                    .poll_server_for_new_certificates(noop_if_newer_than.as_ref())
                    .await
                {
                    match err {
                        CertifPollServerError::Offline => {
                            event_bus.wait_server_online().await;
                            continue;
                        }
                        CertifPollServerError::Stopped => {
                            // Shouldn't occur in practice given the monitors are expected
                            // to be stopped before the opses. In any case we have no
                            // choice but to also stop.
                            return;
                        }
                        // `CertifOps` is already responsible for sending the invalid
                        // certificate event on the event bus, so there nothing more to do !
                        CertifPollServerError::InvalidCertificate(_) => (),
                        CertifPollServerError::Internal(err) => {
                            // Unexpected error occured, better stop the monitor
                            log::warn!("Certificate monitor has crashed: {}", err);
                            let event = EventMonitorCrashed {
                                monitor: CERTIF_POLL_MONITOR_NAME,
                                workspace_id: None,
                                error: Arc::new(err),
                            };
                            event_bus.send(&event);
                            return;
                        }
                    }
                }

                break;
            }
        }
    }
}
