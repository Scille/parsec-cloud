// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::conversion::IntoPy;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyModule;
use pyo3::types::{PyDict, PyFrozenSet, PyTuple};
use pyo3::FromPyObject;
use pyo3::{PyAny, PyObject, PyResult, Python};

pub fn comp_op<T: std::cmp::PartialOrd>(op: CompareOp, h1: T, h2: T) -> PyResult<bool> {
    match op {
        CompareOp::Eq => Ok(h1 == h2),
        CompareOp::Ne => Ok(h1 != h2),
        CompareOp::Lt => Ok(h1 < h2),
        CompareOp::Le => Ok(h1 <= h2),
        CompareOp::Gt => Ok(h1 > h2),
        CompareOp::Ge => Ok(h1 >= h2),
    }
}

pub fn hash_generic(value_to_hash: &str, py: Python) -> PyResult<isize> {
    let builtins = PyModule::import(py, "builtins")?;
    let hash = builtins
        .getattr("hash")?
        .call1((value_to_hash,))?
        .extract::<isize>()?;
    Ok(hash)
}

pub fn py_to_rs_datetime(timestamp: &PyAny) -> PyResult<parsec_api_types::DateTime> {
    let ts_any =
        Python::with_gil(|_py| -> PyResult<&PyAny> { timestamp.getattr("timestamp")?.call0() })?;
    let ts = ts_any.extract::<f64>()?;
    Ok(parsec_api_types::DateTime::from_f64_with_us_precision(ts))
}

pub fn rs_to_py_datetime(py: Python, datetime: parsec_api_types::DateTime) -> PyResult<&PyAny> {
    let pendulum = PyModule::import(py, "pendulum")?;
    let args = PyTuple::new(py, vec![datetime.get_f64_with_us_precision()]);
    pendulum.call_method1("from_timestamp", args)
}

pub fn rs_to_py_realm_role(role: &parsec_api_types::RealmRole) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        let cls = py.import("parsec.api.protocol")?.getattr("RealmRole")?;
        let role_name = match role {
            parsec_api_types::RealmRole::Owner => "OWNER",
            parsec_api_types::RealmRole::Manager => "MANAGER",
            parsec_api_types::RealmRole::Contributor => "CONTRIBUTOR",
            parsec_api_types::RealmRole::Reader => "READER",
        };
        let obj = cls.getattr(role_name)?;
        Ok(obj.into_py(py))
    })
}

pub fn py_to_rs_realm_role(role: &PyAny) -> PyResult<parsec_api_types::RealmRole> {
    let role = match role.getattr("name")?.extract::<&str>()? {
        "OWNER" => parsec_api_types::RealmRole::Owner,
        "MANAGER" => parsec_api_types::RealmRole::Manager,
        "CONTRIBUTOR" => parsec_api_types::RealmRole::Contributor,
        "READER" => parsec_api_types::RealmRole::Reader,
        _ => unreachable!(),
    };
    Ok(role)
}

pub fn kwargs_extract_required<'a, T: FromPyObject<'a>>(
    args: &'a PyDict,
    name: &str,
) -> PyResult<T> {
    match kwargs_extract_optional::<T>(args, name)? {
        Some(v) => Ok(v),
        None => Err(PyValueError::new_err(format!(
            "Missing `{}` argument",
            name
        ))),
    }
}

pub fn kwargs_extract_optional<'a, T: FromPyObject<'a>>(
    args: &'a PyDict,
    name: &str,
) -> PyResult<Option<T>> {
    match args.get_item(name) {
        Some(item) if item.is_none() => Ok(None),
        Some(item) => match item.extract::<T>() {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(PyValueError::new_err(format!(
                "Invalid `{}` argument: {}",
                name, err
            ))),
        },
        None => Ok(None),
    }
}

pub fn kwargs_extract_required_custom<T>(
    args: &PyDict,
    name: &str,
    converter: &dyn Fn(&PyAny) -> PyResult<T>,
) -> PyResult<T> {
    match kwargs_extract_optional_custom(args, name, converter)? {
        Some(v) => Ok(v),
        None => Err(PyValueError::new_err(format!(
            "Missing `{}` argument",
            name
        ))),
    }
}

pub fn kwargs_extract_optional_custom<T>(
    args: &PyDict,
    name: &str,
    converter: &dyn Fn(&PyAny) -> PyResult<T>,
) -> PyResult<Option<T>> {
    match args.get_item(name) {
        Some(item) => match converter(item) {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(PyValueError::new_err(format!(
                "Invalid `{}` argument: {}",
                name, err
            ))),
        },
        None => Ok(None),
    }
}

// This implementation is due to
// https://github.com/PyO3/pyo3/blob/39d2b9d96476e6cc85ca43e720e035e0cdff7a45/src/types/set.rs#L240
// where Hashset is PySet in FromPyObject trait
pub fn kwargs_extract_required_frozenset<'a, T: FromPyObject<'a> + Eq + std::hash::Hash>(
    args: &'a PyDict,
    name: &str,
) -> PyResult<std::collections::HashSet<T>> {
    match kwargs_extract_optional_frozenset::<T>(args, name)? {
        Some(v) => Ok(v),
        None => Err(PyValueError::new_err(format!(
            "Missing `{}` argument",
            name
        ))),
    }
}

pub fn kwargs_extract_optional_frozenset<'a, T: FromPyObject<'a> + Eq + std::hash::Hash>(
    args: &'a PyDict,
    name: &str,
) -> PyResult<Option<std::collections::HashSet<T>>> {
    match args.get_item(name) {
        Some(item) if item.is_none() => Ok(None),
        Some(item) => match item.downcast::<PyFrozenSet>() {
            Ok(set) => Ok(Some(
                set.iter()
                    .map(T::extract)
                    .collect::<PyResult<std::collections::HashSet<T>>>()?,
            )),
            Err(err) => Err(PyValueError::new_err(format!(
                "Invalid `{}` argument: {}",
                name, err
            ))),
        },
        None => Ok(None),
    }
}

pub fn pattern_match(prevent_sync_pattern: &PyAny, pattern: &str) -> PyResult<bool> {
    prevent_sync_pattern
        .call_method1("match", (pattern,))?
        .extract::<Option<PyObject>>()
        .map(|b| b.is_some())
}
