// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use libparsec_testbed::{
    test_get_testbed, test_get_testbed_component_store, TestbedEnv, TestbedEvent,
};
use libparsec_types::prelude::*;

use crate::{ChangeAuthentificationError, LoadDeviceError, SaveDeviceError};

const STORE_ENTRY_KEY: &str = "platform_device_loader";
const KEY_FILE_PASSWORD: &str = "P@ssw0rd."; // Use the same password for all simulated key files

enum MaybePopulated<T> {
    Stalled,
    Populated(T),
}

#[derive(Default)]
struct KeyFilesCache {
    available: Vec<(DeviceAccessStrategy, Arc<LocalDevice>)>,
    destroyed: Vec<DeviceAccessStrategy>,
}

struct ComponentStore {
    available_devices: Mutex<MaybePopulated<Vec<AvailableDevice>>>,
    key_files_cache: Mutex<KeyFilesCache>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        available_devices: Mutex::new(MaybePopulated::Stalled),
        key_files_cache: Mutex::default(),
    })
}

/// For simplicity we store the key file as `<config_dir>/devices/<device_id>.keys`
/// This way `device_load` is more explicit than if we use the slughash in the file
/// name (like it is done in production)
fn get_device_key_file(config_dir: &Path, device_id: &DeviceID) -> PathBuf {
    config_dir.join(format!("devices/{}.keys", device_id))
}

