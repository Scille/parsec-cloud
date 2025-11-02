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

use crate::{
    ArchiveDeviceError, AvailableDevice, AvailableDeviceType, DeviceAccessStrategy,
    DeviceSaveStrategy, LoadDeviceError, RemoveDeviceError, SaveDeviceError, UpdateDeviceError,
};

const STORE_ENTRY_KEY: &str = "platform_device_loader";
const KEY_FILE_PASSWORD: &str = "P@ssw0rd."; // Use the same password for all simulated key files

enum MaybePopulated<T> {
    Stalled,
    Populated(T),
}

#[derive(Default)]
struct KeyFilesCache {
    available: Vec<(PathBuf, DeviceSaveStrategy, Arc<LocalDevice>, DateTime)>,
    destroyed: Vec<PathBuf>,
}

struct ComponentStore {
    /// Whenever a new device needs to be loaded, we populate this field from
    /// the testbed template.
    /// Also note this field also tracks the updates and removals of device files.
    already_accessed_key_files: Mutex<KeyFilesCache>,
    /// Lazily build the list of available devices from the testbed template on
    /// the first call to `maybe_list_available_devices`.
    /// Note this field alone is not enough to return the actual list of available
    /// devices: the changes from `already_accessed_key_files` must also be taken
    /// into account!
    template_available_devices: Mutex<MaybePopulated<Vec<AvailableDevice>>>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        template_available_devices: Mutex::new(MaybePopulated::Stalled),
        already_accessed_key_files: Mutex::default(),
    })
}

fn get_device_key_file(config_dir: &Path, device_id: DeviceID) -> PathBuf {
    config_dir.join(format!("devices/{}.keys", device_id.hex()))
}

/// Generate the `LocalDevice` from the template events, this saves us from
/// password derivation, generation of the key file, only to do it
/// deserialization&decryption right away.
fn load_local_device_from_template(
    key_file: &Path,
    env: &TestbedEnv,
) -> Option<(Arc<LocalDevice>, DateTime)> {
    // Parsec stores the key file as `<config_dir>/devices/<device ID as hex>.keys`,
    // however we also handle the device nickname as it is convenient for testing
    // (e.g. `<config_dir>/devices/alice@dev1.keys`).
    let raw_id = key_file.file_stem()?.to_str()?;
    let device_id = DeviceID::from_hex(raw_id)
        .or_else(|_| DeviceID::test_from_nickname(raw_id))
        .ok()?;

    env.template.events.iter().find_map(|e| match e {
        TestbedEvent::BootstrapOrganization(x) if x.first_user_first_device_id == device_id => {
            Some((
                Arc::new(LocalDevice {
                    organization_addr: (*env.organization_addr()).clone(),
                    device_id,
                    user_id: x.first_user_id,
                    device_label: x.first_user_first_device_label.clone(),
                    human_handle: x.first_user_human_handle.clone(),
                    signing_key: x.first_user_first_device_signing_key.clone(),
                    private_key: x.first_user_private_key.clone(),
                    initial_profile: UserProfile::Admin,
                    user_realm_id: x.first_user_user_realm_id,
                    user_realm_key: x.first_user_user_realm_key.clone(),
                    local_symkey: x.first_user_local_symkey.clone(),
                    time_provider: TimeProvider::default(),
                }),
                x.timestamp,
            ))
        }
        TestbedEvent::NewUser(x) if x.first_device_id == device_id => Some((
            Arc::new(LocalDevice {
                organization_addr: (*env.organization_addr()).clone(),
                device_id,
                user_id: x.user_id,
                device_label: x.first_device_label.clone(),
                human_handle: x.human_handle.clone(),
                signing_key: x.first_device_signing_key.clone(),
                private_key: x.private_key.clone(),
                initial_profile: x.initial_profile,
                user_realm_id: x.user_realm_id,
                user_realm_key: x.user_realm_key.clone(),
                local_symkey: x.local_symkey.clone(),
                time_provider: TimeProvider::default(),
            }),
            x.timestamp,
        )),
        TestbedEvent::NewDevice(d) if d.device_id == device_id => {
            env.template.events.iter().find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(u) if u.first_user_id == d.user_id => Some((
                    Arc::new(LocalDevice {
                        organization_addr: (*env.organization_addr()).clone(),
                        device_id,
                        user_id: u.first_user_id,
                        device_label: d.device_label.clone(),
                        human_handle: u.first_user_human_handle.clone(),
                        signing_key: d.signing_key.clone(),
                        private_key: u.first_user_private_key.clone(),
                        initial_profile: UserProfile::Admin,
                        user_realm_id: u.first_user_user_realm_id,
                        user_realm_key: u.first_user_user_realm_key.clone(),
                        local_symkey: d.local_symkey.clone(),
                        time_provider: TimeProvider::default(),
                    }),
                    d.timestamp,
                )),
                TestbedEvent::NewUser(u) if u.user_id == d.user_id => Some((
                    Arc::new(LocalDevice {
                        organization_addr: (*env.organization_addr()).clone(),
                        device_id,
                        user_id: u.user_id,
                        device_label: d.device_label.clone(),
                        human_handle: u.human_handle.clone(),
                        signing_key: d.signing_key.clone(),
                        private_key: u.private_key.clone(),
                        initial_profile: u.initial_profile,
                        user_realm_id: u.user_realm_id,
                        user_realm_key: u.user_realm_key.clone(),
                        local_symkey: d.local_symkey.clone(),
                        time_provider: TimeProvider::default(),
                    }),
                    d.timestamp,
                )),
                _ => None,
            })
        }
        _ => None,
    })
}

