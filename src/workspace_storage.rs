// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    import_exception,
    prelude::{pyclass, pyfunction, pymethods, IntoPy, PyObject, PyResult, Python},
    types::PyBytes,
    PyErr,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    data::{LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest},
    ids::{BlockID, ChunkID, EntryID},
    local_device::LocalDevice,
    regex::Regex,
    time::DateTime,
};

use libparsec::core_fs::FSError;

import_exception!(parsec.core.fs.exceptions, FSLocalMissError);
import_exception!(parsec.core.fs.exceptions, FSInvalidFileDescriptor);
import_exception!(parsec.core.fs.exceptions, FSLocalStorageOperationError);
import_exception!(parsec.core.fs.exceptions, FSLocalStorageClosedError);
import_exception!(parsec.core.fs.exceptions, FSInternalError);

/// WorkspaceStorage's binding is implemented with allow_threads because its
/// methods are called in trio.to_thread to connect the sync and async world
#[pyclass]
#[derive(Clone)]
pub(crate) struct WorkspaceStorage(pub libparsec::core_fs::WorkspaceStorage, pub Option<Regex>);

#[pymethods]
impl WorkspaceStorage {
    #[new]
    #[args(cache_size = "512 * 1024 * 1024")]
    fn new(
        data_base_dir: PathBuf,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex,
        cache_size: u64,
    ) -> PyResult<Self> {
        Ok(Self(
            libparsec::core_fs::WorkspaceStorage::new(
                data_base_dir,
                device.0,
                workspace_id.0,
                prevent_sync_pattern.0,
                cache_size,
            )
            .map_err(fs_to_python_error)?,
            None,
        ))
    }

    fn set_prevent_sync_pattern(&mut self, py: Python, pattern: &Regex) -> PyResult<()> {
        self.1 = Some(pattern.clone());
        let pattern = &pattern.0;
        py.allow_threads(|| {
            self.0
                .set_prevent_sync_pattern(pattern)
                .map_err(fs_to_python_error)?;
            Ok(())
        })
    }

    fn mark_prevent_sync_pattern_fully_applied(&self, py: Python, pattern: &Regex) -> PyResult<()> {
        let pattern = &pattern.0;
        py.allow_threads(|| {
            self.0
                .mark_prevent_sync_pattern_fully_applied(pattern)
                .map_err(fs_to_python_error)
        })
    }

