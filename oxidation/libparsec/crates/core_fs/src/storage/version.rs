// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::path::{Path, PathBuf};

use parsec_api_types::EntryID;
use parsec_client_types::LocalDevice;

const STORAGE_REVISION: u32 = 1;

pub(crate) fn get_workspace_data_storage_db_path(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    data_base_dir.join(&PathBuf::from(format!(
        "{slug}/{workspace_id}/workspace_data-v{STORAGE_REVISION}.sqlite"
    )))
}

pub(crate) fn get_workspace_cache_storage_db_path(
    data_base_dir: &Path,
    device: &LocalDevice,
    workspace_id: EntryID,
) -> PathBuf {
    let slug = device.slug();
    data_base_dir.join(&PathBuf::from(format!(
        "{slug}/{workspace_id}/workspace_cache-v{STORAGE_REVISION}.sqlite"
    )))
}
