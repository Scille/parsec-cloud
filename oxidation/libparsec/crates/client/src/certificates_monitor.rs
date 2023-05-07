// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::{
    certificates_ops::CertificatesOps,
    event_bus::{EventBus, EventBusConnectionLifetime, EventCertificatesUpdated},
};

pub struct CertificatesMonitor {
    worker: tokio::task::JoinHandle<()>,
    _event_connection: EventBusConnectionLifetime<EventCertificatesUpdated>,
}

impl CertificatesMonitor {
    pub async fn start(certifs_ops: Arc<CertificatesOps>, event_bus: EventBus) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Bytes>();

        let event_connection = event_bus.connect(move |e: &EventCertificatesUpdated| {
            let _ = tx.send(e.certificate.clone());
        });

        let worker = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(certificate) => {
                        certifs_ops.add_new_certificate(certificate).await.unwrap();
                        // TODO: error handling
                    }
                    None => return,
                }
            }
        });

        Self {
            worker,
            _event_connection: event_connection,
        }
    }
}

impl Drop for CertificatesMonitor {
    fn drop(&mut self) {
        self.worker.abort()
    }
}
