// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBool, PyBytes, PyDict, PyTuple, PyType};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU64;

use crate::binding_utils::{
    kwargs_extract_optional, kwargs_extract_optional_custom, kwargs_extract_required,
    kwargs_extract_required_custom, kwargs_extract_required_frozenset, pattern_match,
    py_to_rs_datetime, rs_to_py_datetime,
};
use crate::crypto::SecretKey;
use crate::ids::{ChunkID, DeviceID, EntryID};
use crate::manifest::{
    BlockAccess, EntryName, FileManifest, FolderManifest, UserManifest, WorkspaceEntry,
    WorkspaceManifest,
};

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Chunk(pub parsec_client_types::Chunk);

impl Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
            && self.0.start == other.0.start
            && self.0.stop == other.0.stop
            && self.0.raw_offset == other.0.raw_offset
            && self.0.raw_size == other.0.raw_size
            && self.0.access == other.0.access
    }
    fn eq_int(&self, other: u64) -> bool {
        self.0.start == other
    }
    fn lt_int(&self, other: u64) -> bool {
        self.0.start < other
    }
    fn gt_int(&self, other: u64) -> bool {
        self.0.start > other
    }
}

#[pymethods]
impl Chunk {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let id = kwargs_extract_required::<ChunkID>(args, "id")?;
        let start = kwargs_extract_required(args, "start")?;
        let stop = kwargs_extract_required::<u64>(args, "stop")?;
        let raw_offset = kwargs_extract_required(args, "raw_offset")?;
        let raw_size = kwargs_extract_required::<u64>(args, "raw_size")?;
        let access = kwargs_extract_optional::<BlockAccess>(args, "access")?;

