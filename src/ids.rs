// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{IntoPyDict, PyType},
};
use uuid::Uuid;

#[pyclass]
#[derive(Clone)]
pub(crate) struct OrganizationID(pub libparsec::types::OrganizationID);

crate::binding_utils::gen_proto!(OrganizationID, __repr__);
crate::binding_utils::gen_proto!(OrganizationID, __richcmp__, ord);
crate::binding_utils::gen_proto!(OrganizationID, __hash__);

#[pymethods]
impl OrganizationID {
    #[new]
    fn new(organization_id: &PyAny) -> PyResult<Self> {
        if let Ok(organization_id) = organization_id.extract::<Self>() {
            Ok(organization_id)
        } else if let Ok(organization_id) = organization_id.extract::<&str>() {
            match organization_id.parse::<libparsec::types::OrganizationID>() {
                Ok(organization_id) => Ok(Self(organization_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryID(pub libparsec::types::EntryID);

crate::binding_utils::gen_proto!(EntryID, __repr__);
crate::binding_utils::gen_proto!(EntryID, __richcmp__, ord);
crate::binding_utils::gen_proto!(EntryID, __hash__);

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
                    Ok(as_uuid) => Ok(Self(libparsec::types::EntryID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::EntryID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    pub fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::EntryID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::EntryID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(Clone)]
pub(crate) struct BlockID(pub libparsec::types::BlockID);

crate::binding_utils::gen_proto!(BlockID, __repr__);
crate::binding_utils::gen_proto!(BlockID, __richcmp__, ord);
crate::binding_utils::gen_proto!(BlockID, __hash__);

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
                    Ok(as_uuid) => Ok(Self(libparsec::types::BlockID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::BlockID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::BlockID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::BlockID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(Clone)]
pub(crate) struct RealmID(pub libparsec::types::RealmID);

crate::binding_utils::gen_proto!(RealmID, __repr__);
crate::binding_utils::gen_proto!(RealmID, __richcmp__, ord);
crate::binding_utils::gen_proto!(RealmID, __hash__);

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
                    Ok(as_uuid) => Ok(Self(libparsec::types::RealmID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::RealmID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::RealmID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::RealmID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct VlobID(pub libparsec::types::VlobID);

crate::binding_utils::gen_proto!(VlobID, __repr__);
crate::binding_utils::gen_proto!(VlobID, __richcmp__, ord);
crate::binding_utils::gen_proto!(VlobID, __hash__);

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
                    Ok(as_uuid) => Ok(Self(libparsec::types::VlobID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::VlobID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::VlobID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::VlobID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct UserID(pub libparsec::types::UserID);

crate::binding_utils::gen_proto!(UserID, __repr__);
crate::binding_utils::gen_proto!(UserID, __richcmp__, ord);
crate::binding_utils::gen_proto!(UserID, __hash__);

#[pymethods]
impl UserID {
    #[new]
    fn new(user_id: &PyAny) -> PyResult<Self> {
        if let Ok(user_id) = user_id.extract::<Self>() {
            Ok(user_id)
        } else if let Ok(user_id) = user_id.extract::<&str>() {
            match user_id.parse::<libparsec::types::UserID>() {
                Ok(user_id) => Ok(Self(user_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    fn capitalize(&self) -> PyResult<String> {
        Ok(self.0.to_string().to_uppercase())
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
#[derive(Clone)]
pub(crate) struct DeviceName(pub libparsec::types::DeviceName);

crate::binding_utils::gen_proto!(DeviceName, __repr__);
crate::binding_utils::gen_proto!(DeviceName, __richcmp__, ord);
crate::binding_utils::gen_proto!(DeviceName, __hash__);

#[pymethods]
impl DeviceName {
    #[new]
    fn new(device_name: &PyAny) -> PyResult<Self> {
        if let Ok(device_name) = device_name.extract::<Self>() {
            Ok(device_name)
        } else if let Ok(device_name) = device_name.extract::<&str>() {
            match device_name.parse::<libparsec::types::DeviceName>() {
                Ok(device_name) => Ok(Self(device_name)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::DeviceName::default()))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct DeviceLabel(pub libparsec::types::DeviceLabel);

crate::binding_utils::gen_proto!(DeviceLabel, __repr__);
crate::binding_utils::gen_proto!(DeviceLabel, __richcmp__, ord);
crate::binding_utils::gen_proto!(DeviceLabel, __hash__);

#[pymethods]
impl DeviceLabel {
    #[new]
    fn new(device_label: &PyAny) -> PyResult<Self> {
        if let Ok(device_label) = device_label.extract::<Self>() {
            Ok(device_label)
        } else if let Ok(device_label) = device_label.extract::<&str>() {
            match device_label.parse::<libparsec::types::DeviceLabel>() {
                Ok(device_label) => Ok(Self(device_label)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct DeviceID(pub libparsec::types::DeviceID);

crate::binding_utils::gen_proto!(DeviceID, __repr__);
crate::binding_utils::gen_proto!(DeviceID, __richcmp__, ord);
crate::binding_utils::gen_proto!(DeviceID, __hash__);

#[pymethods]
impl DeviceID {
    #[new]
    fn new(device_id: &PyAny) -> PyResult<Self> {
        if let Ok(device_id) = device_id.extract::<Self>() {
            Ok(device_id)
        } else if let Ok(device_id) = device_id.extract::<&str>() {
            match device_id.parse::<libparsec::types::DeviceID>() {
                Ok(device_id) => Ok(Self(device_id)),
                Err(err) => Err(PyValueError::new_err(err)),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
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
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::DeviceID::default()))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct ChunkID(pub libparsec::types::ChunkID);

crate::binding_utils::gen_proto!(ChunkID, __repr__);
crate::binding_utils::gen_proto!(ChunkID, __richcmp__, ord);
crate::binding_utils::gen_proto!(ChunkID, __hash__);

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
                    Ok(as_uuid) => Ok(Self(libparsec::types::ChunkID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::ChunkID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::ChunkID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::ChunkID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct SequesterServiceID(pub libparsec::types::SequesterServiceID);

crate::binding_utils::gen_proto!(SequesterServiceID, __repr__);
crate::binding_utils::gen_proto!(SequesterServiceID, __str__);
crate::binding_utils::gen_proto!(SequesterServiceID, __richcmp__, ord);
crate::binding_utils::gen_proto!(SequesterServiceID, __hash__);

#[pymethods]
impl SequesterServiceID {
    #[new]
    pub fn new(uuid: &PyAny) -> PyResult<Self> {
        // Check if the PyAny as a hex parameter (meaning it's probably a uuid.UUID)
        match uuid.getattr("hex") {
            Ok(as_hex) => {
                // Convert to string
                let u = as_hex.extract::<&str>()?;
                // Parse it as a Rust Uuid
                match Uuid::parse_str(u) {
                    Ok(as_uuid) => Ok(Self(libparsec::types::SequesterServiceID::from(as_uuid))),
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
    fn from_bytes(_cls: &PyType, bytes: &[u8]) -> PyResult<Self> {
        match uuid::Uuid::from_slice(bytes) {
            Ok(uuid) => Ok(Self(libparsec::types::SequesterServiceID::from(uuid))),
            Err(_) => Err(PyValueError::new_err("Invalid UUID")),
        }
    }

    #[classmethod]
    fn from_hex(_cls: &PyType, hex: &str) -> PyResult<Self> {
        match hex.parse::<libparsec::types::SequesterServiceID>() {
            Ok(entry_id) => Ok(Self(entry_id)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::SequesterServiceID::default()))
    }

    #[getter]
    fn bytes(&self) -> PyResult<&[u8]> {
        Ok(&self.0.as_bytes()[..])
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
#[derive(Clone)]
pub(crate) struct HumanHandle(pub libparsec::types::HumanHandle);

crate::binding_utils::gen_proto!(HumanHandle, __repr__);
crate::binding_utils::gen_proto!(HumanHandle, __richcmp__, ord);
crate::binding_utils::gen_proto!(HumanHandle, __hash__);

#[pymethods]
impl HumanHandle {
    #[new]
    pub fn new(email: &str, label: &str) -> PyResult<Self> {
        match libparsec::types::HumanHandle::new(email, label) {
            Ok(human_handle) => Ok(Self(human_handle)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[getter]
    fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[getter]
    fn email(&self) -> PyResult<&str> {
        Ok(&self.0.email)
    }

    #[getter]
    fn label(&self) -> PyResult<&str> {
        Ok(&self.0.label)
    }
}
