// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    prelude::{pyclass, pyfunction, pymethods},
    PyResult,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    data::LocalUserManifest, ids::EntryID, local_device::LocalDevice, runtime::FutureIntoCoroutine,
};

use super::{fs_to_python_error, FSInternalError};

#[pyclass]
pub(crate) struct UserStorage(
    /// Hold the reference to an unique [libparsec::core_fs::UserStorage].
    ///
    /// # Why the imbricated `Arc<...Arc<...>>` ?
    ///
    /// The first one is to share a lock over the second one.
    /// The lock is here when we do `close_connection` after that the UserStorage should not be accessible.
    /// The second one is because of [FutureIntoCoroutine] that require to provide a `static` future.
    /// To fullfish that requirement we have to clone the reference over [libparsec::core_fs::UserStorage].
    pub Arc<Mutex<Option<Arc<libparsec::core_fs::UserStorage>>>>,
);

impl UserStorage {
    fn get_storage(&self) -> PyResult<Arc<libparsec::core_fs::UserStorage>> {
        let guard = self.0.lock().expect("Mutex is poisoned");

        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| FSInternalError::new_err("Trying to use an already closed user storage"))
    }

    fn drop_storage(&self) -> PyResult<Arc<libparsec::core_fs::UserStorage>> {
        self.0
            .lock()
            .expect("Mutex is poisoned")
            .take()
            .ok_or_else(|| FSInternalError::new_err("Trying to use an already closed user storage"))
    }
}

#[pymethods]
impl UserStorage {
    #[staticmethod]
    #[pyo3(name = "new")]
    fn new_async(
        device: LocalDevice,
        user_manifest_id: EntryID,
        data_base_dir: PathBuf,
    ) -> FutureIntoCoroutine {
        FutureIntoCoroutine::from(async move {
            libparsec::core_fs::UserStorage::from_db_dir(
                device.0,
                user_manifest_id.0,
                &data_base_dir,
            )
            .await
            .map_err(fs_to_python_error)
            .map(|us| Self(Arc::new(Mutex::new(Some(Arc::new(us))))))
        })
    }

    fn close_connections(&self) -> PyResult<()> {
        let us = self.drop_storage()?;
        us.close_connections();
        Ok(())
    }

    fn get_realm_checkpoint(&self) -> FutureIntoCoroutine {
        let us = self.get_storage();

        FutureIntoCoroutine::from(async move { Ok(us?.get_realm_checkpoint().await) })
    }

    fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> FutureIntoCoroutine {
        let us = self.get_storage();

        FutureIntoCoroutine::from(async move {
            let changed_vlobs = changed_vlobs
                .into_iter()
                .map(|(entry, version)| (entry.0, version))
                .collect::<Vec<_>>();
            us?.update_realm_checkpoint(new_checkpoint, &changed_vlobs)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn get_need_sync_entries(&self) -> FutureIntoCoroutine {
        let us = self.get_storage();

        FutureIntoCoroutine::from(async move {
            us?.get_need_sync_entries()
                .await
                .map_err(fs_to_python_error)
                .map(|(local_changes, remote_changes)| {
                    let local_changes: HashSet<EntryID> =
                        local_changes.into_iter().map(EntryID).collect();
                    let remote_changes: HashSet<EntryID> =
                        remote_changes.into_iter().map(EntryID).collect();

                    (local_changes, remote_changes)
                })
        })
    }

    fn get_user_manifest(&self) -> PyResult<LocalUserManifest> {
        self.get_storage()
            .map(|ws| LocalUserManifest(ws.get_user_manifest()))
    }

    fn set_user_manifest(&self, user_manifest: LocalUserManifest) -> FutureIntoCoroutine {
        let us = self.get_storage();

        FutureIntoCoroutine::from(async move {
            us?.set_user_manifest(user_manifest.0)
                .await
                .map_err(fs_to_python_error)
        })
    }

    #[getter]
    fn device(&self) -> PyResult<LocalDevice> {
        self.get_storage().map(|ws| LocalDevice(ws.device.clone()))
    }
}

#[pyfunction]
pub(crate) fn user_storage_non_speculative_init(
    data_base_dir: PathBuf,
    device: LocalDevice,
) -> FutureIntoCoroutine {
    FutureIntoCoroutine::from(async move {
        libparsec::core_fs::user_storage_non_speculative_init(&data_base_dir, device.0)
            .await
            .map_err(fs_to_python_error)
    })
}
