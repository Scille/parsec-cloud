// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod add;
mod poll;
mod storage;
mod validate;

pub use add::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};
pub use poll::PollServerError;
pub use validate::{InvalidMessageError, ValidateMessageError};

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

    // For readability, we define the public interface here and let the actual
    // implementation in separated submodules

    pub async fn poll_server_for_new_certificates(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<IndexInt, PollServerError> {
        poll::poll_server_for_new_certificates(self, latest_known_index).await
    }

    pub async fn validate_message(
        &self,
        certificate_index: IndexInt,
        index: IndexInt,
        sender: &DeviceID,
        timestamp: DateTime,
        body: &[u8],
    ) -> Result<MessageContent, ValidateMessageError> {
        validate::validate_message(self, certificate_index, index, sender, timestamp, body).await
    }
}