fn populate_template_available_devices(
    config_dir: &Path,
    env: &TestbedEnv,
) -> Vec<AvailableDevice> {
    // Populate the storage from the template.
    // Once done we should no longer need the template data
    env.template
        .events
        .iter()
        .filter_map(|e| {
            let (created_on, user_id, device_id, human_handle, device_label) = match e {
                TestbedEvent::BootstrapOrganization(x) => (
                    x.timestamp,
                    x.first_user_id,
                    x.first_user_first_device_id,
                    &x.first_user_human_handle,
                    &x.first_user_first_device_label,
                ),

                TestbedEvent::NewUser(x) => (
                    x.timestamp,
                    x.user_id,
                    x.first_device_id,
                    &x.human_handle,
                    &x.first_device_label,
                ),

                TestbedEvent::NewDevice(x) => {
                    let user_id = x.user_id;
                    let user_human_handle = env
                        .template
                        .events
                        .iter()
                        .find_map(|e| match e {
                            TestbedEvent::BootstrapOrganization(e)
                                if e.first_user_id == user_id =>
                            {
                                Some(&e.first_user_human_handle)
                            }
                            TestbedEvent::NewUser(e) if e.user_id == user_id => {
                                Some(&e.human_handle)
                            }
                            _ => None,
                        })
                        .expect("Must exist");
                    (
                        x.timestamp,
                        user_id,
                        x.device_id,
                        user_human_handle,
                        &x.device_label,
                    )
                }

                _ => return None,
            };

            let server_url = env.server_addr.to_http_url(None).to_string();

            let available_device = AvailableDevice {
                key_file_path: get_device_key_file(config_dir, device_id),
                created_on,
                protected_on: created_on,
                server_url,
                organization_id: env.organization_id.clone(),
                user_id,
                device_id,
                human_handle: human_handle.clone(),
                device_label: device_label.clone(),
                ty: AvailableDeviceType::Password,
            };

            Some(available_device)
        })
        .collect()
}

