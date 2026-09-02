// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList, PyType, PyTypeMethods},
};

// UUID based type

macro_rules! gen_uuid {
    ($class: ident) => {
        #[pymethods]
        impl $class {
            #[classmethod]
            fn from_bytes(
                _cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>,
                bytes: &[u8],
            ) -> PyResult<Self> {
                libparsec_types::$class::try_from(bytes)
                    .map(Self)
                    .map_err(|e| ::pyo3::exceptions::PyValueError::new_err(e.to_string()))
            }

            #[classmethod]
            fn from_hex(
                _cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>,
                hex: &str,
            ) -> PyResult<Self> {
                libparsec_types::$class::from_hex(hex)
                    .map(Self)
                    .map_err(|e| ::pyo3::exceptions::PyValueError::new_err(e.to_string()))
            }

            #[classmethod]
            #[pyo3(name = "new")]
            fn default(_cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>) -> Self {
                Self(libparsec_types::$class::default())
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
    VlobID,
    libparsec_types::VlobID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(VlobID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    BlockID,
    libparsec_types::BlockID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

gen_uuid!(BlockID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    ChunkID,
    libparsec_types::ChunkID,
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
    fn from_block_id(_cls: Bound<'_, PyType>, id: BlockID) -> Self {
        Self(libparsec_types::ChunkID::from(*id.0))
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    SequesterServiceID,
    libparsec_types::SequesterServiceID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(SequesterServiceID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    PKIEnrollmentID,
    libparsec_types::PKIEnrollmentID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(PKIEnrollmentID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    AsyncEnrollmentID,
    libparsec_types::AsyncEnrollmentID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(AsyncEnrollmentID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    GreetingAttemptID,
    libparsec_types::GreetingAttemptID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(GreetingAttemptID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    AccountAuthMethodID,
    libparsec_types::AccountAuthMethodID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(AccountAuthMethodID);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    TOTPOpaqueKeyID,
    libparsec_types::TOTPOpaqueKeyID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(TOTPOpaqueKeyID);

// Other ids

crate::binding_utils::gen_py_wrapper_class_for_id!(
    OrganizationID,
    libparsec_types::OrganizationID,
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
    fn new(organization_id: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(organization_id) = organization_id.extract::<Self>() {
            Ok(organization_id)
        } else if let Ok(organization_id) = organization_id.extract::<&str>() {
            match organization_id.parse::<libparsec_types::OrganizationID>() {
                Ok(organization_id) => Ok(Self(organization_id)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            }
        } else {
            // NOTE: We return `ValueError` instead of `TypeError` to be able to use it with
            // pydantic `PlainValidator`
            Err(PyValueError::new_err(format!(
                "Does not support converting {} to OrganizationID",
                organization_id.get_type().name()?.to_str()?
            )))
        }
    }

    #[getter]
    fn str(&self) -> &str {
        self.0.as_ref()
    }

    #[classmethod]
    #[pyo3(name = "__get_pydantic_core_schema__")]
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        _source_type: &Bound<'_, PyType>,
        _handler: &Bound<'_, PyAny>,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let core_schema = py.import("pydantic_core")?.getattr("core_schema")?;

        let str_schema = core_schema.call_method0("str_schema")?;

        // Indicate to pydantic that it just need to call `str(val)` to serialize the value
        let kwargs = PyDict::new(py);
        let serialization = core_schema.call_method0("to_string_ser_schema")?;
        kwargs.set_item("serialization", serialization)?;

        // Create a string schema to load the value from string.
        let from_str = core_schema.call_method(
            "no_info_after_validator_function",
            (cls, str_schema),
            Some(&kwargs),
        )?;

        // Make pydantic accept own class as value
        let instance_schema = core_schema.call_method1("is_instance_schema", (cls,))?;

        // Build schema that accept both string or instance value.
        let union_schema = core_schema.call_method1(
            "union_schema",
            (PyList::new(py, vec![instance_schema, from_str])?,),
        )?;

        Ok(union_schema)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    UserID,
    libparsec_types::UserID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(UserID);

// Helper stuff to be able to use `alice`, `bob` etc. as a user ID in tests
#[cfg(feature = "test-utils")]
#[pymethods]
impl UserID {
    #[getter]
    fn test_nickname(&self) -> Option<&str> {
        self.0.test_nickname()
    }
    #[staticmethod]
    fn test_from_nickname(nickname: &str) -> PyResult<Self> {
        libparsec_types::UserID::test_from_nickname(nickname)
            .map_err(PyValueError::new_err)
            .map(Self)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    DeviceLabel,
    libparsec_types::DeviceLabel,
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
    fn new(device_label: Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(device_label) = device_label.extract::<Self>() {
            Ok(device_label)
        } else if let Ok(device_label) = device_label.extract::<&str>() {
            match device_label.parse::<libparsec_types::DeviceLabel>() {
                Ok(device_label) => Ok(Self(device_label)),
                Err(err) => Err(PyValueError::new_err(err.to_string())),
            }
        } else {
            Err(PyValueError::new_err("Unimplemented"))
        }
    }

    #[staticmethod]
    fn new_redacted(device_id: DeviceID) -> Self {
        Self(libparsec_types::DeviceLabel::new_redacted(device_id.0))
    }

    #[getter]
    fn str(&self) -> &str {
        self.0.as_ref()
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    DeviceID,
    libparsec_types::DeviceID,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);
gen_uuid!(DeviceID);

// Helper stuff to be able to use `alice@dev1`, `bob@dev2` etc. as a device ID in tests
#[cfg(feature = "test-utils")]
#[pymethods]
impl DeviceID {
    #[getter]
    fn test_nickname(&self) -> Option<&str> {
        self.0.test_nickname()
    }
    #[staticmethod]
    fn test_from_nickname(nickname: &str) -> PyResult<Self> {
        libparsec_types::DeviceID::test_from_nickname(nickname)
            .map_err(PyValueError::new_err)
            .map(Self)
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    HumanHandle,
    libparsec_types::HumanHandle,
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
    pub fn new(email: EmailAddress, label: &str) -> PyResult<Self> {
        match libparsec_types::HumanHandle::new(email.0, label) {
            Ok(human_handle) => Ok(Self(human_handle)),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[staticmethod]
    fn new_redacted(user_id: UserID) -> Self {
        Self(libparsec_types::HumanHandle::new_redacted(user_id.0))
    }

    #[getter]
    fn str(&self) -> &str {
        self.0.as_ref()
    }

    #[getter]
    fn email(&self) -> EmailAddress {
        EmailAddress(self.0.email().clone())
    }

    #[getter]
    fn label(&self) -> &str {
        self.0.label()
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    EmailAddress,
    libparsec_types::EmailAddress,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl EmailAddress {
    #[new]
    pub fn new(raw: &str) -> PyResult<Self> {
        libparsec_types::EmailAddress::from_str(raw)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[getter]
    fn str(&self) -> String {
        self.0.to_string()
    }
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    AccessToken,
    libparsec_types::AccessToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ eq,
    __hash__,
);

#[pymethods]
impl AccessToken {
    #[classmethod]
    fn from_bytes(_cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>, bytes: &[u8]) -> PyResult<Self> {
        libparsec_types::AccessToken::try_from(bytes)
            .map(Self)
            .map_err(|e| ::pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    fn from_hex(_cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>, hex: &str) -> PyResult<Self> {
        libparsec_types::AccessToken::from_hex(hex)
            .map(Self)
            .map_err(|e| ::pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[classmethod]
    #[pyo3(name = "new")]
    fn default(_cls: ::pyo3::Bound<'_, ::pyo3::types::PyType>) -> Self {
        Self(libparsec_types::AccessToken::default())
    }

    #[getter]
    fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[getter]
    fn hex(&self) -> String {
        self.0.hex()
    }
}
