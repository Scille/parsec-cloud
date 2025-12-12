// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{anyhow, DeviceID};

use super::{certificates::get_certificates_storage_db_name, user::get_user_storage_db_name};

async fn drop_db(name: &str) -> anyhow::Result<()> {
    indexed_db::Factory::get()
        .map_err(anyhow::Error::from)?
        .delete_database(name)
        .await
        .map_err(anyhow::Error::from)
}

pub async fn remove_device_data(
    data_base_dir: &Path,
    device_id: DeviceID,
) -> Result<(), crate::RemoveDeviceDataError> {
    drop_db(&get_certificates_storage_db_name(data_base_dir, device_id)).await?;
    drop_db(&get_user_storage_db_name(data_base_dir, device_id)).await?;

    // TODO: Since we don't know the realm IDs, we need to list all the existing
    // databases, but this is not available in web-sys yet.
    // see https://github.com/rustwasm/wasm-bindgen/issues/4439
    // drop_db(&get_workspace_storage_db_name(data_base_dir, device_id)).await?;

    Ok(())
}
