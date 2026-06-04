// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{anyhow, DeviceID};

pub async fn remove_device_data(
    data_base_dir: &Path,
    device_id: DeviceID,
) -> Result<(), crate::RemoveDeviceDataError> {
    let path = data_base_dir.join(device_id.hex());
    log::debug!("Removing device data at {}", path.display());

    match tokio::fs::remove_dir_all(&path).await {
        Ok(()) => Ok(()),
        // Devices created on old organizations never got a per-device data
        // directory, so there may be nothing to remove (see #12807).
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            log::debug!("No device data to remove at {}", path.display());
            Ok(())
        }
        Err(err) => Err(anyhow::Error::from(err).into()),
    }
}

#[cfg(test)]
#[path = "../../tests/unit/native/cleanup.rs"]
mod test;
