// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::prelude::*;
use pyo3::types::{PyList, PySet, PyTuple};

use crate::binding_utils::py_to_rs_datetime;
use crate::ids::ChunkID;
use crate::local_manifest::{Chunk, LocalFileManifest};
use libparsec_fs::file_operations;

#[pyfunction]
pub(crate) fn prepare_read(
    py: Python,
    manifest: LocalFileManifest,
    size: u64,
    offset: u64,
) -> PyResult<&PyTuple> {
    let result = file_operations::prepare_read(&manifest.0, size, offset);
    Ok(PyTuple::new(
        py,
        result.into_iter().map(|x| Chunk(x).into_py(py)),
    ))
}

#[pyfunction]
pub(crate) fn prepare_write<'a>(
    py: Python<'a>,
    manifest: LocalFileManifest,
    size: u64,
    offset: u64,
    timestamp: &PyAny,
) -> PyResult<&'a PyTuple> {
    let (new_manifest, write_operations, to_remove) =
        file_operations::prepare_write(&manifest.0, size, offset, py_to_rs_datetime(timestamp)?);
    Ok(PyTuple::new(
        py,
        vec![
            LocalFileManifest(new_manifest).into_py(py),
            PyList::new(
                py,
                write_operations
                    .into_iter()
                    .map(|(x, y)| (Chunk(x).into_py(py), y)),
            )
            .into_py(py),
            PySet::new(
                py,
                &to_remove
                    .iter()
                    .map(|x| ChunkID(*x).into_py(py))
                    .collect::<Vec<_>>(),
            )?
            .into_py(py),
        ],
    ))
}

#[pyfunction]
pub(crate) fn prepare_truncate<'a>(
    py: Python<'a>,
    manifest: LocalFileManifest,
    size: u64,
    timestamp: &PyAny,
) -> PyResult<&'a PyTuple> {
    let (new_manifest, to_remove) =
        file_operations::prepare_truncate(&manifest.0, size, py_to_rs_datetime(timestamp)?);
    Ok(PyTuple::new(
        py,
        vec![
            LocalFileManifest(new_manifest).into_py(py),
            PySet::new(
                py,
                &to_remove
                    .iter()
                    .map(|x| ChunkID(*x).into_py(py))
                    .collect::<Vec<_>>(),
            )?
            .into_py(py),
        ],
    ))
}
