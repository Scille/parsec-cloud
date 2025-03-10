// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use platform::*;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use std::path::Path;

    pub fn create_device_file(path: &Path, content: &[u8]) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    pub fn key_present_in_system(path: &Path) -> bool {
        path.exists()
    }

    pub fn key_is_archived(path: &Path) -> bool {
        let expected_archive_path = path.with_extension("device.archived");
        !key_present_in_system(path) && key_present_in_system(&expected_archive_path)
    }
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use crate::web::add_item_to_list;
    use std::{collections::HashSet, path::Path};

    fn get_storage() -> web_sys::Storage {
        web_sys::window()
            .unwrap()
            .session_storage()
            .unwrap()
            .unwrap()
    }

    fn key_from_path(path: &Path) -> &str {
        path.to_str().expect("Cannot convert path to key")
    }

    pub fn create_device_file(path: &Path, content: &[u8]) {
        let key = key_from_path(path);
        let b64_data = data_encoding::BASE64.encode(content);
        let storage = get_storage();
        storage
            .set_item(key, &b64_data)
            .expect("Cannot add device to storage");

        add_item_to_list(&storage, "parsec_devices", key).expect("Cannot register device");
    }

    pub fn key_in_list(list_id: &'static str, key: &str) -> bool {
        let storage = get_storage();
        let raw_list = storage
            .get_item(list_id)
            .expect("Cannot get list of devices");

        let list = raw_list
            .as_ref()
            .map(|v| serde_json::from_str::<HashSet<&str>>(v))
            .unwrap_or_else(|| Ok(HashSet::new()))
            .expect("Failed to parse list of devices");

        list.contains(key)
    }

    pub fn key_present_in_system(path: &Path) -> bool {
        let key = key_from_path(path);
        key_in_list("parsec_devices", key)
    }

    pub fn key_is_archived(path: &Path) -> bool {
        let key = key_from_path(path);
        !key_in_list("parsec_devices", key) && key_in_list("archived_parsec_devices", key)
    }
}
