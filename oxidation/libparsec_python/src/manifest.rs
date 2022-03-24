// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyTuple, PyType};

use crate::binding_utils::{
    hash_generic, kwargs_extract_optional, kwargs_extract_optional_custom, kwargs_extract_required,
    kwargs_extract_required_custom, py_to_rs_datetime, py_to_rs_realm_role, rs_to_py_datetime,
    rs_to_py_realm_role,
};
use crate::crypto::{HashDigest, SecretKey, SigningKey, VerifyKey};
use crate::ids::{BlockID, DeviceID, EntryID};

import_exception!(parsec.api.data, EntryNameTooLongError);
import_exception!(parsec.api.data, DataError);
import_exception!(parsec.api.data, DataValidationError);

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryName(pub parsec_api_types::EntryName);

#[pymethods]
impl EntryName {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        match name.parse::<parsec_api_types::EntryName>() {
            Ok(en) => Ok(Self(en)),
            Err(err) => match err {
                parsec_api_types::EntryNameError::NameTooLong => {
                    Err(EntryNameTooLongError::new_err("Invalid data"))
                }
                _ => Err(PyValueError::new_err("Invalid data")),
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryName {}>", self.0))
    }

    fn __richcmp__(&self, py: Python, other: &EntryName, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self.0.as_ref() == other.0.as_ref()).into_py(py),
            CompareOp::Ne => (self.0.as_ref() != other.0.as_ref()).into_py(py),
            CompareOp::Lt => (self.0.as_ref() < other.0.as_ref()).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct WorkspaceEntry(pub parsec_api_types::WorkspaceEntry);

impl WorkspaceEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
            && self.0.name == other.0.name
            && self.0.encryption_revision == other.0.encryption_revision
            && self.0.role == other.0.role
    }
}

#[pymethods]
impl WorkspaceEntry {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let id = kwargs_extract_required::<EntryID>(args, "id")?;
        let name = kwargs_extract_required::<EntryName>(args, "name")?;
        let key = kwargs_extract_required::<SecretKey>(args, "key")?;
        let encryption_revision = kwargs_extract_required::<u32>(args, "encryption_revision")?;
        let encrypted_on =
            kwargs_extract_required_custom(args, "encrypted_on", &py_to_rs_datetime)?;
        let role_cached_on =
            kwargs_extract_required_custom(args, "role_cached_on", &py_to_rs_datetime)?;
        let realm_role = kwargs_extract_required_custom(args, "role", &|item| {
            if item.is_none() {
                Ok(None)
            } else {
                py_to_rs_realm_role(item).map(Some)
            }
        })?;

        Ok(Self(parsec_api_types::WorkspaceEntry {
            id: id.0,
            name: name.0,
            key: key.0,
            encryption_revision,
            encrypted_on,
            role_cached_on,
            role: realm_role,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<EntryName>(args, "name")? {
            r.name = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<SecretKey>(args, "key")? {
            r.key = v.0;
        }
        if let Some(v) = kwargs_extract_optional(args, "encryption_revision")? {
            r.encryption_revision = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "encrypted_on", &py_to_rs_datetime)? {
            r.encrypted_on = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "role_cached_on", &py_to_rs_datetime)?
        {
            r.role_cached_on = v;
        }
        let maybe_role = kwargs_extract_optional_custom(args, "role", &|item| {
            if item.is_none() {
                Ok(None)
            } else {
                py_to_rs_realm_role(item).map(Some)
            }
        })?;
        if let Some(v) = maybe_role {
            r.role = v;
        }

        Ok(Self(r))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType, name: &EntryName, timestamp: &PyAny) -> PyResult<Self> {
        let dt = py_to_rs_datetime(timestamp)?;
        Ok(Self(parsec_api_types::WorkspaceEntry::generate(
            name.0.to_owned(),
            dt,
        )))
    }

    fn is_revoked(&self) -> bool {
        self.0.is_revoked()
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id))
    }

    #[getter]
    fn name(&self) -> PyResult<EntryName> {
        Ok(EntryName(self.0.name.clone()))
    }

    #[getter]
    fn key(&self) -> PyResult<SecretKey> {
        Ok(SecretKey(self.0.key.clone()))
    }

    #[getter]
    fn encryption_revision(&self) -> PyResult<u32> {
        Ok(self.0.encryption_revision)
    }

    #[getter]
    fn encrypted_on<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.encrypted_on)
    }

    #[getter]
    fn role_cached_on<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.role_cached_on)
    }

    #[getter]
    fn role(&self) -> PyResult<Option<PyObject>> {
        match self.0.role {
            Some(role) => rs_to_py_realm_role(&role).map(Some),
            None => Ok(None),
        }
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockAccess(pub parsec_api_types::BlockAccess);

impl BlockAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.offset == other.0.offset && self.0.size == other.0.size
    }
}

