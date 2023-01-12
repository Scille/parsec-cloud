// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use base64::{engine::general_purpose::STANDARD, Engine};
use std::{convert::Infallible, path::Path};

use libparsec_client_types::{AvailableDevice, DeviceFile, DeviceFileType};

use crate::_DEVICE;

/// TODO: Remove me once wasm doesn't need to mock
pub async fn test_gen_default_devices() {
    let window = web_sys::window().unwrap();

    if let Ok(Some(storage)) = window.local_storage() {
        storage.set_item("devices", _DEVICE).unwrap();
    }
}

pub async fn list_available_devices(_config_dir: &Path) -> Vec<AvailableDevice> {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(devices)) = storage.get_item("devices") {
            return devices
                .split(':')
                .filter_map(|x| STANDARD.decode(x).ok())
                .filter_map(|x| DeviceFile::load(&x).ok())
                .map(|x| match x {
                    DeviceFile::Password(device) => (
                        DeviceFileType::Password,
                        device.organization_id,
                        device.device_id,
                        device.human_handle,
                        device.device_label,
                        // There are no legacy device
                        device.slug.unwrap(),
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
