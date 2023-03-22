// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{pyclass, pymethods, PyResult};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    backend_connection::AuthenticatedCmds,
    core_fs::error::to_py_err,
    data::{DeviceCertificate, RealmRoleCertificate, RevokedUserCertificate, UserCertificate},
    enumerate::RealmRole,
    ids::{DeviceID, EntryID, UserID},
    local_device::LocalDevice,
    remote_devices_manager::RemoteDevicesManager,
    runtime::FutureIntoCoroutine,
    time::DateTime,
};

#[pyclass]
pub(crate) struct UserRemoteLoader(Arc<Mutex<libparsec::core_fs::UserRemoteLoader>>);

#[pymethods]
impl UserRemoteLoader {
    #[new]
    fn new(
        device: LocalDevice,
        workspace_id: EntryID,
        backend_cmds: &AuthenticatedCmds,
        remote_devices_manager: &RemoteDevicesManager,
    ) -> PyResult<Self> {
        Ok(Self(Arc::new(Mutex::new(
            libparsec::core_fs::UserRemoteLoader::new(
                device.0,
                workspace_id.0,
                backend_cmds.0.as_ref().clone(),
                remote_devices_manager.0.clone(),
            ),
        ))))
    }

    fn clear_realm_role_certificate_cache(&self) -> FutureIntoCoroutine {
        let reference = self.0.clone();

        FutureIntoCoroutine::from(async move {
            reference.lock().await.clear_realm_role_certificate_cache();
            Ok(())
        })
    }

    fn load_realm_role_certificates(&self, realm_id: Option<EntryID>) -> FutureIntoCoroutine {
        let reference = self.0.clone();

        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .load_realm_role_certificates(realm_id.map(|x| x.0))
                .await
                .map(|x| x.into_iter().map(RealmRoleCertificate).collect::<Vec<_>>())
                .map_err(to_py_err)
        })
    }

    fn load_realm_current_roles(&self, realm_id: Option<EntryID>) -> FutureIntoCoroutine {
        let reference = self.0.clone();

        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .load_realm_current_roles(realm_id.map(|x| x.0))
                .await
                .map(|x| {
                    x.into_iter()
                        .map(|(k, v)| (UserID(k), RealmRole::from_role(v)))
                        .collect::<HashMap<_, _>>()
                })
                .map_err(to_py_err)
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
                .map(|(x, y)| (UserCertificate(x), y.map(RevokedUserCertificate)))
                .map_err(to_py_err)
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
                .map_err(to_py_err)
        })
    }

    fn list_versions(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let reference = self.0.clone();

        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .list_versions(entry_id.0)
                .await
                .map(|x| {
                    x.into_iter()
                        .map(|(k, v)| (k, (DateTime(v.0), DeviceID(v.1))))
                        .collect::<HashMap<_, _>>()
                })
                .map_err(to_py_err)
        })
    }

    fn create_realm(&self, realm_id: EntryID) -> FutureIntoCoroutine {
        let reference = self.0.clone();

        FutureIntoCoroutine::from(async move {
            reference
                .lock()
                .await
                .create_realm(libparsec::types::RealmID::from(*realm_id.0))
                .await
                .map_err(to_py_err)
        })
    }
}
