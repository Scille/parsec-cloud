// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// UserOps is a big boy, hence we split it definition in multiple submodules
mod create;
mod merge;
mod message;
// mod reencryption;
// mod share;
// mod sync;

pub use create::*;
pub use message::*;
// pub use reencryption::*;
// pub use share::*;
// pub use sync::*;

use std::{path::PathBuf, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_platform_storage2::user::UserStorage;
use libparsec_types::prelude::*;

use crate::{certificates_ops::CertificatesOps, event_bus::EventBus};

#[derive(Debug)]
pub struct UserOps {
    data_base_dir: PathBuf, // TODO: use Arc ?
    device: Arc<LocalDevice>,
    storage: UserStorage,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificatesOps>,
    #[allow(dead_code)]
    event_bus: EventBus,
    // Message processing is done in-order, hence it is pointless to do
    // it concurrently
    process_messages_lock: AsyncMutex<()>,
    update_user_manifest_lock: AsyncMutex<()>,
}

#[derive(Debug)]
pub struct ReencryptionJob {}

impl UserOps {
    pub async fn start(
        data_base_dir: PathBuf,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificatesOps>,
        // remote_device_manager,
        event_bus: EventBus,
        // prevent_sync_pattern,
        // preferred_language,
        // workspace_storage_cache_size,
    ) -> Result<Self, anyhow::Error> {
        // TODO: handle errors
        let storage = UserStorage::start(&data_base_dir, device.clone()).await?;
        Ok(Self {
            data_base_dir,
            device,
            storage,
            cmds,
            certificates_ops,
            event_bus,
            process_messages_lock: AsyncMutex::new(()),
            update_user_manifest_lock: AsyncMutex::new(()),
        })
    }

    pub async fn stop(&self) {
        self.storage.stop().await;
    }
}