        Ok(Self(parsec_client_types::Chunk {
            id: id.0,
            start,
            stop: NonZeroU64::try_from(stop)
                .map_err(|_| PyValueError::new_err("Invalid stop field"))?,
            raw_offset,
            raw_size: NonZeroU64::try_from(raw_size)
                .map_err(|_| PyValueError::new_err("Invalid raw_size field"))?,
            access: match access {
                Some(access) => Some(access.0),
                None => None,
            },
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[classmethod]
    fn from_block_access(_cls: &PyType, block_access: BlockAccess) -> PyResult<Self> {
        Ok(Self(
            parsec_client_types::Chunk::from_block_access(block_access.0)
                .map_err(PyValueError::new_err)?,
        ))
    }

    fn evolve_as_block(&self, data: Vec<u8>) -> PyResult<Self> {
        Ok(Self(
            self.0
                .evolve_as_block(&data)
                .map_err(PyValueError::new_err)?,
        ))
    }

    fn is_block(&self) -> PyResult<bool> {
        Ok(self.0.is_block())
    }

    fn is_pseudo_block(&self) -> PyResult<bool> {
        Ok(self.0.is_pseudo_block())
    }

    fn get_block_access(&self) -> PyResult<Option<BlockAccess>> {
        Ok(self
            .0
            .get_block_access()
            .map_err(PyValueError::new_err)?
            .map(BlockAccess))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut r = self.0.clone();

        if let Some(args) = py_kwargs {
            if let Some(v) = kwargs_extract_optional::<ChunkID>(args, "id")? {
                r.id = v.0;
            }
            if let Some(v) = kwargs_extract_optional(args, "start")? {
                r.start = v;
            }
            if let Some(v) = kwargs_extract_optional::<u64>(args, "stop")? {
                r.stop = NonZeroU64::try_from(v)
                    .map_err(|_| PyValueError::new_err("Invalid stop field"))?;
            }
            if let Some(v) = kwargs_extract_optional(args, "raw_offset")? {
                r.raw_offset = v;
            }
            if let Some(v) = kwargs_extract_optional::<u64>(args, "raw_size")? {
                r.raw_size = NonZeroU64::try_from(v)
                    .map_err(|_| PyValueError::new_err("Invalid raw_size field"))?;
            }
            if let Some(v) = kwargs_extract_optional::<Option<BlockAccess>>(args, "access")? {
                r.access = match v {
                    Some(v) => Some(v.0),
                    None => None,
                }
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &PyAny, op: CompareOp) -> PyObject {
        match (other.extract::<u64>(), op) {
            (Ok(other), CompareOp::Eq) => PyBool::new(py, self.eq_int(other)).into_py(py),
            (Ok(other), CompareOp::Ne) => PyBool::new(py, !self.eq_int(other)).into_py(py),
            (Ok(other), CompareOp::Lt) => PyBool::new(py, self.lt_int(other)).into_py(py),
            (Ok(other), CompareOp::Gt) => PyBool::new(py, self.gt_int(other)).into_py(py),
            (Ok(other), CompareOp::Le) => PyBool::new(py, !self.gt_int(other)).into_py(py),
            (Ok(other), CompareOp::Ge) => PyBool::new(py, !self.lt_int(other)).into_py(py),
            _ => match (other.extract::<Chunk>(), op) {
                (Ok(other), CompareOp::Eq) => PyBool::new(py, self.eq(&other)).into_py(py),
                (Ok(other), CompareOp::Ne) => PyBool::new(py, !self.eq(&other)).into_py(py),
                _ => py.NotImplemented(),
            },
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType, start: u64, stop: u64) -> PyResult<Self> {
        assert!(start < stop);
        Ok(Self(parsec_client_types::Chunk {
            id: parsec_api_types::ChunkID::default(),
            start,
            stop: NonZeroU64::try_from(stop).map_err(PyValueError::new_err)?,
            raw_offset: start,
            raw_size: NonZeroU64::try_from(stop - start).map_err(PyValueError::new_err)?,
            access: None,
        }))
    }

    #[getter]
    fn id(&self) -> PyResult<ChunkID> {
        Ok(ChunkID(self.0.id))
    }

    #[getter]
    fn start(&self) -> PyResult<u64> {
        Ok(self.0.start)
    }

    #[getter]
    fn stop(&self) -> PyResult<u64> {
        Ok(self.0.stop.into())
    }

    #[getter]
    fn raw_offset(&self) -> PyResult<u64> {
        Ok(self.0.raw_offset)
    }

    #[getter]
    fn raw_size(&self) -> PyResult<u64> {
        Ok(self.0.raw_size.into())
    }

    #[getter]
    fn access(&self) -> PyResult<Option<BlockAccess>> {
        Ok(self.0.access.clone().map(BlockAccess))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalFileManifest(pub parsec_client_types::LocalFileManifest);

impl LocalFileManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.base == other.0.base
            && self.0.need_sync == other.0.need_sync
            && self.0.updated == other.0.updated
            && self.0.size == other.0.size
            && self.0.blocksize == other.0.blocksize
            && self.0.blocks == other.0.blocks
    }
}

#[pymethods]
impl LocalFileManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let base = kwargs_extract_required::<FileManifest>(args, "base")?;
        let need_sync = kwargs_extract_required(args, "need_sync")?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let size = kwargs_extract_required(args, "size")?;
        let blocksize = kwargs_extract_required::<u64>(args, "blocksize")?;
        let blocks = kwargs_extract_required::<Vec<Vec<Chunk>>>(args, "blocks")?
            .into_iter()
            .map(|b| b.into_iter().map(|b| b.0).collect())
            .collect();

        Ok(Self(parsec_client_types::LocalFileManifest {
            base: base.0,
            need_sync,
            updated,
            size,
            blocksize: parsec_api_types::Blocksize::try_from(blocksize)
                .map_err(|_| PyValueError::new_err("Invalid blocksize field"))?,
            blocks,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn get_chunks(&self, block: usize) -> PyResult<Vec<Chunk>> {
        Ok(self.0.get_chunks(block).into_iter().map(Chunk).collect())
    }

    fn is_reshaped(&self) -> PyResult<bool> {
        Ok(self.0.is_reshaped())
    }

    fn assert_integrity(&self) -> PyResult<()> {
        self.0.assert_integrity();
        Ok(())
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: FileManifest) -> PyResult<Self> {
        Ok(Self(
            parsec_client_types::LocalFileManifest::from_remote(remote.0)
                .map_err(PyValueError::new_err)?,
        ))
    }

    fn to_remote(&self, author: DeviceID, timestamp: &PyAny) -> PyResult<FileManifest> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        Ok(FileManifest(
            self.0
                .to_remote(author.0, timestamp)
                .map_err(PyValueError::new_err)?,
        ))
    }

    fn match_remote(&self, remote_manifest: &FileManifest) -> PyResult<bool> {
        self.0
            .match_remote(&remote_manifest.0)
            .map_err(PyValueError::new_err)
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> PyResult<u32> {
        Ok(self.0.base.version)
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.id))
    }

    #[getter]
    fn created<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        rs_to_py_datetime(py, self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> PyResult<bool> {
        Ok(self.0.base.version == 0)
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match parsec_client_types::LocalFileManifest::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut r = self.0.clone();

        if let Some(args) = py_kwargs {
            if let Some(v) = kwargs_extract_optional::<FileManifest>(args, "base")? {
                r.base = v.0;
            }
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                r.need_sync = v;
            }
            if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
                r.updated = v;
            }
            if let Some(v) = kwargs_extract_optional(args, "size")? {
                r.size = v;
            }
            if let Some(v) = kwargs_extract_optional::<u64>(args, "blocksize")? {
                r.blocksize = parsec_api_types::Blocksize::try_from(v)
                    .map_err(|_| PyValueError::new_err("Invalid blocksize field"))?;
            }
            if let Some(v) = kwargs_extract_optional::<Vec<Vec<Chunk>>>(args, "blocks")? {
                r.blocks = v
                    .into_iter()
                    .map(|v| v.into_iter().map(|chunk| chunk.0).collect())
                    .collect();
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[classmethod]
    #[args(blocksize = "512 * 1024")]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        parent: EntryID,
        timestamp: &PyAny,
        blocksize: u64,
    ) -> PyResult<Self> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        let id = parsec_api_types::EntryID::default();
        let blocksize =
            parsec_api_types::Blocksize::try_from(blocksize).map_err(PyValueError::new_err)?;

        Ok(Self(parsec_client_types::LocalFileManifest {
            base: parsec_api_types::FileManifest {
                author: author.0,
                timestamp,
                id,
                parent: parent.0,
                version: 0,
                created: timestamp,
                updated: timestamp,
                blocksize,
                size: 0,
                blocks: vec![],
            },
            need_sync: true,
            updated: timestamp,
            blocksize,
            size: 0,
            blocks: vec![],
        }))
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        Ok([
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "file".to_object(py)),
            ("size", self.0.size.to_object(py)),
        ]
        .into_py_dict(py))
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let id = EntryID(self.0.base.id).into_py(py);
        let parent = EntryID(self.0.base.parent).into_py(py);
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        let blocks = self
            .0
            .blocks
            .iter()
            .map(|b| b.iter().cloned().map(|c| Chunk(c).into_py(py)).collect())
            .collect::<Vec<Vec<_>>>()
            .into_py(py);
        Ok([
            ("id", id.to_object(py)),
            ("parent", parent.to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("blocks", blocks.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("size", self.0.size.to_object(py)),
            ("blocksize", u64::from(self.0.blocksize).to_object(py)),
        ]
        .into_py_dict(py))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(&self, timestamp: &PyAny, data: Option<&PyDict>) -> PyResult<Self> {
        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = py_to_rs_datetime(timestamp)?;
        if let Some(args) = data {
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                out.0.need_sync = v;
            }
        }

        Ok(out)
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.parent))
    }

    #[getter]
    fn base(&self) -> PyResult<FileManifest> {
        Ok(FileManifest(self.0.base.clone()))
    }

    #[getter]
    fn need_sync(&self) -> PyResult<bool> {
        Ok(self.0.need_sync)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn size(&self) -> PyResult<u64> {
        Ok(self.0.size)
    }

    #[getter]
    fn blocksize(&self) -> PyResult<u64> {
        Ok(self.0.blocksize.into())
    }

    #[getter]
    fn blocks(&self) -> PyResult<Vec<Vec<Chunk>>> {
        Ok(self
            .0
            .blocks
            .clone()
            .into_iter()
            .map(|b| b.into_iter().map(Chunk).collect())
            .collect())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalFolderManifest(pub parsec_client_types::LocalFolderManifest);

impl LocalFolderManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.base == other.0.base
            && self.0.need_sync == other.0.need_sync
            && self.0.updated == other.0.updated
            && self.0.children == other.0.children
            && self.0.local_confinement_points == other.0.local_confinement_points
            && self.0.remote_confinement_points == other.0.remote_confinement_points
    }
}

#[pymethods]
impl LocalFolderManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let base = kwargs_extract_required::<FolderManifest>(args, "base")?;
        let need_sync = kwargs_extract_required(args, "need_sync")?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let children = kwargs_extract_required::<HashMap<EntryName, EntryID>>(args, "children")?
            .into_iter()
            .map(|(name, id)| (name.0, id.0))
            .collect();
        let local_confinement_points =
            kwargs_extract_required_frozenset::<EntryID>(args, "local_confinement_points")?
                .into_iter()
                .map(|id| id.0)
                .collect();
        let remote_confinement_points =
            kwargs_extract_required_frozenset::<EntryID>(args, "remote_confinement_points")?
                .into_iter()
                .map(|id| id.0)
                .collect::<HashSet<_>>();

        Ok(Self(parsec_client_types::LocalFolderManifest {
            base: base.0,
            need_sync,
            updated,
            children,
            local_confinement_points,
            remote_confinement_points,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> PyResult<u32> {
        Ok(self.0.base.version)
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.id))
    }

    #[getter]
    fn created<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        rs_to_py_datetime(py, self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> PyResult<bool> {
        Ok(self.0.base.version == 0)
    }

    fn match_remote(&self, remote_manifest: FolderManifest) -> PyResult<bool> {
        Ok(self.0.match_remote(&remote_manifest.0))
    }

    /*
     * LocalFolderishManifestMixin
     */

    fn evolve_children_and_mark_updated(
        &self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let mut result = self.clone();
        let mut actually_updated = false;
        // Deal with removal first
        for (name, entry_id) in data.iter() {
            // Here `entry_id` can be either:
            // - a new entry id that might overwrite the previous one with the same name if it exists
            // - `None` which means the entry for the corresponding name should be removed
            if !result.0.children.contains_key(&name.0) {
                // Make sure we don't remove a name that does not exist
                assert!(entry_id.is_some());
                continue;
            }
            // Remove old entry
            if let Some(old_entry_id) = result.0.children.remove(&name.0) {
                if !result.0.local_confinement_points.remove(&old_entry_id) {
                    actually_updated = true;
                }
            }
        }
        // Make sure no entry_id is duplicated
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(
                data.values().filter_map(|v| v.as_ref().map(|v| v.0))
            )
            .intersection(&HashSet::from_iter(result.0.children.values().copied()))
            .count(),
            0
        );
        // Deal with additions second

        for (name, entry_id) in data.iter() {
            if let Some(entry_id) = entry_id {
                // Add new entry
                result.0.children.insert(name.0.clone(), entry_id.0);
                if pattern_match(prevent_sync_pattern, name.0.as_ref())? {
                    result.0.local_confinement_points.insert(entry_id.0);
                } else {
                    actually_updated = true;
                }
            }
        }

        if !actually_updated {
            return Ok(result);
        }
        result.evolve_and_mark_updated(timestamp, None)
    }

    fn _filter_local_confinement_points(&self) -> PyResult<Self> {
        Ok(Self(self.0._filter_local_confinement_points()))
    }

    fn _restore_local_confinement_points(
        &self,
        other: &Self,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        // Using self.remote_confinement_points is useful to restore entries that were present locally
        // before applying a new filter that filtered those entries from the remote manifest
        if other.0.local_confinement_points.is_empty()
            && self.0.remote_confinement_points.is_empty()
        {
            return Ok(self.clone());
        }
        // Create a set for fast lookup in order to make sure no entry gets duplicated.
        // This might happen when a synchronized entry is renamed to a confined name locally.
        let self_entry_ids = HashSet::<_, RandomState>::from_iter(self.0.children.values());
        let previously_local_confinement_points = other
            .0
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if !self_entry_ids.contains(entry_id)
                    && (other.0.local_confinement_points.contains(entry_id)
                        || self.0.remote_confinement_points.contains(entry_id))
                {
                    Some((EntryName(name.clone()), Some(EntryID(*entry_id))))
                } else {
                    None
                }
            })
            .collect();

        self.evolve_children_and_mark_updated(
            previously_local_confinement_points,
            prevent_sync_pattern,
            timestamp,
        )
    }

    fn _filter_remote_entries(&self, prevent_sync_pattern: &PyAny) -> PyResult<Self> {
        let mut result = self.clone();

        result.0.remote_confinement_points = self
            .0
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if let Ok(true) = pattern_match(prevent_sync_pattern, name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        if result.0.remote_confinement_points.is_empty() {
            return Ok(self.clone());
        }

        result
            .0
            .children
            .retain(|_, entry_id| !result.0.remote_confinement_points.contains(entry_id));

        Ok(result)
    }

    fn _restore_remote_confinement_points(&self) -> PyResult<Self> {
        Ok(Self(self.0._restore_remote_confinement_points()))
    }

    fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let mut result = self.clone();
        // Filter local confinement points
        result = result._filter_local_confinement_points()?;
        // Restore remote confinement points
        result = result._restore_remote_confinement_points()?;
        // Filter remote confinement_points
        result = result._filter_remote_entries(prevent_sync_pattern)?;
        // Restore local confinement points
        result = result._restore_local_confinement_points(self, prevent_sync_pattern, timestamp)?;
        Ok(result)
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match parsec_client_types::LocalFolderManifest::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut r = self.0.clone();

        if let Some(args) = py_kwargs {
            if let Some(v) = kwargs_extract_optional::<FolderManifest>(args, "base")? {
                r.base = v.0;
            }
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                r.need_sync = v;
            }
            if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
                r.updated = v;
            }
            if let Some(v) =
                kwargs_extract_optional::<HashMap<EntryName, EntryID>>(args, "children")?
            {
                r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
            }
            if let Some(v) =
                kwargs_extract_optional::<HashSet<EntryID>>(args, "local_confinement_points")?
            {
                r.local_confinement_points = v.into_iter().map(|id| id.0).collect();
            }
            if let Some(v) =
                kwargs_extract_optional::<HashSet<EntryID>>(args, "remote_confinement_points")?
            {
                r.remote_confinement_points = v.into_iter().map(|id| id.0).collect();
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[classmethod]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        parent: EntryID,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        let id = parsec_api_types::EntryID::default();

        Ok(Self(parsec_client_types::LocalFolderManifest {
            base: parsec_api_types::FolderManifest {
                author: author.0,
                timestamp,
                id,
                parent: parent.0,
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
        }))
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        let mut children = self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        children.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

        let children = children
            .into_iter()
            .map(EntryName)
            .collect::<Vec<_>>()
            .into_py(py);

        Ok([
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "folder".to_object(py)),
            ("children", children.to_object(py)),
        ]
        .into_py_dict(py))
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let id = EntryID(self.0.base.id).into_py(py);
        let parent = EntryID(self.0.base.parent).into_py(py);
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        let children = self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(k, v)| (EntryName(k), EntryID(v)))
            .collect::<HashMap<_, _>>()
            .into_py(py);
        let local_confinement_points = self
            .0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect::<Vec<_>>()
            .into_py(py);
        let remote_confinement_points = self
            .0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect::<Vec<_>>()
            .into_py(py);
        Ok([
            ("id", id.to_object(py)),
            ("parent", parent.to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("children", children.to_object(py)),
            (
                "local_confinement_points",
                local_confinement_points.to_object(py),
            ),
            (
                "remote_confinement_points",
                remote_confinement_points.to_object(py),
            ),
        ]
        .into_py_dict(py))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(&self, timestamp: &PyAny, data: Option<&PyDict>) -> PyResult<Self> {
        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = py_to_rs_datetime(timestamp)?;
        if let Some(args) = data {
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                out.0.need_sync = v;
            }
        }

        Ok(out)
    }

    #[classmethod]
    fn from_remote(
        _cls: &PyType,
        remote: FolderManifest,
        prevent_sync_pattern: &PyAny,
    ) -> PyResult<Self> {
        let base = remote.0.clone();

        let result = Self(parsec_client_types::LocalFolderManifest {
            base,
            need_sync: false,
            updated: remote.0.updated,
            children: remote.0.children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
        });

        result._filter_remote_entries(prevent_sync_pattern)
    }

    #[classmethod]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: FolderManifest,
        prevent_sync_pattern: &PyAny,
        local_manifest: &Self,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let result = Self::from_remote(_cls, remote, prevent_sync_pattern)?;
        result._restore_local_confinement_points(local_manifest, prevent_sync_pattern, timestamp)
    }

    fn to_remote(&self, author: DeviceID, timestamp: &PyAny) -> PyResult<FolderManifest> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        Ok(FolderManifest(self.0.to_remote(author.0, timestamp)))
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.parent))
    }

