// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::str::FromStr;

use chrono::prelude::{DateTime, Utc};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::import_exception;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyTuple, PyType};

use crate::crypto::{HashDigest, SecretKey, SigningKey, VerifyKey};
use crate::ids::{BlockID, DeviceID, EntryID};

use crate::binding_utils::hash_generic;

import_exception!(parsec.api.data, EntryNameTooLongError);

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct EntryName(pub parsec_api_types::EntryName);

#[pymethods]
impl EntryName {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        match name.parse::<parsec_api_types::EntryName>() {
            Ok(en) => Ok(Self(en)),
            Err(err) => match err {
                "Name too long" => Err(EntryNameTooLongError::new_err("Invalid data")),
                _ => Err(PyValueError::new_err("Invalid data")),
            },
        }
    }

    fn __richcmp__(&self, py: Python, other: &EntryName, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self.0.to_string() == other.0.to_string()).into_py(py),
            CompareOp::Ne => (self.0.to_string() != other.0.to_string()).into_py(py),
            CompareOp::Lt => (self.0.to_string() < other.0.to_string()).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryName {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

fn extract_generic_arg<'a, T: FromPyObject<'a>>(args: &'a PyDict, name: &str) -> Option<T> {
    match args.get_item(name) {
        Some(x) => {
            if x.is_none() {
                None
            } else {
                match x.extract::<T>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
        }
        None => None,
    }
}

fn extract_time_arg(args: &PyDict, name: &str) -> Option<DateTime<Utc>> {
    match args.get_item(name) {
        Some(x) => {
            if x.is_none() {
                None
            } else {
                match Python::with_gil(|_py| -> PyResult<&PyAny> {
                    x.getattr("to_rfc3339_string")?.call0()
                }) {
                    Ok(ts) => match ts.extract::<String>() {
                        Ok(s) => match DateTime::parse_from_rfc3339(&s) {
                            Ok(dt) => Some(DateTime::<Utc>::from(dt)),
                            Err(_) => None,
                        },
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            }
        }
        None => None,
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub(crate) struct WorkspaceEntry(pub parsec_api_types::WorkspaceEntry);

#[pymethods]
impl WorkspaceEntry {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let realm_role = match args.get_item("role") {
            Some(v) => {
                if v.is_none() {
                    None
                } else {
                    match v.getattr("name") {
                        Ok(v) => {
                            match parsec_api_types::RealmRole::from_str(v.extract::<&str>()?) {
                                Ok(role) => Some(role),
                                Err(_) => {
                                    return Err(PyValueError::new_err("Invalid `role` argument"))
                                }
                            }
                        }
                        Err(_) => return Err(PyValueError::new_err("Invalid `role` argument")),
                    }
                }
            }
            None => None,
        };

        let id = extract_generic_arg::<EntryID>(args, "id")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `id` argument"))?;
        let name = extract_generic_arg::<EntryName>(args, "name")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `name` argument"))?;
        let key = extract_generic_arg::<SecretKey>(args, "key")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `key` argument"))?;
        let encryption_revision = extract_generic_arg::<u32>(args, "encryption_revision")
            .ok_or_else(|| {
                PyValueError::new_err("Missing or invalid `encryption_revision argument`")
            })?;
        let encrypted_on = extract_time_arg(args, "encrypted_on")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `encrypted_on` argument"))?;
        let role_cached_on = extract_time_arg(args, "role_cached_on")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `role_cached_on` argument"))?;

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

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if args.contains("id").unwrap_or(false) {
            r.id = extract_generic_arg::<EntryID>(args, "id")
                .ok_or_else(|| PyValueError::new_err("Invalid `id` argument"))?
                .0;
        }
        if args.contains("name").unwrap_or(false) {
            r.name = extract_generic_arg::<EntryName>(args, "name")
                .ok_or_else(|| PyValueError::new_err("Invalid `name` argument"))?
                .0;
        }
        if args.contains("key").unwrap_or(false) {
            r.key = extract_generic_arg::<SecretKey>(args, "key")
                .ok_or_else(|| PyValueError::new_err("Invalid `key` argument"))?
                .0;
        }
        if args.contains("encryption_revision").unwrap_or(false) {
            r.encryption_revision = extract_generic_arg::<u32>(args, "encryption_revision")
                .ok_or_else(|| PyValueError::new_err("Invalid `encryption_revision` argument"))?;
        }
        if args.contains("encrypted_on").unwrap_or(false) {
            r.encrypted_on = extract_time_arg(args, "encrypted_on")
                .ok_or_else(|| PyValueError::new_err("Invalid `encrypted_on` argument"))?;
        }
        if args.contains("role_cached_on").unwrap_or(false) {
            r.role_cached_on = extract_time_arg(args, "role_cached_on")
                .ok_or_else(|| PyValueError::new_err("Invalid `role_cached_on` argument"))?;
        }
        if args.contains("role").unwrap_or(false) {
            r.role = match args.get_item("role") {
                Some(v) => {
                    if v.is_none() {
                        None
                    } else {
                        match v.getattr("name") {
                            Ok(v) => {
                                match parsec_api_types::RealmRole::from_str(v.extract::<&str>()?) {
                                    Ok(role) => Some(role),
                                    Err(_) => {
                                        return Err(PyValueError::new_err(
                                            "Invalid `role` argument",
                                        ))
                                    }
                                }
                            }
                            Err(_) => return Err(PyValueError::new_err("Invalid `role` argument")),
                        }
                    }
                }
                None => None,
            };
        }

        Ok(Self(r))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType, name: &EntryName, timestamp: &PyAny) -> PyResult<Self> {
        let ts = match Python::with_gil(|_py| -> PyResult<&PyAny> {
            timestamp.getattr("to_rfc3339_string")?.call0()
        }) {
            Ok(ts) => match ts.extract::<String>() {
                Ok(s) => match DateTime::parse_from_rfc3339(&s) {
                    Ok(dt) => Ok(DateTime::<Utc>::from(dt)),
                    Err(_) => Err(PyValueError::new_err("Invalid `timestamp` argument")),
                },
                Err(_) => Err(PyValueError::new_err("Invalid `timestamp` argument")),
            },
            Err(_) => Err(PyValueError::new_err("Invalid `timestamp` argument")),
        };

        let ts = match ts {
            Ok(ts) => ts,
            Err(err) => return Err(err),
        };

        Ok(Self(parsec_api_types::WorkspaceEntry::generate(
            name.0.clone(),
            ts,
        )))
    }

    fn is_revoked(&self) -> bool {
        self.0.is_revoked()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "<WorkspaceEntry name={} id={}>",
            self.0.name, self.0.id
        ))
    }

    fn __richcmp__(&self, py: Python, other: &WorkspaceEntry, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(
                py,
                self.0.id == other.0.id
                    && self.0.name == other.0.name
                    && self.0.encryption_revision == other.0.encryption_revision
                    && self.0.role == other.0.role,
            )
            .into_py(py),
            CompareOp::Ne => PyBool::new(
                py,
                self.0.id != other.0.id
                    || self.0.name != other.0.name
                    || self.0.encryption_revision != other.0.encryption_revision
                    || self.0.role != other.0.role,
            )
            .into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id.clone()))
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
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.encrypted_on.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn role_cached_on<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.role_cached_on.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn role(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| -> PyResult<Option<PyObject>> {
            match self.0.role {
                Some(r) => {
                    let cls = py.import("parsec.api.protocol")?.getattr("RealmRole")?;
                    let name = r.to_string();
                    let obj = cls.getattr(name)?;
                    Ok(Some(obj.into_py(py)))
                }
                None => Ok(None),
            }
        })
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockAccess(pub parsec_api_types::BlockAccess);

#[pymethods]
impl BlockAccess {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let id = extract_generic_arg::<BlockID>(args, "id")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `id` argument"))?;
        let key = extract_generic_arg::<SecretKey>(args, "key")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `key` argument"))?;
        let offset = extract_generic_arg::<u32>(args, "offset")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `offset` argument"))?;
        let size = extract_generic_arg::<u32>(args, "size")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `size` argument"))?;
        let digest = extract_generic_arg::<HashDigest>(args, "digest")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `digest` argument"))?;

        Ok(Self(parsec_api_types::BlockAccess {
            id: id.0,
            key: key.0,
            offset,
            size,
            digest: digest.0,
        }))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if args.contains("id").unwrap_or(false) {
            r.id = extract_generic_arg::<BlockID>(args, "id")
                .ok_or_else(|| PyValueError::new_err("Invalid `id` argument"))?
                .0;
        }
        if args.contains("key").unwrap_or(false) {
            r.key = extract_generic_arg::<SecretKey>(args, "key")
                .ok_or_else(|| PyValueError::new_err("Invalid `key` argument"))?
                .0;
        }
        if args.contains("offset").unwrap_or(false) {
            r.offset = extract_generic_arg::<u32>(args, "offset")
                .ok_or_else(|| PyValueError::new_err("Invalid `offset` argument"))?;
        }
        if args.contains("size").unwrap_or(false) {
            r.size = extract_generic_arg::<u32>(args, "size")
                .ok_or_else(|| PyValueError::new_err("Invalid `size` argument"))?;
        }
        if args.contains("digest").unwrap_or(false) {
            r.digest = extract_generic_arg::<HashDigest>(args, "digest")
                .ok_or_else(|| PyValueError::new_err("Invalid `digest` argument"))?
                .0;
        }

        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &BlockAccess, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(
                py,
                self.0.id == other.0.id
                    && self.0.offset == other.0.offset
                    && self.0.size == other.0.size,
            )
            .into_py(py),
            CompareOp::Ne => PyBool::new(
                py,
                self.0.id != other.0.id
                    || self.0.offset != other.0.offset
                    || self.0.size != other.0.size,
            )
            .into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn id(&self) -> PyResult<BlockID> {
        Ok(BlockID(self.0.id.clone()))
    }

    #[getter]
    fn key(&self) -> PyResult<SecretKey> {
        Ok(SecretKey(self.0.key.clone()))
    }

    #[getter]
    fn offset(&self) -> PyResult<u32> {
        Ok(self.0.offset)
    }

    #[getter]
    fn size(&self) -> PyResult<u32> {
        Ok(self.0.size)
    }

    #[getter]
    fn digest(&self) -> PyResult<HashDigest> {
        Ok(HashDigest(self.0.digest.clone()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub(crate) struct FileManifest(pub parsec_api_types::FileManifest);

#[pymethods]
impl FileManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = extract_generic_arg::<DeviceID>(args, "author")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `author` argument"))?;
        let id = extract_generic_arg::<EntryID>(args, "id")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `id` argument"))?;
        let parent = extract_generic_arg::<EntryID>(args, "parent")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `parent` argument"))?;
        let version = extract_generic_arg::<u32>(args, "version")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `version` argument"))?;
        let size = extract_generic_arg::<u32>(args, "size")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `size` argument"))?;
        let blocksize = extract_generic_arg::<u32>(args, "blocksize")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `blocksize` argument"))?;
        let timestamp = extract_time_arg(args, "timestamp")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `timestamp` argument"))?;
        let created = extract_time_arg(args, "created")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `created` argument"))?;
        let updated = extract_time_arg(args, "updated")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `updated` argument"))?;

        let blocks = args.get_item("blocks").unwrap();
        let blocks = blocks.downcast::<PyTuple>().unwrap();
        let blocks = blocks
            .iter()
            .map(|x| x.extract::<BlockAccess>().unwrap().0)
            .collect();

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
        let ts = match Python::with_gil(|_py| -> PyResult<&PyAny> {
            expected_timestamp.getattr("to_rfc3339_string")?.call0()
        }) {
            Ok(ts) => match ts.extract::<String>() {
                Ok(s) => match DateTime::parse_from_rfc3339(&s) {
                    Ok(dt) => DateTime::<Utc>::from(dt),
                    Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
                },
                Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
            },
            Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
        };
        match parsec_api_types::FileManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            &ts,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if args.contains("author").unwrap_or(false) {
            r.author = extract_generic_arg::<DeviceID>(args, "author")
                .ok_or_else(|| PyValueError::new_err("Invalid `author` argument"))?
                .0;
        }
        if args.contains("id").unwrap_or(false) {
            r.id = extract_generic_arg::<EntryID>(args, "id")
                .ok_or_else(|| PyValueError::new_err("Invalid `id` argument"))?
                .0;
        }
        if args.contains("parent").unwrap_or(false) {
            r.parent = extract_generic_arg::<EntryID>(args, "parent")
                .ok_or_else(|| PyValueError::new_err("Invalid `parent` argument"))?
                .0;
        }
        if args.contains("version").unwrap_or(false) {
            r.version = extract_generic_arg::<u32>(args, "version")
                .ok_or_else(|| PyValueError::new_err("Invalid `version` argument"))?;
        }
        if args.contains("timestamp").unwrap_or(false) {
            r.timestamp = extract_time_arg(args, "timestamp")
                .ok_or_else(|| PyValueError::new_err("Invalid `timestamp` argument"))?;
        }
        if args.contains("created").unwrap_or(false) {
            r.created = extract_time_arg(args, "created")
                .ok_or_else(|| PyValueError::new_err("Invalid `created` argument"))?;
        }
        if args.contains("updated").unwrap_or(false) {
            r.updated = extract_time_arg(args, "updated")
                .ok_or_else(|| PyValueError::new_err("Invalid `updated` argument"))?;
        }
        if args.contains("blocks").unwrap_or(false) {
            let blocks = args.get_item("blocks").unwrap();
            let blocks = blocks.downcast::<PyTuple>().unwrap();
            r.blocks = blocks
                .iter()
                .map(|x| x.extract::<BlockAccess>().unwrap().0)
                .collect();
        }
        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &FileManifest, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(
                py,
                self.0.author == other.0.author
                    && self.0.id == other.0.id
                    && self.0.version == other.0.version
                    && self.0.parent == other.0.parent
                    && self.0.timestamp == other.0.timestamp
                    && self.0.created == other.0.created
                    && self.0.updated == other.0.updated
                    && self.0.blocks == other.0.blocks,
            )
            .into_py(py),
            CompareOp::Ne => PyBool::new(
                py,
                self.0.author != other.0.author
                    || self.0.id != other.0.id
                    || self.0.version != other.0.version
                    || self.0.parent == other.0.parent
                    || self.0.timestamp != other.0.timestamp
                    || self.0.created != other.0.created
                    || self.0.updated != other.0.updated
                    || self.0.blocks != other.0.blocks,
            )
            .into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id.clone()))
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.parent.clone()))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn size(&self) -> PyResult<u32> {
        Ok(self.0.size)
    }

    #[getter]
    fn blocksize(&self) -> PyResult<u32> {
        Ok(self.0.blocksize)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.timestamp.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.created.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.updated.to_rfc3339()]);
        fun.call1(args)
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
#[derive(PartialEq, Eq)]
pub(crate) struct FolderManifest(pub parsec_api_types::FolderManifest);

#[pymethods]
impl FolderManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = extract_generic_arg::<DeviceID>(args, "author")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `author` argument"))?;
        let id = extract_generic_arg::<EntryID>(args, "id")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `id` argument"))?;
        let parent = extract_generic_arg::<EntryID>(args, "parent")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `parent` argument"))?;
        let version = extract_generic_arg::<u32>(args, "version")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `version` argument"))?;
        let timestamp = extract_time_arg(args, "timestamp")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `timestamp` argument"))?;
        let created = extract_time_arg(args, "created")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `created` argument"))?;
        let updated = extract_time_arg(args, "updated")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `updated` argument"))?;
        let children = args.get_item("children").unwrap();
        let children = children.downcast::<PyDict>().unwrap();
        let children = children
            .iter()
            .map(|(k, v)| {
                (
                    k.extract::<EntryName>().unwrap().0,
                    v.extract::<EntryID>().unwrap().0,
                )
            })
            .collect();

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
        let ts = match Python::with_gil(|_py| -> PyResult<&PyAny> {
            expected_timestamp.getattr("to_rfc3339_string")?.call0()
        }) {
            Ok(ts) => match ts.extract::<String>() {
                Ok(s) => match DateTime::parse_from_rfc3339(&s) {
                    Ok(dt) => DateTime::<Utc>::from(dt),
                    Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
                },
                Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
            },
            Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
        };
        match parsec_api_types::FolderManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            &ts,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if args.contains("author").unwrap_or(false) {
            r.author = extract_generic_arg::<DeviceID>(args, "author")
                .ok_or_else(|| PyValueError::new_err("Invalid `author` argument"))?
                .0;
        }
        if args.contains("id").unwrap_or(false) {
            r.id = extract_generic_arg::<EntryID>(args, "id")
                .ok_or_else(|| PyValueError::new_err("Invalid `id` argument"))?
                .0;
        }
        if args.contains("parent").unwrap_or(false) {
            r.parent = extract_generic_arg::<EntryID>(args, "parent")
                .ok_or_else(|| PyValueError::new_err("Invalid `parent` argument"))?
                .0;
        }
        if args.contains("version").unwrap_or(false) {
            r.version = extract_generic_arg::<u32>(args, "version")
                .ok_or_else(|| PyValueError::new_err("Invalid `version` argument"))?;
        }
        if args.contains("timestamp").unwrap_or(false) {
            r.timestamp = extract_time_arg(args, "timestamp")
                .ok_or_else(|| PyValueError::new_err("Invalid `timestamp` argument"))?;
        }
        if args.contains("created").unwrap_or(false) {
            r.created = extract_time_arg(args, "created")
                .ok_or_else(|| PyValueError::new_err("Invalid `created` argument"))?;
        }
        if args.contains("updated").unwrap_or(false) {
            r.updated = extract_time_arg(args, "updated")
                .ok_or_else(|| PyValueError::new_err("Invalid `updated` argument"))?;
        }
        if args.contains("children").unwrap_or(false) {
            let children = args.get_item("children").unwrap();
            let children = children.downcast::<PyDict>().unwrap();
            let children = children
                .iter()
                .map(|(k, v)| {
                    (
                        k.extract::<EntryName>().unwrap().0,
                        v.extract::<EntryID>().unwrap().0,
                    )
                })
                .collect();
            r.children = children;
        }
        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &FolderManifest, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(
                py,
                self.0.author == other.0.author
                    && self.0.id == other.0.id
                    && self.0.version == other.0.version
                    && self.0.parent == other.0.parent
                    && self.0.timestamp == other.0.timestamp
                    && self.0.created == other.0.created
                    && self.0.updated == other.0.updated
                    && self.0.children == other.0.children,
            )
            .into_py(py),
            CompareOp::Ne => PyBool::new(
                py,
                self.0.author != other.0.author
                    || self.0.id != other.0.id
                    || self.0.version != other.0.version
                    || self.0.parent == other.0.parent
                    || self.0.timestamp != other.0.timestamp
                    || self.0.created != other.0.created
                    || self.0.updated != other.0.updated
                    || self.0.children != other.0.children,
            )
            .into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id.clone()))
    }

    #[getter]
    fn parent(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.parent.clone()))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.timestamp.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.created.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.updated.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn children<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let d = PyDict::new(py);

        for (k, v) in &self.0.children {
            let en = EntryName(k.clone()).into_py(py);
            let me = EntryID(v.clone()).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}

