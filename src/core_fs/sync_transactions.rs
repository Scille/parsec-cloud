// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    pyclass, pymethods,
    types::PyType,
    PyAny, PyResult,
};
use std::collections::HashSet;

use crate::{
    data::{BlockAccess, FileManifest, FolderManifest, WorkspaceManifest},
    ids::EntryID,
};

#[pyclass]
pub(crate) struct ChangesAfterSync(libparsec::core_fs::ChangesAfterSync);

#[pymethods]
impl ChangesAfterSync {
    #[getter]
    fn added_blocks(&self) -> HashSet<BlockAccess> {
        self.0
            .added_blocks
            .clone()
            .into_iter()
            .map(BlockAccess)
            .collect()
    }

    #[getter]
    fn removed_blocks(&self) -> HashSet<BlockAccess> {
        self.0
            .removed_blocks
            .clone()
            .into_iter()
            .map(BlockAccess)
            .collect()
    }

    #[getter]
    fn added_entries(&self) -> HashSet<EntryID> {
        self.0
            .added_entries
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    #[getter]
    fn removed_entries(&self) -> HashSet<EntryID> {
        self.0
            .removed_entries
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    #[classmethod]
    fn from_manifests(_cls: &PyType, old_manifest: &PyAny, new_manifest: &PyAny) -> PyResult<Self> {
        if let Ok(old_manifest) = old_manifest.extract::<FileManifest>() {
            let new_manifest = new_manifest
                .extract::<FileManifest>()
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(Self(libparsec::core_fs::ChangesAfterSync::from((
                old_manifest.0,
                new_manifest.0,
            ))))
        } else if let Ok(old_manifest) = old_manifest.extract::<FolderManifest>() {
            let new_manifest = new_manifest
                .extract::<FolderManifest>()
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(Self(libparsec::core_fs::ChangesAfterSync::from((
                old_manifest.0,
                new_manifest.0,
            ))))
        } else if let Ok(old_manifest) = old_manifest.extract::<WorkspaceManifest>() {
            let new_manifest = new_manifest
                .extract::<WorkspaceManifest>()
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(Self(libparsec::core_fs::ChangesAfterSync::from((
                old_manifest.0,
                new_manifest.0,
            ))))
        } else {
            return Err(PyTypeError::new_err("UserManifest should never get there"));
        }
    }
}
