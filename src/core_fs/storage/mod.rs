// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{IntoPy, PyObject, PyResult, Python};

use crate::data::{
    LocalFileManifest, LocalFolderManifest, LocalUserManifest, LocalWorkspaceManifest,
};

mod user_storage;
mod workspace_storage;
mod workspace_storage_snapshot;

pub(crate) use user_storage::{user_storage_non_speculative_init, UserStorage};
pub(crate) use workspace_storage::{workspace_storage_non_speculative_init, WorkspaceStorage};
pub(crate) use workspace_storage_snapshot::WorkspaceStorageSnapshot;

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
