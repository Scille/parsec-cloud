// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

macro_rules! pydantic_json_schema {
    ($class:ident, $($key:ident=$value:expr),+) => {
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

        /// Tell pydantic that the object is like a string.
        pub fn str_schema(&self) -> PyResult<CoreSchema<'py>> {
            self.0.call_method0("str_schema").map(CoreSchema)
        }

        /// Tell pydantic that the object is like a int
        pub fn int_schema(&self) -> PyResult<CoreSchema<'py>> {
            self.0.call_method0("int_schema").map(CoreSchema)
        }

        /// Allow the provided schema to be nullable
        ///
        /// Equivalent to the following Python type `T | None` or `Option<T>`
        pub fn nullable_schema(&self, schema: CoreSchema<'py>) -> PyResult<CoreSchema<'py>> {
            self.0
                .call_method1("nullable_schema", (schema,))
                .map(CoreSchema)
        }

        /// Tell pydantic to call `__str__` when serializing the value.
        pub fn to_string_ser_schema(&self) -> PyResult<CoreSchema<'py>> {
            self.0.call_method0("to_string_ser_schema").map(CoreSchema)
        }

        /// Identity schema, allow own value to be used.
        pub fn instance_schema(&self, cls: &Bound<'py, PyType>) -> PyResult<CoreSchema<'py>> {
            self.0
                .call_method1("is_instance_schema", (cls,))
                .map(CoreSchema)
        }

        /// Combine multiple schemas into one.
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

        /// Define a simple serializer that call the provided function to serialize the value
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

        /// Define a validator that use the provided function to deserialize the value
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
        #[expect(dead_code)]
        UnlessNone,
        /// Use when serializing to JSON
        #[expect(dead_code)]
        Json,
        /// Use when serializing to JSON and the value is not None
        #[expect(dead_code)]
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

macro_rules! str_like_validator {
    ($core_schema:ident, $cls:ident, ser=$serializer:expr, der=$deserializer:expr, $py:ident) => {{
        let str_schema = $core_schema.str_schema()?;

        $crate::pydantic_support::validator!(
            $core_schema,
            $cls,
            schema = str_schema,
            ser = $serializer,
            der = $deserializer,
            $py
        )
    }};
}

pub(crate) use str_like_validator;

macro_rules! validator {
    ($core_schema:ident, $cls:ident, schema=$schema:expr, ser=$serializer:expr, der=$deserializer:expr, $py:ident) => {{
        use ::pyo3::IntoPyObject as _;

        let validator = $core_schema.no_info_after_validator_function(
            $deserializer,
            $schema,
            Some($serializer),
            $py,
        )?;
        let instance_schema = $core_schema.instance_schema($cls)?;
        let union_schema = $core_schema.union_schema([instance_schema, validator], $py)?;

        union_schema.into_pyobject($py).map_err(Into::into)
    }};
}

pub(crate) use validator;

macro_rules! str_like_serializer {
    ($core_schema:ident, $serializer:expr, $py:ident) => {{
        let str_schema = $core_schema.str_schema()?;
        $crate::pydantic_support::serializer!($core_schema, $serializer, Some(str_schema), $py)
    }};
}

pub(crate) use str_like_serializer;

macro_rules! serializer {
    ($core_schema:ident, $serializer:expr, $schema:expr, $py:ident) => {{
        $core_schema.plain_serializer_function_ser_schema(
            $serializer,
            $schema,
            Some($crate::pydantic_support::inner::WhenUsed::Always),
            $py,
        )
    }};
}

pub(crate) use serializer;
