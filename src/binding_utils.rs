// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// TODO: Remove these lines when all functions/macros are used
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use fancy_regex::Regex;
use pyo3::{
    conversion::IntoPy,
    exceptions::{PyNotImplementedError, PyValueError},
    prelude::PyModule,
    pyclass::CompareOp,
    types::{PyFrozenSet, PyTuple},
    FromPyObject, {PyAny, PyObject, PyResult, Python},
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

#[deprecated]
pub(crate) fn hash_generic_legacy(value_to_hash: &str, py: Python) -> PyResult<isize> {
    let builtins = PyModule::import(py, "builtins")?;
    let hash = builtins
        .getattr("hash")?
        .call1((value_to_hash,))?
        .extract::<isize>()?;
    Ok(hash)
}

pub fn rs_to_py_realm_role(role: &libparsec::types::RealmRole) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        let cls = py.import("parsec.api.protocol")?.getattr("RealmRole")?;
        let role_name = match role {
            libparsec::types::RealmRole::Owner => "OWNER",
            libparsec::types::RealmRole::Manager => "MANAGER",
            libparsec::types::RealmRole::Contributor => "CONTRIBUTOR",
            libparsec::types::RealmRole::Reader => "READER",
        };
        let obj = cls.getattr(role_name)?;
        Ok(obj.into_py(py))
    })
}

pub fn py_to_rs_realm_role(role: &PyAny) -> PyResult<Option<libparsec::types::RealmRole>> {
    if role.is_none() {
        return Ok(None);
    }
    use libparsec::types::RealmRole::*;
    Ok(Some(match role.getattr("name")?.extract::<&str>()? {
        "OWNER" => Owner,
        "MANAGER" => Manager,
        "CONTRIBUTOR" => Contributor,
        "READER" => Reader,
        _ => unreachable!(),
    }))
}

pub fn py_to_rs_user_profile(profile: &PyAny) -> PyResult<libparsec::types::UserProfile> {
    use libparsec::types::UserProfile::*;
    Ok(match profile.getattr("name")?.extract::<&str>()? {
        "ADMIN" => Admin,
        "STANDARD" => Standard,
        "OUTSIDER" => Outsider,
        _ => unreachable!(),
    })
}

pub fn rs_to_py_user_profile(profile: &libparsec::types::UserProfile) -> PyResult<PyObject> {
    use libparsec::types::UserProfile::*;
    Python::with_gil(|py| -> PyResult<PyObject> {
        let cls = py.import("parsec.api.protocol")?.getattr("UserProfile")?;
        let profile_name = match profile {
            Admin => "ADMIN",
            Standard => "STANDARD",
            Outsider => "OUTSIDER",
        };
        let obj = cls.getattr(profile_name)?;
        Ok(obj.into_py(py))
    })
}

pub fn py_to_rs_invitation_status(status: &PyAny) -> PyResult<libparsec::types::InvitationStatus> {
    use libparsec::types::InvitationStatus::*;
    Ok(match status.getattr("name")?.extract::<&str>()? {
        "IDLE" => Idle,
        "READY" => Ready,
        "DELETED" => Deleted,
        _ => unreachable!(),
    })
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

pub fn py_to_rs_regex(regex: &PyAny) -> PyResult<Regex> {
    let regex = regex
        .getattr("pattern")
        .unwrap_or(regex)
        .extract::<&str>()?
        .replace("\\Z", "\\z")
        .replace("\\ ", "\x20");
    Regex::new(&regex).map_err(|e| PyValueError::new_err(e.to_string()))
}

pub fn rs_to_py_regex<'py>(py: Python<'py>, regex: &Regex) -> PyResult<&'py PyAny> {
    let re = py.import("re")?;
    let args = PyTuple::new(
        py,
        vec![regex.as_str().replace("\\z", "\\Z").replace('\x20', "\\ ")],
    );
    re.call_method1("compile", args)
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
}

pub(crate) use gen_proto;
pub(crate) use parse_kwargs;
pub(crate) use parse_kwargs_optional;
