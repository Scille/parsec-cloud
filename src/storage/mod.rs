// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    import_exception,
    prelude::{PyModule, PyObject},
    wrap_pyfunction, IntoPy, PyErr, PyResult, Python,
};

use libparsec::core_fs::FSError;

use crate::{
    data::{LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest},
    ids::EntryID,
};

pub(crate) mod user_storage;
pub(crate) mod workspace_storage;
pub(crate) mod workspace_storage_snapshot;

pub(crate) fn add_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<workspace_storage::WorkspaceStorage>()?;
    m.add_class::<workspace_storage_snapshot::WorkspaceStorageSnapshot>()?;
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
import_exception!(parsec.core.fs.exceptions, FSInvalidFileDescriptor);
import_exception!(parsec.core.fs.exceptions, FSLocalMissError);

pub(super) fn fs_to_python_error(e: FSError) -> PyErr {
    match e {
        FSError::DatabaseQueryError(_) | FSError::DatabaseOperationalError(_) => {
            FSLocalStorageOperationalError::new_err(e.to_string())
        }
        FSError::DatabaseClosed(_) => FSLocalStorageClosedError::new_err(e.to_string()),
        FSError::InvalidFileDescriptor(fd) => {
            FSInvalidFileDescriptor::new_err(format!("Invalid file descriptor {}", fd.0))
        }
        FSError::LocalMiss(entry_id) => {
            FSLocalMissError::new_err(EntryID(libparsec::types::EntryID::from(entry_id)))
        }
        _ => FSInternalError::new_err(e.to_string()),
    }
}

pub(crate) fn file_or_folder_manifest_from_py_object(
    py: Python<'_>,
    py_manifest: &PyObject,
) -> PyResult<libparsec::core_fs::LocalFileOrFolderManifest> {
    use libparsec::core_fs::LocalFileOrFolderManifest;

    let manifest = if let Ok(manifest) = py_manifest.extract::<LocalFileManifest>(py) {
        LocalFileOrFolderManifest::File(manifest.0)
    } else {
        py_manifest
            .extract::<LocalFolderManifest>(py)
            .map(|manifest| LocalFileOrFolderManifest::Folder(manifest.0))?
    };

    Ok(manifest)
}

pub(crate) fn manifest_into_py_object(
    manifest: libparsec::client_types::LocalManifest,
) -> PyObject {
    Python::with_gil(|py| match manifest {
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
    })
}

pub(crate) fn file_or_folder_manifest_into_py_object(
    manifest: libparsec::core_fs::LocalFileOrFolderManifest,
) -> PyObject {
    manifest_into_py_object(manifest.into())
}
