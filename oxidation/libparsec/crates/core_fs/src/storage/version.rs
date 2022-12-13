// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec_client_types::LocalDevice;
use libparsec_types::EntryID;

const STORAGE_REVISION: u32 = 1;

pub(crate) fn get_user_data_storage_db_path(data_base_dir: &Path, device: &LocalDevice) -> PathBuf {
    let slug = device.slug();
    let mut path = PathBuf::from(data_base_dir);
    path.extend([slug, format!("user_data-v{STORAGE_REVISION}.sqlite")]);
    path
}

pub(crate) fn get_workspace_data_storage_db_path(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    let mut path = PathBuf::from(data_base_dir);
    path.extend([
        slug,
        workspace_id.hex(),
        format!("workspace_data-v{STORAGE_REVISION}.sqlite"),
    ]);
    path
}

pub(crate) fn get_workspace_cache_storage_db_path(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    let mut path = PathBuf::from(data_base_dir);
    path.extend([
        slug,
        workspace_id.hex(),
        format!("workspace_cache-v{STORAGE_REVISION}.sqlite"),
    ]);
    path
}