    #[getter]
    fn base(&self) -> PyResult<FolderManifest> {
        Ok(FolderManifest(self.0.base.clone()))
    }

    #[getter]
    fn need_sync(&self) -> PyResult<bool> {
        Ok(self.0.need_sync)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn children(&self) -> PyResult<HashMap<EntryName, EntryID>> {
        Ok(self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(name, id)| (EntryName(name), EntryID(id)))
            .collect())
    }

    #[getter]
    fn local_confinement_points(&self) -> PyResult<HashSet<EntryID>> {
        Ok(self
            .0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect())
    }

    #[getter]
    fn remote_confinement_points(&self) -> PyResult<HashSet<EntryID>> {
        Ok(self
            .0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalWorkspaceManifest(pub parsec_client_types::LocalWorkspaceManifest);

impl LocalWorkspaceManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.base == other.0.base
            && self.0.need_sync == other.0.need_sync
            && self.0.updated == other.0.updated
            && self.0.children == other.0.children
            && self.0.local_confinement_points == other.0.local_confinement_points
            && self.0.remote_confinement_points == other.0.remote_confinement_points
            && self.0.speculative == other.0.speculative
    }
}

#[pymethods]
impl LocalWorkspaceManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let base = kwargs_extract_required::<WorkspaceManifest>(args, "base")?;
        let need_sync = kwargs_extract_required(args, "need_sync")?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let children = kwargs_extract_required::<HashMap<EntryName, EntryID>>(args, "children")?
            .into_iter()
            .map(|(name, id)| (name.0, id.0))
            .collect();
        let local_confinement_points =
            kwargs_extract_required_frozenset::<EntryID>(args, "local_confinement_points")?
                .into_iter()
                .map(|id| id.0)
                .collect();
        let remote_confinement_points =
            kwargs_extract_required_frozenset::<EntryID>(args, "remote_confinement_points")?
                .into_iter()
                .map(|id| id.0)
                .collect();
        let speculative = kwargs_extract_required(args, "speculative")?;

