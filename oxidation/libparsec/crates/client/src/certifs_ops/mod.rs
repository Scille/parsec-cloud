// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#![allow(dead_code)]

use std::{path::Path, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_storage2::CertifsStorage;
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

pub enum Author {
    OrganizationBootstrap,
    Device(DeviceID),
}

pub struct UserInfo {
    author: Author,
    created_on: DateTime,
    revoked_on: Option<DateTime>,
    profile: UserProfile,
}

#[derive(Debug, thiserror::Error)]
pub enum SecureLoadError {
    #[error("Server not currently available")]
    Offline,
    #[error("Corrupted data: {0}")]
    CorruptedData(DataError),
}

#[derive(Debug)]
pub struct CertifsOps {
    device: Arc<LocalDevice>,
    event_bus: EventBus,
    cmds: Arc<AuthenticatedCmds>,
    storage: CertifsStorage,
}

impl CertifsOps {
    pub async fn new(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        event_bus: EventBus,
        cmds: Arc<AuthenticatedCmds>,
    ) -> Result<Self, DynError> {
        let storage = CertifsStorage::start(data_base_dir, device.clone()).await?;
        Ok(Self {
            device,
            event_bus,
            cmds,
            storage,
        })
    }

    pub async fn stop(&self) {
        self.storage.stop().await;
    }

    pub async fn insert_new_certificate(&self, _certificate: &[u8]) {
        todo!()
    }

    pub async fn secure_load_user_manifest(
        &self,
        blob: &[u8],
        expected_id: EntryID,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_version: Option<VersionInt>,
    ) -> Result<UserManifest, SecureLoadError> {
        let expected_author = self.load_device_certificate(expected_author).await?;

        let user_manifest = UserManifest::decrypt_verify_and_load(
            blob,
            &self.device.user_manifest_key,
            &expected_author.verify_key,
            &expected_author.device_id,
            expected_timestamp,
            Some(expected_id),
            expected_version,
        )
        .map_err(SecureLoadError::CorruptedData)?;

        // No rules to enforce: only the user can access it own user manifest, and this
        // access is total and forever

        Ok(user_manifest)
    }

    async fn fetch_missing_certificates(&self) {
        todo!()
        // let index = self.storage.get_last_index();

        // let request = authenticated_cmds::v3::
        // let response = self.cmds.send(request).await;
    }

    async fn load_device_certificate(
        &self,
        _device_id: &DeviceID,
    ) -> Result<DeviceCertificate, SecureLoadError> {
        todo!()
        // match self.storage.get_device_certificate(device_id).await {
        //     None =>
        //     Some()
        // }
    }
}
