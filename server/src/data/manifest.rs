// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pyfunction, pymethods,
    types::{PyBytes, PyDict, PyTuple, PyType},
    IntoPy, PyObject, PyResult, Python,
};
use std::{collections::HashMap, num::NonZeroU64};

use crate::{BlockID, DateTime, DeviceID, HashDigest, SecretKey, SigningKey, VerifyKey, VlobID};
use libparsec_types::ChildManifest;

crate::binding_utils::gen_py_wrapper_class_for_id!(
    EntryName,
    libparsec_types::EntryName,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl EntryName {
    #[new]
    fn new(name: &str) -> PyResult<Self> {
        libparsec_types::EntryName::try_from(name)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(Self)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryName {}>", self.0))
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    BlockAccess,
    libparsec_types::BlockAccess,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl BlockAccess {
    #[new]
    #[pyo3(signature = (id, key_index, offset, size, digest))]
    fn new(
        id: BlockID,
        key_index: u64,
        offset: u64,
        size: u64,
        digest: HashDigest,
    ) -> PyResult<Self> {
        Ok(Self(libparsec_types::BlockAccess {
            id: id.0,
            key_index,
            key: None,
            offset,
            size: NonZeroU64::try_from(size)
                .map_err(|_| PyValueError::new_err("Invalid `size` field"))?,
            digest: digest.0,
        }))
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [id: BlockID, "id"],
            [key_index: u64, "key"],
            [offset: u64, "offset"],
            [size: u64, "size"],
            [digest: HashDigest, "digest"],
        );
        let mut r = self.0.clone();

        if let Some(v) = id {
            r.id = v.0;
        }
        if let Some(v) = key_index {
            r.key = None;
            r.key_index = v;
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

    #[getter]
    fn id(&self) -> PyResult<BlockID> {
        Ok(BlockID(self.0.id))
    }

    #[getter]
    fn key(&self) -> PyResult<Option<SecretKey>> {
        Ok(self.0.key.as_ref().map(|key| SecretKey(key.to_owned())))
    }

    #[getter]
    fn key_index(&self) -> PyResult<u64> {
        Ok(self.0.key_index)
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

    fn __hash__(&self) -> PyResult<u64> {
        crate::binding_utils::hash_generic(self.0.id)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    FileManifest,
    libparsec_types::FileManifest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl FileManifest {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (author, timestamp, id, parent, version, created, updated, size, blocksize, blocks))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        parent: VlobID,
        version: u32,
        created: DateTime,
        updated: DateTime,
        size: u64,
        blocksize: u64,
        blocks: Vec<BlockAccess>,
    ) -> PyResult<Self> {
        Ok(Self(libparsec_types::FileManifest {
            author: author.0,
            timestamp: timestamp.0,
            id: id.0,
            parent: parent.0,
            version,
            created: created.0,
            updated: updated.0,
            size,
            blocksize: libparsec_types::Blocksize::try_from(blocksize)
                .map_err(|_| PyValueError::new_err("Invalid `blocksize` field"))?,
            blocks: blocks.into_iter().map(|b| b.0).collect(),
        }))
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
        expected_id: Option<VlobID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec_types::FileManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: VlobID, "id"],
            [parent: VlobID, "parent"],
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
            r.blocksize = libparsec_types::Blocksize::try_from(v)
                .map_err(|_| PyValueError::new_err("Invalid `blocksize` field"))?;
        }
        if let Some(v) = blocks {
            r.blocks = v.into_iter().map(|b| b.0).collect();
        }

        Ok(Self(r))
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.id))
    }

    #[getter]
    fn parent(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.parent))
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

crate::binding_utils::gen_py_wrapper_class!(
    FolderManifest,
    libparsec_types::FolderManifest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl FolderManifest {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (author, timestamp, id, parent, version, created, updated, children))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        parent: VlobID,
        version: u32,
        created: DateTime,
        updated: DateTime,
        children: HashMap<EntryName, VlobID>,
    ) -> PyResult<Self> {
        Ok(Self(libparsec_types::FolderManifest {
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
        expected_id: Option<VlobID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec_types::FolderManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: VlobID, "id"],
            [parent: VlobID, "parent"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, VlobID>, "children"],
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

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.id))
    }

    #[getter]
    fn parent(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.parent))
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
            let me = VlobID(*v).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    WorkspaceManifest,
    libparsec_types::WorkspaceManifest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl WorkspaceManifest {
    #[new]
    #[pyo3(signature = (author, timestamp, id, version, created, updated, children))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        version: u32,
        created: DateTime,
        updated: DateTime,
        children: HashMap<EntryName, VlobID>,
    ) -> PyResult<Self> {
        Ok(Self(libparsec_types::WorkspaceManifest {
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
        expected_id: Option<VlobID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec_types::WorkspaceManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[classmethod]
    #[allow(clippy::too_many_arguments)]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
        expected_id: Option<VlobID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec_types::WorkspaceManifest::verify_and_load(
            signed,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: VlobID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
            [children: HashMap<EntryName, VlobID>, "children"],
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

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.id))
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
            let me = VlobID(*v).into_py(py);
            let _ = d.set_item(en, me);
        }
        Ok(d)
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    UserManifest,
    libparsec_types::UserManifest,
    __repr__,
    __copy__,
    __deepcopy__,
    __richcmp__ eq,
);

#[pymethods]
impl UserManifest {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (author, timestamp, id, version, created, updated))]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        id: VlobID,
        version: u32,
        created: DateTime,
        updated: DateTime,
    ) -> PyResult<Self> {
        Ok(Self(libparsec_types::UserManifest {
            author: author.0,
            timestamp: timestamp.0,
            id: id.0,
            version,
            created: created.0,
            updated: updated.0,
            workspaces_legacy_initial_info: vec![],
        }))
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
        expected_id: Option<VlobID>,
        expected_version: Option<u32>,
    ) -> PyResult<Self> {
        libparsec_types::UserManifest::decrypt_verify_and_load(
            encrypted,
            &key.0,
            &author_verify_key.0,
            &expected_author.0,
            expected_timestamp.0,
            expected_id.map(|id| id.0),
            expected_version,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
    }

    #[pyo3(signature = (**py_kwargs))]
    fn evolve(&self, py_kwargs: Option<&PyDict>) -> PyResult<Self> {
        crate::binding_utils::parse_kwargs_optional!(
            py_kwargs,
            [author: DeviceID, "author"],
            [timestamp: DateTime, "timestamp"],
            [id: VlobID, "id"],
            [version: u32, "version"],
            [created: DateTime, "created"],
            [updated: DateTime, "updated"],
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

        Ok(Self(r))
    }

    #[getter]
    fn author(&self) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.author.clone()))
    }

    #[getter]
    fn id(&self) -> PyResult<VlobID> {
        Ok(VlobID(self.0.id))
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
}

