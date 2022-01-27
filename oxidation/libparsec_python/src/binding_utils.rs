// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::basic::CompareOp;
use pyo3::prelude::PyModule;
use pyo3::{PyResult, Python};

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
