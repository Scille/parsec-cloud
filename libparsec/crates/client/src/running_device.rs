// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(dead_code)]

use std::{
    fmt::Debug,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_types::prelude::*;

use crate::{
    certificates_monitor::CertificatesMonitor, certificates_ops::CertificatesOps,
    connection_monitor::ConnectionMonitor, event_bus::EventBus, user_ops::UserOps,
};

// pub struct RunningDeviceConfig {
//     pub config_dir: PathBuf,
//     pub data_base_dir: PathBuf,
//     pub mountpoint_base_dir: PathBuf,
//     pub prevent_sync_pattern: Option<Path>,
// }

// Should not be `Clone` given it manages underlying resources !
pub struct RunningDevice {
    stopped: AtomicBool,
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    pub certificates_ops: Arc<CertificatesOps>,
    pub user_ops: UserOps,
    connection_monitor: ConnectionMonitor,
    certificates_monitor: CertificatesMonitor,
}

impl Debug for RunningDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningDevice")
            .field("device", &self.device.device_id)
            .finish()
    }
}

impl RunningDevice {
    pub fn device_id(&self) -> &DeviceID {
        &self.device.device_id
    }

    pub async fn start(
        device: Arc<LocalDevice>,
        data_base_dir: &Path,
    ) -> Result<Self, anyhow::Error> {
        let event_bus = EventBus::default();
        // TODO: error handling
        let cmds = Arc::new(AuthenticatedCmds::new(
            data_base_dir,
            device.clone(),
            ProxyConfig::default(),
        )?);

        // TODO: error handling
        let certificates_ops = Arc::new(
            CertificatesOps::start(
                data_base_dir,
                device.clone(),
                event_bus.clone(),
                cmds.clone(),
            )
            .await?,
        );
        let user_ops = UserOps::start(
            data_base_dir.to_owned(),
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

impl Drop for RunningDevice {
    fn drop(&mut self) {
        if !self.stopped.load(Ordering::Relaxed) {
            log::error!("RunningDevice dropped without prior stop !");
        }
    }
}
