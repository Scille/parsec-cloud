// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::types::{PyByteArray, PyBytes};
use pyo3::{import_exception, prelude::*};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::binding_utils::{py_to_rs_regex, rs_to_py_regex};
use crate::ids::{BlockID, ChunkID, EntryID, FileDescriptor};
use crate::local_device::LocalDevice;
use crate::local_manifest::{
    LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest,
};

import_exception!(parsec.core.fs.exceptions, FSLocalMissError);
import_exception!(parsec.core.fs.exceptions, FSInvalidFileDescriptor);

#[pyclass]
pub(crate) struct WorkspaceStorage(pub parsec_core_fs::WorkspaceStorage, pub Option<PyObject>);

#[pymethods]
impl WorkspaceStorage {
    #[new]
    #[args(cache_size = "512 * 1024 * 1024")]
    #[allow(unused_variables)]
    fn new(
        data_base_dir: &PyAny,
        device: LocalDevice,
        workspace_id: EntryID,
        cache_size: u64,
        data_vacuum_threshold: Option<u64>,
    ) -> PyResult<Self> {
        let data_base_dir =
            PathBuf::from(data_base_dir.call_method0("__str__")?.extract::<String>()?);
        Ok(Self(
            parsec_core_fs::WorkspaceStorage::new(
                &data_base_dir,
                device.0,
                workspace_id.0,
                cache_size,
            )
            .map_err(|e| PyValueError::new_err(e.to_string()))?,
            None,
        ))
    }

    fn set_prevent_sync_pattern(&mut self, py: Python, pattern: &PyAny) -> PyResult<()> {
        self.1 = Some(PyObject::from(pattern));
        println!("1");
        crate::timer!(let pattern = py_to_rs_regex(pattern)?;
        py.allow_threads(|| {
            self.0
                .set_prevent_sync_pattern(&pattern)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            Ok(())
        }))
    }

    fn mark_prevent_sync_pattern_fully_applied(
        &mut self,
        py: Python,
        pattern: &PyAny,
    ) -> PyResult<()> {
        println!("2");
        crate::timer!(let pattern = py_to_rs_regex(pattern)?;
        py.allow_threads(|| {
            self.0
                .mark_prevent_sync_pattern_fully_applied(&pattern)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        }))
    }

    fn get_prevent_sync_pattern<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyAny> {
        if let Some(regex) = &self.1 {
            regex
                .cast_as::<PyAny>(py)
                .map_err(|_| PyValueError::new_err("Impossible to fail"))
        } else {
            let prevent_sync_pattern = py.allow_threads(|| self.0.get_prevent_sync_pattern());
            rs_to_py_regex(py, &prevent_sync_pattern)
        }
    }

    fn get_prevent_sync_pattern_fully_applied(&self, py: Python) -> PyResult<bool> {
        py.allow_threads(|| Ok(self.0.get_prevent_sync_pattern_fully_applied()))
    }

    fn get_workspace_manifest(&self, py: Python) -> PyResult<LocalWorkspaceManifest> {
        py.allow_threads(|| {
            Ok(LocalWorkspaceManifest(
                self.0
                    .get_workspace_manifest()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            ))
        })
    }

    fn get_manifest(&self, py: Python, entry_id: EntryID) -> PyResult<PyObject> {
        let manifest = py
            .allow_threads(|| self.0.get_manifest(entry_id.0))
            .map_err(|_| FSLocalMissError::new_err(entry_id))?;
        Ok(match manifest {
            parsec_client_types::LocalManifest::File(manifest) => {
                LocalFileManifest(manifest).into_py(py)
            }
            parsec_client_types::LocalManifest::Folder(manifest) => {
                LocalFolderManifest(manifest).into_py(py)
            }
            parsec_client_types::LocalManifest::Workspace(manifest) => {
                LocalWorkspaceManifest(manifest).into_py(py)
            }
            parsec_client_types::LocalManifest::User(manifest) => {
                LocalUserManifest(manifest).into_py(py)
            }
        })
    }

