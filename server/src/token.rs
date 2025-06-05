// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::prelude::*;

macro_rules! gen_token {
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
                self.0.as_bytes()
            }

            #[getter]
            fn hex(&self) -> String {
                self.0.hex()
            }
        }
    };
}

crate::binding_utils::gen_py_wrapper_class_for_id!(
    BootstrapToken,
    libparsec_types::BootstrapToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ eq,
    __hash__,
);
gen_token!(BootstrapToken);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    InvitationToken,
    libparsec_types::InvitationToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ eq,
    __hash__,
);
gen_token!(InvitationToken);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    EmailValidationToken,
    libparsec_types::EmailValidationToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ eq,
    __hash__,
);
gen_token!(EmailValidationToken);

crate::binding_utils::gen_py_wrapper_class_for_id!(
    AccountDeletionToken,
    libparsec_types::AccountDeletionToken,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ eq,
    __hash__,
);
gen_token!(AccountDeletionToken);
