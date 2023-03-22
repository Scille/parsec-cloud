// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod sync_transactions;

pub(crate) use sync_transactions::ChangesAfterSync;

use pyo3::{types::PyModule, PyResult, Python};

pub(crate) fn add_mod(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // EventBus
    m.add_class::<ChangesAfterSync>()?;
    Ok(())
}
