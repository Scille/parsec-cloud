// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3::types::{PyList, PySet, PyTuple};

use libparsec::core_fs::file_operations;

use crate::ids::ChunkID;
use crate::local_manifest::{Chunk, LocalFileManifest};
use crate::time::DateTime;

// Conversion helpers

fn to_py_chunks(py: Python, chunks: Vec<libparsec::client_types::Chunk>) -> &PyTuple {
    PyTuple::new(py, chunks.into_iter().map(|x| Chunk(x).into_py(py)))
}

fn to_py_removed_ids(
    py: Python,
    to_remove: HashSet<libparsec::types::ChunkID>,
) -> PyResult<&PySet> {
    PySet::new(
        py,
        &to_remove
            .iter()
            .map(|x| ChunkID(*x).into_py(py))
            .collect::<Vec<_>>(),
    )
}

fn to_py_write_operations(
    py: Python,
    write_operations: Vec<(libparsec::client_types::Chunk, i64)>,
) -> &PyList {
    PyList::new(
        py,
        write_operations
            .into_iter()
            .map(|(x, y)| (Chunk(x).into_py(py), y)),
    )
}

// Exposed functions

#[pyfunction]
pub(crate) fn prepare_read(
    py: Python,
    manifest: LocalFileManifest,
    size: u64,
    offset: u64,
) -> PyResult<&PyTuple> {
    let result = file_operations::prepare_read(&manifest.0, size, offset);
    Ok(to_py_chunks(py, result))
}

#[pyfunction]
pub(crate) fn prepare_write(
    py: Python<'_>,
    mut manifest: LocalFileManifest,
    size: u64,
    offset: u64,
    timestamp: DateTime,
) -> PyResult<&PyTuple> {
    let (write_operations, to_remove) =
        file_operations::prepare_write(&mut manifest.0, size, offset, timestamp.0);
    Ok(PyTuple::new(
        py,
        vec![
            LocalFileManifest(manifest.0).into_py(py),
            to_py_write_operations(py, write_operations).into_py(py),
            to_py_removed_ids(py, to_remove)?.into_py(py),
        ],
    ))
}

#[pyfunction]
pub(crate) fn prepare_resize(
    py: Python<'_>,
    mut manifest: LocalFileManifest,
    size: u64,
    timestamp: DateTime,
) -> PyResult<&PyTuple> {
    let (write_operations, to_remove) =
        file_operations::prepare_resize(&mut manifest.0, size, timestamp.0);
    Ok(PyTuple::new(
        py,
        vec![
            LocalFileManifest(manifest.0).into_py(py),
            to_py_write_operations(py, write_operations).into_py(py),
            to_py_removed_ids(py, to_remove)?.into_py(py),
        ],
    ))
}

#[pyfunction]
pub(crate) fn prepare_reshape(py: Python, manifest: LocalFileManifest) -> PyResult<&PyList> {
    let iterator = file_operations::prepare_reshape(&manifest.0);
    let collected: Vec<_> = iterator
        .map(|(block, old_chunks, new_chunk, write_back, to_remove)| {
            Ok(PyTuple::new(
                py,
                vec![
                    block.into_py(py),
                    to_py_chunks(py, old_chunks).into_py(py),
                    Chunk(new_chunk).into_py(py),
                    write_back.into_py(py),
                    to_py_removed_ids(py, to_remove)?.into_py(py),
                ],
            ))
        })
        .collect::<PyResult<_>>()?;
    Ok(PyList::new(py, collected))
}
