// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    import_exception,
    prelude::{pyclass, pyfunction, pymethods, IntoPy, PyObject, PyResult, Python},
    types::{PyByteArray, PyBytes},
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    data::{LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest},
    ids::{BlockID, ChunkID, EntryID},
    local_device::LocalDevice,
    regex::Regex,
    runtime::FutureIntoCoroutine,
    time::DateTime,
};

use super::{fs_to_python_error, FSInternalError};

use libparsec::core_fs::FSError;

import_exception!(parsec.core.fs.exceptions, FSLocalMissError);
import_exception!(parsec.core.fs.exceptions, FSInvalidFileDescriptor);

/// WorkspaceStorage's binding is implemented with allow_threads because its
/// methods are called in trio.to_thread to connect the sync and async world
#[pyclass]
#[derive(Clone)]
pub(crate) struct WorkspaceStorage(
    pub Arc<libparsec::core_fs::WorkspaceStorage>,
    pub Option<Regex>,
);

#[pymethods]
impl WorkspaceStorage {
    #[staticmethod]
    #[args(cache_size = "512 * 1024 * 1024")]
    #[pyo3(name = "new")]
    fn new_async(
        data_base_dir: PathBuf,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex,
        cache_size: u64,
    ) -> FutureIntoCoroutine {
        FutureIntoCoroutine::from(async move {
            libparsec::core_fs::WorkspaceStorage::new(
                data_base_dir,
                device.0,
                workspace_id.0,
                prevent_sync_pattern.0,
                cache_size,
            )
            .await
            .map_err(fs_to_python_error)
            .map(|ws| Self(Arc::new(ws), None))
        })
    }

