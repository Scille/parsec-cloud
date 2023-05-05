// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

use crate::{event_bus::EventBus, user_ops::UserOps};

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

// pub struct RunningDeviceConfig {
//     pub config_dir: PathBuf,
//     pub data_base_dir: PathBuf,
//     pub mountpoint_base_dir: PathBuf,
//     pub prevent_sync_pattern: Option<Path>,
// }

// Should not be `Clone` given it manages underlying resources !
pub struct RunningDevice {
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    event_bus: Arc<EventBus>,
    pub user_ops: UserOps,
    stopped: AtomicBool,
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

    pub async fn start(device: Arc<LocalDevice>, data_base_dir: &Path) -> Result<Self, DynError> {
        // TODO: error handling
        let event_bus = Arc::new(EventBus::default());
        let cmds = Arc::new(
            AuthenticatedCmds::new(data_base_dir, device.clone(), ProxyConfig::default()).unwrap(),
        );
        let user_ops = UserOps::start(
            data_base_dir.to_owned(),
            device.clone(),
            cmds.clone(),
            event_bus.clone(),
        )
        .await?;
        // TODO: init workspace ops
        Ok(Self {
            device,
            cmds,
            event_bus,
            user_ops,
            stopped: AtomicBool::new(false),
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
