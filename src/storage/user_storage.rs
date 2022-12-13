use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use pyo3::prelude::{pyclass, pyfunction, pymethods, PyResult};

use crate::{data::LocalUserManifest, ids::EntryID, local_device::LocalDevice};

use super::fs_to_python_error;

#[pyclass]
pub(crate) struct UserStorage(pub libparsec::core_fs::UserStorage);

#[pymethods]
impl UserStorage {
    #[new]
    fn new(device: LocalDevice, user_manifest_id: EntryID, data_base_dir: &str) -> PyResult<Self> {
        let storage = libparsec::core_fs::UserStorage::from_db_dir(
            device.0,
            user_manifest_id.0,
            data_base_dir.as_ref(),
        )
        .map_err(fs_to_python_error)?;
        Ok(Self(storage))
    }

    fn close_connections(&self) {
        self.0.close_connections()
    }

    fn get_realm_checkpoint(&self) -> i64 {
        self.0.get_realm_checkpoint()
    }

    fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> PyResult<()> {
        let changed_vlobs = changed_vlobs
            .into_iter()
            .map(|(entry, version)| (entry.0, version))
            .collect::<Vec<_>>();
        self.0
            .update_realm_checkpoint(new_checkpoint, &changed_vlobs)
            .map_err(fs_to_python_error)
    }

    fn get_need_sync_entries(&self) -> PyResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.0
            .get_need_sync_entries()
            .map_err(fs_to_python_error)
            .map(|(local_changes, remote_changes)| {
                let local_changes = local_changes.into_iter().map(EntryID).collect();
                let remote_changes = remote_changes.into_iter().map(EntryID).collect();

                (local_changes, remote_changes)
            })
    }

    fn get_user_manifest(&self) -> PyResult<LocalUserManifest> {
        self.0
            .get_user_manifest()
            .map_err(fs_to_python_error)
            .map(LocalUserManifest)
    }

    fn load_user_manifest(&self) -> PyResult<()> {
        self.0.load_user_manifest().map_err(fs_to_python_error)
    }

    fn set_user_manifest(&self, user_manifest: LocalUserManifest) -> PyResult<()> {
        self.0
            .set_user_manifest(user_manifest.0)
            .map_err(fs_to_python_error)
    }
}

#[pyfunction]
pub(crate) fn user_storage_non_speculative_init(
    data_base_dir: &str,
    device: LocalDevice,
) -> PyResult<()> {
    libparsec::core_fs::user_storage_non_speculative_init(&PathBuf::from(data_base_dir), device.0)
        .map_err(fs_to_python_error)
}
