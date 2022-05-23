// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use crate::local_manifest::{Chunk, LocalFileManifest};
use libparsec_fs::file_operations;

#[pyfunction]
pub(crate) fn prepare_read(
    py: Python,
    manifest: LocalFileManifest,
    size: u64,
    offset: u64,
) -> PyResult<&PyTuple> {
    let result = file_operations::prepare_read(manifest.0, size, offset);
    Ok(PyTuple::new(
        py,
        result.into_iter().map(|x| Chunk(x).into_py(py)),
    ))
}
