// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

pub struct NeedSyncEntries {
    pub remote: Vec<EntryID>,
    pub local: Vec<EntryID>,
}

#[derive(Debug)]
pub struct UserStorageUpdater {}

impl UserStorageUpdater {
    pub async fn set_user_manifest(
        &self,
        _user_manifest: Arc<LocalUserManifest>,
    ) -> anyhow::Result<()> {
        todo!();
    }
}

#[derive(Debug)]
pub struct UserStorage {}

impl UserStorage {
    pub async fn start(_data_base_dir: &Path, _device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        todo!();
    }

    pub(crate) async fn no_populate_start(
        _data_base_dir: &Path,
        _device: Arc<LocalDevice>,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub async fn stop(&self) {
        todo!();
    }

    pub async fn get_realm_checkpoint(&self) -> anyhow::Result<IndexInt> {
        todo!();
    }

    pub async fn update_realm_checkpoint(
        &self,
        _new_checkpoint: IndexInt,
        _remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        todo!();
    }

    pub fn get_user_manifest(&self) -> Arc<LocalUserManifest> {
        todo!();
    }

    pub async fn for_update(&self) -> (UserStorageUpdater, Arc<LocalUserManifest>) {
        todo!();
    }
}

pub async fn user_storage_non_speculative_init(
    _data_base_dir: &Path,
    _device: &LocalDevice,
) -> anyhow::Result<()> {
    todo!();
}
