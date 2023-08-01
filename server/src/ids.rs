// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{exceptions::PyValueError, prelude::*, types::PyType};

// UUID based type

macro_rules! gen_uuid {
    ($class: ident) => {
        #[pymethods]
        impl $class {
            #[classmethod]
            fn from_bytes(_cls: &::pyo3::types::PyType, bytes: &[u8]) -> PyResult<Self> {
                libparsec::low_level::types::$class::try_from(bytes)
                    .map(Self)
                    .map_err(::pyo3::exceptions::PyValueError::new_err)
            }

            #[classmethod]
            fn from_hex(_cls: &::pyo3::types::PyType, hex: &str) -> PyResult<Self> {
                libparsec::low_level::types::$class::from_hex(hex)
                    .map(Self)
                    .map_err(::pyo3::exceptions::PyValueError::new_err)
            }

            #[classmethod]
            #[pyo3(name = "new")]
            fn default(_cls: &::pyo3::types::PyType) -> Self {
                Self(libparsec::low_level::types::$class::default())
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

crate::binding_utils::gen_py_wrapper_class_for_id!(
    EntryID,
    libparsec::low_level::types::EntryID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(EntryID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    BlockID,
    libparsec::low_level::types::BlockID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(BlockID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    RealmID,
    libparsec::low_level::types::RealmID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(RealmID);

#[pymethods]
impl RealmID {
    fn to_entry_id(&self) -> EntryID {
        EntryID(libparsec::low_level::types::EntryID::from(*self.0))
    }
    #[classmethod]
    fn from_entry_id(_cls: &PyType, id: EntryID) -> Self {
        Self(libparsec::low_level::types::RealmID::from(*id.0))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    VlobID,
    libparsec::low_level::types::VlobID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(VlobID);

#[pymethods]
impl VlobID {
    fn to_entry_id(&self) -> EntryID {
        EntryID(libparsec::low_level::types::EntryID::from(*self.0))
    }
    #[classmethod]
    fn from_entry_id(_cls: &PyType, id: EntryID) -> Self {
        Self(libparsec::low_level::types::VlobID::from(*id.0))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    ChunkID,
    libparsec::low_level::types::ChunkID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(ChunkID);

#[pymethods]
impl ChunkID {
    #[classmethod]
    fn from_block_id(_cls: &PyType, id: BlockID) -> Self {
        Self(libparsec::low_level::types::ChunkID::from(*id.0))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    SequesterServiceID,
    libparsec::low_level::types::SequesterServiceID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(SequesterServiceID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    InvitationToken,
    libparsec::low_level::types::InvitationToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(InvitationToken);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    EnrollmentID,
    libparsec::low_level::types::EnrollmentID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(EnrollmentID);

// Other ids

crate::binding_utils::gen_py_wrapper_class_for_id!(
    OrganizationID,
    libparsec::low_level::types::OrganizationID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl OrganizationID {
    #[new]
    fn new(organization_id: &PyAny) -> PyResult<Self> {
        if let Ok(organization_id) = organization_id.extract::<Self>() {
            Ok(organization_id)
        } else if let Ok(organization_id) = organization_id.extract::<&str>() {
            match organization_id.parse::<libparsec::low_level::types::OrganizationID>() {
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

crate::binding_utils::gen_py_wrapper_class_for_id!(
    UserID,
    libparsec::low_level::types::UserID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl UserID {
    #[new]
    fn new(user_id: &PyAny) -> PyResult<Self> {
        if let Ok(user_id) = user_id.extract::<Self>() {
            Ok(user_id)
        } else if let Ok(user_id) = user_id.extract::<&str>() {
            match user_id.parse::<libparsec::low_level::types::UserID>() {
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

    fn to_device_id(&self, device_name: DeviceName) -> PyResult<DeviceID> {
        Ok(DeviceID(self.0.to_device_id(device_name.0)))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    DeviceName,
    libparsec::low_level::types::DeviceName,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl DeviceName {
    #[new]
    fn new(device_name: &PyAny) -> PyResult<Self> {
        if let Ok(device_name) = device_name.extract::<Self>() {
            Ok(device_name)
        } else if let Ok(device_name) = device_name.extract::<&str>() {
            match device_name.parse::<libparsec::low_level::types::DeviceName>() {
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
        Ok(Self(libparsec::low_level::types::DeviceName::default()))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    DeviceLabel,
    libparsec::low_level::types::DeviceLabel,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl DeviceLabel {
    #[new]
    fn new(device_label: &PyAny) -> PyResult<Self> {
        if let Ok(device_label) = device_label.extract::<Self>() {
            Ok(device_label)
        } else if let Ok(device_label) = device_label.extract::<&str>() {
            match device_label.parse::<libparsec::low_level::types::DeviceLabel>() {
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

crate::binding_utils::gen_py_wrapper_class_for_id!(
    DeviceID,
    libparsec::low_level::types::DeviceID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl DeviceID {
    #[new]
    fn new(device_id: &PyAny) -> PyResult<Self> {
        if let Ok(device_id) = device_id.extract::<Self>() {
            Ok(device_id)
        } else if let Ok(device_id) = device_id.extract::<&str>() {
            match device_id.parse::<libparsec::low_level::types::DeviceID>() {
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
        Ok(UserID(self.0.user_id().clone()))
    }

    #[getter]
    fn device_name(&self) -> PyResult<DeviceName> {
        Ok(DeviceName(self.0.device_name().clone()))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn class_new(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::low_level::types::DeviceID::default()))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    HumanHandle,
    libparsec::low_level::types::HumanHandle,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl HumanHandle {
    #[new]
    pub fn new(email: &str, label: &str) -> PyResult<Self> {
        match libparsec::low_level::types::HumanHandle::new(email, label) {
            Ok(human_handle) => Ok(Self(human_handle)),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }

    #[getter]
    fn str(&self) -> PyResult<&str> {
        Ok(self.0.as_ref())
    }

    #[getter]
    fn email(&self) -> PyResult<&str> {
        Ok(self.0.email())
    }

    #[getter]
    fn label(&self) -> PyResult<&str> {
        Ok(self.0.label())
    }
}
