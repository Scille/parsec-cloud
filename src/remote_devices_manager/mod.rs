// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;

use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    api_crypto::VerifyKey,
    backend_connection::AuthenticatedCmds,
    data::{DeviceCertificate, RevokedUserCertificate, UserCertificate},
    ids::{DeviceID, UserID},
    remote_devices_manager::error::{
        RemoteDevicesManagerBackendOfflineError, RemoteDevicesManagerDeviceNotFoundError,
        RemoteDevicesManagerError, RemoteDevicesManagerExc,
        RemoteDevicesManagerInvalidTrustchainError, RemoteDevicesManagerNotFoundError,
        RemoteDevicesManagerUserNotFoundError,
    },
    runtime::FutureIntoCoroutine,
    time::TimeProvider,
};

#[pyclass]
pub(crate) struct RemoteDevicesManager(pub Arc<Mutex<libparsec::core::RemoteDevicesManager>>);

#[pymethods]
impl RemoteDevicesManager {
    #[new]
    fn new(
        backend_cmds: &AuthenticatedCmds,
        root_verify_key: VerifyKey,
        time_provider: TimeProvider,
    ) -> Self {
        Self(Arc::new(Mutex::new(
            libparsec::core::RemoteDevicesManager::new(
                backend_cmds.0.as_ref().clone(),
                root_verify_key.0,
                time_provider.0,
            ),
        )))
    }

    #[getter]
    fn cache_validity(&self) -> i64 {
        libparsec::core::RemoteDevicesManager::cache_validity()
    }

    fn invalidate_user_cache(&self, user_id: UserID) -> FutureIntoCoroutine {
        let reference = self.0.clone();
        FutureIntoCoroutine::from(async move {
            reference.lock().await.invalidate_user_cache(&user_id.0);
            Ok(())
        })
    }

    #[args(no_cache = false)]
    fn get_user(&self, user_id: UserID, no_cache: bool) -> FutureIntoCoroutine {
        let reference = self.0.clone();
        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .get_user(&user_id.0, no_cache)
                .await
                .map(|(uc, ruc)| (UserCertificate(uc), ruc.map(RevokedUserCertificate)))
                .map_err(RemoteDevicesManagerExc::from)
                .map_err(PyErr::from)
        })
    }

    #[args(no_cache = false)]
    fn get_device(&self, device_id: DeviceID, no_cache: bool) -> FutureIntoCoroutine {
        let reference = self.0.clone();
        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .get_device(&device_id.0, no_cache)
                .await
                .map(DeviceCertificate)
                .map_err(RemoteDevicesManagerExc::from)
                .map_err(PyErr::from)
        })
    }

    fn get_user_and_devices(&self, user_id: UserID) -> FutureIntoCoroutine {
        let reference = self.0.clone();
        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .get_user_and_devices(&user_id.0)
                .await
                .map(|(uc, ruc, dc)| {
                    (
                        UserCertificate(uc),
                        ruc.map(RevokedUserCertificate),
                        dc.into_iter().map(DeviceCertificate).collect::<Vec<_>>(),
                    )
                })
                .map_err(RemoteDevicesManagerExc::from)
                .map_err(PyErr::from)
        })
    }
}

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Error
    m.add(
        "RemoteDevicesManagerError",
        py.get_type::<RemoteDevicesManagerError>(),
    )?;
    m.add(
        "RemoteDevicesManagerBackendOfflineError",
        py.get_type::<RemoteDevicesManagerBackendOfflineError>(),
    )?;
    m.add(
        "RemoteDevicesManagerNotFoundError",
        py.get_type::<RemoteDevicesManagerNotFoundError>(),
    )?;
    m.add(
        "RemoteDevicesManagerUserNotFoundError",
        py.get_type::<RemoteDevicesManagerUserNotFoundError>(),
    )?;
    m.add(
        "RemoteDevicesManagerDeviceNotFoundError",
        py.get_type::<RemoteDevicesManagerDeviceNotFoundError>(),
    )?;
    m.add(
        "RemoteDevicesManagerInvalidTrustchainError",
        py.get_type::<RemoteDevicesManagerInvalidTrustchainError>(),
    )?;

    m.add_class::<RemoteDevicesManager>()?;
    Ok(())
}
