use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

use pyo3::{
    prelude::{pyclass, pyfunction, pymethods},
    PyResult,
};

use crate::{
    data::LocalUserManifest, ids::EntryID, local_device::LocalDevice, runtime::FutureIntoCoroutine,
};

use super::fs_to_python_error;

#[pyclass]
pub(crate) struct UserStorage(pub Arc<libparsec::core_fs::UserStorage>);

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
            .map(|us| Self(Arc::new(us)))
        })
    }

    fn close_connections(&self) -> FutureIntoCoroutine {
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move {
            us.close_connections().await;
            Ok(())
        })
    }

    fn get_realm_checkpoint(&self) -> FutureIntoCoroutine {
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move { Ok(us.get_realm_checkpoint().await) })
    }

    fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> FutureIntoCoroutine {
        let changed_vlobs = changed_vlobs
            .into_iter()
            .map(|(entry, version)| (entry.0, version))
            .collect::<Vec<_>>();
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move {
            us.update_realm_checkpoint(new_checkpoint, &changed_vlobs)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn get_need_sync_entries(&self) -> FutureIntoCoroutine {
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move {
            us.get_need_sync_entries()
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
        self.0
            .get_user_manifest()
            .map_err(fs_to_python_error)
            .map(LocalUserManifest)
    }

    fn load_user_manifest(&self) -> FutureIntoCoroutine {
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move {
            us.load_user_manifest().await.map_err(fs_to_python_error)
        })
    }

    fn set_user_manifest(&self, user_manifest: LocalUserManifest) -> FutureIntoCoroutine {
        let us = self.0.clone();

        FutureIntoCoroutine::from(async move {
            us.set_user_manifest(user_manifest.0)
                .await
                .map_err(fs_to_python_error)
        })
    }

    #[getter]
    fn device(&self) -> LocalDevice {
        LocalDevice(self.0.device.clone())
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
