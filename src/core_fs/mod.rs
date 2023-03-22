// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod event_bus;
mod remote_loader;
mod storage;
mod sync_transactions;

pub(crate) use event_bus::FSBlockEventBus;
pub(crate) use remote_loader::UserRemoteLoader;
pub(crate) use storage::{
    user_storage_non_speculative_init, workspace_storage_non_speculative_init, UserStorage,
    WorkspaceStorage, WorkspaceStorageSnapshot,
};
pub(crate) use sync_transactions::ChangesAfterSync;

use pyo3::{types::PyModule, wrap_pyfunction, PyResult, Python};

pub(crate) fn add_mod(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // EventBus
    m.add_class::<FSBlockEventBus>()?;

    // Sync Transactions
    m.add_class::<ChangesAfterSync>()?;

    // Storage
    m.add_class::<WorkspaceStorage>()?;
    m.add_class::<WorkspaceStorageSnapshot>()?;
    m.add_function(wrap_pyfunction!(workspace_storage_non_speculative_init, m)?)?;

    m.add_class::<UserStorage>()?;
    m.add_function(wrap_pyfunction!(user_storage_non_speculative_init, m)?)?;

    m.add_class::<UserRemoteLoader>()?;
    Ok(())
}
