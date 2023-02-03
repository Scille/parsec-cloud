// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    prelude::{pyclass, pyfunction, pymethods, IntoPy, PyObject, PyResult, Python},
    types::PyBytes,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    binding_utils::{BytesWrapper, UnwrapBytesWrapper},
    data::{LocalFileManifest, LocalWorkspaceManifest},
    ids::{BlockID, ChunkID, EntryID},
    local_device::LocalDevice,
    regex::Regex,
    runtime::FutureIntoCoroutine,
    time::DateTime,
};

use super::{
    fs_to_python_error, manifest_from_py_object, manifest_into_py_object,
    workspace_storage_snapshot::WorkspaceStorageSnapshot, FSInternalError, FSInvalidFileDescriptor,
    FSLocalMissError,
};

use libparsec::core_fs::FSError;

/// WorkspaceStorage's binding is implemented with allow_threads because its
/// methods are called in trio.to_thread to connect the sync and async world
#[pyclass]
#[derive(Clone)]
pub(crate) struct WorkspaceStorage(
    /// Hold the reference to an unique [libparsec::core_fs::WorkspaceStorage].
    ///
    /// # Why the imbricated `Arc<...Arc<...>>` ?
    ///
    /// The first one is to share a lock over the second one.
    /// The lock is here when we do `close_connection` after that the WorkspaceStorage should not be accessible.
    /// The second one is because of [FutureIntoCoroutine] that require to provide a `static` future.
    /// To fullfish that requirement we have to clone the reference over [libparsec::core_fs::WorkspaceStorage].
    pub Arc<RwLock<Option<Arc<libparsec::core_fs::WorkspaceStorage>>>>,
    pub Option<Regex>,
);

