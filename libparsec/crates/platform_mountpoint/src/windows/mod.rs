// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::sync::Arc;

use libparsec_client::WorkspaceOps;
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct Mountpoint {}

impl Mountpoint {
    pub fn path(&self) -> &std::path::Path {
        todo!()
    }

    pub fn to_os_path(&self, _parsec_path: &FsPath) -> std::path::PathBuf {
        todo!()
    }

    pub async fn mount(_ops: Arc<WorkspaceOps>) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn unmount(self) -> anyhow::Result<()> {
        todo!()
    }
}