#[pyclass]
#[derive(PartialEq, Eq)]
pub(crate) struct WorkspaceManifest(pub parsec_api_types::WorkspaceManifest);

#[pymethods]
impl WorkspaceManifest {
    #[new]
    #[args(py_kwargs = "**")]
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let author = extract_generic_arg::<DeviceID>(args, "author")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `author` argument"))?;
        let id = extract_generic_arg::<EntryID>(args, "id")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `id` argument"))?;
        let version = extract_generic_arg::<u32>(args, "version")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `version` argument"))?;
        let timestamp = extract_time_arg(args, "timestamp")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `timestamp` argument"))?;
        let created = extract_time_arg(args, "created")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `created` argument"))?;
        let updated = extract_time_arg(args, "updated")
            .ok_or_else(|| PyValueError::new_err("Missing or invalid `updated` argument"))?;
        let children = args.get_item("children").unwrap();
        let children = children.downcast::<PyDict>().unwrap();
        let children = children
            .iter()
            .map(|(k, v)| {
                (
                    k.extract::<EntryName>().unwrap().0,
                    v.extract::<EntryID>().unwrap().0,
                )
            })
            .collect();

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
        let ts = match Python::with_gil(|_py| -> PyResult<&PyAny> {
            expected_timestamp.getattr("to_rfc3339_string")?.call0()
        }) {
            Ok(ts) => match ts.extract::<String>() {
                Ok(s) => match DateTime::parse_from_rfc3339(&s) {
                    Ok(dt) => DateTime::<Utc>::from(dt),
                    Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
                },
                Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
            },
            Err(_) => return Err(PyValueError::new_err("Invalid timestamp")),
        };
        match parsec_api_types::WorkspaceManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            &ts,
        ) {
            Ok(x) => Ok(Self(x)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        let args = py_kwargs.unwrap();

        let mut r = self.0.clone();

        if args.contains("author").unwrap_or(false) {
            r.author = extract_generic_arg::<DeviceID>(args, "author")
                .ok_or_else(|| PyValueError::new_err("Invalid `author` argument"))?
                .0;
        }
        if args.contains("id").unwrap_or(false) {
            r.id = extract_generic_arg::<EntryID>(args, "id")
                .ok_or_else(|| PyValueError::new_err("Invalid `id` argument"))?
                .0;
        }
        if args.contains("version").unwrap_or(false) {
            r.version = extract_generic_arg::<u32>(args, "version")
                .ok_or_else(|| PyValueError::new_err("Invalid `version` argument"))?;
        }
        if args.contains("timestamp").unwrap_or(false) {
            r.timestamp = extract_time_arg(args, "timestamp")
                .ok_or_else(|| PyValueError::new_err("Invalid `timestamp` argument"))?;
        }
        if args.contains("created").unwrap_or(false) {
            r.created = extract_time_arg(args, "created")
                .ok_or_else(|| PyValueError::new_err("Invalid `created` argument"))?;
        }
        if args.contains("updated").unwrap_or(false) {
            r.updated = extract_time_arg(args, "updated")
                .ok_or_else(|| PyValueError::new_err("Invalid `updated` argument"))?;
        }
        if args.contains("children").unwrap_or(false) {
            let children = args.get_item("children").unwrap();
            let children = children.downcast::<PyDict>().unwrap();
            let children = children
                .iter()
                .map(|(k, v)| {
                    (
                        k.extract::<EntryName>().unwrap().0,
                        v.extract::<EntryID>().unwrap().0,
                    )
                })
                .collect();
            r.children = children;
        }
        Ok(Self(r))
    }

    fn __richcmp__(&self, py: Python, other: &WorkspaceManifest, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => PyBool::new(
                py,
                self.0.author == other.0.author
                    && self.0.id == other.0.id
                    && self.0.version == other.0.version
                    && self.0.timestamp == other.0.timestamp
                    && self.0.created == other.0.created
                    && self.0.updated == other.0.updated
                    && self.0.children == other.0.children,
            )
            .into_py(py),
            CompareOp::Ne => PyBool::new(
                py,
                self.0.author != other.0.author
                    || self.0.id != other.0.id
                    || self.0.version != other.0.version
                    || self.0.timestamp != other.0.timestamp
                    || self.0.created != other.0.created
                    || self.0.updated != other.0.updated
                    || self.0.children != other.0.children,
            )
            .into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<EntryID> {
        Ok(EntryID(self.0.id.clone()))
    }

    #[getter]
    fn version(&self) -> PyResult<u32> {
        Ok(self.0.version)
    }

    #[getter]
    fn timestamp<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.timestamp.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn created<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.created.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn updated<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let pendulum = PyModule::import(py, "pendulum")?;
        let fun = pendulum.getattr("parse")?;
        let args = PyTuple::new(py, vec![self.0.updated.to_rfc3339()]);
        fun.call1(args)
    }

    #[getter]
    fn children<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDict> {
        let d = PyDict::new(py);

        for (k, v) in &self.0.children {
            let en = EntryName(k.clone()).into_py(py);
            let me = EntryID(v.clone()).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}
