// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::{tokio_spawn, unbounded_channel, JoinHandle};
use libparsec_types::prelude::*;

use crate::{
    certificates_ops::{CertificatesOps, PollServerError},
    event_bus::{
        EventBus, EventCertificatesMonitorCrashed, EventCertificatesUpdated,
        EventMissedServerEvents,
    },
};

pub struct CertificatesMonitor {
    worker: JoinHandle<()>,
}

impl CertificatesMonitor {
    pub async fn start(certifs_ops: Arc<CertificatesOps>, event_bus: EventBus) -> Self {
        enum Action {
            NewCertificate(IndexInt),
            MissedServerEvents,
        }
        let (tx, mut rx) = unbounded_channel::<Action>();
        let _ = tx.send(Action::MissedServerEvents);

        let events_connection_lifetime = (
            {
                let tx = tx.clone();
                event_bus.connect(move |_: &EventMissedServerEvents| {
                    let _ = tx.send(Action::MissedServerEvents);
                })
            },
            event_bus.connect(move |e: &EventCertificatesUpdated| {
                let _ = tx.send(Action::NewCertificate(e.index));
            }),
        );

        let worker = tokio_spawn(async move {
            let _events_connection_lifetime = events_connection_lifetime;

            loop {
                let noop_if_newer_than = match rx.recv().await {
                    Some(Action::MissedServerEvents) => None,
                    Some(Action::NewCertificate(index)) => Some(index),
                    // Sender has left, time to shutdown !
                    // In theory this should never happen given `CertificatesMonitor`
                    // abort our coroutine on teardown instead.
                    None => return,
                };
                // Need a loop here to retry the operation in case the server is not available
                loop {
                    if let Err(err) = certifs_ops
                        .poll_server_for_new_certificates(noop_if_newer_than)
                        .await
                    {
                        match err {
                            PollServerError::Offline => {
                                event_bus.wait_server_online().await;
                                continue;
                            }
                            PollServerError::InvalidCertificate(error) => {
                                // Note `CertificateOps` is responsible for sending the
                                // invalid certificate event on the event bus
                                log::warn!("Invalid certificate detected: {}", error);
                            }
                            PollServerError::Internal(err) => {
                                // Unexpected error occured, better stop the monitor
                                log::warn!("Certificate monitor has crashed: {}", err);
                                let event = EventCertificatesMonitorCrashed(err);
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

impl Drop for CertificatesMonitor {
    fn drop(&mut self) {
        // TODO: aborting the coroutine might be too violent here !
        // My guess is aborting is fine as long as `certif_ops` (and, most importantly,
        // it internals like the certificate storage) is never reused.
        //
        // If that the case, then the abort is analogue to an unexpected application crash,
        // or a suddent shutdown of the computer. Then the only remaning data are the one
        // in the sqlite database which are designed to sustain such event.
        //
        // However if the structures in memory get re-used, then we can end up with
        // inconsistencies.
        //
        // Typically consider an operation where:
        // 1) an async lock is taken,
        // 2) a cache in memory is modified
        // 3) an await is done (that's when the abort may occur !)
        // 4) the cache is again modified
        // 5) the async lock is released
        //
        // So the async lock is suppose to guarantee the state at step 2) is never
        // visible by another coroutine, but that's not the case if the coroutine got
        // aborted.
        // Also note poisoning won't save us here given async mutex doesn't have it,
        // and anyway poisoning is to handle panic where here the unfinished future got
        // a regular drop.
        self.worker.abort()
    }
}