pub(crate) fn maybe_list_available_devices(config_dir: &Path) -> Option<Vec<AvailableDevice>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut available_devices = vec![];
            let already_accessed_key_files = store
                .already_accessed_key_files
                .lock()
                .expect("Mutex is poisoned");

            // 1. Start with the devices we already know about

            for (key_file, save_strategy, device, created_on) in
                already_accessed_key_files.available.iter()
            {
                // Sanity check
                assert!(!already_accessed_key_files.destroyed.contains(key_file));

                let server_url = {
                    ParsecAddr::new(
                        device.organization_addr.hostname().to_owned(),
                        Some(device.organization_addr.port()),
                        device.organization_addr.use_ssl(),
                    )
                    .to_http_url(None)
                    .to_string()
                };
                available_devices.push(AvailableDevice {
                    key_file_path: key_file.clone(),
                    server_url,
                    created_on: *created_on,
                    protected_on: *created_on,
                    organization_id: device.organization_id().to_owned(),
                    user_id: device.user_id,
                    device_id: device.device_id,
                    device_label: device.device_label.clone(),
                    human_handle: device.human_handle.clone(),
                    ty: save_strategy.ty(),
                })
            }

            // 2. Top up with the ones from the template

            let mut maybe_populated = store
                .template_available_devices
                .lock()
                .expect("Mutex is poisoned");
            let template_available_devices = match &*maybe_populated {
                MaybePopulated::Populated(template_available_devices) => template_available_devices,
                MaybePopulated::Stalled => {
                    let env = test_get_testbed(config_dir).expect("Must exist");
                    let template_available_devices =
                        populate_template_available_devices(config_dir, &env);
                    *maybe_populated = MaybePopulated::Populated(template_available_devices);
                    match &*maybe_populated {
                        MaybePopulated::Populated(template_available_devices) => {
                            template_available_devices
                        }
                        MaybePopulated::Stalled => unreachable!(),
                    }
                }
            };
            for available_device in template_available_devices.iter() {
                if already_accessed_key_files
                    .destroyed
                    .contains(&available_device.key_file_path)
                {
                    // Device file has been removed in the meantime, just ignore it
                    continue;
                }
                if available_devices
                    .iter()
                    .any(|x| x.key_file_path == available_device.key_file_path)
                {
                    // Device file has been overwritten in the meantime, just ignore it
                    continue;
                }
                available_devices.push(available_device.to_owned());
            }

            // 3. Finally sort by file name to simulate filesystem access

            available_devices.sort_by(|a, b| a.key_file_path.cmp(&b.key_file_path));
            available_devices
        })
}