    fn get_prevent_sync_pattern<'py>(&'py self, py: Python<'py>) -> PyResult<Regex> {
        let pattern = self.1.as_ref().cloned().unwrap_or_else(|| {
            let pattern = py.allow_threads(|| self.0.get_prevent_sync_pattern());
            Regex(pattern)
        });
        Ok(pattern)
    }

    fn get_prevent_sync_pattern_fully_applied(&self, py: Python) -> PyResult<bool> {
        py.allow_threads(|| Ok(self.0.get_prevent_sync_pattern_fully_applied()))
    }

    fn get_workspace_manifest(&self, py: Python) -> PyResult<LocalWorkspaceManifest> {
        py.allow_threads(|| {
            Ok(LocalWorkspaceManifest(
                self.0
                    .get_workspace_manifest()
                    .map_err(fs_to_python_error)?,
            ))
        })
    }

    fn get_manifest(&self, py: Python, entry_id: EntryID) -> PyResult<PyObject> {
        let manifest = py
            .allow_threads(|| self.0.get_manifest(entry_id.0))
            .map_err(|_| FSLocalMissError::new_err(entry_id))?;
        Ok(match manifest {
            libparsec::client_types::LocalManifest::File(manifest) => {
                LocalFileManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Folder(manifest) => {
                LocalFolderManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Workspace(manifest) => {
                LocalWorkspaceManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::User(manifest) => {
                LocalUserManifest(manifest).into_py(py)
            }
        })
    }

    #[args(cache_only = false, removed_ids = "None")]
    fn set_manifest(
        &self,
        py: Python,
        entry_id: EntryID,
        manifest: PyObject,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkID>>,
    ) -> PyResult<()> {
        let manifest = if let Ok(manifest) = manifest.extract::<LocalFileManifest>(py) {
            libparsec::client_types::LocalManifest::File(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalFolderManifest>(py) {
            libparsec::client_types::LocalManifest::Folder(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalWorkspaceManifest>(py) {
            libparsec::client_types::LocalManifest::Workspace(manifest.0)
        } else {
            libparsec::client_types::LocalManifest::User(
                manifest.extract::<LocalUserManifest>(py)?.0,
            )
        };
        py.allow_threads(|| {
            self.0
                .set_manifest(
                    entry_id.0,
                    manifest,
                    cache_only,
                    false,
                    removed_ids.map(|x| {
                        x.into_iter()
                            .map(|id| libparsec::core_fs::ChunkOrBlockID::ChunkID(id.0))
                            .collect()
                    }),
                )
                .map_err(fs_to_python_error)
        })
    }

    fn clear_manifest(&self, py: Python, entry_id: EntryID) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .clear_manifest(entry_id.0, false)
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(entry_id),
                    _ => fs_to_python_error(e),
                })
        })
    }

    fn create_file_descriptor(&self, py: Python, manifest: LocalFileManifest) -> PyResult<u32> {
        py.allow_threads(|| Ok(self.0.create_file_descriptor(manifest.0).0))
    }

    fn load_file_descriptor(&self, py: Python, fd: u32) -> PyResult<LocalFileManifest> {
        py.allow_threads(|| {
            Ok(LocalFileManifest(
                self.0
                    .load_file_descriptor(libparsec::types::FileDescriptor(fd))
                    .map_err(|e| FSInvalidFileDescriptor::new_err(e.to_string()))?,
            ))
        })
    }

    fn remove_file_descriptor(&self, py: Python, fd: u32) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .remove_file_descriptor(libparsec::types::FileDescriptor(fd))
                .map(|_| ())
                .ok_or_else(|| FSInvalidFileDescriptor::new_err(""))
        })
    }

    fn set_clean_block(&self, py: Python, block_id: BlockID, block: &[u8]) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .set_clean_block(block_id.0, block)
                .map_err(fs_to_python_error)
        })
    }

    fn clear_clean_block(&self, py: Python, block_id: BlockID) -> PyResult<()> {
        py.allow_threads(|| {
            self.0.clear_clean_block(block_id.0);
            Ok(())
        })
    }

    fn get_dirty_block<'py>(&self, py: Python<'py>, block_id: BlockID) -> PyResult<&'py PyBytes> {
        let block = py
            .allow_threads(|| self.0.get_dirty_block(block_id.0))
            .map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(block_id),
                _ => fs_to_python_error(e),
            })?;
        Ok(PyBytes::new(py, &block))
    }

    fn get_chunk<'py>(&self, py: Python<'py>, chunk_id: ChunkID) -> PyResult<&'py PyBytes> {
        let chunk = py
            .allow_threads(|| self.0.get_chunk(chunk_id.0))
            .map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                _ => fs_to_python_error(e),
            })?;
        Ok(PyBytes::new(py, &chunk))
    }

    // Pyo3 is inefficient with Vec<u8> but set_chunk must handle PyByteArray and PyBytes
    // but can't safely call unsafe { block.as_bytes() } because it's called in multiple threads
    fn set_chunk(&self, py: Python, chunk_id: ChunkID, block: Vec<u8>) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .set_chunk(chunk_id.0, &block)
                .map_err(fs_to_python_error)
        })
    }

    #[args(miss_ok = false)]
    fn clear_chunk(&self, py: Python, chunk_id: ChunkID, miss_ok: bool) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .clear_chunk(chunk_id.0, miss_ok)
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                    _ => fs_to_python_error(e),
                })
        })
    }

    fn get_realm_checkpoint(&self, py: Python) -> PyResult<i64> {
        py.allow_threads(|| Ok(self.0.get_realm_checkpoint()))
    }

    fn update_realm_checkpoint(
        &self,
        py: Python,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .update_realm_checkpoint(
                    new_checkpoint,
                    &changed_vlobs
                        .into_iter()
                        .map(|(id, x)| (id.0, x))
                        .collect::<Vec<_>>()[..],
                )
                .map_err(fs_to_python_error)
        })
    }

    fn get_need_sync_entries(&self, py: Python) -> PyResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        py.allow_threads(|| {
            self.0
                .get_need_sync_entries()
                .map(|(res0, res1)| {
                    (
                        res0.into_iter().map(EntryID).collect(),
                        res1.into_iter().map(EntryID).collect(),
                    )
                })
                .map_err(fs_to_python_error)
        })
    }

    fn ensure_manifest_persistent(&self, py: Python, entry_id: EntryID) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .ensure_manifest_persistent(entry_id.0, false)
                .map_err(fs_to_python_error)
        })
    }

    #[args(flush = true)]
    fn clear_memory_cache(&self, py: Python, flush: bool) -> PyResult<()> {
        py.allow_threads(|| self.0.clear_memory_cache(flush).map_err(fs_to_python_error))
    }

    fn run_vacuum(&self, py: Python) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .run_vacuum()
                .map_err(|_| FSInternalError::new_err("Vacuum failed"))
        })
    }

    fn get_local_block_ids(&self, py: Python, chunk_ids: Vec<ChunkID>) -> PyResult<Vec<ChunkID>> {
        py.allow_threads(|| {
            Ok(self
                .0
                .get_local_block_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .map_err(fs_to_python_error)?
                .into_iter()
                .map(ChunkID)
                .collect())
        })
    }

    fn get_local_chunk_ids(&self, py: Python, chunk_ids: Vec<ChunkID>) -> PyResult<Vec<ChunkID>> {
        py.allow_threads(|| {
            Ok(self
                .0
                .get_local_chunk_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .map_err(fs_to_python_error)?
                .into_iter()
                .map(ChunkID)
                .collect())
        })
    }

    fn to_timestamp(&self) -> PyResult<WorkspaceStorageSnapshot> {
        WorkspaceStorageSnapshot::new(self.clone())
    }

    #[getter]
    fn device(&self) -> PyResult<LocalDevice> {
        Ok(LocalDevice(self.0.device.clone()))
    }

    #[getter]
    fn workspace_id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.workspace_id))
    }

    fn is_manifest_cache_ahead_of_persistance(
        &self,
        py: Python,
        entry_id: EntryID,
    ) -> PyResult<bool> {
        Ok(py.allow_threads(|| self.0.is_manifest_cache_ahead_of_persistance(&entry_id.0)))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct WorkspaceStorageSnapshot(
    pub libparsec::core_fs::WorkspaceStorageSnapshot,
    pub Option<Regex>,
);

#[pymethods]
impl WorkspaceStorageSnapshot {
    #[new]
    fn new(workspace_storage: WorkspaceStorage) -> PyResult<Self> {
        Ok(Self(
            libparsec::core_fs::WorkspaceStorageSnapshot::from(workspace_storage.0),
            workspace_storage.1,
        ))
    }

    fn create_file_descriptor(&self, py: Python, manifest: LocalFileManifest) -> PyResult<u32> {
        py.allow_threads(|| Ok(self.0.create_file_descriptor(manifest.0).0))
    }

    fn load_file_descriptor(&self, py: Python, fd: u32) -> PyResult<LocalFileManifest> {
        py.allow_threads(|| {
            Ok(LocalFileManifest(
                self.0
                    .load_file_descriptor(libparsec::types::FileDescriptor(fd))
                    .map_err(|e| FSInvalidFileDescriptor::new_err(e.to_string()))?,
            ))
        })
    }

    fn remove_file_descriptor(&self, py: Python, fd: u32) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .remove_file_descriptor(libparsec::types::FileDescriptor(fd))
                .map(|_| ())
                .ok_or_else(|| FSInvalidFileDescriptor::new_err(""))
        })
    }

    fn get_chunk<'py>(&self, py: Python<'py>, chunk_id: ChunkID) -> PyResult<&'py PyBytes> {
        let chunk = py
            .allow_threads(|| self.0.get_chunk(chunk_id.0))
            .map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                _ => fs_to_python_error(e),
            })?;
        Ok(PyBytes::new(py, &chunk))
    }

    fn get_workspace_manifest(&self, py: Python) -> PyResult<LocalWorkspaceManifest> {
        py.allow_threads(|| {
            Ok(LocalWorkspaceManifest(
                self.0
                    .get_workspace_manifest()
                    .map_err(fs_to_python_error)?,
            ))
        })
    }

    fn get_manifest(&self, py: Python, entry_id: EntryID) -> PyResult<PyObject> {
        let manifest = py
            .allow_threads(|| self.0.get_manifest(entry_id.0))
            .map_err(|_| FSLocalMissError::new_err(entry_id))?;
        Ok(match manifest {
            libparsec::client_types::LocalManifest::File(manifest) => {
                LocalFileManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Folder(manifest) => {
                LocalFolderManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Workspace(manifest) => {
                LocalWorkspaceManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::User(manifest) => {
                LocalUserManifest(manifest).into_py(py)
            }
        })
    }

    fn set_manifest(&self, py: Python, entry_id: EntryID, manifest: PyObject) -> PyResult<()> {
        let manifest = if let Ok(manifest) = manifest.extract::<LocalFileManifest>(py) {
            libparsec::client_types::LocalManifest::File(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalFolderManifest>(py) {
            libparsec::client_types::LocalManifest::Folder(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalWorkspaceManifest>(py) {
            libparsec::client_types::LocalManifest::Workspace(manifest.0)
        } else {
            libparsec::client_types::LocalManifest::User(
                manifest.extract::<LocalUserManifest>(py)?.0,
            )
        };
        py.allow_threads(|| {
            self.0
                .set_manifest(entry_id.0, manifest, false)
                .map_err(fs_to_python_error)
        })
    }

    fn get_prevent_sync_pattern<'py>(&'py self, py: Python<'py>) -> PyResult<Regex> {
        let pattern = self.1.as_ref().cloned().unwrap_or_else(|| {
            let pattern = py.allow_threads(|| self.0.get_prevent_sync_pattern());
            Regex(pattern)
        });
        Ok(pattern)
    }

    fn get_prevent_sync_pattern_fully_applied(&self, py: Python) -> PyResult<bool> {
        py.allow_threads(|| Ok(self.0.get_prevent_sync_pattern_fully_applied()))
    }

    fn set_clean_block(&self, py: Python, block_id: BlockID, block: &[u8]) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .set_clean_block(block_id.0, block)
                .map_err(fs_to_python_error)
        })
    }

    fn clear_clean_block(&self, py: Python, block_id: BlockID) -> PyResult<()> {
        py.allow_threads(|| {
            self.0.clear_clean_block(block_id.0);
            Ok(())
        })
    }

    fn get_dirty_block<'py>(&self, py: Python<'py>, block_id: BlockID) -> PyResult<&'py PyBytes> {
        let block = py
            .allow_threads(|| self.0.get_dirty_block(block_id.0))
            .map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(block_id),
                _ => fs_to_python_error(e),
            })?;
        Ok(PyBytes::new(py, &block))
    }

    fn to_timestamp(&self) -> PyResult<Self> {
        Ok(Self(self.0.to_timestamp(), self.1.clone()))
    }

    fn clear_local_cache(&self, py: Python) {
        py.allow_threads(|| self.0.clear_local_cache())
    }

    #[getter]
    fn device(&self) -> PyResult<LocalDevice> {
        Ok(LocalDevice(self.0.workspace_storage.device.clone()))
    }

    #[getter]
    fn workspace_id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.workspace_storage.workspace_id))
    }
}

#[pyfunction]
pub(crate) fn workspace_storage_non_speculative_init(
    py: Python,
    data_base_dir: PathBuf,
    device: LocalDevice,
    workspace_id: EntryID,
    timestamp: Option<DateTime>,
) -> PyResult<()> {
    py.allow_threads(|| {
        libparsec::core_fs::workspace_storage_non_speculative_init(
            &data_base_dir,
            device.0,
            workspace_id.0,
            timestamp.map(|wrapped| wrapped.0),
        )
        .map_err(fs_to_python_error)
    })
}

fn fs_to_python_error(e: FSError) -> PyErr {
    match e {
        FSError::DatabaseQueryError(_) => FSLocalStorageOperationError::new_err(e.to_string()),
        FSError::NoSpaceLeftOnDevice => {
            FSLocalStorageOperationError::new_err("database or disk is full".to_string())
        }
        FSError::DatabaseClosed(_) => FSLocalStorageClosedError::new_err(e.to_string()),
        _ => FSInternalError::new_err(e.to_string()),
    }
}
