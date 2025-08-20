// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::event::{Event, EventListener};
use libparsec_testbed::{test_get_testbed_component_store, TestbedEnv};
use libparsec_types::prelude::*;

use crate::TryLockDeviceForUseError;

const STORE_ENTRY_KEY: &str = "platform_ipc";

#[derive(Debug)]
pub struct InUseDeviceLockGuard(InUseDeviceLockGuardInner);

impl From<crate::platform::InUseDeviceLockGuard> for InUseDeviceLockGuard {
    fn from(value: crate::platform::InUseDeviceLockGuard) -> Self {
        InUseDeviceLockGuard(InUseDeviceLockGuardInner::Real(value))
    }
}

#[derive(Debug)]
enum InUseDeviceLockGuardInner {
    #[allow(unused)]
    Real(crate::platform::InUseDeviceLockGuard),
    Mocked {
        device_id: DeviceID,
        store: Arc<ComponentStore>,
    },
}

impl Drop for InUseDeviceLockGuard {
    fn drop(&mut self) {
        match &self.0 {
            InUseDeviceLockGuardInner::Real(_) => (),
            InUseDeviceLockGuardInner::Mocked { device_id, store } => {
                let mut devices_in_use = store.devices_in_use.lock().expect("Mutex is poisoned");
                let position = devices_in_use
                    .iter()
                    .position(|candidate| candidate == device_id)
                    .expect("must be present");
                devices_in_use.swap_remove(position);
                store.device_released_event.notify(u32::MAX);
            }
        }
    }
}

#[derive(Debug)]
struct ComponentStore {
    devices_in_use: Mutex<Vec<DeviceID>>,
    device_released_event: Event,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        devices_in_use: Mutex::new(vec![]),
        device_released_event: Event::new(),
    })
}

pub(crate) fn maybe_try_lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Option<Result<InUseDeviceLockGuard, TryLockDeviceForUseError>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            {
                let mut devices_in_use = store.devices_in_use.lock().expect("Mutex is poisoned");
                if devices_in_use.contains(&device_id) {
                    return Err(TryLockDeviceForUseError::AlreadyInUse);
                } else {
                    devices_in_use.push(device_id);
                }
            }
            Ok(InUseDeviceLockGuard(InUseDeviceLockGuardInner::Mocked {
                device_id,
                store,
            }))
        })
}

pub(crate) async fn maybe_lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Option<InUseDeviceLockGuard> {
    loop {
        match lock_or_get_event(config_dir, device_id) {
            None => return None,
            Some(Ok(ok)) => return Some(ok),
            Some(Err(event)) => event.await,
        }
    }
}

fn lock_or_get_event(
    config_dir: &Path,
    device_id: DeviceID,
) -> Option<Result<InUseDeviceLockGuard, EventListener>> {
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            {
                let mut devices_in_use = store.devices_in_use.lock().expect("Mutex is poisoned");
                if devices_in_use.contains(&device_id) {
                    return Err(store.device_released_event.listen());
                } else {
                    devices_in_use.push(device_id);
                }
            }
            Ok(InUseDeviceLockGuard(InUseDeviceLockGuardInner::Mocked {
                device_id,
                store,
            }))
        })
}
