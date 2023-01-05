use pyo3::exceptions::{PyException, PyValueError};

crate::binding_utils::create_exception!(Data, PyException, libparsec::types::DataError);
crate::binding_utils::create_exception!(EntryName, PyValueError, libparsec::types::EntryNameError);