#[pymethods]
impl BlockAccess {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let id = kwargs_extract_required::<BlockID>(args, "id")?;
        let key = kwargs_extract_required::<SecretKey>(args, "key")?;
        let offset = kwargs_extract_required::<u64>(args, "offset")?;
        let size = kwargs_extract_required::<u64>(args, "size")?;
        let digest = kwargs_extract_required::<HashDigest>(args, "digest")?;

        Ok(Self(parsec_api_types::BlockAccess {
            id: id.0,
            key: key.0,
            offset,
            size,
            digest: digest.0,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<BlockID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<SecretKey>(args, "key")? {
            r.key = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<u64>(args, "offset")? {
            r.offset = v;
        }
        if let Some(v) = kwargs_extract_optional::<u64>(args, "size")? {
            r.size = v;
        }
        if let Some(v) = kwargs_extract_optional::<HashDigest>(args, "digest")? {
            r.digest = v.0;
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

    #[getter]
    fn id(&self) -> PyResult<BlockID> {
        Ok(BlockID(self.0.id))
    }

    #[getter]
    fn key(&self) -> PyResult<SecretKey> {
        Ok(SecretKey(self.0.key.clone()))
    }

    #[getter]
    fn offset(&self) -> PyResult<u64> {
        Ok(self.0.offset)
    }

    #[getter]
    fn size(&self) -> PyResult<u64> {
        Ok(self.0.size)
    }

    #[getter]
    fn digest(&self) -> PyResult<HashDigest> {
        Ok(HashDigest(self.0.digest.clone()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct FileManifest(pub parsec_api_types::FileManifest);

impl FileManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.author == other.0.author
            && self.0.id == other.0.id
            && self.0.version == other.0.version
            && self.0.parent == other.0.parent
            && self.0.timestamp == other.0.timestamp
            && self.0.created == other.0.created
            && self.0.updated == other.0.updated
            && self.0.blocks == other.0.blocks
    }
}

#[pymethods]
impl FileManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = kwargs_extract_required::<DeviceID>(args, "author")?;
        let timestamp = kwargs_extract_required_custom(args, "timestamp", &py_to_rs_datetime)?;
        let id = kwargs_extract_required::<EntryID>(args, "id")?;
        let parent = kwargs_extract_required::<EntryID>(args, "parent")?;
        let version = kwargs_extract_required::<u32>(args, "version")?;
        let created = kwargs_extract_required_custom(args, "created", &py_to_rs_datetime)?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let size = kwargs_extract_required::<u64>(args, "size")?;
        let blocksize = kwargs_extract_required_custom(args, "blocksize", &|item| {
            let raw = item.extract::<u64>()?;
            parsec_api_types::Blocksize::try_from(raw)
                .map_err(|_| PyValueError::new_err("Invalid blocksize"))
        })?;
        let blocks = kwargs_extract_required_custom(args, "blocks", &|item| {
            let pyblocks = item.downcast::<PyTuple>()?;
            let mut rsblocks = Vec::with_capacity(pyblocks.len());
            for pyblock in pyblocks {
                let rsblock = pyblock.extract::<BlockAccess>()?;
                rsblocks.push(rsblock.0);
            }
            Ok(rsblocks)
        })?;

        Ok(Self(parsec_api_types::FileManifest {
            author: author.0,
            timestamp,
            id: id.0,
            parent: parent.0,
            version,
            created,
            updated,
            size,
            blocksize,
            blocks,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_sign_and_encrypt<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
        key: &SecretKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump_sign_and_encrypt(&author_signkey.0, &key.0),
        ))
    }

    #[classmethod]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: &PyAny,
    ) -> PyResult<Self> {
        let expected_timestamp = py_to_rs_datetime(expected_timestamp)?;
        match parsec_api_types::FileManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<DeviceID>(args, "author")? {
            r.author = v.0;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "timestamp", &py_to_rs_datetime)? {
            r.timestamp = v;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "parent")? {
            r.parent = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<u32>(args, "version")? {
            r.version = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "created", &py_to_rs_datetime)? {
            r.created = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
            r.updated = v;
        }
        if let Some(v) = kwargs_extract_optional::<u64>(args, "size")? {
            r.size = v;
        }
        let maybe_blocksize = kwargs_extract_optional_custom(args, "blocksize", &|item| {
            let raw = item.extract::<u64>()?;
            parsec_api_types::Blocksize::try_from(raw)
                .map_err(|_| PyValueError::new_err("Invalid blocksize"))
        })?;
        if let Some(v) = maybe_blocksize {
            r.blocksize = v;
        }
        let maybe_blocks = kwargs_extract_optional_custom(args, "blocks", &|item| {
            let pyblocks = item.downcast::<PyTuple>()?;
            let mut rsblocks = Vec::with_capacity(pyblocks.len());
            for pyblock in pyblocks {
                let rsblock = pyblock.extract::<BlockAccess>()?;
                rsblocks.push(rsblock.0);
            }
            Ok(rsblocks)
        })?;
        if let Some(v) = maybe_blocks {
            r.blocks = v;
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

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id))
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.parent))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
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
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.timestamp)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.created)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn blocks<'p>(&self, py: Python<'p>) -> PyResult<&'p PyTuple> {
        let elems: Vec<PyObject> = self
            .0
            .blocks
            .iter()
            .map(|x| BlockAccess(x.clone()).into_py(py))
            .collect();
        Ok(PyTuple::new(py, elems))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct FolderManifest(pub parsec_api_types::FolderManifest);

impl FolderManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.author == other.0.author
            && self.0.id == other.0.id
            && self.0.version == other.0.version
            && self.0.parent == other.0.parent
            && self.0.timestamp == other.0.timestamp
            && self.0.created == other.0.created
            && self.0.updated == other.0.updated
            && self.0.children == other.0.children
    }
}

