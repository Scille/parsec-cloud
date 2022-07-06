// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use rstest::fixture;
use std::path::Path;

use crate::{WorkspaceStorage, DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE};
use api_types::EntryID;
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