#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn child_manifest_decrypt_verify_and_load(
    py: Python,
    encrypted: &[u8],
    key: &SecretKey,
    author_verify_key: &VerifyKey,
    expected_author: &DeviceID,
    expected_timestamp: DateTime,
    expected_id: Option<VlobID>,
    expected_version: Option<u32>,
) -> PyResult<PyObject> {
    ChildManifest::decrypt_verify_and_load(
        encrypted,
        &key.0,
        &author_verify_key.0,
        &expected_author.0,
        expected_timestamp.0,
        expected_id.map(|id| id.0),
        expected_version,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))
    .map(|blob| unwrap_child_manifest(py, blob))
}

#[pyfunction]
pub(crate) fn child_manifest_verify_and_load(
    py: Python,
    signed: &[u8],
    author_verify_key: &VerifyKey,
    expected_author: &DeviceID,
    expected_timestamp: DateTime,
    expected_id: Option<VlobID>,
    expected_version: Option<u32>,
) -> PyResult<PyObject> {
    ChildManifest::verify_and_load(
        signed,
        &author_verify_key.0,
        &expected_author.0,
        expected_timestamp.0,
        expected_id.map(|id| id.0),
        expected_version,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))
    .map(|blob| unwrap_child_manifest(py, blob))
}

fn unwrap_child_manifest(py: Python, manifest: ChildManifest) -> PyObject {
    match manifest {
        ChildManifest::File(file) => FileManifest(file).into_py(py),
        ChildManifest::Folder(folder) => FolderManifest(folder).into_py(py),
    }
}
