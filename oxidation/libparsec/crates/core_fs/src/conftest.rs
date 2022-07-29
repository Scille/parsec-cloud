// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use rstest::fixture;

use crate::{WorkspaceStorage, DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE};
use libparsec_types::EntryID;
use tests_fixtures::{alice, tmp_path, Device, TmpPath};

pub struct TmpWorkspaceStorage {
    ws: WorkspaceStorage,
    // Parent fixture must be kept to ensure its teardown-on-drop is done after ours
    _tmp_path: TmpPath,
}

impl std::ops::Deref for TmpWorkspaceStorage {
    type Target = WorkspaceStorage;
    fn deref(&self) -> &Self::Target {
        &self.ws
    }
}

impl std::ops::DerefMut for TmpWorkspaceStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ws
    }
}

#[fixture]
pub fn alice_workspace_storage(alice: &Device, tmp_path: TmpPath) -> TmpWorkspaceStorage {
    let db_path = tmp_path.join("workspace_storage.sqlite");

    let ws = WorkspaceStorage::new(
        db_path,
        alice.local_device(),
        EntryID::default(),
        DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
    )
    .unwrap();

    TmpWorkspaceStorage {
        ws,
        _tmp_path: tmp_path,
    }
}