        Ok(Self(parsec_client_types::LocalWorkspaceManifest {
            base: base.0,
            need_sync,
            updated,
            children,
            local_confinement_points,
            remote_confinement_points,
            speculative,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> PyResult<u32> {
        Ok(self.0.base.version)
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.id))
    }

    #[getter]
    fn created<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        rs_to_py_datetime(py, self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> PyResult<bool> {
        Ok(self.0.base.version == 0)
    }

    fn match_remote(&self, remote_manifest: &WorkspaceManifest) -> PyResult<bool> {
        Ok(self.0.match_remote(&remote_manifest.0))
    }

    /*
     * LocalFolderishManifestMixin
     */

    fn evolve_children_and_mark_updated(
        &self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let mut result = self.clone();
        let mut actually_updated = false;
        // Deal with removal first
        for (name, entry_id) in data.iter() {
            // Here `entry_id` can be either:
            // - a new entry id that might overwrite the previous one with the same name if it exists
            // - `None` which means the entry for the corresponding name should be removed
            if !result.0.children.contains_key(&name.0) {
                // Make sure we don't remove a name that does not exist
                assert!(entry_id.is_some());
                continue;
            }
            // Remove old entry
            if let Some(old_entry_id) = result.0.children.remove(&name.0) {
                if !result.0.local_confinement_points.remove(&old_entry_id) {
                    actually_updated = true;
                }
            }
        }
        // Make sure no entry_id is duplicated
        assert_eq!(
            HashSet::<_, RandomState>::from_iter(
                data.values().filter_map(|v| v.as_ref().map(|v| v.0))
            )
            .intersection(&HashSet::from_iter(result.0.children.values().copied()))
            .count(),
            0
        );
        // Deal with additions second

        for (name, entry_id) in data.iter() {
            if let Some(entry_id) = entry_id {
                // Add new entry
                result.0.children.insert(name.0.clone(), entry_id.0);
                if pattern_match(prevent_sync_pattern, name.0.as_ref())? {
                    result.0.local_confinement_points.insert(entry_id.0);
                } else {
                    actually_updated = true;
                }
            }
        }

        if !actually_updated {
            return Ok(result);
        }
        result.evolve_and_mark_updated(timestamp, None)
    }

    fn _filter_local_confinement_points(&self) -> PyResult<Self> {
        Ok(Self(self.0._filter_local_confinement_points()))
    }

    fn _restore_local_confinement_points(
        &self,
        other: &Self,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        // Using self.remote_confinement_points is useful to restore entries that were present locally
        // before applying a new filter that filtered those entries from the remote manifest
        if other.0.local_confinement_points.is_empty()
            && self.0.remote_confinement_points.is_empty()
        {
            return Ok(self.clone());
        }
        // Create a set for fast lookup in order to make sure no entry gets duplicated.
        // This might happen when a synchronized entry is renamed to a confined name locally.
        let self_entry_ids = HashSet::<_, RandomState>::from_iter(self.0.children.values());
        let previously_local_confinement_points = other
            .0
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if !self_entry_ids.contains(entry_id)
                    && (other.0.local_confinement_points.contains(entry_id)
                        || self.0.remote_confinement_points.contains(entry_id))
                {
                    Some((EntryName(name.clone()), Some(EntryID(*entry_id))))
                } else {
                    None
                }
            })
            .collect();

        self.evolve_children_and_mark_updated(
            previously_local_confinement_points,
            prevent_sync_pattern,
            timestamp,
        )
    }

