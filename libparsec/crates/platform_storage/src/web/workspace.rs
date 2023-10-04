// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

pub async fn workspace_storage_non_speculative_init(
    _data_base_dir: &Path,
    _device: &LocalDevice,
    _workspace_id: VlobID,
) -> anyhow::Result<()> {
    todo!()
}

/*
 * Workspace manifest updater
 */

#[derive(Debug)]
pub struct WorkspaceDataStorageWorkspaceManifestUpdater {}

impl WorkspaceDataStorageWorkspaceManifestUpdater {
    /// Create a brand new manifest (in order to add it as child).
    ///
    /// Never use this method to update an existing file manifest as it ignores
    /// the work-ahead-of-db items (hence data may end up in an invalid state) !
    pub async fn new_child_file_manifest(
        &self,
        _manifest: Arc<LocalFileManifest>,
    ) -> anyhow::Result<()> {
        todo!();
    }

    /// Create a brand new manifest (in order to add it as child).
    pub async fn new_child_folder_manifest(
        &self,
        _manifest: Arc<LocalFolderManifest>,
    ) -> anyhow::Result<()> {
        todo!();
    }

    pub async fn update_workspace_manifest(
        self,
        _manifest: Arc<LocalWorkspaceManifest>,
    ) -> anyhow::Result<()> {
        todo!();
    }
}

/*
 * Child manifests updater & lock
 */

#[derive(Debug)]
pub struct WorkspaceDataStorageChildManifestUpdater {}

