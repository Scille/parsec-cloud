// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBytes, PyString, PyType};
use uuid::Uuid;

use crate::binding_utils::{comp_op, hash_generic};

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct OrganizationID(pub parsec_api_types::OrganizationID);

#[pymethods]
impl OrganizationID {
    #[new]
    fn new(organization_id: &PyAny) -> PyResult<Self> {
        if let Ok(organization_id) = organization_id.extract::<Self>() {
            Ok(organization_id)
        } else if let Ok(organization_id) = organization_id.extract::<&str>() {
            match organization_id.parse::<parsec_api_types::OrganizationID>() {
                Ok(organization_id) => Ok(Self(organization_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<OrganizationID {}>", self.0))
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
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryID(pub parsec_api_types::EntryID);

#[pymethods]
impl EntryID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(parsec_api_types::EntryID::from(as_uuid))),
                    Err(_) => Err(PyValueError::new_err("Not a UUID")),
                }
            }
            Err(_) => Err(PyValueError::new_err("Not a UUID")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<EntryID {}>", self.0))
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::EntryID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    pub fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::EntryID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::EntryID::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &EntryID, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct BlockID(pub parsec_api_types::BlockID);

#[pymethods]
impl BlockID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(parsec_api_types::BlockID::from(as_uuid))),
                    Err(_) => Err(PyValueError::new_err("Not a UUID")),
                }
            }
            Err(_) => Err(PyValueError::new_err("Not a UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::BlockID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::BlockID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::BlockID::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &BlockID, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<BlockID {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct RealmID(pub parsec_api_types::RealmID);

#[pymethods]
impl RealmID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(parsec_api_types::RealmID::from(as_uuid))),
                    Err(_) => Err(PyValueError::new_err("Not a UUID")),
                }
            }
            Err(_) => Err(PyValueError::new_err("Not a UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::RealmID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::RealmID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::RealmID::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &RealmID, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<RealmID {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct VlobID(pub parsec_api_types::VlobID);

#[pymethods]
impl VlobID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(parsec_api_types::VlobID::from(as_uuid))),
                    Err(_) => Err(PyValueError::new_err("Not a UUID")),
                }
            }
            Err(_) => Err(PyValueError::new_err("Not a UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::VlobID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::VlobID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::VlobID::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, py: Python, other: &VlobID, op: CompareOp) -> PyResult<bool> {
        let h1 = self.__hash__(py).unwrap();
        let h2 = other.__hash__(py).unwrap();
        comp_op(op, h1, h2)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<VlobID {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.as_hyphenated(), py)
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct UserID(pub parsec_api_types::UserID);

#[pymethods]
impl UserID {
    #[new]
    fn new(user_id: &PyAny) -> PyResult<Self> {
        if let Ok(user_id) = user_id.extract::<Self>() {
            Ok(user_id)
        } else if let Ok(user_id) = user_id.extract::<&str>() {
            match user_id.parse::<parsec_api_types::UserID>() {
                Ok(user_id) => Ok(Self(user_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<UserID {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    fn capitalize(&self) -> PyResult<String> {
        let str = self.0.to_string();
        let mut it = str.chars();
        match it.next() {
            Some(c) => Ok(c.to_uppercase().chain(it).collect()),
            None => Ok(String::new()),
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn to_device_id(&self, device_name: &DeviceName) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.to_device_id(&device_name.0)))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct DeviceName(pub parsec_api_types::DeviceName);

#[pymethods]
impl DeviceName {
    #[new]
    fn new(device_name: &PyAny) -> PyResult<Self> {
        if let Ok(device_name) = device_name.extract::<Self>() {
            Ok(device_name)
        } else if let Ok(device_name) = device_name.extract::<&str>() {
            match device_name.parse::<parsec_api_types::DeviceName>() {
                Ok(device_name) => Ok(Self(device_name)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }
    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<DeviceName {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::DeviceName::default()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct DeviceLabel(pub parsec_api_types::DeviceLabel);

#[pymethods]
impl DeviceLabel {
    #[new]
    fn new(device_label: &PyAny) -> PyResult<Self> {
        if let Ok(device_label) = device_label.extract::<Self>() {
            Ok(device_label)
        } else if let Ok(device_label) = device_label.extract::<&str>() {
            match device_label.parse::<parsec_api_types::DeviceLabel>() {
                Ok(device_label) => Ok(Self(device_label)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<DeviceLabel {}>", self.0))
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
pub(crate) struct DeviceID(pub parsec_api_types::DeviceID);

#[pymethods]
impl DeviceID {
    #[new]
    fn new(device_id: &PyAny) -> PyResult<Self> {
        if let Ok(device_id) = device_id.extract::<Self>() {
            Ok(device_id)
        } else if let Ok(device_id) = device_id.extract::<&str>() {
            match device_id.parse::<parsec_api_types::DeviceID>() {
                Ok(device_id) => Ok(Self(device_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<DeviceID {}>", self.0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[getter]
    fn user_id(&self) -> PyResult<UserID> {
        Ok(UserID(self.0.user_id.clone()))
    }

    #[getter]
    fn device_name(&self) -> PyResult<DeviceName> {
        Ok(DeviceName(self.0.device_name.clone()))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::DeviceID::default()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct ChunkID(pub parsec_api_types::ChunkID);

#[pymethods]
impl ChunkID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(parsec_api_types::ChunkID::from(as_uuid))),
                    Err(_) => Err(PyValueError::new_err("Not a UUID")),
                }
            }
            Err(_) => Err(PyValueError::new_err("Not a UUID")),
        }
    }

    #[getter]
    fn uuid<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let uuid = PyModule::import(py, "uuid")?;
        let kwargs = vec![("hex", self.hex().unwrap())].into_py_dict(py);
        match uuid.getattr("UUID")?.call((), Some(kwargs)) {
            Ok(any) => Ok(any),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, bytes: &PyBytes) -> PyResult<Self> {
        let b = bytes.as_bytes();
        match uuid::Uuid::from_slice(b) {
            Ok(uuid) => Ok(Self(parsec_api_types::ChunkID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &PyString) -> PyResult<Self> {
        match hex.to_string().parse::<parsec_api_types::ChunkID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    fn __richcmp__(&self, py: Python, other: &Self, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<ChunkID {}>", self.0))
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn _class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(parsec_api_types::ChunkID::default()))
    }

    #[getter]
    fn bytes<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(py, self.0.as_bytes()))
    }

    #[getter]
    fn hex(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct HumanHandle(pub parsec_api_types::HumanHandle);

#[pymethods]
impl HumanHandle {
    #[new]
    pub fn new(email: &str, label: &str) -> PyResult<Self> {
        match parsec_api_types::HumanHandle::new(email, label) {
            Ok(human_handle) => Ok(Self(human_handle)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<HumanHandle {} >", self.0))
    }

    fn __richcmp__(&self, py: Python, other: &HumanHandle, op: CompareOp) -> PyObject {
        match op {
            CompareOp::Eq => (self.0.email == other.0.email).into_py(py),
            CompareOp::Ne => (self.0.email != other.0.email).into_py(py),
            CompareOp::Lt => (self.0.email < other.0.email).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.email, py)
    }

    #[getter]
    fn email(&self) -> PyResult<&String> {
        Ok(&self.0.email)
    }

    #[getter]
    fn label(&self) -> PyResult<&String> {
        Ok(&self.0.label)
    }
}
