use std::num::NonZeroU64;

use pyo3::{
    exceptions::PyValueError,
    import_exception, pyclass,
    pyclass::CompareOp,
    pymethods,
    types::{PyAny, PyBool, PyDict, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use crate::{
    api_crypto::{HashDigest, SecretKey},
    binding_utils::{
        hash_generic, py_to_rs_datetime, py_to_rs_realm_role, rs_to_py_datetime,
        rs_to_py_realm_role,
    },
    ids::{BlockID, EntryID},
};

import_exception!(parsec.api.data, EntryNameTooLongError);
import_exception!(parsec.api.data, DataError);
import_exception!(parsec.api.data, DataValidationError);

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryName(pub libparsec::types::EntryName);

#[pymethods]
impl EntryName {
    #[new]
    pub fn new(name: String) -> PyResult<Self> {
        match name.parse::<libparsec::types::EntryName>() {
            Ok(en) => Ok(Self(en)),
            Err(err) => match err {
                libparsec::types::EntryNameError::NameTooLong => {
                    Err(EntryNameTooLongError::new_err("Invalid data"))
                }
                _ => Err(PyValueError::new_err("Invalid data")),
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryName {}>", self.0))
    }

    fn __richcmp__(&self, other: &EntryName, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0.as_ref() == other.0.as_ref(),
            CompareOp::Ne => self.0.as_ref() != other.0.as_ref(),
            CompareOp::Lt => self.0.as_ref() < other.0.as_ref(),
            CompareOp::Gt => self.0.as_ref() > other.0.as_ref(),
            CompareOp::Le => self.0.as_ref() <= other.0.as_ref(),
            CompareOp::Ge => self.0.as_ref() >= other.0.as_ref(),
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
pub(crate) struct WorkspaceEntry(pub libparsec::types::WorkspaceEntry);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [id: EntryID, "id"],
            [name: EntryName, "name"],
            [key: SecretKey, "key"],
            [encryption_revision: u32, "encryption_revision"],
            [encrypted_on, "encrypted_on", py_to_rs_datetime],
            [role_cached_on, "role_cached_on", py_to_rs_datetime],
            [role, "role", py_to_rs_realm_role],
        );

        Ok(Self(libparsec::types::WorkspaceEntry {
            id: id.0,
            name: name.0,
            key: key.0,
            encryption_revision,
            encrypted_on,
            role_cached_on,
            role,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [id: EntryID, "id"],
            [name: EntryName, "name"],
            [key: SecretKey, "key"],
            [encryption_revision: u32, "encryption_revision"],
            [encrypted_on, "encrypted_on", py_to_rs_datetime],
            [role_cached_on, "role_cached_on", py_to_rs_datetime],
            [role, "role", py_to_rs_realm_role],
        );

        let mut r = self.0.clone();

        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = name {
            r.name = v.0;
        }
        if let Some(v) = key {
            r.key = v.0;
        }
        if let Some(v) = encryption_revision {
            r.encryption_revision = v;
        }
        if let Some(v) = encrypted_on {
            r.encrypted_on = v;
        }
        if let Some(v) = role_cached_on {
            r.role_cached_on = v;
        }
        if let Some(v) = role {
            r.role = v;
        }

        Ok(Self(r))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType, name: &EntryName, timestamp: &PyAny) -> PyResult<Self> {
        let dt = py_to_rs_datetime(timestamp)?;
        Ok(Self(libparsec::types::WorkspaceEntry::generate(
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
pub(crate) struct BlockAccess(pub libparsec::types::BlockAccess);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [id: BlockID, "id"],
            [key: SecretKey, "key"],
            [offset: u64, "offset"],
            [size: u64, "size"],
            [digest: HashDigest, "digest"],
        );

        Ok(Self(libparsec::types::BlockAccess {
            id: id.0,
            key: key.0,
            offset,
            size: NonZeroU64::try_from(size)
                .map_err(|_| PyValueError::new_err("Invalid `size` field"))?,
            digest: digest.0,
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [id: BlockID, "id"],
            [key: SecretKey, "key"],
            [offset: u64, "offset"],
            [size: u64, "size"],
            [digest: HashDigest, "digest"],
        );
        let mut r = self.0.clone();

        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = key {
            r.key = v.0;
        }
        if let Some(v) = offset {
            r.offset = v;
        }
        if let Some(v) = size {
            r.size = NonZeroU64::try_from(v)
                .map_err(|_| PyValueError::new_err("Invalid `size` field"))?;
        }
        if let Some(v) = digest {
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
        Ok(self.0.size.into())
    }

    #[getter]
    fn digest(&self) -> PyResult<HashDigest> {
        Ok(HashDigest(self.0.digest.clone()))
    }
}