pub(crate) fn maybe_load_device(
    config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Option<Result<Arc<LocalDevice>, LoadDeviceError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .and_then(|store| {
            // 1) Try to load from the cache

            let mut cache = store
                .already_accessed_key_files
                .lock()
                .expect("Mutex is poisoned");
            let found =
                cache
                    .available
                    .iter()
                    .find_map(|(c_key_file, c_save_strategy, c_device, _)| {
                        if access.key_file() != c_key_file {
                            return None;
                        }
                        match (access, c_save_strategy) {
                            (
                                DeviceAccessStrategy::Password { password: pwd, .. },
                                DeviceSaveStrategy::Password { password: c_pwd },
                            ) => {
                                if c_pwd == pwd {
                                    Some(Ok(c_device.to_owned()))
                                } else {
                                    Some(Err(LoadDeviceError::DecryptionFailed))
                                }
                            }
                            (
                                DeviceAccessStrategy::Smartcard { .. },
                                DeviceSaveStrategy::Smartcard { .. },
                            ) => Some(Ok(c_device.to_owned())),
                            (DeviceAccessStrategy::Keyring { .. }, DeviceSaveStrategy::Keyring) => {
                                Some(Ok(c_device.to_owned()))
                            }
                            (
                                DeviceAccessStrategy::AccountVault { operations, .. },
                                DeviceSaveStrategy::AccountVault {
                                    operations: c_operations,
                                },
                            ) => {
                                if operations.account_email() == c_operations.account_email() {
                                    Some(Ok(c_device.to_owned()))
                                } else {
                                    Some(Err(LoadDeviceError::DecryptionFailed))
                                }
                            }
                            (
                                DeviceAccessStrategy::OpenBao { operations, .. },
                                DeviceSaveStrategy::OpenBao {
                                    operations: c_operations,
                                },
                            ) => {
                                if operations.openbao_entity_id()
                                    == c_operations.openbao_entity_id()
                                {
                                    Some(Ok(c_device.to_owned()))
                                } else {
                                    Some(Err(LoadDeviceError::DecryptionFailed))
                                }
                            }
                            // Don't use a `_ => None` fallthrough match here to avoid
                            // silent bug whenever a new variant is added :/
                            (
                                DeviceAccessStrategy::Password { .. }
                                | DeviceAccessStrategy::Smartcard { .. }
                                | DeviceAccessStrategy::Keyring { .. }
                                | DeviceAccessStrategy::AccountVault { .. }
                                | DeviceAccessStrategy::OpenBao { .. },
                                DeviceSaveStrategy::Password { .. }
                                | DeviceSaveStrategy::Smartcard { .. }
                                | DeviceSaveStrategy::Keyring
                                | DeviceSaveStrategy::AccountVault { .. }
                                | DeviceSaveStrategy::OpenBao { .. },
                            ) => None,
                        }
                    });

            if found.is_some() {
                return found;
            }

            let key_file = access.key_file().to_owned();
            if !cache.destroyed.contains(&key_file) {
                // 2) Try to load from the template

                let decryption_success = match access {
                    DeviceAccessStrategy::Keyring { .. } => true,
                    DeviceAccessStrategy::Password { password, .. } => {
                        let decryption_success = password.as_str() == KEY_FILE_PASSWORD;
                        decryption_success
                    }
                    DeviceAccessStrategy::Smartcard { .. } => true,
                    DeviceAccessStrategy::AccountVault { .. } => true,
                    DeviceAccessStrategy::OpenBao { .. } => true,
                };
                // We don't try to resolve the path of `key_file` into an absolute one here !
                // This is because in practice the path is always provided absolute given it
                // is obtained in the first place by `list_template_available_devices`.
                let env = test_get_testbed(config_dir).expect("Must exist");
                let (device, created_on) = load_local_device_from_template(&key_file, &env)?; // Short circuit if not found
                if !decryption_success {
                    return Some(Err(LoadDeviceError::DecryptionFailed));
                }
                // Save strategy contains more informations than access strategy, so we have
                // to mock the missing stuff here.
                let save_strategy = {
                    let extra_info = match access {
                        DeviceAccessStrategy::Keyring { .. } => AvailableDeviceType::Keyring,
                        DeviceAccessStrategy::Password { .. } => AvailableDeviceType::Password,
                        DeviceAccessStrategy::Smartcard { .. } => todo!(), // TODO #11269
                        DeviceAccessStrategy::AccountVault { .. } => {
                            AvailableDeviceType::AccountVault
                        }
                        DeviceAccessStrategy::OpenBao { .. } => todo!(),
                    };
                    access
                        .clone()
                        .into_save_strategy(extra_info)
                        .expect("valid variant")
                };
                cache.available.push((
                    access.key_file().to_owned(),
                    save_strategy,
                    device.to_owned(),
                    created_on,
                ));

                Some(Ok(device))
            } else {
                None
            }
        })
}

pub(crate) fn maybe_save_device(
    config_dir: &Path,
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    key_file: PathBuf,
) -> Option<Result<AvailableDevice, SaveDeviceError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            // We don't try to resolve the path of `key_file` into an absolute one here !
            // This is because in practice the path is always provided absolute given it
            // is obtained in the first place by `list_available_devices`.

            let mut cache = store
                .already_accessed_key_files
                .lock()
                .expect("Mutex is poisoned");
            cache
                .available
                .retain(|(c_key_file, _, _, _)| *c_key_file != key_file);
            // The device is newly created
            let created_on = device.now();
            cache.destroyed.retain(|c_key_file| *c_key_file != key_file);
            cache.available.push((
                key_file.clone(),
                strategy.clone(),
                Arc::new(device.to_owned()),
                created_on,
            ));

            let server_url = {
                ParsecAddr::new(
                    device.organization_addr.hostname().to_owned(),
                    Some(device.organization_addr.port()),
                    device.organization_addr.use_ssl(),
                )
                .to_http_url(None)
                .to_string()
            };

            Ok(AvailableDevice {
                key_file_path: key_file,
                server_url,
                created_on,
                protected_on: created_on,
                organization_id: device.organization_id().to_owned(),
                user_id: device.user_id,
                device_id: device.device_id,
                device_label: device.device_label.clone(),
                human_handle: device.human_handle.clone(),
                ty: strategy.ty(),
            })
        })
}

