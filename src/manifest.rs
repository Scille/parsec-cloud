use std::{collections::HashMap, num::NonZeroU64};

use libparsec::types::Manifest;
use pyo3::{
    exceptions::PyValueError,
    import_exception, pyclass,
    pyclass::CompareOp,
    pyfunction, pymethods,
    types::{PyBool, PyBytes, PyDict, PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};

use crate::{
    api_crypto::{HashDigest, SecretKey, SigningKey, VerifyKey},
    binding_utils::{hash_generic, py_to_rs_realm_role, rs_to_py_realm_role},
    ids::{BlockID, DeviceID, EntryID},
    time::DateTime,
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
        hash_generic(self.0.as_ref(), py)
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
            [encrypted_on: DateTime, "encrypted_on"],
            [role_cached_on: DateTime, "role_cached_on"],
            [role, "role", py_to_rs_realm_role],
        );

        Ok(Self(libparsec::types::WorkspaceEntry {
            id: id.0,
            name: name.0,
            key: key.0,
            encryption_revision,
            encrypted_on: encrypted_on.0,
            role_cached_on: role_cached_on.0,
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
            [encrypted_on: DateTime, "encrypted_on"],
            [role_cached_on: DateTime, "role_cached_on"],
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
            r.encrypted_on = v.0;
        }
        if let Some(v) = role_cached_on {
            r.role_cached_on = v.0;
        }
        if let Some(v) = role {
            r.role = v;
        }

        Ok(Self(r))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType, name: &EntryName, timestamp: DateTime) -> PyResult<Self> {
        Ok(Self(libparsec::types::WorkspaceEntry::generate(
            name.0.to_owned(),
            timestamp.0,
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
    fn encrypted_on(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.encrypted_on))
    }

    #[getter]
    fn role_cached_on(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.role_cached_on))
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

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct FileManifest(pub libparsec::types::FileManifest);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [parent: EntryID, "parent"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [size: u64, "size"],
            [blocksize: u64, "blocksize"],
            [blocks: Vec<BlockAccess>, "blocks"],
        );

        Ok(Self(libparsec::types::FileManifest {
            author: author.0,
            timestamp: timestamp.0,
            id: id.0,
            parent: parent.0,
            version,
            created: created.0,
            updated: updated.0,
            size,
            blocksize: libparsec::types::Blocksize::try_from(blocksize)
                .map_err(|_| PyValueError::new_err("Invalid `blocksize` field"))?,
            blocks: blocks.into_iter().map(|b| b.0).collect(),
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_and_sign<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
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
    #[allow(clippy::too_many_arguments)]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<EntryID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec::types::FileManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map(Self)
        .map_err(|e| DataError::new_err(e.to_string()))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [parent: EntryID, "parent"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [size: u64, "size"],
            [blocksize: u64, "blocksize"],
            [blocks: Vec<BlockAccess>, "blocks"],
        );

        let mut r = self.0.clone();

        if let Some(v) = author {
            r.author = v.0;
        }
        if let Some(v) = timestamp {
            r.timestamp = v.0;
        }
        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = parent {
            r.parent = v.0;
        }
        if let Some(v) = version {
            r.version = v;
        }
        if let Some(v) = created {
            r.created = v.0;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = size {
            r.size = v;
        }
        if let Some(v) = blocksize {
            r.blocksize = libparsec::types::Blocksize::try_from(v)
                .map_err(|_| PyValueError::new_err("Invalid `blocksize` field"))?;
        }
        if let Some(v) = blocks {
            r.blocks = v.into_iter().map(|b| b.0).collect();
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
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn created(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created))
    }

    #[getter]
    fn updated(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.updated))
    }

    #[getter]
    fn blocks<'p>(&self, py: Python<'p>) -> PyResult<&'p PyTuple> {
        let elements: Vec<PyObject> = self
            .0
            .blocks
            .iter()
            .map(|x| BlockAccess(x.clone()).into_py(py))
            .collect();
        Ok(PyTuple::new(py, elements))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct FolderManifest(pub libparsec::types::FolderManifest);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [parent: EntryID, "parent"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
        );

        Ok(Self(libparsec::types::FolderManifest {
            author: author.0,
            timestamp: timestamp.0,
            version,
            id: id.0,
            parent: parent.0,
            created: created.0,
            updated: updated.0,
            children: children
                .into_iter()
                .map(|(name, id)| (name.0, id.0))
                .collect(),
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_and_sign<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
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
    #[allow(clippy::too_many_arguments)]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<EntryID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec::types::FolderManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map(Self)
        .map_err(|e| DataError::new_err(e.to_string()))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [parent: EntryID, "parent"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
        );

        let mut r = self.0.clone();

        if let Some(v) = author {
            r.author = v.0;
        }
        if let Some(v) = timestamp {
            r.timestamp = v.0;
        }
        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = parent {
            r.parent = v.0;
        }
        if let Some(v) = version {
            r.version = v;
        }
        if let Some(v) = created {
            r.created = v.0;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = children {
            r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
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
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn created(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created))
    }

    #[getter]
    fn updated(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.updated))
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
pub(crate) struct WorkspaceManifest(pub libparsec::types::WorkspaceManifest);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
        );

        Ok(Self(libparsec::types::WorkspaceManifest {
            author: author.0,
            timestamp: timestamp.0,
            id: id.0,
            version,
            created: created.0,
            updated: updated.0,
            children: children
                .into_iter()
                .map(|(name, id)| (name.0, id.0))
                .collect(),
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_and_sign<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
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
    #[allow(clippy::too_many_arguments)]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<EntryID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec::types::WorkspaceManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map(Self)
        .map_err(|e| DataError::new_err(e.to_string()))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, EntryID>, "children"],
        );

        let mut r = self.0.clone();

        if let Some(v) = author {
            r.author = v.0;
        }
        if let Some(v) = timestamp {
            r.timestamp = v.0;
        }
        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = version {
            r.version = v;
        }
        if let Some(v) = created {
            r.created = v.0;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = children {
            r.children = v.into_iter().map(|(name, id)| (name.0, id.0)).collect();
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
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn created(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created))
    }

    #[getter]
    fn updated(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.updated))
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
pub(crate) struct UserManifest(pub libparsec::types::UserManifest);

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
        crate::binding_utils::parse_kwargs!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [last_processed_message: u64, "last_processed_message"],
            [workspaces: Vec<WorkspaceEntry>, "workspaces"],
        );

        Ok(Self(libparsec::types::UserManifest {
            author: author.0,
            timestamp: timestamp.0,
            id: id.0,
            version,
            created: created.0,
            updated: updated.0,
            last_processed_message,
            workspaces: workspaces.into_iter().map(|w| w.0).collect(),
        }))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    fn dump_and_sign<'p>(
        &self,
        py: Python<'p>,
        author_signkey: &SigningKey,
    ) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0)))
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
    #[allow(clippy::too_many_arguments)]
    fn decrypt_verify_and_load(
        _cls: &PyType,
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<EntryID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec::types::UserManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map(Self)
        .map_err(|e| DataError::new_err(e.to_string()))
    }

    #[args(py_kwargs = "**")]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: EntryID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [last_processed_message: u64, "last_processed_message"],
            [workspaces: Vec<WorkspaceEntry>, "workspaces"],
        );

        let mut r = self.0.clone();

        if let Some(v) = author {
            r.author = v.0;
        }
        if let Some(v) = timestamp {
            r.timestamp = v.0;
        }
        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = version {
            r.version = v;
        }
        if let Some(v) = created {
            r.created = v.0;
        }
        if let Some(v) = updated {
            r.updated = v.0;
        }
        if let Some(v) = last_processed_message {
            r.last_processed_message = v;
        }
        if let Some(v) = workspaces {
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
    fn timestamp(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.timestamp))
    }

    #[getter]
    fn created(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.created))
    }

    #[getter]
    fn updated(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.updated))
    }

    #[getter]
    fn last_processed_message(&self) -> PyResult<u64> {
        Ok(self.0.last_processed_message)
    }

    #[getter]
    fn workspaces<'p>(&self, py: Python<'p>) -> PyResult<&'p PyTuple> {
        let elements: Vec<PyObject> = self
            .0
            .workspaces
            .clone()
            .into_iter()
            .map(|x| WorkspaceEntry(x).into_py(py))
            .collect();
        Ok(PyTuple::new(py, elements))
    }
}

