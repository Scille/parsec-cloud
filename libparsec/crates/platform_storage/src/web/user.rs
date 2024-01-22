// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

#[allow(unused)]
pub struct NeedSyncEntries {
    pub remote: Vec<VlobID>,
    pub local: Vec<VlobID>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct UserStorageUpdater {}

#[allow(unused)]
impl UserStorageUpdater {
    pub async fn set_user_manifest(
        &self,
        _user_manifest: Arc<LocalUserManifest>,
    ) -> anyhow::Result<()> {
        todo!();
    }
}

#[derive(Debug)]
pub struct PlatformUserStorage {}

impl PlatformUserStorage {
    #[allow(dead_code)]
    pub async fn start(_data_base_dir: &Path, _device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        todo!();
    }

    pub(crate) async fn no_populate_start(
        _data_base_dir: &Path,
        _device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
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

    pub async fn get_user_manifest(&self) -> anyhow::Result<Option<Vec<u8>>> {
        todo!();
    }

    #[allow(dead_code)]
    pub async fn for_update(&self) -> (UserStorageUpdater, Arc<LocalUserManifest>) {
        todo!();
    }

    pub async fn update_user_manifest(
        &mut self,
        _encrypted: &[u8],
        _need_sync: bool,
        _base_version: VersionInt,
    ) -> anyhow::Result<()> {
        todo!();
    }

    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!();
    }
}

pub async fn user_storage_non_speculative_init(
    _data_base_dir: &Path,
    _device: &LocalDevice,
) -> anyhow::Result<()> {
    todo!();
}