impl WorkspaceDataStorageChildManifestUpdater {
    /// Create a brand new manifest (in order to add it as child).
    ///
    /// Never use this method to update an existing file manifest as it ignores
    /// the work-ahead-of-db items (hence data may end up in an invalid state) !
    pub async fn new_child_file_manifest(
        &self,
        _manifest: Arc<LocalFileManifest>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    /// Create a brand new manifest (in order to add it as child).
    pub async fn new_child_folder_manifest(
        &self,
        _manifest: Arc<LocalFolderManifest>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    /// Update the given manifest as file.
    ///
    /// A manifest entry is supposed to always point to the same type of manifest,
    /// it is the caller responsibility to make sure it updates with the right
    ///
    /// If nothing to remove, use `[].into_iter()` for `to_remove` param.
    ///
    /// `delay_flush` is to be used when the file is opened (given then the `flush`
    /// syscall should be used to guarantee the data are persistent)
    pub async fn update_as_file_manifest(
        self,
        _manifest: Arc<LocalFileManifest>,
        _delay_flush: bool,
        _to_remove: impl Iterator<Item = ChunkID>,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    pub async fn update_as_folder_manifest(
        self,
        _manifest: Arc<LocalFolderManifest>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

/*
 * WorkspaceDataStorage & friends
 */

#[derive(Debug)]
pub struct WorkspaceDataStorage {
    pub realm_id: VlobID,
    pub device: Arc<LocalDevice>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetChildManifestError {
    #[error("Manifest not present in data storage")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct NeedSyncEntries {
    pub local: Vec<VlobID>,
    pub remote: Vec<VlobID>,
}

impl WorkspaceDataStorage {
    pub async fn start(
        _data_base_dir: &Path,
        _device: Arc<LocalDevice>,
        _realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub(crate) async fn no_populate_start(
        _data_base_dir: &Path,
        _device: Arc<LocalDevice>,
        _realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        todo!();
    }

    /// Updating the workspace manifest is error prone:
    /// 1) the workspace manifest update lock must be held
    /// 2) the workspace manifest must be fetched *after* the lock is held
    /// This method (and the related updater structure) make sure both requirements
    /// are met before providing the method to actually update the manifest.
    pub async fn for_update_workspace_manifest(
        &self,
    ) -> (
        WorkspaceDataStorageWorkspaceManifestUpdater,
        Arc<LocalWorkspaceManifest>,
    ) {
        todo!();
    }

    pub async fn for_update_child_manifest(
        &self,
        _entry_id: VlobID,
    ) -> Result<
        (
            WorkspaceDataStorageChildManifestUpdater,
            Option<ArcLocalChildManifest>,
        ),
        anyhow::Error,
    > {
        todo!();
    }

    // Sync pattern

    // TODO: Should we introduce a lock on the prevent sync pattern apply
    // operations ?
    // Typically this operation would be cancelled if another call to it
    // occurs.

    /// Set the "prevent sync" pattern for the corresponding workspace.
    ///
    /// This operation is idempotent,
    /// i.e. it does not reset the `fully_applied` flag if the pattern hasn't changed.
    pub async fn set_prevent_sync_pattern(&self, _pattern: Regex) -> Result<bool, anyhow::Error> {
        todo!();
    }

    /// Mark the provided pattern as fully applied.
    ///
    /// This is meant to be called after one made sure that all the manifests in the
    /// workspace are compliant with the new pattern. The applied pattern is provided
    /// as an argument in order to avoid concurrency issues.
    pub async fn mark_prevent_sync_pattern_fully_applied(
        &self,
        _pattern: Regex,
    ) -> Result<bool, anyhow::Error> {
        todo!();
    }

    // Checkpoint operations

    // Each time a vlob is created/uploaded, this increment a counter in the
    // corresponding realm: this is the realm checkpoint.
    // The idea is the client (*us* !) can query the server with the last checkpoint
    // it knows of, and receive the new checkpoint along with a list of all the
    // vlob that have been created/updated.
    //
    // However this system comes with it own challenges: we only save on database
    // the remote version of the vlob we know about.
    // 1) Client knows about realm checkpoint 10
    // 1) Client fetches vlob 0x42 at version 1 from server
    // 2) Vlob 0x42 is updated at version 2 on server
    // 3) Client ask the server what has changed since the realm checkpoint 10.
    //    Server answer with realm checkpoint 11 with vlob 0x42 now at version 2
    // 4) Client doesn't have vlob 0x42 on database hence discard the related
    //    information but update the realm checkpoint in database to 11
    // 5) Client now update vlob 0x42 with information from step 1)
    // 6) Now the database contains realm checkpoint 11 but vlob 0x42 with realm
    //    version 1 ! The vlob won't be synced until it gets another update !
    //
    // The solution to this is to make "fetch vlob + update in database" operation
    // exclusive with "poll vlob changes + update in database". This should be
    // achieved by the caller, typically with read/write lock.

    pub async fn get_realm_checkpoint(&self) -> Result<IndexInt, anyhow::Error> {
        todo!();
    }

    pub async fn update_realm_checkpoint(
        &self,
        _new_checkpoint: IndexInt,
        _changed_vlobs: Vec<(VlobID, VersionInt)>,
    ) -> Result<(), anyhow::Error> {
        todo!();
    }

    pub async fn get_need_sync_entries(&self) -> Result<NeedSyncEntries, anyhow::Error> {
        todo!();
    }

    // Manifest operations

    pub fn get_workspace_manifest(&self) -> Arc<LocalWorkspaceManifest> {
        todo!();
    }

    pub async fn get_child_manifest(
        &self,
        _entry_id: VlobID,
    ) -> Result<ArcLocalChildManifest, GetChildManifestError> {
        todo!();
    }

    pub async fn ensure_manifest_persistent(&self, _entry_id: VlobID) -> Result<(), anyhow::Error> {
        todo!();
    }
}

/*
 * WorkspaceCacheStorage
 */

#[derive(Debug)]
pub struct WorkspaceCacheStorage {
    pub realm_id: VlobID,
    pub device: Arc<LocalDevice>,
}

impl WorkspaceCacheStorage {
    pub async fn start(
        _data_base_dir: &Path,
        _cache_size: u64,
        _device: Arc<LocalDevice>,
        _realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub(crate) async fn no_populate_start(
        _data_base_dir: &Path,
        _cache_size: u64,
        _device: Arc<LocalDevice>,
        _realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        todo!();
    }

    pub async fn stop(&self) {
        todo!();
    }

    /// Returns the block data in cleartext or None if not present
    pub async fn get_block(&self, _id: BlockID) -> Result<Option<Vec<u8>>, anyhow::Error> {
        todo!();
    }

    pub async fn set_block(
        &self,
        _id: BlockID,
        _cleartext_block: &[u8],
    ) -> Result<(), anyhow::Error> {
        todo!();
    }

    pub async fn clear_block(&self, _id: BlockID) -> Result<(), anyhow::Error> {
        todo!();
    }

    // TODO: do we really need this one ?
    pub async fn get_local_block_ids(
        &self,
        _ids: Vec<BlockID>,
    ) -> Result<Vec<BlockID>, anyhow::Error> {
        todo!()
    }

    pub async fn cleanup(&self) -> Result<(), anyhow::Error> {
        todo!();
    }

    pub async fn vacuum(&self) -> Result<(), anyhow::Error> {
        todo!();
    }
}
