// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use libparsec_types::{AvailableDevice, DeviceFile, DeviceFileType};

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = crate::testbed::maybe_list_available_devices(config_dir) {
        return result;
    }
    #[cfg(not(feature = "test-with-testbed"))]
    let _config_dir = config_dir;

    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(devices)) = storage.get_item("devices") {
            let devices = serde_json::from_str::<Vec<DeviceFile>>(&devices).unwrap_or_default();

            return devices
                .into_iter()
                .map(|x| match x {
                    DeviceFile::Password(device) => (
                        DeviceFileType::Password,
                        device.organization_id,
                        device.device_id,
                        device.human_handle,
                        device.device_label,
                        device.slug,
                    ),
                    DeviceFile::Recovery(device) => (
                        DeviceFileType::Recovery,
                        device.organization_id,
                        device.device_id,
                        device.human_handle,
                        device.device_label,
                        device.slug,
                    ),
                    DeviceFile::Smartcard(device) => (
                        DeviceFileType::Smartcard,
                        device.organization_id,
                        device.device_id,
                        device.human_handle,
                        device.device_label,
                        device.slug,
                    ),
                })
                .map(
                    |(ty, organization_id, device_id, human_handle, device_label, slug)| {
                        AvailableDevice {
                            key_file_path: "".into(),
                            organization_id,
                            device_id,
                            human_handle,
                            device_label,
                            slug,
                            ty,
                        }
                    },
                )
                .collect();
        }
    }

    Vec::new()
}