    #[args(cache_only = false, check_lock_status = true, removed_ids = "None")]
    #[allow(unused_variables)]
    fn set_manifest(
        &self,
        py: Python,
        entry_id: EntryID,
        manifest: PyObject,
        cache_only: bool,
        check_lock_status: bool,
        removed_ids: Option<HashSet<ChunkID>>,
    ) -> PyResult<()> {
        println!("7");
        crate::timer!(let manifest = if let Ok(manifest) = manifest.extract::<LocalFileManifest>(py) {
            parsec_client_types::LocalManifest::File(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalFolderManifest>(py) {
            parsec_client_types::LocalManifest::Folder(manifest.0)
        } else if let Ok(manifest) = manifest.extract::<LocalWorkspaceManifest>(py) {
            parsec_client_types::LocalManifest::Workspace(manifest.0)
        } else {
            parsec_client_types::LocalManifest::User(manifest.extract::<LocalUserManifest>(py)?.0)
        };
        py.allow_threads(|| {
            self.0
                .set_manifest(
                    entry_id.0,
                    manifest,
                    cache_only,
                    removed_ids.map(|x| {
                        x.into_iter()
                            .map(|id| parsec_core_fs::ChunkOrBlockID::ChunkID(id.0))
                            .collect()
                    }),
                )
                .map_err(|e| PyValueError::new_err(e.to_string()))
        }))
    }

    fn clear_manifest(&self, py: Python, entry_id: EntryID) -> PyResult<()> {
        println!("8");
        crate::timer!(py.allow_threads(|| {
            self.0
                .clear_manifest(entry_id.0)
                .map_err(|_| FSLocalMissError::new_err(entry_id))
        }))
    }

    fn create_file_descriptor(
        &self,
        py: Python,
        manifest: LocalFileManifest,
    ) -> PyResult<FileDescriptor> {
        py.allow_threads(|| Ok(FileDescriptor(self.0.create_file_descriptor(manifest.0))))
    }

    fn load_file_descriptor(&self, py: Python, fd: FileDescriptor) -> PyResult<LocalFileManifest> {
        py.allow_threads(|| {
            Ok(LocalFileManifest(
                self.0
                    .load_file_descriptor(fd.0)
                    .map_err(|e| FSInvalidFileDescriptor::new_err(e.to_string()))?,
            ))
        })
    }

    fn remove_file_descriptor(&self, py: Python, fd: FileDescriptor) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .remove_file_descriptor(fd.0)
                .map(|_| ())
                .ok_or_else(|| FSInvalidFileDescriptor::new_err(""))
        })
    }

    fn set_clean_block(&self, py: Python, block_id: BlockID, block: &[u8]) -> PyResult<()> {
        println!("12");
        crate::timer!(py.allow_threads(|| {
            self.0
                .set_clean_block(block_id.0, block)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        }))
    }

    fn clear_clean_block(&self, py: Python, block_id: BlockID) -> PyResult<()> {
        println!("13");
        crate::timer!(py.allow_threads(|| {
            self.0.clear_clean_block(block_id.0);
            Ok(())
        }))
    }

    fn get_dirty_block<'py>(&self, py: Python<'py>, block_id: BlockID) -> PyResult<&'py PyBytes> {
        println!("14");
        crate::timer!(let block = py
            .allow_threads(|| self.0.get_dirty_block(block_id.0))
            .map_err(|_| FSLocalMissError::new_err(block_id))?;
        Ok(PyBytes::new(py, &block)))
    }

    fn get_chunk<'py>(&self, py: Python<'py>, chunk_id: ChunkID) -> PyResult<&'py PyBytes> {
        println!("15");
        crate::timer!(let chunk = py
            .allow_threads(|| self.0.get_chunk(chunk_id.0))
            .map_err(|_| FSLocalMissError::new_err(chunk_id))?;
        Ok(PyBytes::new(py, &chunk)))
    }

    fn set_chunk(&self, py: Python, chunk_id: ChunkID, block: PyObject) -> PyResult<()> {
        println!("16");
        crate::timer!(let block = if let Ok(block) = block.extract::<&PyByteArray>(py) {
            // Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            unsafe { block.as_bytes() }
        } else if let Ok(block) = block.extract::<&[u8]>(py) {
            block
        } else {
            return Err(PyValueError::new_err(
                "evolve_as_block: invalid input for block",
            ));
        };

        py.allow_threads(|| {
            self.0
                .set_chunk(chunk_id.0, block)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        }))
    }

    #[args(miss_ok = false)]
    fn clear_chunk(&self, py: Python, chunk_id: ChunkID, miss_ok: bool) -> PyResult<()> {
        println!("17");
        crate::timer!(py.allow_threads(|| {
            self.0
                .clear_chunk(chunk_id.0, miss_ok)
                .map_err(|_| FSLocalMissError::new_err(chunk_id))
        }))
    }

    fn get_realm_checkpoint(&self, py: Python) -> PyResult<i32> {
        py.allow_threads(|| Ok(self.0.get_realm_checkpoint()))
    }

    fn update_realm_checkpoint(
        &self,
        py: Python,
        new_checkpoint: i32,
        changed_vlobs: HashMap<EntryID, i32>,
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
                .map_err(|e| PyValueError::new_err(e.to_string()))
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
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }

    fn ensure_manifest_persistent(&self, py: Python, entry_id: EntryID) -> PyResult<()> {
        println!("20");
        crate::timer!(py.allow_threads(|| {
            self.0
                .ensure_manifest_persistent(entry_id.0)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        }))
    }

    #[args(flush = true)]
    fn clear_memory_cache(&self, py: Python, flush: bool) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .clear_memory_cache(flush)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }

    fn run_vacuum(&self, py: Python) -> PyResult<()> {
        py.allow_threads(|| {
            self.0
                .run_vacuum()
                .map_err(|_| PyValueError::new_err("Vacuum failed"))
        })
    }

    fn block_storage_get_local_chunk_ids(
        &self,
        py: Python,
        chunk_ids: Vec<ChunkID>,
    ) -> PyResult<Vec<ChunkID>> {
        println!("23");
        crate::timer!(py.allow_threads(|| {
            Ok(self
                .0
                .block_storage_get_local_chunk_ids(
                    &chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>(),
                )
                .map_err(|e| PyValueError::new_err(e.to_string()))?
                .into_iter()
                .map(ChunkID)
                .collect())
        }))
    }

    fn chunk_storage_get_local_chunk_ids(
        &self,
        py: Python,
        chunk_ids: Vec<ChunkID>,
    ) -> PyResult<Vec<ChunkID>> {
        println!("24");
        crate::timer!(py.allow_threads(|| {
            Ok(self
                .0
                .chunk_storage_get_local_chunk_ids(
                    &chunk_ids.into_iter().map(|id| id.0).collect::<Vec<_>>(),
                )
                .map_err(|e| PyValueError::new_err(e.to_string()))?
                .into_iter()
                .map(ChunkID)
                .collect())
        }))
    }

    #[getter]
    fn device(&self) -> PyResult<LocalDevice> {
        Ok(LocalDevice(self.0.device.clone()))
    }

    #[getter]
    fn workspace_id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.workspace_id))
    }
}

#[macro_export]
macro_rules! timer {
    ($($tt:tt)*) => {
        {
            let t0 = std::time::Instant::now();
            let v = { $($tt)* };
            println!("{:?}", t0.elapsed());
            v
        }
    };
}
