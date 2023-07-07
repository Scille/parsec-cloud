// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_testbed::{
    test_get_testbed, test_get_testbed_component_store, TestbedEnv, TestbedEvent,
};
use libparsec_types::prelude::*;

const STORE_ENTRY_KEY: &str = "platform_device_loader";

// TODO: Here we'd like to have a fast path for when only login is used.
// The idea would be as long as no api doing modification in local devices is
// called, we can generate the LocalDevice object directly from template data
// without having to do password derivation, deserialization and decryption.

enum MaybePopulated<T> {
    Stalled,
    Populated(T),
}

struct ComponentStore {
    available_devices: Mutex<MaybePopulated<Vec<AvailableDevice>>>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        available_devices: Mutex::new(MaybePopulated::Stalled),
    })
}

fn populate_available_devices(config_dir: &Path, env: &TestbedEnv) -> Vec<AvailableDevice> {
    // Populate the storage from the template.
    // Once done we should no longer need the template data
    env.template
        .events
        .iter()
        .filter_map(|e| {
            let (device_id, human_handle, device_label) = match e {
                TestbedEvent::BootstrapOrganization(x) => (
                    &x.first_user_device_id,
                    &x.first_user_human_handle,
                    &x.first_user_first_device_label,
                ),

                TestbedEvent::NewUser(x) => (&x.device_id, &x.human_handle, &x.first_device_label),

                TestbedEvent::NewDevice(x) => {
                    let user_id = x.device_id.user_id();
                    let user_human_handle = env
                        .template
                        .events
                        .iter()
                        .find_map(|e| match e {
                            TestbedEvent::BootstrapOrganization(e)
                                if e.first_user_device_id.user_id() == user_id =>
                            {
                                Some(&e.first_user_human_handle)
                            }
                            TestbedEvent::NewUser(e) if e.device_id.user_id() == user_id => {
                                Some(&e.human_handle)
                            }
                            _ => None,
                        })
                        .expect("Must exist");
                    (&x.device_id, user_human_handle, &x.device_label)
                }

                _ => return None,
            };

            let available_device = AvailableDevice {
                key_file_path: config_dir.join(format!("{}.key", device_id)),
                organization_id: env.organization_id.clone(),
                device_id: device_id.clone(),
                human_handle: human_handle.clone(),
                device_label: device_label.clone(),
                slug: local_device_slug(
                    &env.organization_id,
                    device_id,
                    env.organization_addr().root_verify_key(),
                ),
                ty: DeviceFileType::Password,
            };

            Some(available_device)
        })
        .collect()
}

pub(crate) fn maybe_list_available_devices(config_dir: &Path) -> Option<Vec<AvailableDevice>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut maybe_populated = store.available_devices.lock().expect("Mutex is poisoned");
            match &*maybe_populated {
                MaybePopulated::Populated(available_devices) => available_devices.clone(),
                MaybePopulated::Stalled => {
                    let env = test_get_testbed(config_dir).expect("Must exist");
                    let available_devices = populate_available_devices(config_dir, &env);
                    *maybe_populated = MaybePopulated::Populated(available_devices.clone());
                    available_devices
                }
            }
        })
}
