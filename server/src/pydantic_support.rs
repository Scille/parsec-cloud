// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

macro_rules! pydantic_json_schema {
    ($class:ident, $($key:ident=$value:expr),+) => {
        #[cfg(feature = "pydantic-support")]
        #[pymethods]
        impl $class {
            #[classmethod]
            #[pyo3(name = "__get_pydantic_json_schema__")]
            fn get_pydantic_json_schema<'py>(
                _cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>,
                _schema: &::pyo3::Bound<'_, ::pyo3::types::PyAny>,
                _handler: &::pyo3::Bound<'_, ::pyo3::types::PyAny>,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyDict>> {
                let dict = ::pyo3::types::PyDict::new(py);
                $(
                    dict.set_item(stringify!($key), $value)?;
                )+
                Ok(dict)
            }
        }
    };
}

pub(crate) use pydantic_json_schema;

#[cfg(feature = "pydantic-support")]
pub mod inner {
    use pyo3::{
        conversion::IntoPyObject,
        types::{PyAny, PyAnyMethods, PyDict, PyList, PyType},
        Bound, PyResult, Python,
    };

    pub struct CoreSchemaModule<'py>(Bound<'py, PyAny>);

    pub type AnyItem<'py> = Bound<'py, PyAny>;

    impl<'py> CoreSchemaModule<'py> {
        pub fn new(py: Python<'py>) -> PyResult<Self> {
            py.import("pydantic_core")
                .and_then(|module| module.getattr("core_schema"))
                .map(Self)
        }

        pub fn str_schema(&self) -> PyResult<CoreSchema<'py>> {
            self.0.call_method0("str_schema").map(CoreSchema)
        }

        pub fn to_string_ser_schema(&self) -> PyResult<CoreSchema<'py>> {
            self.0.call_method0("to_string_ser_schema").map(CoreSchema)
        }

        pub fn instance_schema(&self, cls: &Bound<'py, PyType>) -> PyResult<CoreSchema<'py>> {
            self.0
                .call_method1("is_instance_schema", (cls,))
                .map(CoreSchema)
        }

        pub fn union_schema<T>(
            &self,
            schemas: impl IntoIterator<Item = T>,
            py: Python<'py>,
        ) -> PyResult<CoreSchema<'py>>
        where
            T: IntoPyObject<'py>,
        {
            self.0
                .call_method1("union_schema", (PyList::new(py, schemas)?,))
                .map(CoreSchema)
        }

        pub fn plain_serializer_function_ser_schema(
            &self,
            func: impl IntoPyObject<'py>,
            return_schema: Option<CoreSchema<'py>>,
            when_used: Option<WhenUsed>,
            py: Python<'py>,
        ) -> PyResult<CoreSchema<'py>> {
            let kwargs = PyDict::new(py);
            if let Some(return_schema) = return_schema {
                kwargs.set_item("return_schema", return_schema)?;
            }
            if let Some(when_used) = when_used {
                kwargs.set_item("when_used", when_used.as_str())?;
            }

            self.0
                .call_method(
                    "plain_serializer_function_ser_schema",
                    (func,),
                    Some(&kwargs),
                )
                .map(CoreSchema)
        }

        pub fn no_info_after_validator_function(
            &self,
            func: impl IntoPyObject<'py>,
            schema: CoreSchema<'py>,
            serialization: Option<CoreSchema<'py>>,
            py: Python<'py>,
        ) -> PyResult<CoreSchema<'py>> {
            let kwargs = if let Some(serialization) = serialization {
                let kwargs = PyDict::new(py);
                kwargs.set_item("serialization", serialization)?;
                Some(kwargs)
            } else {
                None
            };
            self.0
                .call_method(
                    "no_info_after_validator_function",
                    (func, schema),
                    kwargs.as_ref(),
                )
                .map(CoreSchema)
        }
    }

    #[derive(Clone)]
    pub struct CoreSchema<'py>(AnyItem<'py>);

    impl<'py> From<CoreSchema<'py>> for AnyItem<'py> {
        fn from(value: CoreSchema<'py>) -> Self {
            value.0
        }
    }

    impl<'py> IntoPyObject<'py> for CoreSchema<'py> {
        type Target = PyAny;

        type Output = Bound<'py, Self::Target>;

        type Error = std::convert::Infallible;

        fn into_pyobject(self, _py: Python<'py>) -> Result<Self::Output, Self::Error> {
            Ok(self.0)
        }
    }

    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    pub enum WhenUsed {
        /// Means always use
        #[default]
        Always,
        /// Use unless the value is None
        UnlessNone,
        /// Use when serializing to JSON
        Json,
        /// Use when serializing to JSON and the value is not None
        JsonUnlessNone,
    }

    impl WhenUsed {
        pub const fn as_str(&self) -> &'static str {
            match self {
                WhenUsed::Always => "always",
                WhenUsed::UnlessNone => "unless-none",
                WhenUsed::Json => "json",
                WhenUsed::JsonUnlessNone => "json-unless-none",
            }
        }
    }
}