/// Generate the `LocalDevice` from the template events, this saves us from
/// password derivation, generation of the key file, only to do it
/// deserialization&decryption right away.
fn load_local_device(key_file: &Path, env: &TestbedEnv) -> Option<Arc<LocalDevice>> {
    // Try to extract the DeviceID from the key file, then make sure the key file
    // location correspond to where we store the testbed fake files
    let device_id = key_file.file_stem()?.to_str()?.parse::<DeviceID>().ok()?;
    let expected_key_file = get_device_key_file(&env.discriminant_dir, &device_id);
    if key_file != expected_key_file {
        return None;
    }

    env.template.events.iter().find_map(|e| match e {
        TestbedEvent::BootstrapOrganization(x) if x.first_user_device_id == device_id => {
            Some(Arc::new(LocalDevice {
                organization_addr: (*env.organization_addr()).clone(),
                device_id: device_id.clone(),
                device_label: x.first_user_first_device_label.clone(),
                human_handle: x.first_user_human_handle.clone(),
                signing_key: x.first_user_first_device_signing_key.clone(),
                private_key: x.first_user_private_key.clone(),
                initial_profile: UserProfile::Admin,
                user_realm_id: x.first_user_user_realm_id,
                user_realm_key: x.first_user_user_realm_key.clone(),
                local_symkey: x.first_user_local_symkey.clone(),
                time_provider: TimeProvider::default(),
            }))
        }
        TestbedEvent::NewUser(x) if x.device_id == device_id => Some(Arc::new(LocalDevice {
            organization_addr: (*env.organization_addr()).clone(),
            device_id: device_id.clone(),
            device_label: x.first_device_label.clone(),
            human_handle: x.human_handle.clone(),
            signing_key: x.first_device_signing_key.clone(),
            private_key: x.private_key.clone(),
            initial_profile: x.initial_profile,
            user_realm_id: x.user_realm_id,
            user_realm_key: x.user_realm_key.clone(),
            local_symkey: x.local_symkey.clone(),
            time_provider: TimeProvider::default(),
        })),
        TestbedEvent::NewDevice(d) if d.device_id == device_id => {
            env.template.events.iter().find_map(|e| match e {
                TestbedEvent::NewUser(u) if u.device_id.user_id() != device_id.user_id() => {
                    Some(Arc::new(LocalDevice {
                        organization_addr: (*env.organization_addr()).clone(),
                        device_id: device_id.clone(),
                        device_label: d.device_label.clone(),
                        human_handle: u.human_handle.clone(),
                        signing_key: d.signing_key.clone(),
                        private_key: u.private_key.clone(),
                        initial_profile: u.initial_profile,
                        user_realm_id: u.user_realm_id,
                        user_realm_key: u.user_realm_key.clone(),
                        local_symkey: d.local_symkey.clone(),
                        time_provider: TimeProvider::default(),
                    }))
                }
                _ => None,
            })
        }
        _ => None,
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
                key_file_path: get_device_key_file(config_dir, device_id),
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

pub(crate) fn maybe_load_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Option<Result<Arc<LocalDevice>, LoadDeviceError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .and_then(|store| {
            // 1) Try to load from the cache

            let mut cache = store.key_files_cache.lock().expect("Mutex is poisoned");
            let found =
                cache
                    .available
                    .iter()
                    .find_map(|(c_access, c_device)| match (access, c_access) {
                        (
                            DeviceAccessStrategy::Password {
                                key_file: kf,
                                password: pwd,
                            },
                            DeviceAccessStrategy::Password {
                                key_file: c_kf,
                                password: c_pwd,
                            },
                        ) if c_kf == kf => {
                            if c_pwd == pwd {
                                Some(Ok(c_device.to_owned()))
                            } else {
                                Some(Err(LoadDeviceError::DecryptionFailed))
                            }
                        }
                        (
                            DeviceAccessStrategy::Smartcard { key_file: kf },
                            DeviceAccessStrategy::Smartcard { key_file: c_kf },
                        ) if c_kf == kf => Some(Ok(c_device.to_owned())),
                        _ => None,
                    });

            if found.is_some() {
                return found;
            }

            if !cache.destroyed.contains(access) {
                // 2) Try to load from the template

                let key_file = access.key_file();
                let decryption_success = match access {
                    DeviceAccessStrategy::Keyring { .. } => true,
                    DeviceAccessStrategy::Password { password, .. } => {
                        let decryption_success = password.as_str() == KEY_FILE_PASSWORD;
                        decryption_success
                    }
                    DeviceAccessStrategy::Smartcard { .. } => true,
                };
                // We don't try to resolve the path of `key_file` into an absolute one here !
                // This is because in practice the path is always provided absolute given it
                // is obtained in the first place by `list_available_devices`.
                let env = test_get_testbed(config_dir).expect("Must exist");
                let device = load_local_device(key_file, &env)?; // Short circuit if not found
                if !decryption_success {
                    return Some(Err(LoadDeviceError::DecryptionFailed));
                }
                cache.available.push((access.to_owned(), device.to_owned()));

                Some(Ok(device))
            } else {
                None
            }
        })
}

pub(crate) fn maybe_save_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
) -> Option<Result<(), SaveDeviceError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let key_file = access.key_file();
            // We don't try to resolve the path of `key_file` into an absolute one here !
            // This is because in practice the path is always provided absolute given it
            // is obtained in the first place by `list_available_devices`.

            let mut cache = store.key_files_cache.lock().expect("Mutex is poisoned");
            cache.available.retain(|(c_access, _)| {
                let c_key_file = c_access.key_file();
                c_key_file != key_file
            });
            // The device is newly created
            cache.destroyed.retain(|c_access| c_access != access);
            cache
                .available
                .push((access.to_owned(), Arc::new(device.to_owned())));
            Ok(())
        })
}

pub(crate) fn maybe_change_authentication(
    config_dir: &Path,
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
) -> Option<Result<(), ChangeAuthentificationError>> {
    if let Some(result) = maybe_load_device(config_dir, current_access) {
        let device = match result {
            Ok(device) => device,
            Err(e) => return Some(Err(ChangeAuthentificationError::from(e))),
        };

        match maybe_save_device(config_dir, new_access, &device) {
            Some(Ok(())) => (),
            Some(Err(e)) => return Some(Err(ChangeAuthentificationError::from(e))),
            None => return None,
        }

        let key_file = current_access.key_file();
        let new_key_file = new_access.key_file();

        if key_file != new_key_file {
            return test_get_testbed_component_store::<ComponentStore>(
                config_dir,
                STORE_ENTRY_KEY,
                store_factory,
            )
            .map(|store| {
                let mut cache = store.key_files_cache.lock().expect("Mutex is poisoned");
                cache.destroyed.push(current_access.clone());
                cache.available.retain(|(c_access, _)| {
                    let c_key_file = c_access.key_file();
                    c_key_file != key_file
                });
                Ok(())
            });
        }

        return Some(Ok(()));
    }

    None
}
