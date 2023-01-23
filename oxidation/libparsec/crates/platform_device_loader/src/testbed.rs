// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_types::*;
use libparsec_testbed::{test_get_testbed, TestbedTemplate};
use libparsec_types::*;
use std::path::Path;

const STORE_ENTRY_KEY: &str = "platform_device_loader";

// TODO: Here we'd like to have a fast path for when only login is used.
// The idea would be as long as no api doing modification in local devices is
// called, we can generate the LocalDevice object directly from template data
// without having to do password derivation, deserialization and decryption.

struct PseudoPersistentStorage {
    available_devices: Vec<AvailableDevice>,
}

impl PseudoPersistentStorage {
    pub fn new(
        config_dir: &Path,
        organization_addr: &BackendOrganizationAddr,
        template: &TestbedTemplate,
    ) -> Self {
        // Populate the storage from the template.
        // Once done we should no longer need the template data
        let available_devices = template
            .device_files
            .iter()
            .map(|device_file| {
                let device = template
                    .devices
                    .iter()
                    .find(|d| d.device_id == device_file.device_id)
                    .unwrap();
                let user = template
                    .users
                    .iter()
                    .find(|u| &u.user_id == device_file.device_id.user_id())
                    .unwrap();

                AvailableDevice {
                    key_file_path: config_dir
                        .join(format!("{}.key", device_file.device_id.as_ref())),
                    organization_id: organization_addr.organization_id().to_owned(),
                    device_id: device_file.device_id.clone(),
                    human_handle: user.human_handle.clone(),
                    device_label: device.device_label.clone(),
                    slug: local_device_slug(
                        organization_addr.organization_id(),
                        &device.device_id,
                        organization_addr.root_verify_key(),
                    ),
                    ty: DeviceFileType::Password,
                }
            })
            .collect();

        Self { available_devices }
    }
}

/// Retrieve (or create) our pseudo persistent storage, check it type and pass it to `cb`.
/// Return `None` if `config_dir` doesn't correspond to a testbed env.
fn with_pseudo_persistent_storage<T>(
    config_dir: &Path,
    cb: impl FnOnce(&mut PseudoPersistentStorage) -> T,
) -> Option<T> {
    test_get_testbed(config_dir).map(|env| {
        let mut global_store = env.persistence_store.lock().expect("Mutex is poisoned");
        let store = global_store.entry(STORE_ENTRY_KEY).or_insert_with(|| {
            Box::new(PseudoPersistentStorage::new(
                config_dir,
                &env.organization_addr,
                env.template.as_ref(),
            ))
        });
        let store = store
            .downcast_mut::<PseudoPersistentStorage>()
            .expect("Unexpected pseudo persistent storage type for platform_device_loader");
        cb(store)
    })
}

pub fn maybe_list_available_devices(config_dir: &Path) -> Option<Vec<AvailableDevice>> {
    with_pseudo_persistent_storage(config_dir, |store| store.available_devices.clone())
}