pub(crate) fn maybe_update_device(
    config_dir: &Path,
    current_access: &DeviceAccessStrategy,
    new_strategy: &DeviceSaveStrategy,
    new_key_file: &Path,
    overwrite_server_addr: Option<ParsecAddr>,
) -> Option<Result<(AvailableDevice, ParsecAddr), UpdateDeviceError>> {
    if let Some(result) = maybe_load_device(config_dir, current_access) {
        let mut device = match result {
            Ok(device) => device,
            Err(e) => {
                return Some(Err(match e {
                    LoadDeviceError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
                    LoadDeviceError::InvalidPath(err) => UpdateDeviceError::InvalidPath(err),
                    LoadDeviceError::InvalidData => UpdateDeviceError::InvalidData,
                    LoadDeviceError::DecryptionFailed => UpdateDeviceError::DecryptionFailed,
                    LoadDeviceError::Internal(err) => UpdateDeviceError::Internal(err),
                    LoadDeviceError::RemoteOpaqueKeyFetchOffline(err) => {
                        UpdateDeviceError::RemoteOpaqueKeyOperationOffline(err)
                    }
                    LoadDeviceError::RemoteOpaqueKeyFetchFailed(err) => {
                        UpdateDeviceError::RemoteOpaqueKeyOperationFailed(err)
                    }
                }))
            }
        };

        let old_server_addr = ParsecAddr::new(
            device.organization_addr.hostname().to_owned(),
            Some(device.organization_addr.port()),
            device.organization_addr.use_ssl(),
        );
        if let Some(overwrite_server_addr) = overwrite_server_addr {
            Arc::make_mut(&mut device).organization_addr = ParsecOrganizationAddr::new(
                overwrite_server_addr,
                device.organization_addr.organization_id().to_owned(),
                device.organization_addr.root_verify_key().to_owned(),
            );
        }

        let available_device = match maybe_save_device(
            config_dir,
            new_strategy,
            &device,
            new_key_file.to_path_buf(),
        )
        .expect("testbed env already accessed")
        {
            Ok(available_device) => available_device,
            Err(e) => {
                return Some(Err(match e {
                    SaveDeviceError::StorageNotAvailable => UpdateDeviceError::StorageNotAvailable,
                    SaveDeviceError::InvalidPath(err) => UpdateDeviceError::InvalidPath(err),
                    SaveDeviceError::Internal(err) => UpdateDeviceError::Internal(err),
                    SaveDeviceError::RemoteOpaqueKeyUploadOffline(err) => {
                        UpdateDeviceError::RemoteOpaqueKeyOperationOffline(err)
                    }
                    SaveDeviceError::RemoteOpaqueKeyUploadFailed(err) => {
                        UpdateDeviceError::RemoteOpaqueKeyOperationFailed(err)
                    }
                }))
            }
        };

        let key_file = current_access.key_file();

        if key_file != new_key_file {
            maybe_remove_device(config_dir, key_file)
                .expect("testbed env already accessed")
                .expect("current_access already accessed");
        }

        return Some(Ok((available_device, old_server_addr)));
    }

    None
}

pub(crate) fn maybe_remove_device(
    config_dir: &Path,
    key_file: &Path,
) -> Option<Result<(), RemoveDeviceError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            // We don't try to resolve the path of `key_file` into an absolute one here !
            // This is because in practice the path is always provided absolute given it
            // is obtained in the first place by `list_template_available_devices`.

            let mut cache = store
                .already_accessed_key_files
                .lock()
                .expect("Mutex is poisoned");
            cache
                .available
                .retain(|(c_key_file, _, _, _)| c_key_file != key_file);
            let key_file = key_file.to_owned();
            if !cache.destroyed.contains(&key_file) {
                cache.destroyed.push(key_file);
            }

            Ok(())
        })
}

pub(crate) fn maybe_archive_device(
    config_dir: &Path,
    key_file: &Path,
) -> Option<Result<(), ArchiveDeviceError>> {
    maybe_remove_device(config_dir, key_file).map(|r| {
        r.expect("error never returned");
        Ok(())
    })
}
