// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::prelude::*;
use std::sync::OnceLock;

mod meths;

static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
static ASYNCIO: OnceLock<PyObject> = OnceLock::new();

fn tokio_runtime() -> &'static tokio::runtime::Runtime {
    TOKIO_RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("Cannot start tokio runtime for libparsec")
    })
}

fn asyncio(py: Python) -> &Bound<PyAny> {
    ASYNCIO
        .get_or_init(|| {
            py.import_bound("asyncio")
                .expect("Cannot import Python asyncio module for libparsec")
                .to_object(py)
        })
        .bind(py)
}

#[pymodule]
fn libparsec(m: Bound<'_, PyModule>) -> PyResult<()> {
    env_logger::init();
    meths::register_meths(&m)
}