#[pymethods]
impl FolderManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = kwargs_extract_required::<DeviceID>(args, "author")?;
        let timestamp = kwargs_extract_required_custom(args, "timestamp", &py_to_rs_datetime)?;
        let id = kwargs_extract_required::<EntryID>(args, "id")?;
        let parent = kwargs_extract_required::<EntryID>(args, "parent")?;
        let version = kwargs_extract_required::<u32>(args, "version")?;
        let created = kwargs_extract_required_custom(args, "created", &py_to_rs_datetime)?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let children = kwargs_extract_required_custom(args, "children", &|item| {
            let pychildren = item.downcast::<PyDict>()?;
            let mut rschildren = std::collections::HashMap::with_capacity(pychildren.len());
            for (pyk, pyv) in pychildren.iter() {
                let rsk = pyk.extract::<EntryName>()?;
                let rsv = pyv.extract::<EntryID>()?;
                rschildren.insert(rsk.0, rsv.0);
            }
            Ok(rschildren)
        })?;

        Ok(Self(parsec_api_types::FolderManifest {
            author: author.0,
            timestamp,
            version,
            id: id.0,
            parent: parent.0,
            created,
            updated,
            children,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_sign_and_encrypt<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
        key: &SecretKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump_sign_and_encrypt(&author_signkey.0, &key.0),
        ))
    }

    #[classmethod]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: &PyAny,
    ) -> PyResult<Self> {
        let expected_timestamp = py_to_rs_datetime(expected_timestamp)?;
        match parsec_api_types::FolderManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<DeviceID>(args, "author")? {
            r.author = v.0;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "timestamp", &py_to_rs_datetime)? {
            r.timestamp = v;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "parent")? {
            r.parent = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<u32>(args, "version")? {
            r.version = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "created", &py_to_rs_datetime)? {
            r.created = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
            r.updated = v;
        }
        let maybe_children = kwargs_extract_optional_custom(args, "children", &|item| {
            let pychildren = item.downcast::<PyDict>()?;
            let mut rschildren = std::collections::HashMap::with_capacity(pychildren.len());
            for (pyk, pyv) in pychildren.iter() {
                let rsk = pyk.extract::<EntryName>()?;
                let rsv = pyv.extract::<EntryID>()?;
                rschildren.insert(rsk.0, rsv.0);
            }
            Ok(rschildren)
        })?;
        if let Some(v) = maybe_children {
            r.children = v;
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

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id))
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.parent))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.timestamp)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.created)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn children<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let d = PyDict::new(py);

        for (k, v) in &self.0.children {
            let en = EntryName(k.clone()).into_py(py);
            let me = EntryID(*v).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct WorkspaceManifest(pub parsec_api_types::WorkspaceManifest);

impl WorkspaceManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.author == other.0.author
            && self.0.id == other.0.id
            && self.0.version == other.0.version
            && self.0.timestamp == other.0.timestamp
            && self.0.created == other.0.created
            && self.0.updated == other.0.updated
            && self.0.children == other.0.children
    }
}

