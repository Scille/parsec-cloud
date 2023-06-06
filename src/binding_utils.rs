// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError,
    pyclass::CompareOp,
    types::{PyByteArray, PyBytes, PyTuple},
    FromPyObject, IntoPy, PyObject, PyResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(FromPyObject)]
pub(crate) struct PathWrapper(pub std::path::PathBuf);

impl IntoPy<PyObject> for PathWrapper {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        // Pathlib is part of the standard library
        let pathlib_module = py
            .import("pathlib")
            .expect("import `pathlib` module failed.");
        let path_ctor = pathlib_module
            .getattr("Path")
            .expect("can't get `Path` from `pathlib`.");

        path_ctor
            .call1(PyTuple::new(py, [self.0]))
            .expect("call to `Path` constructor failed.")
            .into_py(py)
    }
}

#[derive(FromPyObject)]
pub enum BytesWrapper<'py> {
    Bytes(&'py PyBytes),
    ByteArray(&'py PyByteArray),
}

impl From<BytesWrapper<'_>> for Vec<u8> {
    fn from(wrapper: BytesWrapper) -> Self {
        match wrapper {
            BytesWrapper::Bytes(bytes) => bytes.as_bytes().to_vec(),
            BytesWrapper::ByteArray(byte_array) => byte_array.to_vec(),
        }
    }
}

impl From<BytesWrapper<'_>> for libparsec::types::Bytes {
    fn from(wrapper: BytesWrapper) -> Self {
        match wrapper {
            BytesWrapper::Bytes(bytes) => bytes.as_bytes().to_owned().into(),
            BytesWrapper::ByteArray(byte_array) => {
                // SAFETY: Using PyByteArray::as_bytes is safe as long as the corresponding memory is not modified.
                // Here, the GIL is held during the entire access to `bytes` so there is no risk of another
                // python thread modifying the bytearray behind our back.
                unsafe { byte_array.as_bytes() }.to_owned().into()
            }
        }
    }
}

pub trait UnwrapBytesWrapper {
    type ResultType;
    fn unwrap_bytes(self) -> Self::ResultType;
}

impl UnwrapBytesWrapper for BytesWrapper<'_> {
    type ResultType = libparsec::types::Bytes;
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

macro_rules! py_object {
    ($_self: ident, $class: path, $subclass: path, $py: ident) => {{
        let initializer = ::pyo3::PyClassInitializer::from(($subclass, $class($_self)));
        // SAFETY: `PyObjectInit::into_new_object` requires `subtype` used to generate a new object to be the same type
        // or a sub-type of `T` (the type of `initializer` here).
        // Here `initializer` is created using the type `<$subclass>` and the same type of `<$subclass>`
        // will be used as the type of `subtype` in the call of `into_new_object`.
        unsafe {
            use ::pyo3::{pyclass_init::PyObjectInit, PyTypeInfo};
            initializer.into_new_object($py, { use $subclass as base; base::type_object_raw($py) })
                .map(|ptr| ::pyo3::PyObject::from_owned_ptr($py, ptr))
        }
    }};
}

macro_rules! parse_kwargs_optional {
    ($kwargs: ident $(,[$var: ident $(:$ty: ty)?, $name: literal $(,$function: ident)?])* $(,)?) => {
        $(let mut $var = None;)*
        if let Some($kwargs) = $kwargs {
            for arg in $kwargs {
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

macro_rules! check_mandatory_kwargs {
    ($([$var: ident, $name: literal]),* $(,)?) => {
        $(let $var = $var.expect(concat!("Missing `", stringify!($name), "` argument"));)*
    };
}

macro_rules! parse_kwargs {
    ($kwargs: ident $(,[$var: ident $(:$ty: ty)?, $name: literal $(,$function: ident)?])* $(,)?) => {
        crate::binding_utils::parse_kwargs_optional!(
            $kwargs,
            $([$var $(:$ty)?, $name $(,$function)?],)*
        );
        crate::binding_utils::check_mandatory_kwargs!($([$var, $name],)*);
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

macro_rules! create_exception_from {
    ($name: ident, $py_exc: ident, $rs_err: path) => {
        ::paste::paste! {
            ::pyo3::create_exception!(_parsec, [<$name Error>], $py_exc);

            pub(crate) struct [<$name Exc>](Box<$rs_err>);

            impl From<[<$name Exc>]> for ::pyo3::PyErr {
                fn from(err: [<$name Exc>]) -> Self {
                    <[<$name Error>]>::new_err(err.0.to_string())
                }
            }

            impl From<$rs_err> for [<$name Exc>] {
                fn from(err: $rs_err) -> Self {
                    Self(Box::new(err))
                }
            }

            impl From<Box<$rs_err>> for [<$name Exc>] {
                fn from(err: Box<$rs_err>) -> Self {
                    Self(err)
                }
            }
        }
    };
}

macro_rules! create_exception {
    ($name: ident, $py_exc: ident, $rs_err: path) => {
        ::paste::paste! {
            crate::binding_utils::create_exception_from!($name, $py_exc, $rs_err);
            pub(crate) type [<$name Result>]<T> = Result<T, [<$name Exc>]>;
        }
    };
    ($name: ident, $py_exc: ident, $rs_err: path, no_result_type) => {
        ::paste::paste! {
            crate::binding_utils::create_exception_from!($name, $py_exc, $rs_err);
        }
    };
}

macro_rules! impl_enum_field {
    ($enum_class: ident, $([$pyo3_name: literal, $fn_name: ident, $field_value: expr]),+) => {
        #[pymethods]
        impl $enum_class {
            $(
                #[classattr]
                #[pyo3(name = $pyo3_name)]
                pub(crate) fn $fn_name() -> &'static PyObject {
                    lazy_static::lazy_static! {
                        static ref VALUE: PyObject = {
                            Python::with_gil(|py| {
                                $enum_class($field_value).into_py(py)
                            })
                        };
                    };

                    &VALUE
                }
            )*

            #[classattr]
            #[pyo3(name = "VALUES")]
            fn values() -> &'static PyObject {
                lazy_static::lazy_static! {
                    static ref VALUES: PyObject = {
                        Python::with_gil(|py| {
                            PyTuple::new(py, [
                                $(
                                    $enum_class :: $fn_name ()
                                ),*
                            ]).into_py(py)
                        })
                    };
                };

                &VALUES
            }

            #[classmethod]
            fn from_str(_cls: &PyType, value: &str) -> PyResult<&'static PyObject> {
                Ok(match value {
                    $($pyo3_name => Self:: $fn_name ()),*,
                    _ => return Err(PyValueError::new_err(format!("Invalid value `{}`", value))),
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

pub(crate) use _unwrap_bytes;
pub(crate) use check_mandatory_kwargs;
pub(crate) use create_exception;
pub(crate) use create_exception_from;
pub(crate) use gen_proto;
pub(crate) use impl_enum_field;
pub(crate) use parse_kwargs;
pub(crate) use parse_kwargs_optional;
pub(crate) use py_object;
pub(crate) use unwrap_bytes;
