// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    pyclass::CompareOp,
    types::{PyAnyMethods, PyByteArray, PyByteArrayMethods, PyBytes, PyBytesMethods},
    Bound, FromPyObject, PyAny, PyResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(FromPyObject)]
pub enum BytesWrapper<'py> {
    Bytes(Bound<'py, PyBytes>),
    ByteArray(Bound<'py, PyByteArray>),
}

impl From<BytesWrapper<'_>> for Vec<u8> {
    fn from(wrapper: BytesWrapper) -> Self {
        match wrapper {
            BytesWrapper::Bytes(bytes) => bytes.as_bytes().to_vec(),
            BytesWrapper::ByteArray(byte_array) => byte_array.to_vec(),
        }
    }
}

impl From<BytesWrapper<'_>> for libparsec_types::Bytes {
    fn from(wrapper: BytesWrapper) -> Self {
        <Vec<u8>>::from(wrapper).into()
    }
}

pub trait UnwrapBytesWrapper {
    type ResultType;
    fn unwrap_bytes(self) -> Self::ResultType;
}

impl UnwrapBytesWrapper for BytesWrapper<'_> {
    type ResultType = libparsec_types::Bytes;
    fn unwrap_bytes(self) -> Self::ResultType {
        self.into()
    }
}

impl<T> UnwrapBytesWrapper for Option<T>
where
    T: UnwrapBytesWrapper,
{
    type ResultType = Option<T::ResultType>;
    fn unwrap_bytes(self) -> Self::ResultType {
        self.map(|x| x.unwrap_bytes())
    }
}

impl<T> UnwrapBytesWrapper for Vec<T>
where
    T: UnwrapBytesWrapper,
{
    type ResultType = Vec<T::ResultType>;
    fn unwrap_bytes(self) -> Self::ResultType {
        self.into_iter().map(|x| x.unwrap_bytes()).collect()
    }
}

macro_rules! _unwrap_bytes {
    ($name:ident) => {
        let $name = $name.unwrap_bytes();
    };
    ($x:ident, $($y:ident),+) => {
        crate::binding_utils::_unwrap_bytes!($x);
        crate::binding_utils::_unwrap_bytes!($($y),+);
    }
}

macro_rules! unwrap_bytes {
    ($($name:ident),+) => {
        use crate::binding_utils::UnwrapBytesWrapper;
        crate::binding_utils::_unwrap_bytes!($($name),+);
    }
}

pub fn comp_eq<T: std::cmp::PartialEq>(op: CompareOp, h1: T, h2: T) -> PyResult<bool> {
    Ok(match op {
        CompareOp::Eq => h1 == h2,
        CompareOp::Ne => h1 != h2,
        _ => return Err(PyNotImplementedError::new_err("")),
    })
}

pub fn comp_ord<T: std::cmp::PartialOrd>(op: CompareOp, h1: T, h2: T) -> PyResult<bool> {
    Ok(match op {
        CompareOp::Eq => h1 == h2,
        CompareOp::Ne => h1 != h2,
        CompareOp::Lt => h1 < h2,
        CompareOp::Le => h1 <= h2,
        CompareOp::Gt => h1 > h2,
        CompareOp::Ge => h1 >= h2,
    })
}

pub(crate) fn hash_generic<T: Hash>(value_to_hash: T) -> PyResult<u64> {
    let mut s = DefaultHasher::new();
    value_to_hash.hash(&mut s);
    Ok(s.finish())
}

pub(crate) fn bytes_from_any<'a, 'py>(
    // data: Borrowed<'a, 'py, PyAny>,
    data: &'a Bound<'py, PyAny>,
) -> Result<&'a [u8], pyo3::DowncastError<'a, 'py>>
where
    'py: 'a,
{
    data.downcast::<PyBytes>()
        .map(|b| b.as_bytes())
        .or_else(|_| {
            data.downcast::<PyByteArray>().map(|b|
                // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
                // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
                // python thread modifying the bytearray behind our back.
                unsafe { b.as_bytes() })
        })
}

macro_rules! parse_kwargs_optional {
    ($kwargs: ident $(,[$var: ident $(:$ty: ty)?, $name: literal $(,$function: ident)?])* $(,)?) => {
        $(let mut $var = None;)*
        if let Some($kwargs) = $kwargs {
            for arg in $kwargs {
                use pyo3::prelude::PyAnyMethods;
                match arg.0.extract::<&str>()? {
                    $($name => $var = {
                        let temp = arg.1;
                        $(let temp = temp.extract::<$ty>()?;)?
                        $(let temp = $function(&temp)?;)?
                        Some(temp)
                    },)*
                    name => panic!("unexpected param `{}`", name),
                }
            }
        }
    };
}

macro_rules! gen_py_wrapper_class {
    ($class: ident, $wrapped_struct: path $(,$magic_meth: ident $($magic_meth_arg: ident)? )* $(,)?) => {
        #[pyclass]
        #[derive(Clone)]
        pub(crate) struct $class(pub $wrapped_struct);

        impl From<$wrapped_struct> for $class {
            fn from(value: $wrapped_struct) -> Self {
                Self(value)
            }
        }

        $(crate::binding_utils::gen_proto!($class, $magic_meth $(,$magic_meth_arg)?);)*
    };
}

macro_rules! gen_py_wrapper_class_for_id {
    ($class: ident, $wrapped_struct: path $(,$magic_meth: ident $($magic_meth_arg: ident)? )* $(,)?) => {
        #[pyclass]
        #[derive(Clone, Eq, PartialEq, Hash)]
        pub(crate) struct $class(pub $wrapped_struct);

        impl From<$wrapped_struct> for $class {
            fn from(value: $wrapped_struct) -> Self {
                Self(value)
            }
        }

        $(crate::binding_utils::gen_proto!($class, $magic_meth $(,$magic_meth_arg)?);)*
    };
}