    fn _filter_remote_entries(&self, prevent_sync_pattern: &PyAny) -> PyResult<Self> {
        let mut result = self.clone();

        result.0.remote_confinement_points = self
            .0
            .children
            .iter()
            .filter_map(|(name, entry_id)| {
                if let Ok(true) = pattern_match(prevent_sync_pattern, name.as_ref()) {
                    Some(*entry_id)
                } else {
                    None
                }
            })
            .collect();

        if result.0.remote_confinement_points.is_empty() {
            return Ok(self.clone());
        }

        result
            .0
            .children
            .retain(|_, entry_id| !result.0.remote_confinement_points.contains(entry_id));

        Ok(result)
    }

    fn _restore_remote_confinement_points(&self) -> PyResult<Self> {
        Ok(Self(self.0._restore_remote_confinement_points()))
    }

    fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &PyAny,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let mut result = self.clone();
        // Filter local confinement points
        result = result._filter_local_confinement_points()?;
        // Restore remote confinement points
        result = result._restore_remote_confinement_points()?;
        // Filter remote confinement_points
        result = result._filter_remote_entries(prevent_sync_pattern)?;
        // Restore local confinement points
        result._restore_local_confinement_points(self, prevent_sync_pattern, timestamp)
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match parsec_client_types::LocalWorkspaceManifest::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut r = self.0.clone();