#[pymethods]
impl WorkspaceManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = kwargs_extract_required::<DeviceID>(args, "author")?;
        let timestamp = kwargs_extract_required_custom(args, "timestamp", &py_to_rs_datetime)?;
        let id = kwargs_extract_required::<EntryID>(args, "id")?;
        let version = kwargs_extract_required::<u32>(args, "version")?;
        let created = kwargs_extract_required_custom(args, "created", &py_to_rs_datetime)?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let children = kwargs_extract_required_custom(args, "children", &|item| {
            let pychildren = item.downcast::<PyDict>()?;
            let mut rschildren = std::collections::HashMap::with_capacity(pychildren.len());
            for (pyk, pyv) in pychildren.iter() {
                let rsk = pyk.extract::<EntryName>()?;
                let rsv = pyv.extract::<EntryID>()?;
                rschildren.insert(rsk.0, rsv.0);
            }
            Ok(rschildren)
        })?;

        Ok(Self(parsec_api_types::WorkspaceManifest {
            author: author.0,
            timestamp,
            id: id.0,
            version,
            created,
            updated,
            children,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_sign_and_encrypt<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
        key: &SecretKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump_sign_and_encrypt(&author_signkey.0, &key.0),
        ))
    }

    #[classmethod]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: &PyAny,
    ) -> PyResult<Self> {
        let expected_timestamp = py_to_rs_datetime(expected_timestamp)?;
        match parsec_api_types::WorkspaceManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<DeviceID>(args, "author")? {
            r.author = v.0;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "timestamp", &py_to_rs_datetime)? {
            r.timestamp = v;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional::<u32>(args, "version")? {
            r.version = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "created", &py_to_rs_datetime)? {
            r.created = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
            r.updated = v;
        }
        let maybe_children = kwargs_extract_optional_custom(args, "children", &|item| {
            let pychildren = item.downcast::<PyDict>()?;
            let mut rschildren = std::collections::HashMap::with_capacity(pychildren.len());
            for (pyk, pyv) in pychildren.iter() {
                let rsk = pyk.extract::<EntryName>()?;
                let rsv = pyv.extract::<EntryID>()?;
                rschildren.insert(rsk.0, rsv.0);
            }
            Ok(rschildren)
        })?;
        if let Some(v) = maybe_children {
            r.children = v;
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &WorkspaceManifest, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.timestamp)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.created)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.updated)
    }

    #[getter]
    fn children<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let d = PyDict::new(py);

        for (k, v) in &self.0.children {
            let en = EntryName(k.clone()).into_py(py);
            let me = EntryID(*v).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct UserManifest(pub parsec_api_types::UserManifest);

impl UserManifest {
    fn eq(&self, other: &Self) -> bool {
        self.0.author == other.0.author
            && self.0.id == other.0.id
            && self.0.version == other.0.version
            && self.0.timestamp == other.0.timestamp
            && self.0.created == other.0.created
            && self.0.updated == other.0.updated
            && self.0.last_processed_message == other.0.last_processed_message
            && self.0.workspaces == other.0.workspaces
    }
}

#[pymethods]
impl UserManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = kwargs_extract_required::<DeviceID>(args, "author")?;
        let timestamp = kwargs_extract_required_custom(args, "timestamp", &py_to_rs_datetime)?;
        let id = kwargs_extract_required::<EntryID>(args, "id")?;
        let version = kwargs_extract_required(args, "version")?;
        let created = kwargs_extract_required_custom(args, "created", &py_to_rs_datetime)?;
        let updated = kwargs_extract_required_custom(args, "updated", &py_to_rs_datetime)?;
        let last_processed_message = kwargs_extract_required(args, "last_processed_message")?;
        let workspaces = kwargs_extract_required::<Vec<WorkspaceEntry>>(args, "workspaces")?
            .into_iter()
            .map(|w| w.0)
            .collect();

        Ok(Self(parsec_api_types::UserManifest {
            author: author.0,
            timestamp,
            id: id.0,
            version,
            created,
            updated,
            last_processed_message,
            workspaces,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_sign_and_encrypt<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
        key: &SecretKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump_sign_and_encrypt(&author_signkey.0, &key.0),
        ))
    }

    #[classmethod]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: &PyAny,
        expected_id: Option<EntryID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        let expected_timestamp = py_to_rs_datetime(expected_timestamp)?;
        let data = parsec_api_types::UserManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp,
        )
        .map_err(|e| DataError::new_err(e.to_string()))?;
        if let Some(expected_id) = expected_id {
            if data.id != expected_id.0 {
                return Err(DataValidationError::new_err(format!(
                    "Invalid entry ID: expected `{}`, got `{}`",
                    expected_id.0, data.id
                )));
            }
        }
        if let Some(expected_version) = expected_version {
            if data.version != expected_version {
                return Err(DataValidationError::new_err(format!(
                    "Invalid version: expected `{}`, got `{}`",
                    expected_version, data.version
                )));
            }
        }
        Ok(Self(data))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if let Some(v) = kwargs_extract_optional::<DeviceID>(args, "author")? {
            r.author = v.0;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "timestamp", &py_to_rs_datetime)? {
            r.timestamp = v;
        }
        if let Some(v) = kwargs_extract_optional::<EntryID>(args, "id")? {
            r.id = v.0;
        }
        if let Some(v) = kwargs_extract_optional(args, "version")? {
            r.version = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "created", &py_to_rs_datetime)? {
            r.created = v;
        }
        if let Some(v) = kwargs_extract_optional_custom(args, "updated", &py_to_rs_datetime)? {
            r.updated = v;
        }
        if let Some(v) = kwargs_extract_optional(args, "last_processed_message")? {
            r.last_processed_message = v;
        }
        if let Some(v) = kwargs_extract_optional::<Vec<WorkspaceEntry>>(args, "workspaces")? {
            r.workspaces = v.into_iter().map(|w| w.0).collect();
        }

        Ok(Self(r))
    }

    fn get_workspace_entry(&self, workspace_id: EntryID) -> PyResult<Option<WorkspaceEntry>> {
        Ok(self
            .0
            .get_workspace_entry(workspace_id.0)
            .cloned()
            .map(WorkspaceEntry))
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(py, self.eq(other)).into_py(py),
            CompareOp::Ne => PyBool::new(py, !self.eq(other)).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.timestamp)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        rs_to_py_datetime(py, self.0.created)
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
    fn workspaces<'p>(&self, py: Python<'p>) -> PyResult<&'p PyTuple> {
        let elems: Vec<PyObject> = self
            .0
            .workspaces
            .clone()
            .into_iter()
            .map(|x| WorkspaceEntry(x).into_py(py))
            .collect();
        Ok(PyTuple::new(py, elems))
    }
}
