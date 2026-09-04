// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

macro_rules! pydantic_json_schema {
    ($class:ident, $($key:ident=$value:expr),+) => {
        #[cfg(feature = "pydantic-support")]
        #[pymethods]
        impl $class {
            #[classmethod]
            #[pyo3(name = "__get_pydantic_json_schema__")]
            fn get_pydantic_json_schema<'py>(
                _cls: &Bound<'_, PyType>,
                _schema: &Bound<'_, PyAny>,
                _handler: &Bound<'_, PyAny>,
                py: Python<'py>,
            ) -> PyResult<Bound<'py, PyDict>> {
                let dict = PyDict::new(py);
                $(
                    dict.set_item(stringify!($key), $value)?;
                )+
                Ok(dict)
            }
        }
    };
}

pub(crate) use pydantic_json_schema;