        if let Some(args) = py_kwargs {
            if let Some(v) = kwargs_extract_optional::<WorkspaceManifest>(args, "base")? {
                r.base = v.0;
            }
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                r.need_sync = v;
            }
            if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
                r.updated = v;
            }
            if let Some(v) =
                kwargs_extract_optional::<HashMap<EntryName, EntryID>>(args, "children")?
            {
                r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
            }
            if let Some(v) =
                kwargs_extract_optional::<HashSet<EntryID>>(args, "local_confinement_points")?
            {
                r.local_confinement_points = v.into_iter().map(|id| id.0).collect();
            }
            if let Some(v) =
                kwargs_extract_optional::<HashSet<EntryID>>(args, "remote_confinement_points")?
            {
                r.remote_confinement_points = v.into_iter().map(|id| id.0).collect();
            }
            if let Some(v) = kwargs_extract_optional(args, "speculative")? {
                r.speculative = v;
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[classmethod]
    #[args(id = "None", speculative = "false")]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        timestamp: &PyAny,
        id: Option<EntryID>,
        speculative: bool,
    ) -> PyResult<Self> {
        let timestamp = py_to_rs_datetime(timestamp)?;

        Ok(Self(parsec_client_types::LocalWorkspaceManifest {
            base: parsec_api_types::WorkspaceManifest {
                author: author.0,
                timestamp,
                id: id.map(|id| id.0).unwrap_or_default(),
                version: 0,
                created: timestamp,
                updated: timestamp,
                children: HashMap::new(),
            },
            need_sync: true,
            updated: timestamp,
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative,
        }))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(&self, timestamp: &PyAny, data: Option<&PyDict>) -> PyResult<Self> {
        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = py_to_rs_datetime(timestamp)?;
        if let Some(args) = data {
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                out.0.need_sync = v;
            }
        }

        Ok(out)
    }

    #[classmethod]
    fn from_remote(
        _cls: &PyType,
        remote: WorkspaceManifest,
        prevent_sync_pattern: &PyAny,
    ) -> PyResult<Self> {
        let base = remote.0.clone();

        let result = Self(parsec_client_types::LocalWorkspaceManifest {
            base,
            need_sync: false,
            updated: remote.0.updated,
            children: remote.0.children,
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            speculative: false,
        });

        result._filter_remote_entries(prevent_sync_pattern)
    }

    #[classmethod]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: WorkspaceManifest,
        prevent_sync_pattern: &PyAny,
        local_manifest: Self,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        let result = Self::from_remote(_cls, remote, prevent_sync_pattern)?;
        result._restore_local_confinement_points(&local_manifest, prevent_sync_pattern, timestamp)
    }

    fn to_remote(&self, author: DeviceID, timestamp: &PyAny) -> PyResult<WorkspaceManifest> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        Ok(WorkspaceManifest(self.0.to_remote(author.0, timestamp)))
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        let mut children = self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        children.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

        let children = children
            .into_iter()
            .map(EntryName)
            .collect::<Vec<_>>()
            .into_py(py);

        Ok([
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "folder".to_object(py)),
            ("children", children.to_object(py)),
        ]
        .into_py_dict(py))
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let id = EntryID(self.0.base.id).into_py(py);
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;
        let children = self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(k, v)| (EntryName(k), EntryID(v)))
            .collect::<HashMap<_, _>>()
            .into_py(py);
        let local_confinement_points = self
            .0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect::<Vec<_>>()
            .into_py(py);
        let remote_confinement_points = self
            .0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect::<Vec<_>>()
            .into_py(py);
        Ok([
            ("id", id.to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("speculative", self.0.speculative.to_object(py)),
            ("children", children.to_object(py)),
            (
                "local_confinement_points",
                local_confinement_points.to_object(py),
            ),
            (
                "remote_confinement_points",
                remote_confinement_points.to_object(py),
            ),
        ]
        .into_py_dict(py))
    }

    #[getter]
    fn base(&self) -> PyResult<WorkspaceManifest> {
        Ok(WorkspaceManifest(self.0.base.clone()))
    }

    #[getter]
    fn need_sync(&self) -> PyResult<bool> {
        Ok(self.0.need_sync)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn children(&self) -> PyResult<HashMap<EntryName, EntryID>> {
        Ok(self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(name, id)| (EntryName(name), EntryID(id)))
            .collect())
    }

    #[getter]
    fn local_confinement_points(&self) -> PyResult<HashSet<EntryID>> {
        Ok(self
            .0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect())
    }

    #[getter]
    fn remote_confinement_points(&self) -> PyResult<HashSet<EntryID>> {
        Ok(self
            .0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect())
    }

    #[getter]
    fn speculative(&self) -> PyResult<bool> {
        Ok(self.0.speculative)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalUserManifest(pub parsec_client_types::LocalUserManifest);

impl LocalUserManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.base == other.0.base
            && self.0.need_sync == other.0.need_sync
            && self.0.updated == other.0.updated
            && self.0.last_processed_message == other.0.last_processed_message
            && self.0.workspaces == other.0.workspaces
            && self.0.speculative == other.0.speculative
    }
}

