// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod add;
mod poll;
mod storage;
mod validate_manifest;
mod validate_message;

pub use add::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};
pub use poll::PollServerError;
pub use validate_manifest::{InvalidManifestError, ValidateManifestError};
pub use validate_message::{InvalidMessageError, ValidateMessageError};

use std::{collections::HashMap, path::Path, sync::Arc};

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
        validate_message::validate_message(self, certificate_index, index, sender, timestamp, body)
            .await
    }

    pub async fn validate_user_manifest(
        &self,
        certificate_index: IndexInt,
        author: &DeviceID,
        version: VersionInt,
        timestamp: DateTime,
        encrypted: &[u8],
    ) -> Result<UserManifest, ValidateManifestError> {
        validate_manifest::validate_user_manifest(
            self,
            certificate_index,
            author,
            version,
            timestamp,
            encrypted,
        )
        .await
    }

    // Semi-public interface

    pub(crate) async fn encrypt_for_user(
        &self,
        user_id: &UserID,
        data: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let guard = self.storage.read().await;
        match guard
            .get_user_certificate(storage::UpTo::Current, user_id)
            .await
        {
            Ok(certif) => Ok(Some(certif.public_key.encrypt_for_self(data))),
            Err(storage::GetCertificateError::NonExisting) => Ok(None),
            Err(storage::GetCertificateError::ExistButTooRecent { .. }) => {
                unreachable!()
            }
            Err(err @ storage::GetCertificateError::Internal(_)) => Err(err.into()),
        }
    }

    pub(crate) async fn encrypt_for_sequester_services(
        &self,
        data: &[u8],
    ) -> anyhow::Result<Option<HashMap<SequesterServiceID, Bytes>>> {
        let guard = self.storage.read().await;
        match guard
            .get_sequester_authority_certificate(storage::UpTo::Current)
            .await
        {
            Ok(_) => {
                let services = guard
                    .get_sequester_service_certificates(storage::UpTo::Current)
                    .await?;
                let per_service_encrypted = services
                    .map(|service| {
                        (
                            service.service_id,
                            service.encryption_key_der.encrypt(data).into(),
                        )
                    })
                    .collect();
                Ok(Some(per_service_encrypted))
            }
            Err(storage::GetCertificateError::NonExisting) => {
                // Not a sequestered organization
                Ok(None)
            }
            Err(storage::GetCertificateError::ExistButTooRecent { .. }) => {
                unreachable!()
            }
            Err(err @ storage::GetCertificateError::Internal(_)) => Err(err.into()),
        }
    }
}
