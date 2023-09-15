// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::prelude::*;

use super::super::db::{LocalDatabase, VacuumMode};
use super::super::model::{
    get_workspace_data_storage_db_relative_path, initialize_model_if_needed,
};

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
    realm_id: VlobID,
) -> anyhow::Result<()> {
    // 1) Open the database

    let db_relative_path = get_workspace_data_storage_db_relative_path(device, &realm_id);
    let db =
        LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default()).await?;

    // 2) Initialize the database

    initialize_model_if_needed(&db).await?;

    // 3) Populate the database with the workspace manifest

    let timestamp = device.now();
    let manifest =
        LocalWorkspaceManifest::new(device.device_id.clone(), timestamp, Some(realm_id), false);
    super::data::db_set_workspace_manifest(&db, device, &manifest).await?;

    // 4) All done ! Don't forget to close the database before exiting ;-)

    db.close().await;
    Ok(())
}
