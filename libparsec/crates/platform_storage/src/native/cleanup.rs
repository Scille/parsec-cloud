// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{anyhow, DeviceID};

pub async fn remove_device_data(
    data_base_dir: &Path,
    device_id: DeviceID,
) -> Result<(), crate::RemoveDeviceDataError> {
    let path = data_base_dir.join(device_id.hex());
    log::debug!("Removing device data at {}", path.display());

    tokio::fs::remove_dir_all(&path)
        .await
        .map_err(anyhow::Error::from)
        .map_err(Into::into)
}

#[cfg(test)]
#[path = "../../tests/unit/native/cleanup.rs"]
mod test;
