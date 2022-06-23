// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use rstest::fixture;
use std::{path::Path, str::FromStr};

use crate::{FileTransactions, Language, WorkspaceStorage, DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE};
use libparsec_client_types::{LocalManifest, LocalWorkspaceManifest};
use libparsec_types::{DateTime, EntryID, EntryName, WorkspaceEntry};
use tests_fixtures::{alice, tmp_path, Device, TmpPath};

#[fixture]
pub fn alice_workspace_storage(alice: &Device, tmp_path: TmpPath) -> WorkspaceStorage {
    let db_path = tmp_path.join("workspace_storage.sqlite");
    WorkspaceStorage::new(
        Path::new(&db_path),
        alice.local_device(),
        EntryID::default(),
        DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
    )
    .unwrap()
}

#[fixture]
pub async fn alice_file_transactions(
    alice_workspace_storage: WorkspaceStorage,
) -> FileTransactions {
    let timestamp = DateTime::from_ymd(2000, 1, 1);
    let device = alice_workspace_storage.device.clone();
    let workspace_entry =
        WorkspaceEntry::generate(EntryName::from_str("test").unwrap(), device.timestamp());
    let workspace_manifest = LocalWorkspaceManifest::new(
        device.device_id.clone(),
        timestamp,
        Some(workspace_entry.id),
        false,
    );

    let mutex = alice_workspace_storage
        .lock_entry_id(workspace_entry.id)
        .await;
    let guard = mutex.lock().await;

    alice_workspace_storage
        .set_manifest(
            workspace_entry.id,
            LocalManifest::Workspace(workspace_manifest),
            false,
            true,
            None,
        )
        .await
        .unwrap();

    alice_workspace_storage
        .release_entry_id(workspace_entry.id, guard)
        .await;

    // TODO: RemoteLoader

    FileTransactions::new(
        workspace_entry.id,
        device,
        alice_workspace_storage,
        Language::En,
    )
}
