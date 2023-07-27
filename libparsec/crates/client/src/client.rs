// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#![allow(dead_code)]

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{
    certificates_monitor::CertificatesMonitor, certificates_ops::CertificatesOps,
    config::ClientConfig, connection_monitor::ConnectionMonitor, event_bus::EventBus,
    user_ops::UserOps,
};

// Should not be `Clone` given it manages underlying resources !
pub struct Client {
    stopped: AtomicBool,
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    pub certificates_ops: Arc<CertificatesOps>,
    pub user_ops: UserOps,
    connection_monitor: ConnectionMonitor,
    certificates_monitor: CertificatesMonitor,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("device", &self.device.device_id)
            .finish()
    }
}

impl Client {
    pub fn device_id(&self) -> &DeviceID {
        &self.device.device_id
    }

    pub async fn start(
        config: Arc<ClientConfig>,
        event_bus: EventBus,
        device: Arc<LocalDevice>,
    ) -> Result<Self, anyhow::Error> {
        // TODO: error handling
        let cmds = Arc::new(AuthenticatedCmds::new(
            &config.config_dir,
            device.clone(),
            config.proxy.clone(),
        )?);

        // TODO: error handling
        let certificates_ops = Arc::new(
            CertificatesOps::start(
                config.clone(),
                device.clone(),
                event_bus.clone(),
                cmds.clone(),
            )
            .await?,
        );
        let user_ops = UserOps::start(
            config.clone(),
            device.clone(),
            cmds.clone(),
            certificates_ops.clone(),
            event_bus.clone(),
        )
        .await?;
        // TODO: init workspace ops

        let certifs_monitor =
            CertificatesMonitor::start(certificates_ops.clone(), event_bus.clone()).await;
        // Start the connection monitors last, as it send events to others
        let connection_monitor = ConnectionMonitor::start(cmds.clone(), event_bus.clone()).await;

        Ok(Self {
            stopped: AtomicBool::new(false),
            config,
            device,
            event_bus,
            cmds,
            certificates_ops,
            user_ops,
            connection_monitor,
            certificates_monitor: certifs_monitor,
        })
    }

    pub async fn stop(&self) {
        if self.stopped.load(Ordering::Relaxed) {
            return;
        }
        self.user_ops.stop().await;
        // TODO: stop workspace ops
        self.stopped.store(true, Ordering::Relaxed);
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if !self.stopped.load(Ordering::Relaxed) {
            log::error!("Client dropped without prior stop !");
        }
    }
}
