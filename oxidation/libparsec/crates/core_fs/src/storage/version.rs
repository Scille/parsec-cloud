// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_types::{EntryID, LocalDevice};

const STORAGE_REVISION: u32 = 1;

/// Path relative to config dir
pub(crate) fn get_user_data_storage_db_relative_path(device: &LocalDevice) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([slug, format!("user_data-v{STORAGE_REVISION}.sqlite")])
}

/// Path relative to config dir
pub(crate) fn get_workspace_data_storage_db_relative_path(
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        workspace_id.hex(),
        format!("workspace_data-v{STORAGE_REVISION}.sqlite"),
    ])
}

/// Path relative to config dir
pub(crate) fn get_workspace_cache_storage_db_relative_path(
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    PathBuf::from_iter([
        slug,
        workspace_id.hex(),
        format!("workspace_cache-v{STORAGE_REVISION}.sqlite"),
    ])
}
