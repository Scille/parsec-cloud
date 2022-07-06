// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::path::{Path, PathBuf};

use api_types::EntryID;
use client_types::LocalDevice;

const STORAGE_REVISION: u32 = 1;

pub(crate) fn get_workspace_data_storage_db_path(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    let mut path = PathBuf::from(data_base_dir);
    path.extend([
        slug,
        workspace_id.to_string(),
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
        workspace_id.to_string(),
        format!("workspace_cache-v{STORAGE_REVISION}.sqlite"),
    ]);
    path
}
