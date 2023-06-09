// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// CertificatesOps is a big boy, hence we split it definition in multiple submodules
mod add;
mod poll;
mod storage;
mod validate;

pub use add::*;
pub use poll::*;
pub use validate::*;

use std::{path::Path, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::RwLock;
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

#[derive(Debug)]
pub struct CertificatesOps {
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    storage: RwLock<storage::CertificatesCachedStorage>,
}

impl CertificatesOps {
    pub async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        event_bus: EventBus,
        cmds: Arc<AuthenticatedCmds>,
    ) -> anyhow::Result<Self> {
        let storage =
            storage::CertificatesCachedStorage::start(data_base_dir, device.clone()).await?;
        Ok(Self {
            device,
            event_bus,
            cmds,
            storage: RwLock::new(storage),
        })
    }

    pub async fn stop(&self) {
        self.storage.read().await.stop().await;
    }
}
