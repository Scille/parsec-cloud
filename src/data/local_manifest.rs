// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec::{
    client_types::LocalManifest,
    types::{Blocksize, DEFAULT_BLOCK_SIZE},
};
use pyo3::{
    basic::CompareOp,
    exceptions::{
        PyAssertionError, PyIndexError, PyNotImplementedError, PyTypeError, PyValueError,
    },
    prelude::*,
    types::{IntoPyDict, PyByteArray, PyBytes, PyDict, PyTuple, PyType},
};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
    panic,
};

use crate::{
    api_crypto::SecretKey,
    binding_utils::py_to_rs_set,
    data::{
        BlockAccess, DataResult, EntryName, FileManifest, FolderManifest, UserManifest,
        WorkspaceEntry, WorkspaceManifest,
    },
    ids::{ChunkID, DeviceID, EntryID},
    regex::Regex,
    time::DateTime,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct Chunk(pub libparsec::client_types::Chunk);

crate::binding_utils::gen_proto!(Chunk, __repr__);

#[pymethods]
impl Chunk {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [id: ChunkID, "id"],
            [start: u64, "start"],
            [stop: u64, "stop"],
            [raw_offset: u64, "raw_offset"],
            [raw_size: u64, "raw_size"],
            [access: Option<BlockAccess>, "access"],
        );

        Ok(Self(libparsec::client_types::Chunk {
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

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [id: ChunkID, "id"],
            [start: u64, "start"],
            [stop: u64, "stop"],
            [raw_offset: u64, "raw_offset"],
            [raw_size: u64, "raw_size"],
            [access: Option<BlockAccess>, "access"],
        );

        let mut r = self.0.clone();

        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = start {
            r.start = v;
        }
        if let Some(v) = stop {
            r.stop =
                NonZeroU64::try_from(v).map_err(|_| PyValueError::new_err("Invalid stop field"))?;
        }
        if let Some(v) = raw_offset {
            r.raw_offset = v;
        }
        if let Some(v) = raw_size {
            r.raw_size = NonZeroU64::try_from(v)
                .map_err(|_| PyValueError::new_err("Invalid raw_size field"))?;
        }
        if let Some(v) = access {
            r.access = match v {
                Some(v) => Some(v.0),
                None => None,
            }
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        Ok(match (other.extract::<u64>(), op) {
            (Ok(other), CompareOp::Eq) => self.0 == other,
            (Ok(other), CompareOp::Ne) => self.0 != other,
            (Ok(other), CompareOp::Lt) => self.0 < other,
            (Ok(other), CompareOp::Gt) => self.0 > other,
            (Ok(other), CompareOp::Le) => self.0 <= other,
            (Ok(other), CompareOp::Ge) => self.0 >= other,
            _ => match (other.extract::<Chunk>(), op) {
                (Ok(other), CompareOp::Eq) => self.0 == other.0,
                (Ok(other), CompareOp::Ne) => self.0 != other.0,
                _ => return Err(PyNotImplementedError::new_err("")),
            },
        })
    }

    #[classmethod]
    fn from_block_access(_cls: &PyType, block_access: BlockAccess) -> PyResult<Self> {
        libparsec::client_types::Chunk::from_block_access(block_access.0)
            .map(Self)
            .map_err(PyValueError::new_err)
    }

    fn evolve_as_block(&self, py: Python, data: PyObject) -> PyResult<Self> {
        let data = if let Ok(data) = data.extract::<&PyByteArray>(py) {
            // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
            // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
            // python thread modifying the bytearray behind our back.
            unsafe { data.as_bytes() }
        } else if let Ok(data) = data.extract::<&[u8]>(py) {
            data
        } else {
            return Err(PyValueError::new_err(
                "evolve_as_block: invalid input for data",
            ));
        };
        Ok(Self(
            self.0
                .clone()
                .evolve_as_block(data)
                .map_err(PyValueError::new_err)?,
        ))
    }

    fn is_block(&self) -> bool {
        self.0.is_block()
    }

    fn is_pseudo_block(&self) -> bool {
        self.0.is_pseudo_block()
    }

    fn get_block_access(&self) -> PyResult<BlockAccess> {
        Ok(BlockAccess(
            self.0
                .get_block_access()
                .map_err(PyValueError::new_err)?
                .clone(),
        ))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType, start: u64, stop: u64) -> PyResult<Self> {
        Ok(Self(libparsec::client_types::Chunk::new(
            start,
            NonZeroU64::try_from(stop).map_err(PyValueError::new_err)?,
        )))
    }

    #[getter]
    fn id(&self) -> ChunkID {
        ChunkID(self.0.id)
    }

    #[getter]
    fn start(&self) -> u64 {
        self.0.start
    }

    #[getter]
    fn stop(&self) -> u64 {
        self.0.stop.into()
    }

    #[getter]
    fn raw_offset(&self) -> u64 {
        self.0.raw_offset
    }

    #[getter]
    fn raw_size(&self) -> u64 {
        self.0.raw_size.into()
    }

    #[getter]
    fn access(&self) -> Option<BlockAccess> {
        self.0.access.clone().map(BlockAccess)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalFileManifest(pub libparsec::client_types::LocalFileManifest);

crate::binding_utils::gen_proto!(LocalFileManifest, __repr__);
crate::binding_utils::gen_proto!(LocalFileManifest, __richcmp__, eq);

#[pymethods]
impl LocalFileManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [base: FileManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [size: u64, "size"],
            [blocksize: u64, "blocksize"],
            [blocks: Vec<Vec<Chunk>>, "blocks"],
        );

        Ok(Self(libparsec::client_types::LocalFileManifest {
            base: base.0,
            need_sync,
            updated: updated.0,
            size,
            blocksize: libparsec::types::Blocksize::try_from(blocksize)
                .map_err(|_| PyValueError::new_err("Invalid blocksize field"))?,
            blocks: blocks
                .into_iter()
                .map(|b| b.into_iter().map(|b| b.0).collect())
                .collect(),
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [base: FileManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [size: u64, "size"],
            [blocksize: u64, "blocksize"],
            [blocks: Vec<Vec<Chunk>>, "blocks"],
        );

        let mut r = self.0.clone();

        if let Some(v) = base {
            r.base = v.0;
        }
        if let Some(v) = need_sync {
            r.need_sync = v;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = size {
            r.size = v;
        }
        if let Some(v) = blocksize {
            r.blocksize = libparsec::types::Blocksize::try_from(v)
                .map_err(|_| PyValueError::new_err("Invalid blocksize field"))?;
        }
        if let Some(v) = blocks {
            r.blocks = v
                .into_iter()
                .map(|v| v.into_iter().map(|chunk| chunk.0).collect())
                .collect();
        }

        Ok(Self(r))
    }

    fn evolve_single_block(&self, block: u64, new_chunk: Chunk) -> PyResult<Self> {
        let mut new_manifest = self.0.clone();
        new_manifest
            .set_single_block(block, new_chunk.0)
            .map_err(PyIndexError::new_err)?;
        Ok(Self(new_manifest))
    }

    fn get_chunks<'py>(&self, py: Python<'py>, block: usize) -> &'py PyTuple {
        let elements = self
            .0
            .get_chunks(block)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|x| Chunk(x).into_py(py));
        PyTuple::new(py, elements)
    }

    fn is_reshaped(&self) -> bool {
        self.0.is_reshaped()
    }

    fn assert_integrity(&self) -> PyResult<()> {
        let result = panic::catch_unwind(|| self.0.assert_integrity());
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(PyAssertionError::new_err("assert")),
        }
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: FileManifest) -> PyResult<Self> {
        libparsec::client_types::LocalFileManifest::from_remote(remote.0)
            .map(Self)
            .map_err(PyValueError::new_err)
    }

    fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> PyResult<FileManifest> {
        self.0
            .to_remote(author.0, timestamp.0)
            .map(FileManifest)
            .map_err(PyValueError::new_err)
    }

    #[classmethod]
    // Python signature includes variables that are unused in LocalFileManifest
    #[allow(unused_variables)]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: FileManifest,
        prevent_sync_pattern: &PyAny,
        local_manifest: Self,
        timestamp: &PyAny,
    ) -> PyResult<Self> {
        Self::from_remote(_cls, remote)
    }

    fn match_remote(&self, remote_manifest: &FileManifest) -> bool {
        self.0.match_remote(&remote_manifest.0)
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> u32 {
        self.0.base.version
    }

    #[getter]
    fn id(&self) -> EntryID {
        EntryID(self.0.base.id)
    }

    #[getter]
    fn created(&self) -> DateTime {
        DateTime(self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> bool {
        self.0.base.version == 0
    }

    fn dump_and_encrypt<'py>(&self, py: Python<'py>, key: &SecretKey) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_encrypt(&key.0))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> DataResult<Self> {
        Ok(
            libparsec::client_types::LocalFileManifest::decrypt_and_load(encrypted, &key.0)
                .map(Self)?,
        )
    }

    #[classmethod]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        parent: EntryID,
        timestamp: DateTime,
        blocksize: Option<u64>,
    ) -> PyResult<Self> {
        let blocksize = blocksize
            .map(|blocksize| Blocksize::try_from(blocksize).map_err(PyValueError::new_err))
            .unwrap_or(Ok(DEFAULT_BLOCK_SIZE))?;

        Ok(Self(libparsec::client_types::LocalFileManifest::new(
            author.0,
            parent.0,
            timestamp.0,
            blocksize,
        )))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(
        &self,
        timestamp: DateTime,
        data: Option<&PyDict>,
    ) -> PyResult<Self> {
        if let Some(args) = data {
            if args.get_item("need_sync").is_some() {
                return Err(PyTypeError::new_err(
                    "Unexpected keyword argument `need_sync`",
                ));
            }
        }

        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = timestamp.0;

        Ok(out)
    }

    #[getter]
    fn parent(&self) -> EntryID {
        EntryID(self.0.base.parent)
    }

    #[getter]
    fn base(&self) -> FileManifest {
        FileManifest(self.0.base.clone())
    }

    #[getter]
    fn need_sync(&self) -> bool {
        self.0.need_sync
    }

    #[getter]
    fn updated(&self) -> DateTime {
        DateTime(self.0.updated)
    }

    #[getter]
    fn size(&self) -> u64 {
        self.0.size
    }

    #[getter]
    fn blocksize(&self) -> u64 {
        self.0.blocksize.into()
    }

    #[getter]
    fn blocks<'py>(&self, py: Python<'py>) -> &'py PyTuple {
        PyTuple::new(
            py,
            self.0
                .blocks
                .iter()
                .cloned()
                .map(|chunks| PyTuple::new(py, chunks.into_iter().map(|c| Chunk(c).into_py(py)))),
        )
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> &'py PyDict {
        [
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            (
                "created",
                DateTime(self.0.base.created).into_py(py).to_object(py),
            ),
            (
                "updated",
                DateTime(self.0.updated).into_py(py).to_object(py),
            ),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "file".to_object(py)),
            ("size", self.0.size.to_object(py)),
        ]
        .into_py_dict(py)
    }

    fn asdict<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let parent = EntryID(self.0.base.parent).into_py(py);
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
        let blocks = self
            .0
            .blocks
            .iter()
            .map(|b| b.iter().cloned().map(|c| Chunk(c).into_py(py)).collect())
            .collect::<Vec<Vec<_>>>()
            .into_py(py);
        [
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
        .into_py_dict(py)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalFolderManifest(pub libparsec::client_types::LocalFolderManifest);

crate::binding_utils::gen_proto!(LocalFolderManifest, __repr__);
crate::binding_utils::gen_proto!(LocalFolderManifest, __richcmp__, eq);

#[pymethods]
impl LocalFolderManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [base: FolderManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
            [local, "local_confinement_points", py_to_rs_set],
            [remote, "remote_confinement_points", py_to_rs_set],
        );

        Ok(Self(libparsec::client_types::LocalFolderManifest {
            base: base.0,
            need_sync,
            updated: updated.0,
            children: children
                .into_iter()
                .map(|(name, id)| (name.0, id.0))
                .collect(),
            local_confinement_points: local.into_iter().map(|id: EntryID| id.0).collect(),
            remote_confinement_points: remote.into_iter().map(|id: EntryID| id.0).collect(),
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [base: FolderManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
            [
                local_confinement_points,
                "local_confinement_points",
                py_to_rs_set
            ],
            [
                remote_confinement_points,
                "remote_confinement_points",
                py_to_rs_set
            ],
        );

        let mut r = self.0.clone();

        if let Some(v) = base {
            r.base = v.0;
        }
        if let Some(v) = need_sync {
            r.need_sync = v;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = children {
            r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
        }
        if let Some(v) = local_confinement_points {
            r.local_confinement_points = v.into_iter().map(|id: EntryID| id.0).collect();
        }
        if let Some(v) = remote_confinement_points {
            r.remote_confinement_points = v.into_iter().map(|id: EntryID| id.0).collect();
        }

        Ok(Self(r))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> u32 {
        self.0.base.version
    }

    #[getter]
    fn id(&self) -> EntryID {
        EntryID(self.0.base.id)
    }

    #[getter]
    fn created(&self) -> DateTime {
        DateTime(self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> bool {
        self.0.base.version == 0
    }

    fn match_remote(&self, remote_manifest: FolderManifest) -> bool {
        self.0.match_remote(&remote_manifest.0)
    }

    /*
     * LocalFolderishManifestMixin
     */

    fn evolve_children_and_mark_updated(
        &self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let data = data
            .into_iter()
            .map(|(en, ei)| (en.0, ei.map(|ei| ei.0)))
            .collect();
        Self(self.0.clone().evolve_children_and_mark_updated(
            data,
            &prevent_sync_pattern.0,
            timestamp.0,
        ))
    }

    fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        Self(
            self.0
                .apply_prevent_sync_pattern(&prevent_sync_pattern.0, timestamp.0),
        )
    }

    fn dump_and_encrypt<'py>(&self, py: Python<'py>, key: &SecretKey) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_encrypt(&key.0))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> DataResult<Self> {
        Ok(
            libparsec::client_types::LocalFolderManifest::decrypt_and_load(encrypted, &key.0)
                .map(Self)?,
        )
    }

    #[classmethod]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        parent: EntryID,
        timestamp: DateTime,
    ) -> Self {
        Self(libparsec::client_types::LocalFolderManifest::new(
            author.0,
            parent.0,
            timestamp.0,
        ))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(
        &self,
        timestamp: DateTime,
        data: Option<&PyDict>,
    ) -> PyResult<Self> {
        if let Some(args) = data {
            if args.get_item("need_sync").is_some() {
                return Err(PyTypeError::new_err(
                    "Unexpected keyword argument `need_sync`",
                ));
            }
        }

        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = timestamp.0;

        Ok(out)
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: FolderManifest, prevent_sync_pattern: &Regex) -> Self {
        Self(libparsec::client_types::LocalFolderManifest::from_remote(
            remote.0,
            &prevent_sync_pattern.0,
        ))
    }

    #[classmethod]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: FolderManifest,
        prevent_sync_pattern: &Regex,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        Self(
            libparsec::client_types::LocalFolderManifest::from_remote_with_local_context(
                remote.0,
                &prevent_sync_pattern.0,
                &local_manifest.0,
                timestamp.0,
            ),
        )
    }

    fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> FolderManifest {
        FolderManifest(self.0.to_remote(author.0, timestamp.0))
    }

    #[getter]
    fn parent(&self) -> EntryID {
        EntryID(self.0.base.parent)
    }

    #[getter]
    fn base(&self) -> FolderManifest {
        FolderManifest(self.0.base.clone())
    }

    #[getter]
    fn need_sync(&self) -> bool {
        self.0.need_sync
    }

    #[getter]
    fn updated(&self) -> DateTime {
        DateTime(self.0.updated)
    }

    #[getter]
    fn children(&self) -> HashMap<EntryName, EntryID> {
        self.0
            .children
            .clone()
            .into_iter()
            .map(|(name, id)| (EntryName(name), EntryID(id)))
            .collect()
    }

    #[getter]
    fn local_confinement_points(&self) -> HashSet<EntryID> {
        self.0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    #[getter]
    fn remote_confinement_points(&self) -> HashSet<EntryID> {
        self.0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
        let mut children = self.0.children.clone().into_keys().collect::<Vec<_>>();

        children.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

        let children = children
            .into_iter()
            .map(EntryName)
            .collect::<Vec<_>>()
            .into_py(py);

        [
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "folder".to_object(py)),
            ("children", children.to_object(py)),
        ]
        .into_py_dict(py)
    }

    fn asdict<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let parent = EntryID(self.0.base.parent).into_py(py);
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
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
        [
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
        .into_py_dict(py)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalWorkspaceManifest(pub libparsec::client_types::LocalWorkspaceManifest);

crate::binding_utils::gen_proto!(LocalWorkspaceManifest, __repr__);
crate::binding_utils::gen_proto!(LocalWorkspaceManifest, __richcmp__, eq);

#[pymethods]
impl LocalWorkspaceManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [base: WorkspaceManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
            [local, "local_confinement_points", py_to_rs_set],
            [remote, "remote_confinement_points", py_to_rs_set],
            [speculative: bool, "speculative"],
        );

        Ok(Self(libparsec::client_types::LocalWorkspaceManifest {
            base: base.0,
            need_sync,
            updated: updated.0,
            children: children
                .into_iter()
                .map(|(name, id)| (name.0, id.0))
                .collect(),
            local_confinement_points: local.into_iter().map(|id: EntryID| id.0).collect(),
            remote_confinement_points: remote.into_iter().map(|id: EntryID| id.0).collect(),
            speculative,
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [base: WorkspaceManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
            [local, "local_confinement_points", py_to_rs_set],
            [remote, "remote_confinement_points", py_to_rs_set],
            [speculative: bool, "speculative"],
        );

        let mut r = self.0.clone();

        if let Some(v) = base {
            r.base = v.0;
        }
        if let Some(v) = need_sync {
            r.need_sync = v;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = children {
            r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
        }
        if let Some(v) = local {
            r.local_confinement_points = v.into_iter().map(|id: EntryID| id.0).collect();
        }
        if let Some(v) = remote {
            r.remote_confinement_points = v.into_iter().map(|id: EntryID| id.0).collect();
        }
        if let Some(v) = speculative {
            r.speculative = v;
        }

        Ok(Self(r))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> u32 {
        self.0.base.version
    }

    #[getter]
    fn id(&self) -> EntryID {
        EntryID(self.0.base.id)
    }

    #[getter]
    fn created(&self) -> DateTime {
        DateTime(self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> bool {
        self.0.base.version == 0
    }

    fn match_remote(&self, remote_manifest: &WorkspaceManifest) -> bool {
        self.0.match_remote(&remote_manifest.0)
    }

    /*
     * LocalFolderishManifestMixin
     */

    fn evolve_children_and_mark_updated(
        &self,
        data: HashMap<EntryName, Option<EntryID>>,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        let data = data
            .into_iter()
            .map(|(en, ei)| (en.0, ei.map(|ei| ei.0)))
            .collect();
        Self(self.0.clone().evolve_children_and_mark_updated(
            data,
            &prevent_sync_pattern.0,
            timestamp.0,
        ))
    }

    fn apply_prevent_sync_pattern(
        &self,
        prevent_sync_pattern: &Regex,
        timestamp: DateTime,
    ) -> Self {
        Self(
            self.0
                .apply_prevent_sync_pattern(&prevent_sync_pattern.0, timestamp.0),
        )
    }

    fn dump_and_encrypt<'py>(&self, py: Python<'py>, key: &SecretKey) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_encrypt(&key.0))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> DataResult<Self> {
        Ok(
            libparsec::client_types::LocalWorkspaceManifest::decrypt_and_load(encrypted, &key.0)
                .map(Self)?,
        )
    }

    #[classmethod]
    #[args(id = "None", speculative = "false")]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        timestamp: DateTime,
        id: Option<EntryID>,
        speculative: bool,
    ) -> Self {
        Self(libparsec::client_types::LocalWorkspaceManifest::new(
            author.0,
            timestamp.0,
            id.map(|id| id.0),
            speculative,
        ))
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(
        &self,
        timestamp: DateTime,
        data: Option<&PyDict>,
    ) -> PyResult<Self> {
        if let Some(args) = data {
            if args.get_item("need_sync").is_some() {
                return Err(PyTypeError::new_err(
                    "Unexpected keyword argument `need_sync`",
                ));
            }
        }

        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = timestamp.0;

        Ok(out)
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: WorkspaceManifest, prevent_sync_pattern: &Regex) -> Self {
        Self(
            libparsec::client_types::LocalWorkspaceManifest::from_remote(
                remote.0,
                &prevent_sync_pattern.0,
            ),
        )
    }

    #[classmethod]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: WorkspaceManifest,
        prevent_sync_pattern: &Regex,
        local_manifest: &Self,
        timestamp: DateTime,
    ) -> Self {
        Self(
            libparsec::client_types::LocalWorkspaceManifest::from_remote_with_local_context(
                remote.0,
                &prevent_sync_pattern.0,
                &local_manifest.0,
                timestamp.0,
            ),
        )
    }

    fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> WorkspaceManifest {
        WorkspaceManifest(self.0.to_remote(author.0, timestamp.0))
    }

    #[getter]
    fn base(&self) -> WorkspaceManifest {
        WorkspaceManifest(self.0.base.clone())
    }

    #[getter]
    fn need_sync(&self) -> bool {
        self.0.need_sync
    }

    #[getter]
    fn updated(&self) -> DateTime {
        DateTime(self.0.updated)
    }

    #[getter]
    fn children(&self, py: Python) -> PyResult<PyObject> {
        let types_mod = PyModule::import(py, "parsec.types")?;
        let frozen_dict = types_mod.getattr("FrozenDict")?;

        let children: HashMap<EntryName, EntryID> = self
            .0
            .children
            .clone()
            .into_iter()
            .map(|(name, id)| (EntryName(name), EntryID(id)))
            .collect();

        Ok(frozen_dict.call1((children,))?.into_py(py))
    }

    #[getter]
    fn local_confinement_points(&self) -> HashSet<EntryID> {
        self.0
            .local_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    #[getter]
    fn remote_confinement_points(&self) -> HashSet<EntryID> {
        self.0
            .remote_confinement_points
            .clone()
            .into_iter()
            .map(EntryID)
            .collect()
    }

    #[getter]
    fn speculative(&self) -> bool {
        self.0.speculative
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
        let mut children = self.0.children.clone().into_keys().collect::<Vec<_>>();

        children.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));

        let children = children
            .into_iter()
            .map(EntryName)
            .collect::<Vec<_>>()
            .into_py(py);

        [
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("type", "folder".to_object(py)),
            ("children", children.to_object(py)),
        ]
        .into_py_dict(py)
    }

    fn asdict<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
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
        [
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
        .into_py_dict(py)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct LocalUserManifest(pub libparsec::client_types::LocalUserManifest);

crate::binding_utils::gen_proto!(LocalUserManifest, __repr__);
crate::binding_utils::gen_proto!(LocalUserManifest, __richcmp__, eq);

#[pymethods]
impl LocalUserManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [base: UserManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [last_processed_message: u64, "last_processed_message"],
            [workspaces: Vec<WorkspaceEntry>, "workspaces"],
            [speculative: bool, "speculative"],
        );

        Ok(Self(libparsec::client_types::LocalUserManifest {
            base: base.0,
            need_sync,
            updated: updated.0,
            last_processed_message,
            workspaces: workspaces.into_iter().map(|w| w.0).collect(),
            speculative,
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [base: UserManifest, "base"],
            [need_sync: bool, "need_sync"],
            [updated: DateTime, "updated"],
            [last_processed_message: u64, "last_processed_message"],
            [workspaces: Vec<WorkspaceEntry>, "workspaces"],
            [speculative: bool, "speculative"],
        );

        let mut r = self.0.clone();

        if let Some(v) = base {
            r.base = v.0;
        }
        if let Some(v) = need_sync {
            r.need_sync = v;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = last_processed_message {
            r.last_processed_message = v;
        }
        if let Some(v) = workspaces {
            r.workspaces = v.into_iter().map(|we| we.0).collect();
        }
        if let Some(v) = speculative {
            r.speculative = v;
        }

        Ok(Self(r))
    }

    /*
     * BaseLocalManifest
     */

    #[getter]
    fn base_version(&self) -> u32 {
        self.0.base.version
    }

    #[getter]
    fn id(&self) -> EntryID {
        EntryID(self.0.base.id)
    }

    #[getter]
    fn created(&self) -> DateTime {
        DateTime(self.0.base.created)
    }

    #[getter]
    fn is_placeholder(&self) -> bool {
        self.0.base.version == 0
    }

    #[args(data = "**")]
    fn evolve_and_mark_updated(
        &self,
        timestamp: DateTime,
        data: Option<&PyDict>,
    ) -> PyResult<Self> {
        if let Some(args) = data {
            if args.get_item("need_sync").is_some() {
                return Err(PyTypeError::new_err(
                    "Unexpected keyword argument `need_sync`",
                ));
            }
        }

        let mut out = self.evolve(data)?;

        out.0.need_sync = true;
        out.0.updated = timestamp.0;

        Ok(out)
    }

    fn evolve_workspaces_and_mark_updated(
        &self,
        timestamp: DateTime,
        workspace: WorkspaceEntry,
    ) -> PyResult<Self> {
        let out = self.evolve_and_mark_updated(timestamp, None)?;
        Ok(Self(out.0.evolve_workspaces(workspace.0)))
    }

    fn evolve_workspaces(&self, workspace: WorkspaceEntry) -> Self {
        Self(self.0.clone().evolve_workspaces(workspace.0))
    }

    fn dump_and_encrypt<'p>(&self, py: Python<'p>, key: &SecretKey) -> &'p PyBytes {
        PyBytes::new(py, &self.0.dump_and_encrypt(&key.0))
    }

    #[classmethod]
    fn decrypt_and_load(_cls: &PyType, encrypted: &[u8], key: &SecretKey) -> DataResult<Self> {
        Ok(
            libparsec::client_types::LocalUserManifest::decrypt_and_load(encrypted, &key.0)
                .map(Self)?,
        )
    }

    #[classmethod]
    #[args(id = "None", speculative = "false")]
    fn new_placeholder(
        _cls: &PyType,
        author: DeviceID,
        timestamp: DateTime,
        id: Option<EntryID>,
        speculative: bool,
    ) -> Self {
        Self(libparsec::client_types::LocalUserManifest::new(
            author.0,
            timestamp.0,
            id.map(|id| id.0),
            speculative,
        ))
    }

    fn get_workspace_entry(&self, workspace_id: EntryID) -> Option<WorkspaceEntry> {
        self.0
            .get_workspace_entry(workspace_id.0)
            .cloned()
            .map(WorkspaceEntry)
    }

    #[classmethod]
    fn from_remote(_cls: &PyType, remote: UserManifest) -> Self {
        Self(libparsec::client_types::LocalUserManifest::from_remote(
            remote.0,
        ))
    }

    #[classmethod]
    // Python signature includes variables that are unused in LocalFileManifest
    #[allow(unused_variables)]
    fn from_remote_with_local_context(
        _cls: &PyType,
        remote: UserManifest,
        prevent_sync_pattern: &PyAny,
        local_manifest: Self,
        timestamp: &PyAny,
    ) -> Self {
        Self::from_remote(_cls, remote)
    }

    fn to_remote(&self, author: DeviceID, timestamp: DateTime) -> UserManifest {
        UserManifest(self.0.to_remote(author.0, timestamp.0))
    }

    #[getter]
    fn base(&self) -> UserManifest {
        UserManifest(self.0.base.clone())
    }

    #[getter]
    fn need_sync(&self) -> bool {
        self.0.need_sync
    }

    #[getter]
    fn updated(&self) -> DateTime {
        DateTime(self.0.updated)
    }

    #[getter]
    fn last_processed_message(&self) -> u64 {
        self.0.last_processed_message
    }

    #[getter]
    fn workspaces<'py>(&self, py: Python<'py>) -> &'py PyTuple {
        PyTuple::new(
            py,
            self.0
                .workspaces
                .clone()
                .into_iter()
                .map(|w| WorkspaceEntry(w).into_py(py)),
        )
    }

    #[getter]
    fn speculative(&self) -> bool {
        self.0.speculative
    }

    fn match_remote(&self, remote_manifest: &UserManifest) -> bool {
        self.0.match_remote(&remote_manifest.0)
    }

    fn to_stats<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);

        [
            ("id", EntryID(self.0.base.id).into_py(py).to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
        ]
        .into_py_dict(py)
    }

    fn asdict<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let created = DateTime(self.0.base.created).into_py(py);
        let updated = DateTime(self.0.updated).into_py(py);
        let workspaces: Vec<PyObject> = self
            .0
            .workspaces
            .iter()
            .map(|x| WorkspaceEntry(x.clone()).into_py(py))
            .collect();
        [
            ("base_version", self.0.base.version.to_object(py)),
            ("is_placeholder", (self.0.base.version == 0).to_object(py)),
            ("need_sync", self.0.need_sync.to_object(py)),
            ("created", created.to_object(py)),
            ("updated", updated.to_object(py)),
            (
                "last_processed_message",
                self.0.last_processed_message.to_object(py),
            ),
            ("speculative", self.0.speculative.to_object(py)),
            ("workspaces", workspaces.to_object(py)),
        ]
        .into_py_dict(py)
    }
}

#[pyfunction]
pub(crate) fn local_manifest_decrypt_and_load<'py>(
    py: Python<'py>,
    encrypted: &[u8],
    key: &SecretKey,
) -> DataResult<PyObject> {
    let decrypt_and_load = LocalManifest::decrypt_and_load(encrypted, &key.0)?;

    match decrypt_and_load {
        LocalManifest::File(file) => Ok(LocalFileManifest(file).into_py(py)),
        LocalManifest::Folder(folder) => Ok(LocalFolderManifest(folder).into_py(py)),
        LocalManifest::Workspace(workspace) => Ok(LocalWorkspaceManifest(workspace).into_py(py)),
        LocalManifest::User(user) => Ok(LocalUserManifest(user).into_py(py)),
    }
}
