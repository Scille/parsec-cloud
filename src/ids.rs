// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, prelude::*, types::PyType};

// UUID based type

macro_rules! gen_uuid {
    ($class: ident) => {
        #[pymethods]
        impl $class {
            #[classmethod]
            fn from_bytes(_cls: &::pyo3::types::PyType, bytes: &[u8]) -> PyResult<Self> {
                libparsec::types::$class::try_from(bytes)
                    .map(Self)
                    .map_err(::pyo3::exceptions::PyValueError::new_err)
            }

            #[classmethod]
            fn from_hex(_cls: &::pyo3::types::PyType, hex: &str) -> PyResult<Self> {
                libparsec::types::$class::from_hex(hex)
                    .map(Self)
                    .map_err(::pyo3::exceptions::PyValueError::new_err)
            }

            #[classmethod]
            #[pyo3(name = "new")]
            fn default(_cls: &::pyo3::types::PyType) -> Self {
                Self(libparsec::types::$class::default())
            }

            #[getter]
            fn bytes(&self) -> &[u8] {
                &self.0.as_bytes()[..]
            }

            #[getter]
            fn hex(&self) -> String {
                self.0.hex()
            }

            #[getter]
            fn int(&self) -> u128 {
                self.0.as_u128()
            }

            #[getter]
            fn hyphenated(&self) -> String {
                self.0.as_hyphenated().to_string()
            }
        }
    };
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct EntryID(pub libparsec::types::EntryID);

crate::binding_utils::gen_proto!(EntryID, __repr__);
crate::binding_utils::gen_proto!(EntryID, __str__);
crate::binding_utils::gen_proto!(EntryID, __richcmp__, ord);
crate::binding_utils::gen_proto!(EntryID, __hash__);
gen_uuid!(EntryID);

#[pymethods]
impl EntryID {
    #[new]
    fn new(id: &PyAny) -> PyResult<Self> {
        Ok(Self(if let Ok(RealmID(id)) = id.extract() {
            libparsec::types::EntryID::from(*id)
        } else if let Ok(VlobID(id)) = id.extract() {
            libparsec::types::EntryID::from(*id)
        } else {
            return Err(PyValueError::new_err(format!(
                "Cannot convert {id} into `EntryID`"
            )));
        }))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct BlockID(pub libparsec::types::BlockID);

crate::binding_utils::gen_proto!(BlockID, __repr__);
crate::binding_utils::gen_proto!(BlockID, __str__);
crate::binding_utils::gen_proto!(BlockID, __richcmp__, ord);
crate::binding_utils::gen_proto!(BlockID, __hash__);
gen_uuid!(BlockID);

#[pyclass]
#[derive(Clone)]
pub(crate) struct RealmID(pub libparsec::types::RealmID);

crate::binding_utils::gen_proto!(RealmID, __repr__);
crate::binding_utils::gen_proto!(RealmID, __richcmp__, ord);
crate::binding_utils::gen_proto!(RealmID, __hash__);
gen_uuid!(RealmID);

#[pymethods]
impl RealmID {
    #[new]
    fn new(id: EntryID) -> Self {
        Self(libparsec::types::RealmID::from(*id.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct VlobID(pub libparsec::types::VlobID);

crate::binding_utils::gen_proto!(VlobID, __repr__);
crate::binding_utils::gen_proto!(VlobID, __richcmp__, ord);
crate::binding_utils::gen_proto!(VlobID, __hash__);
gen_uuid!(VlobID);

#[pymethods]
impl VlobID {
    #[new]
    fn new(id: EntryID) -> Self {
        Self(libparsec::types::VlobID::from(*id.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct ChunkID(pub libparsec::types::ChunkID);

crate::binding_utils::gen_proto!(ChunkID, __repr__);
crate::binding_utils::gen_proto!(ChunkID, __richcmp__, ord);
crate::binding_utils::gen_proto!(ChunkID, __hash__);
gen_uuid!(ChunkID);

#[pymethods]
impl ChunkID {
    #[new]
    fn new(id: BlockID) -> Self {
        Self(libparsec::types::ChunkID::from(*id.0))
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct SequesterServiceID(pub libparsec::types::SequesterServiceID);

crate::binding_utils::gen_proto!(SequesterServiceID, __repr__);
crate::binding_utils::gen_proto!(SequesterServiceID, __richcmp__, ord);
crate::binding_utils::gen_proto!(SequesterServiceID, __hash__);
gen_uuid!(SequesterServiceID);

#[pyclass]
#[derive(Clone)]
pub(crate) struct InvitationToken(pub libparsec::types::InvitationToken);

crate::binding_utils::gen_proto!(InvitationToken, __repr__);
crate::binding_utils::gen_proto!(InvitationToken, __richcmp__, eq);
crate::binding_utils::gen_proto!(InvitationToken, __hash__);
gen_uuid!(InvitationToken);

#[pyclass]
#[derive(Clone)]
pub(crate) struct EnrollmentID(pub libparsec::types::EnrollmentID);

crate::binding_utils::gen_proto!(EnrollmentID, __repr__);
crate::binding_utils::gen_proto!(EnrollmentID, __richcmp__, eq);
gen_uuid!(EnrollmentID);

// Other ids

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
