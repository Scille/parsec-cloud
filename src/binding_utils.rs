// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyNotImplementedError, pyclass::CompareOp, types::PyFrozenSet, FromPyObject, PyAny,
    PyResult,
};
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

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

// This implementation is due to
// https://github.com/PyO3/pyo3/blob/39d2b9d96476e6cc85ca43e720e035e0cdff7a45/src/types/set.rs#L240
// where HashSet is PySet in FromPyObject trait
pub fn py_to_rs_set<'a, T: FromPyObject<'a> + Eq + Hash>(set: &'a PyAny) -> PyResult<HashSet<T>> {
    set.downcast::<PyFrozenSet>()?
        .iter()
        .map(T::extract)
        .collect::<PyResult<std::collections::HashSet<T>>>()
}

macro_rules! py_object {
    ($_self: ident, $subclass: ident, $py: ident) => {{
        let initializer = PyClassInitializer::from(($subclass, Self($_self)));
        // SAFETY: `PyObjectInit::into_new_object` requires `subtype` used to generate a new object to be the same type
        // or a sub-type of `T` (the type of `initializer` here).
        // Here `initializer` is created using the type `<$subclass>` and the same type of `<$subclass>`
        // will be used as the type of `subtype` in the call of `into_new_object`.
        unsafe {
            let ptr = initializer.into_new_object($py, $subclass::type_object_raw($py))?;
            PyObject::from_owned_ptr($py, ptr)
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
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! parse_kwargs {
    ($kwargs: ident $(,[$var: ident $(:$ty: ty)?, $name: literal $(,$function: ident)?])* $(,)?) => {
        crate::binding_utils::parse_kwargs_optional!(
            $kwargs,
            $([$var $(:$ty)?, $name $(,$function)?],)*
        );
        $(let $var = $var.expect(concat!("Missing `", stringify!($name), "` argument"));)*
    };
}

macro_rules! gen_proto {
    ($class: ident, __repr__) => {
        #[pymethods]
        impl $class {
            fn __repr__(&self) -> ::pyo3::PyResult<String> {
                Ok(format!("{:?}", self.0))
            }
        }
    };
    ($class: ident, __repr__, pyref) => {
        #[pymethods]
        impl $class {
            fn __repr__(_self: PyRef<'_, Self>) -> ::pyo3::PyResult<String> {
                Ok(format!("{:?}", _self.as_ref().0))
            }
        }
    };
    ($class: ident, __str__) => {
        #[pymethods]
        impl $class {
            fn __str__(&self) -> ::pyo3::PyResult<String> {
                Ok(self.0.to_string())
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

macro_rules! create_exception {
    ($name: ident, $py_exc: ident, $rs_err: path) => {
        ::paste::paste! {
            ::pyo3::create_exception!(_parsec, [<$name Error>], $py_exc);

            pub(crate) struct [<$name Exc>]($rs_err);

            impl From<[<$name Exc>]> for ::pyo3::PyErr {
                fn from(err: [<$name Exc>]) -> Self {
                    <[<$name Error>]>::new_err(err.0.to_string())
                }
            }

            impl From<$rs_err> for [<$name Exc>] {
                fn from(err: $rs_err) -> Self {
                    Self(err)
                }
            }

            pub(crate) type [<$name Result>]<T> = Result<T, [<$name Exc>]>;
        }
    };
}

pub(crate) use create_exception;
pub(crate) use gen_proto;
pub(crate) use parse_kwargs;
pub(crate) use parse_kwargs_optional;
pub(crate) use py_object;