impl WorkspaceStorage {
    pub(crate) fn get_storage(&self) -> PyResult<Arc<libparsec::core_fs::WorkspaceStorage>> {
        self.0
            .read()
            .expect("RwLock is poisoned")
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                FSInternalError::new_err("Trying to use an already closed WorkspaceStorage")
            })
    }

    fn drop_storage(&self) -> PyResult<Arc<libparsec::core_fs::WorkspaceStorage>> {
        self.0
            .write()
            .expect("RwLock is poisoned")
            .take()
            .ok_or_else(|| {
                FSInternalError::new_err("Trying to use an already closed WorkspaceStorage")
            })
    }
}

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
            .map(|ws| Self(Arc::new(RwLock::new(Some(Arc::new(ws)))), None))
        })
    }

    fn close_connections(&self) -> PyResult<FutureIntoCoroutine> {
        let ws = self.drop_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.close_connections().await.map_err(fs_to_python_error)
        }))
    }

    fn set_prevent_sync_pattern(&mut self, pattern: &Regex) -> PyResult<FutureIntoCoroutine> {
        let pattern = pattern.clone();
        let ws = self.get_storage()?;
        self.1 = Some(pattern.clone());

        Ok(FutureIntoCoroutine::from(async move {
            let pattern = &pattern.0;
            ws.set_prevent_sync_pattern(pattern)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    fn mark_prevent_sync_pattern_fully_applied(
        &self,
        pattern: &Regex,
    ) -> PyResult<FutureIntoCoroutine> {
        let pattern = pattern.clone();
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            let pattern = &pattern.0;
            ws.mark_prevent_sync_pattern_fully_applied(pattern)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    pub(crate) fn get_prevent_sync_pattern(&self) -> PyResult<Regex> {
        if let Some(re) = self.1.as_ref() {
            Ok(re.clone())
        } else {
            self.get_storage()
                .map(|ws| Regex(ws.get_prevent_sync_pattern()))
        }
    }

    pub(crate) fn get_prevent_sync_pattern_fully_applied(&self) -> PyResult<bool> {
        self.get_storage()
            .map(|ws| ws.get_prevent_sync_pattern_fully_applied())
    }

    pub(crate) fn get_workspace_manifest(&self) -> PyResult<LocalWorkspaceManifest> {
        self.get_storage().and_then(|ws| {
            ws.get_workspace_manifest()
                .map_err(fs_to_python_error)
                .map(LocalWorkspaceManifest)
        })
    }

    fn get_manifest(&self, entry_id: EntryID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        let fut = if let Some(manifest) = ws.get_manifest_in_cache(&entry_id.0) {
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
        };

        Ok(fut)
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
        let ws = self.get_storage()?;
        let manifest = manifest_from_py_object(py, manifest)?;
        let removed_ids = removed_ids.map(|x| {
            x.into_iter()
                .map(|id| libparsec::core_fs::ChunkOrBlockID::ChunkID(id.0))
                .collect()
        });

        let fut = if cache_only {
            FutureIntoCoroutine::ready(
                ws.set_manifest_in_cache(entry_id.0, manifest, false, removed_ids)
                    .map_err(fs_to_python_error)
                    .map(|_| py.None()),
            )
        } else {
            FutureIntoCoroutine::from(async move {
                ws.set_manifest(entry_id.0, manifest, cache_only, false, removed_ids)
                    .await
                    .map_err(fs_to_python_error)
            })
        };

        Ok(fut)
    }

    fn clear_manifest(&self, entry_id: EntryID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.clear_manifest(&entry_id.0, false)
                .await
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(entry_id),
                    _ => fs_to_python_error(e),
                })
        }))
    }

    fn create_file_descriptor(&self, manifest: LocalFileManifest) -> PyResult<u32> {
        self.get_storage()
            .map(|ws| ws.create_file_descriptor(manifest.0).0)
    }

    fn load_file_descriptor(&self, py: Python<'_>, fd: u32) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;
        let fd = libparsec::types::FileDescriptor(fd);

        let fut = match ws.load_file_descriptor_in_cache(fd) {
            Ok(manifest) => FutureIntoCoroutine::ready(Ok(LocalFileManifest(manifest).into_py(py))),
            Err(libparsec::core_fs::FSError::LocalMiss(_)) => {
                FutureIntoCoroutine::from(async move {
                    ws.load_file_descriptor(fd)
                        .await
                        .map_err(fs_to_python_error)
                        .map(LocalFileManifest)
                })
            }
            Err(e) => return Err(FSInvalidFileDescriptor::new_err(e.to_string())),
        };

        Ok(fut)
    }

    fn remove_file_descriptor(&self, fd: u32) -> PyResult<()> {
        self.get_storage().and_then(|ws| {
            ws.remove_file_descriptor(libparsec::types::FileDescriptor(fd))
                .map(|_| ())
                .ok_or_else(|| {
                    FSInvalidFileDescriptor::new_err(format!("File descriptor `{fd}` is invalid"))
                })
        })
    }

    pub(crate) fn set_clean_block(
        &self,
        block_id: BlockID,
        block: &[u8],
    ) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;
        let block = block.to_vec();

        Ok(FutureIntoCoroutine::from(async move {
            ws.set_clean_block(block_id.0, &block)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    pub(crate) fn clear_clean_block(&self, block_id: BlockID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.clear_clean_block(block_id.0).await;
            Ok(())
        }))
    }

    pub(crate) fn get_dirty_block(&self, block_id: BlockID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from_raw(async move {
            let block = ws.get_dirty_block(block_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(block_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &block).into_py(py)))
        }))
    }

    pub(crate) fn get_chunk(&self, chunk_id: ChunkID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from_raw(async move {
            let chunk = ws.get_chunk(chunk_id.0).await.map_err(|e| match e {
                FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                _ => fs_to_python_error(e),
            })?;

            Ok(Python::with_gil(|py| PyBytes::new(py, &chunk).into_py(py)))
        }))
    }

    fn set_chunk(&self, chunk_id: ChunkID, block: BytesWrapper) -> PyResult<FutureIntoCoroutine> {
        let block = block.unwrap_bytes();

        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.set_chunk(chunk_id.0, &block)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    #[args(miss_ok = false)]
    fn clear_chunk(&self, chunk_id: ChunkID, miss_ok: bool) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.clear_chunk(chunk_id.0, miss_ok)
                .await
                .map_err(|e| match e {
                    FSError::LocalMiss(_) => FSLocalMissError::new_err(chunk_id),
                    _ => fs_to_python_error(e),
                })
        }))
    }

    fn get_realm_checkpoint(&self) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            Ok(ws.get_realm_checkpoint().await)
        }))
    }

    fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: HashMap<EntryID, i64>,
    ) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.update_realm_checkpoint(
                new_checkpoint,
                &changed_vlobs
                    .into_iter()
                    .map(|(id, x)| (id.0, x))
                    .collect::<Vec<_>>()[..],
            )
            .await
            .map_err(fs_to_python_error)
        }))
    }

    fn get_need_sync_entries(&self) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.get_need_sync_entries()
                .await
                .map(|(res0, res1)| {
                    (
                        res0.into_iter().map(EntryID).collect::<HashSet<EntryID>>(),
                        res1.into_iter().map(EntryID).collect::<HashSet<EntryID>>(),
                    )
                })
                .map_err(fs_to_python_error)
        }))
    }

    fn ensure_manifest_persistent(&self, entry_id: EntryID) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.ensure_manifest_persistent(entry_id.0, false)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    #[args(flush = true)]
    fn clear_memory_cache(&self, flush: bool) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.clear_memory_cache(flush)
                .await
                .map_err(fs_to_python_error)
        }))
    }

    fn run_vacuum(&self) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.run_vacuum()
                .await
                .map_err(|_| FSInternalError::new_err("Vacuum failed"))
        }))
    }

    fn get_local_block_ids(&self, chunk_ids: Vec<ChunkID>) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.get_local_block_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .await
                .map(|chunk_ids| chunk_ids.into_iter().map(ChunkID).collect::<Vec<_>>())
                .map_err(fs_to_python_error)
        }))
    }

    fn get_local_chunk_ids(&self, chunk_ids: Vec<ChunkID>) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            ws.get_local_chunk_ids(&chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>())
                .await
                .map(|chunk_ids| chunk_ids.into_iter().map(ChunkID).collect::<Vec<_>>())
                .map_err(fs_to_python_error)
        }))
    }

    fn to_timestamp(&self) -> WorkspaceStorageSnapshot {
        WorkspaceStorageSnapshot::from(self.clone())
    }

    #[getter]
    pub(crate) fn device(&self) -> PyResult<LocalDevice> {
        self.get_storage().map(|ws| LocalDevice(ws.device.clone()))
    }

    #[getter]
    pub(crate) fn workspace_id(&self) -> PyResult<EntryID> {
        self.get_storage().map(|ws| EntryID(ws.workspace_id))
    }

    fn is_manifest_cache_ahead_of_persistance(
        &self,
        entry_id: EntryID,
    ) -> PyResult<FutureIntoCoroutine> {
        let ws = self.get_storage()?;

        Ok(FutureIntoCoroutine::from(async move {
            Ok(ws.is_manifest_cache_ahead_of_persistance(&entry_id.0).await)
        }))
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