    fn close_connections(&self) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.close_connections().await;
            Ok(())
        })
    }

    fn set_prevent_sync_pattern(&mut self, pattern: &Regex) -> FutureIntoCoroutine {
        let pattern = pattern.clone();
        let ws = self.0.clone();
        self.1 = Some(pattern.clone());

        FutureIntoCoroutine::from(async move {
            let pattern = &pattern.0;
            ws.set_prevent_sync_pattern(pattern)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FutureIntoCoroutine {
        let pattern = pattern.clone();
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            let pattern = &pattern.0;
            ws.mark_prevent_sync_pattern_fully_applied(pattern)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn get_prevent_sync_pattern(&self) -> Regex {
        self.1
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Regex(self.0.get_prevent_sync_pattern()))
    }

    fn get_prevent_sync_pattern_fully_applied(&self) -> bool {
        self.0.get_prevent_sync_pattern_fully_applied()
    }

    fn get_workspace_manifest(&self) -> PyResult<LocalWorkspaceManifest> {
        self.0
            .get_workspace_manifest()
            .map_err(fs_to_python_error)
            .map(LocalWorkspaceManifest)
    }

    fn get_manifest(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        if let Some(manifest) = ws.get_manifest_in_cache(&entry_id.0) {
            let manifest = manifest_into_py_object(manifest);

            FutureIntoCoroutine::ready(Ok(manifest))
        } else {
            FutureIntoCoroutine::from_raw(async move {
                let manifest = ws
                    .get_manifest(entry_id.0)
                    .await
                    .map_err(|_| FSLocalMissError::new_err(entry_id))?;
                let manifest = manifest_into_py_object(manifest);
                Ok(manifest)
            })
        }
    }

    #[args(cache_only = false, removed_ids = "None")]
    fn set_manifest(
        &self,
        py: Python,
        entry_id: EntryID,
        manifest: PyObject,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkID>>,
    ) -> PyResult<FutureIntoCoroutine> {
        let manifest = manifest_from_py_object(py, manifest)?;
        let ws = self.0.clone();

        let fut = FutureIntoCoroutine::from(async move {
            ws.set_manifest(
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
            .await
            .map_err(fs_to_python_error)
        });

        Ok(fut)
    }

    fn clear_manifest(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.clear_manifest(entry_id.0, false)
                .await
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(entry_id),
                    _ => fs_to_python_error(e),
                })
        })
    }

    fn create_file_descriptor(&self, manifest: LocalFileManifest) -> u32 {
        self.0.create_file_descriptor(manifest.0).0
    }

    fn load_file_descriptor(&self, py: Python<'_>, fd: u32) -> PyResult<FutureIntoCoroutine> {
        let fd = libparsec::types::FileDescriptor(fd);

        let fut = match self.0.load_file_descriptor_in_cache(fd) {
            Ok(manifest) => FutureIntoCoroutine::ready(Ok(LocalFileManifest(manifest).into_py(py))),
            Err(libparsec::core_fs::FSError::LocalMiss(_)) => {
                let ws = self.0.clone();

                FutureIntoCoroutine::from(async move {
                    ws.load_file_descriptor(fd)
                        .await
                        .map_err(|e| FSInvalidFileDescriptor::new_err(e.to_string()))
                        .map(LocalFileManifest)
                })
            }
            Err(e) => return Err(FSInvalidFileDescriptor::new_err(e.to_string())),
        };

        Ok(fut)
    }

    fn remove_file_descriptor(&self, fd: u32) -> PyResult<()> {
        self.0
            .remove_file_descriptor(libparsec::types::FileDescriptor(fd))
            .map(|_| ())
            .ok_or_else(|| {
                FSInvalidFileDescriptor::new_err(format!("File descriptor `{fd}` is invalid"))
            })
    }

    fn set_clean_block(&self, block_id: BlockID, block: &[u8]) -> FutureIntoCoroutine {
        let ws = self.0.clone();
        let block = block.to_vec();

        FutureIntoCoroutine::from(async move {
            ws.set_clean_block(block_id.0, &block)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn clear_clean_block(&self, block_id: BlockID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.clear_clean_block(block_id.0).await;
            Ok(())
        })
    }

    fn get_dirty_block(&self, block_id: BlockID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let block = ws.get_dirty_block(block_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(block_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &block).into_py(py)))
        })
    }

    fn get_chunk(&self, chunk_id: ChunkID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let chunk = ws.get_chunk(chunk_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &chunk).into_py(py)))
        })
    }

    fn set_chunk(
        &self,
        py: Python,
        chunk_id: ChunkID,
        block: PyObject,
    ) -> PyResult<FutureIntoCoroutine> {
        let block = match block.extract::<&PyByteArray>(py) {
            Ok(x) => x.to_vec(),
            Err(_) => block.extract::<&PyBytes>(py)?.as_bytes().to_vec(),
        };

        let ws = self.0.clone();

        Ok(FutureIntoCoroutine::from(async move {
            ws.set_chunk(chunk_id.0, &block)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    #[args(miss_ok = false)]
    fn clear_chunk(&self, chunk_id: ChunkID, miss_ok: bool) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.clear_chunk(chunk_id.0, miss_ok)
                .await
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                    _ => fs_to_python_error(e),
                })
        })
    }

    fn get_realm_checkpoint(&self) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move { Ok(ws.get_realm_checkpoint().await) })
    }

    fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.update_realm_checkpoint(
                new_checkpoint,
                &changed_vlobs
                    .into_iter()
                    .map(|(id, x)| (id.0, x))
                    .collect::<Vec<_>>()[..],
            )
            .await
            .map_err(fs_to_python_error)
        })
    }

    fn get_need_sync_entries(&self) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.get_need_sync_entries()
                .await
                .map(|(res0, res1)| {
                    (
                        res0.into_iter().map(EntryID).collect::<HashSet<EntryID>>(),
                        res1.into_iter().map(EntryID).collect::<HashSet<EntryID>>(),
                    )
                })
                .map_err(fs_to_python_error)
        })
    }

    fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.ensure_manifest_persistent(entry_id.0, false)
                .await
                .map_err(fs_to_python_error)
        })
    }

    #[args(flush = true)]
    fn clear_memory_cache(&self, flush: bool) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.clear_memory_cache(flush)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn run_vacuum(&self) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.run_vacuum()
                .await
                .map_err(|_| FSInternalError::new_err("Vacuum failed"))
        })
    }

    fn get_local_block_ids(&self, chunk_ids: Vec<ChunkID>) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.get_local_block_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .await
                .map(|chunk_ids| chunk_ids.into_iter().map(ChunkID).collect::<Vec<_>>())
                .map_err(fs_to_python_error)
        })
    }

    fn get_local_chunk_ids(&self, chunk_ids: Vec<ChunkID>) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.get_local_chunk_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .await
                .map(|chunk_ids| chunk_ids.into_iter().map(ChunkID).collect::<Vec<_>>())
                .map_err(fs_to_python_error)
        })
    }

    fn to_timestamp(&self) -> WorkspaceStorageSnapshot {
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

    fn is_manifest_cache_ahead_of_persistance(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            Ok(ws.is_manifest_cache_ahead_of_persistance(&entry_id.0).await)
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct WorkspaceStorageSnapshot(
    pub Arc<libparsec::core_fs::WorkspaceStorageSnapshot>,
    pub Option<Regex>,
);

#[pymethods]
impl WorkspaceStorageSnapshot {
    #[new]
    fn new(workspace_storage: WorkspaceStorage) -> Self {
        Self(
            Arc::new(libparsec::core_fs::WorkspaceStorageSnapshot::from(
                workspace_storage.0.as_ref().clone(),
            )),
            workspace_storage.1,
        )
    }

    fn create_file_descriptor(&self, manifest: LocalFileManifest) -> u32 {
        self.0.create_file_descriptor(manifest.0).0
    }

    fn load_file_descriptor(&self, py: Python<'_>, fd: u32) -> FutureIntoCoroutine {
        let manifest = self
            .0
            .load_file_descriptor(libparsec::types::FileDescriptor(fd))
            .map_err(|e| FSInvalidFileDescriptor::new_err(e.to_string()))
            .map(LocalFileManifest)
            .map(|manifest| manifest.into_py(py));
        FutureIntoCoroutine::ready(manifest)
    }

    fn remove_file_descriptor(&self, fd: u32) -> PyResult<()> {
        self.0
            .remove_file_descriptor(libparsec::types::FileDescriptor(fd))
            .map(|_| ())
            .ok_or_else(|| FSInvalidFileDescriptor::new_err(""))
    }

    fn get_chunk(&self, chunk_id: ChunkID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let chunk = ws.get_chunk(chunk_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &chunk).into_py(py)))
        })
    }

    fn get_workspace_manifest(&self) -> PyResult<LocalWorkspaceManifest> {
        self.0
            .get_workspace_manifest()
            .map_err(fs_to_python_error)
            .map(LocalWorkspaceManifest)
    }

    fn get_manifest(&self, entry_id: EntryID) -> FutureIntoCoroutine {
        let manifest = self
            .0
            .get_manifest(entry_id.0)
            .map_err(|_| FSLocalMissError::new_err(entry_id))
            .map(manifest_into_py_object);

        FutureIntoCoroutine::ready(manifest)
    }

    fn set_manifest(
        &self,
        py: Python,
        entry_id: EntryID,
        manifest: PyObject,
    ) -> PyResult<FutureIntoCoroutine> {
        let manifest = manifest_from_py_object(py, manifest)?;
        let ws = self.0.clone();

        let fut = FutureIntoCoroutine::from(async move {
            ws.set_manifest(entry_id.0, manifest, false)
                .await
                .map_err(fs_to_python_error)
        });

        Ok(fut)
    }

    fn get_prevent_sync_pattern(&self) -> Regex {
        self.1
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Regex(self.0.get_prevent_sync_pattern()))
    }

    fn get_prevent_sync_pattern_fully_applied(&self) -> bool {
        self.0.get_prevent_sync_pattern_fully_applied()
    }

    fn set_clean_block(&self, block_id: BlockID, block: &[u8]) -> FutureIntoCoroutine {
        let ws = self.0.clone();
        let block = block.to_vec();

        FutureIntoCoroutine::from(async move {
            ws.set_clean_block(block_id.0, &block)
                .await
                .map_err(fs_to_python_error)
        })
    }

    fn clear_clean_block(&self, block_id: BlockID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from(async move {
            ws.clear_clean_block(block_id.0).await;
            Ok(())
        })
    }

    fn get_dirty_block(&self, block_id: BlockID) -> FutureIntoCoroutine {
        let ws = self.0.clone();

        FutureIntoCoroutine::from_raw(async move {
            let block = ws.get_dirty_block(block_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(block_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &block).into_py(py)))
        })
    }

    fn to_timestamp(&self) -> PyResult<Self> {
        Ok(Self(Arc::new(self.0.to_timestamp()), self.1.clone()))
    }

    fn clear_local_cache(&self, py: Python<'_>) -> FutureIntoCoroutine {
        self.0.clear_local_cache();

        FutureIntoCoroutine::ready(Ok(py.None()))
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
    data_base_dir: PathBuf,
    device: LocalDevice,
    workspace_id: EntryID,
    timestamp: Option<DateTime>,
) -> FutureIntoCoroutine {
    FutureIntoCoroutine::from(async move {
        libparsec::core_fs::workspace_storage_non_speculative_init(
            &data_base_dir,
            device.0,
            workspace_id.0,
            timestamp.map(|wrapped| wrapped.0),
        )
        .await
        .map_err(fs_to_python_error)
    })
}

fn manifest_into_py_object(manifest: libparsec::client_types::LocalManifest) -> PyObject {
    Python::with_gil(|py| {
        let object: PyObject = match manifest {
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
        };
        object
    })
}

fn manifest_from_py_object(
    py: Python<'_>,
    py_manifest: PyObject,
) -> PyResult<libparsec::client_types::LocalManifest> {
    let manifest = if let Ok(manifest) = py_manifest.extract::<LocalFileManifest>(py) {
        libparsec::client_types::LocalManifest::File(manifest.0)
    } else if let Ok(manifest) = py_manifest.extract::<LocalFolderManifest>(py) {
        libparsec::client_types::LocalManifest::Folder(manifest.0)
    } else if let Ok(manifest) = py_manifest.extract::<LocalWorkspaceManifest>(py) {
        libparsec::client_types::LocalManifest::Workspace(manifest.0)
    } else {
        libparsec::client_types::LocalManifest::User(
            py_manifest.extract::<LocalUserManifest>(py)?.0,
        )
    };

    Ok(manifest)
}