macro_rules! gen_proto {
    ($class: ident, __repr__) => {
        #[pymethods]
        impl $class {
            fn __repr__(&self) -> String {
                format!("{:?}", self.0)
            }
        }
    };
    ($class: ident, __repr__, pyref) => {
        #[pymethods]
        impl $class {
            fn __repr__(_self: PyRef<'_, Self>) -> String {
                format!("{:?}", _self.as_ref().0)
            }
        }
    };
    ($class: ident, __str__) => {
        #[pymethods]
        impl $class {
            fn __str__(&self) -> String {
                self.0.to_string()
            }
        }
    };
    ($class: ident, __richcmp__, eq) => {
        #[pymethods]
        impl $class {
            fn __richcmp__(
                &self,
                other: &Self,
                op: ::pyo3::pyclass::CompareOp,
            ) -> ::pyo3::PyResult<bool> {
                crate::binding_utils::comp_eq(op, &self.0, &other.0)
            }
        }
    };
    ($class: ident, __richcmp__, ord) => {
        #[pymethods]
        impl $class {
            fn __richcmp__(
                &self,
                other: &Self,
                op: ::pyo3::pyclass::CompareOp,
            ) -> ::pyo3::PyResult<bool> {
                crate::binding_utils::comp_ord(op, &self.0, &other.0)
            }
        }
    };
    ($class: ident, __hash__) => {
        #[pymethods]
        impl $class {
            fn __hash__(&self) -> ::pyo3::PyResult<u64> {
                crate::binding_utils::hash_generic(&self.0)
            }
        }
    };
    ($class: ident, __copy__) => {
        #[pymethods]
        impl $class {
            fn __copy__(&self) -> Self {
                Self(self.0.clone())
            }
        }
    };
    ($class: ident, __deepcopy__) => {
        #[pymethods]
        impl $class {
            fn __deepcopy__(&self, _memo: ::pyo3::PyObject) -> Self {
                Self(self.0.clone())
            }
        }
    };
}

macro_rules! gen_py_wrapper_class_for_enum {
    ($class: ident, $wrapped_enum: ty $(,[$pyo3_name: literal, $fn_name: ident, $field_value: path])+  $(,)?) => {
        #[pyclass]
        #[derive(Clone, PartialEq, Eq, Hash)]
        // 2nd element is a dummy type to ensure the struct must be constructed
        // using the `convert` method
        pub(crate) struct $class(pub $wrapped_enum);

        crate::binding_utils::gen_proto!($class, __repr__);
        crate::binding_utils::gen_proto!($class, __richcmp__, eq);
        crate::binding_utils::gen_proto!($class, __hash__);
        crate::binding_utils::gen_proto!($class, __copy__);
        crate::binding_utils::gen_proto!($class, __deepcopy__);

        impl $class {
            #[allow(dead_code)]
            pub fn convert(val: $wrapped_enum) -> &'static pyo3::PyObject {
                match val {
                    $($field_value => Self :: $fn_name ()),*
                }
            }
        }

        #[pymethods]
        impl $class {
            $(
                #[classattr]
                #[pyo3(name = $pyo3_name)]
                pub(crate) fn $fn_name() -> &'static pyo3::PyObject {
                    lazy_static::lazy_static! {
                        static ref VALUE: pyo3::PyObject = {
                            use ::pyo3::conversion::IntoPyObjectExt;

                            Python::with_gil(|py| {
                                $class($field_value)
                                    .into_py_any(py)
                                    .expect(std::concat!(
                                        "Cannot create static value for ",
                                        std::stringify!($class::$fn_name)
                                    ))
                            })
                        };
                    };

                    &VALUE
                }
            )*

            #[classattr]
            #[pyo3(name = "VALUES")]
            fn values() -> &'static pyo3::PyObject {
                lazy_static::lazy_static! {
                    static ref VALUES: ::pyo3::PyObject = {
                        Python::with_gil(|py| {
                            use ::pyo3::conversion::IntoPyObjectExt;

                            ::pyo3::types::PyTuple::new(py, [
                                $(
                                    $class :: $fn_name ()
                                ),*
                            ])
                                .and_then(|v| v.into_py_any(py))
                                .expect(std::concat!(
                                    "Cannot create static values for class ",
                                    std::stringify!($class)
                                ))
                        })
                    };
                };

                &VALUES
            }

            #[classmethod]
            fn from_str(_cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>, value: &str) -> pyo3::PyResult<&'static ::pyo3::PyObject> {
                Ok(match value {
                        $($pyo3_name => Self:: $fn_name ()),*,
                    _ => return Err(::pyo3::exceptions::PyValueError::new_err(format!("Invalid value `{}`", value))),
                })
            }


            #[getter]
            fn str(&self) -> &'static str {
                match self.0 {
                    $($field_value => $pyo3_name),*
                }
            }
        }
    };
}

macro_rules! export_exception {
    ($module:expr, $python:expr, $exception:ty) => {
        $module.add(stringify!($exception), $python.get_type::<$exception>())?;
    };
}

pub(crate) use _unwrap_bytes;
pub(crate) use export_exception;
pub(crate) use gen_proto;
pub(crate) use gen_py_wrapper_class;
pub(crate) use gen_py_wrapper_class_for_enum;
pub(crate) use gen_py_wrapper_class_for_id;
pub(crate) use parse_kwargs_optional;
pub(crate) use unwrap_bytes;
