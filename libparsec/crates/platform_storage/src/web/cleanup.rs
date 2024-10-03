// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{anyhow, DeviceID};

pub async fn remove_device_data(
    _data_base_dir: &Path,
    _device_id: &DeviceID,
) -> anyhow::Result<()> {
    todo!()
}
