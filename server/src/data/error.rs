// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::{PyException, PyValueError},
    PyErr,
};

crate::binding_utils::create_exception!(Data, PyException, libparsec::low_level::types::DataError);
crate::binding_utils::create_exception!(
    EntryName,
    PyValueError,
    libparsec::low_level::types::EntryNameError
);

// PkiEnrollmentError

pyo3::create_exception!(_parsec, PkiEnrollmentError, PyException);

// PkiEnrollmentLocalPendingError

pyo3::create_exception!(_parsec, PkiEnrollmentLocalPendingError, PkiEnrollmentError);
pyo3::create_exception!(
    _parsec,
    PkiEnrollmentLocalPendingCannotReadError,
    PkiEnrollmentLocalPendingError
);
pyo3::create_exception!(
    _parsec,
    PkiEnrollmentLocalPendingCannotRemoveError,
    PkiEnrollmentLocalPendingError
);
pyo3::create_exception!(
    _parsec,
    PkiEnrollmentLocalPendingCannotSaveError,
    PkiEnrollmentLocalPendingError
);
pyo3::create_exception!(
    _parsec,
    PkiEnrollmentLocalPendingValidationError,
    PkiEnrollmentLocalPendingError
);

pub(crate) struct PkiEnrollmentLocalPendingExc(
    libparsec::low_level::types::PkiEnrollmentLocalPendingError,
);

impl From<PkiEnrollmentLocalPendingExc> for PyErr {
    fn from(err: PkiEnrollmentLocalPendingExc) -> Self {
        use libparsec::low_level::types::PkiEnrollmentLocalPendingError::*;
        match err.0 {
            CannotRead { .. } => {
                PkiEnrollmentLocalPendingCannotReadError::new_err(err.0.to_string())
            }
            CannotRemove { .. } => {
                PkiEnrollmentLocalPendingCannotRemoveError::new_err(err.0.to_string())
            }
            CannotSave { .. } => {
                PkiEnrollmentLocalPendingCannotSaveError::new_err(err.0.to_string())
            }
            Validation { .. } => {
                PkiEnrollmentLocalPendingValidationError::new_err(err.0.to_string())
            }
        }
    }
}

impl From<libparsec::low_level::types::PkiEnrollmentLocalPendingError>
    for PkiEnrollmentLocalPendingExc
{
    fn from(err: libparsec::low_level::types::PkiEnrollmentLocalPendingError) -> Self {
        PkiEnrollmentLocalPendingExc(err)
    }
}

pub(crate) type PkiEnrollmentLocalPendingResult<T> = Result<T, PkiEnrollmentLocalPendingExc>;
