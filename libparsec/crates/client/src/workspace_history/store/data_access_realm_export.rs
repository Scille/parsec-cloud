// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use super::{
    DataAccessFetchBlockError, DataAccessFetchManifestError, WorkspaceHistoryRealmExportDecryptor,
};

pub(super) struct RealmExportDataAccess {
    #[allow(dead_code)]
    export_db_path: std::path::PathBuf,
    #[allow(dead_code)]
    decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
}

impl RealmExportDataAccess {
    pub async fn start(
        _export_db_path: std::path::PathBuf,
        _decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
    ) -> (Self, VlobID) {
        todo!()
    }

    pub async fn fetch_manifest(
        &self,
        _at: DateTime,
        _entry_id: VlobID,
    ) -> Result<Option<ArcChildManifest>, DataAccessFetchManifestError> {
        todo!()
    }

    pub async fn fetch_block(
        &self,
        _manifest: &FileManifest,
        _access: &BlockAccess,
    ) -> Result<Bytes, DataAccessFetchBlockError> {
        todo!()
    }

    pub async fn get_workspace_manifest_v1(
        &self,
    ) -> Result<Arc<FolderManifest>, DataAccessFetchManifestError> {
        todo!()
    }
}