#[pymethods]
impl LocalUserManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let base = kwargs_extract_required::<UserManifest>(args, "base")?;
        let need_sync = kwargs_extract_required(args, "need_sync")?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let last_processed_message = kwargs_extract_required(args, "last_processed_message")?;
        let workspaces = kwargs_extract_required::<Vec<WorkspaceEntry>>(args, "workspaces")?
            .into_iter()
            .map(|w| w.0)
            .collect();
        let speculative = kwargs_extract_required(args, "speculative")?;

        Ok(Self(parsec_client_types::LocalUserManifest {
            base: base.0,
            need_sync,
            updated,
            last_processed_message,
            workspaces,
            speculative,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> PyResult<u32> {
        Ok(self.0.base.version)
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.base.id))
    }

    #[getter]
    fn created<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        rs_to_py_datetime(py, self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> PyResult<bool> {
        Ok(self.0.base.version == 0)
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(&self, timestamp: &PyAny, data: Option<&PyDict>) -> PyResult<Self> {
        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = py_to_rs_datetime(timestamp)?;
        if let Some(args) = data {
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                out.0.need_sync = v;
            }
        }

        Ok(out)
    }

    fn evolve_workspaces_and_mark_updated(
        &self,
        timestamp: &PyAny,
        workspace: WorkspaceEntry,
    ) -> PyResult<Self> {
        let out = self.evolve_and_mark_updated(timestamp, None)?;
        Ok(Self(out.0.evolve_workspaces(workspace.0)))
    }

    fn evolve_workspaces(&self, workspace: WorkspaceEntry) -> PyResult<Self> {
        Ok(Self(self.0.evolve_workspaces(workspace.0)))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_encrypt(&key.0)))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> PyResult<Self> {
        match parsec_client_types::LocalUserManifest::decrypt_and_load(encrypted, &key.0) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut r = self.0.clone();

        if let Some(args) = py_kwargs {
            if let Some(v) = kwargs_extract_optional::<UserManifest>(args, "base")? {
                r.base = v.0;
            }
            if let Some(v) = kwargs_extract_optional(args, "need_sync")? {
                r.need_sync = v;
            }
            if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
                r.updated = v;
            }
            if let Some(v) = kwargs_extract_optional(args, "last_processed_message")? {
                r.last_processed_message = v;
            }
            if let Some(v) = kwargs_extract_optional::<Vec<WorkspaceEntry>>(args, "workspaces")? {
                r.workspaces = v.into_iter().map(|we| we.0).collect();
            }
            if let Some(v) = kwargs_extract_optional(args, "speculative")? {
                r.speculative = v;
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[classmethod]
    #[args(id = "None", speculative = "false")]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        timestamp: &PyAny,
        id: Option<EntryID>,
        speculative: bool,
    ) -> PyResult<Self> {
        let timestamp = py_to_rs_datetime(timestamp)?;

        Ok(Self(parsec_client_types::LocalUserManifest {
            base: parsec_api_types::UserManifest {
                author: author.0,
                timestamp,
                id: id.map(|id| id.0).unwrap_or_default(),
                version: 0,
                created: timestamp,
                updated: timestamp,
                last_processed_message: 0,
                workspaces: vec![],
            },
            need_sync: true,
            updated: timestamp,
            last_processed_message: 0,
            workspaces: vec![],
            speculative,
        }))
    }

    fn get_workspace_entry(&self, workspace_id: EntryID) -> PyResult<Option<WorkspaceEntry>> {
        Ok(self
            .0
            .get_workspace_entry(workspace_id.0)
            .cloned()
            .map(WorkspaceEntry))
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: UserManifest) -> PyResult<Self> {
        Ok(Self(parsec_client_types::LocalUserManifest::from_remote(
            remote.0,
        )))
    }

    fn to_remote(&self, author: DeviceID, timestamp: &PyAny) -> PyResult<UserManifest> {
        let timestamp = py_to_rs_datetime(timestamp)?;
        Ok(UserManifest(self.0.to_remote(author.0, timestamp)))
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        let created = rs_to_py_datetime(py, self.0.base.created)?;
        let updated = rs_to_py_datetime(py, self.0.updated)?;

        Ok([
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
        ]
        .into_py_dict(py))
    }

    #[getter]
    fn base(&self) -> PyResult<UserManifest> {
        Ok(UserManifest(self.0.base.clone()))
    }

    #[getter]
    fn need_sync(&self) -> PyResult<bool> {
        Ok(self.0.need_sync)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn last_processed_message(&self) -> PyResult<u32> {
        Ok(self.0.last_processed_message)
    }

    #[getter]
    fn workspaces<'py>(&self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(
            py,
            self.0
                .workspaces
                .clone()
                .into_iter()
                .map(|w| WorkspaceEntry(w).into_py(py)),
        ))
    }

    #[getter]
    fn speculative(&self) -> PyResult<bool> {
        Ok(self.0.speculative)
    }
}
