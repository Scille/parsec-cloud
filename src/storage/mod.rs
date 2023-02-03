// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    import_exception,
    prelude::{PyModule, PyObject},
    wrap_pyfunction, IntoPy, PyErr, PyResult, Python,
};

use libparsec::core_fs::FSError;

use crate::data::{
    LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest,
};

pub(crate) mod user_storage;
pub(crate) mod workspace_storage;

pub(crate) fn add_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<workspace_storage::WorkspaceStorage>()?;
    m.add_class::<workspace_storage::WorkspaceStorageSnapshot>()?;
    m.add_function(wrap_pyfunction!(
        workspace_storage::workspace_storage_non_speculative_init,
        m
    )?)?;

    m.add_class::<user_storage::UserStorage>()?;
    m.add_function(wrap_pyfunction!(
        user_storage::user_storage_non_speculative_init,
        m
    )?)?;

    Ok(())
}

import_exception!(parsec.core.fs.exceptions, FSLocalStorageOperationalError);
import_exception!(parsec.core.fs.exceptions, FSLocalStorageClosedError);
import_exception!(parsec.core.fs.exceptions, FSInternalError);

pub(super) fn fs_to_python_error(e: FSError) -> PyErr {
    match e {
        FSError::DatabaseQueryError(_) | FSError::DatabaseOperationalError(_) => {
            FSLocalStorageOperationalError::new_err(e.to_string())
        }
        FSError::DatabaseClosed(_) => FSLocalStorageClosedError::new_err(e.to_string()),
        _ => FSInternalError::new_err(e.to_string()),
    }
}

pub(crate) fn manifest_from_py_object(
    py: Python<'_>,
    py_manifest: PyObject,
) -> PyResult<libparsec::client_types::LocalManifest> {
    let manifest = if let Ok(manifest) = py_manifest.extract::<LocalFileManifest>(py) {
        libparsec::client_types::LocalManifest::File(manifest.0)
    } else if let Ok(manifest) = py_manifest.extract::<LocalFolderManifest>(py) {
        libparsec::client_types::LocalManifest::Folder(manifest.0)
    } else if let Ok(manifest) = py_manifest.extract::<LocalWorkspaceManifest>(py) {
        libparsec::client_types::LocalManifest::Workspace(manifest.0)
    } else {
        libparsec::client_types::LocalManifest::User(
            py_manifest.extract::<LocalUserManifest>(py)?.0,
        )
    };

    Ok(manifest)
}

pub(crate) fn manifest_into_py_object(
    manifest: libparsec::client_types::LocalManifest,
) -> PyObject {
    Python::with_gil(|py| {
        let object: PyObject = match manifest {
            libparsec::client_types::LocalManifest::File(manifest) => {
                LocalFileManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Folder(manifest) => {
                LocalFolderManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::Workspace(manifest) => {
                LocalWorkspaceManifest(manifest).into_py(py)
            }
            libparsec::client_types::LocalManifest::User(manifest) => {
                LocalUserManifest(manifest).into_py(py)
            }
        };
        object
    })
}