#[pyfunction]
pub(crate) fn manifest_decrypt_and_load<'py>(
    py: Python<'py>,
    encrypted: &[u8],
    key: &SecretKey,
) -> PyResult<PyObject> {
    let decrypt_and_load = match Manifest::decrypt_and_load(encrypted, &key.0) {
        Ok(value) => value,
        Err(err) => return Err(DataError::new_err(err)),
    };

    unwrap_manifest(py, decrypt_and_load)
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn manifest_decrypt_verify_and_load<'py>(
    py: Python<'py>,
    encrypted: &[u8],
    key: &SecretKey,
    author_verify_key: &VerifyKey,
    expected_author: &DeviceID,
    expected_timestamp: DateTime,
    expected_id: Option<EntryID>,
    expected_version: Option<u32>,
) -> PyResult<PyObject> {
    let blob = match Manifest::decrypt_verify_and_load(
        encrypted,
        &key.0,
        &author_verify_key.0,
        &expected_author.0,
        expected_timestamp.0,
        expected_id.map(|id| id.0),
        expected_version,
    ) {
        Ok(value) => value,
        Err(err) => return Err(DataError::new_err(err.to_string())),
    };

    unwrap_manifest(py, blob)
}

#[pyfunction]
pub(crate) fn manifest_verify_and_load<'py>(
    py: Python<'py>,
    signed: &[u8],
    author_verify_key: &VerifyKey,
    expected_author: &DeviceID,
    expected_timestamp: DateTime,
    expected_id: Option<EntryID>,
    expected_version: Option<u32>,
) -> PyResult<PyObject> {
    let blob = match Manifest::verify_and_load(
        signed,
        &author_verify_key.0,
        &expected_author.0,
        expected_timestamp.0,
        expected_id.map(|id| id.0),
        expected_version,
    ) {
        Ok(value) => value,
        Err(err) => return Err(DataError::new_err(err.to_string())),
    };

    unwrap_manifest(py, blob)
}

fn unwrap_manifest(py: Python, manifest: Manifest) -> PyResult<PyObject> {
    match manifest {
        Manifest::File(file) => Ok(FileManifest(file).into_py(py)),
        Manifest::Folder(folder) => Ok(FolderManifest(folder).into_py(py)),
        Manifest::Workspace(workspace) => Ok(WorkspaceManifest(workspace).into_py(py)),
        Manifest::User(user) => Ok(UserManifest(user).into_py(py)),
    }
}
