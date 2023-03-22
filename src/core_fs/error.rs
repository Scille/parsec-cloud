// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::PyErr;

use crate::ids::EntryID;

pyo3::import_exception!(parsec.core.fs.exceptions, FSError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSRemoteOperationError);

pyo3::import_exception!(parsec.core.fs.exceptions, FSBackendOfflineError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSWorkspaceNoReadAccess);
pyo3::import_exception!(parsec.core.fs.exceptions, FSRemoteManifestNotFound);
pyo3::import_exception!(parsec.core.fs.exceptions, FSWorkspaceInMaintenance);

// RemoteDevicesManager errors
pyo3::import_exception!(parsec.core.fs.exceptions, FSUserNotFoundError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSDeviceNotFoundError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSInvalidTrustchainError);

// Storage errors
pyo3::import_exception!(parsec.core.fs.exceptions, FSLocalStorageOperationalError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSLocalStorageClosedError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSInternalError);
pyo3::import_exception!(parsec.core.fs.exceptions, FSInvalidFileDescriptor);
pyo3::import_exception!(parsec.core.fs.exceptions, FSLocalMissError);

pub(crate) fn to_py_err(e: libparsec::core_fs::FSError) -> PyErr {
    match e {
        // Generic error
        libparsec::core_fs::FSError::Custom(_) => FSError::new_err(e.to_string()),

        // FS RemoteLoader error
        libparsec::core_fs::FSError::BackendOffline(_) => {
            FSBackendOfflineError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::RemoteManifestNotFound(_) => {
            FSRemoteManifestNotFound::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::RemoteOperation(_) => {
            FSRemoteOperationError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::WorkspaceInMaintenance => {
            FSWorkspaceInMaintenance::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::WorkspaceNoReadAccess => {
            FSWorkspaceNoReadAccess::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::InvalidRealmRoleCertificates(_) => {
            FSError::new_err(e.to_string())
        }

        // RemoteDevicesManager errors
        libparsec::core_fs::FSError::DeviceNotFound(_) => {
            FSDeviceNotFoundError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::UserNotFound(_) => FSUserNotFoundError::new_err(e.to_string()),
        libparsec::core_fs::FSError::InvalidTrustchain(_) => {
            FSInvalidTrustchainError::new_err(e.to_string())
        }

        // Storage errors
        libparsec::core_fs::FSError::DatabaseQueryError(_) => {
            FSLocalStorageOperationalError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::DatabaseOperationalError(_) => {
            FSLocalStorageOperationalError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::DatabaseClosed(_) => {
            FSLocalStorageClosedError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::InvalidFileDescriptor(fd) => {
            FSInvalidFileDescriptor::new_err(format!("Invalid file descriptor {}", fd.0))
        }
        libparsec::core_fs::FSError::LocalMiss(entry_id) => {
            FSLocalMissError::new_err(EntryID(libparsec::types::EntryID::from(entry_id)))
        }

        libparsec::core_fs::FSError::Configuration(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Connection(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::CreateTable(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::CreateDir => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Crypto(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::InvalidIndexes { .. } => {
            FSInternalError::new_err(e.to_string())
        }
        libparsec::core_fs::FSError::QueryTable(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Permission => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Pool => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Runtime(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::UpdateTable(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::UserManifestMissing => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::Vacuum(_) => FSInternalError::new_err(e.to_string()),
        libparsec::core_fs::FSError::WorkspaceStorageTimestamped => {
            FSInternalError::new_err(e.to_string())
        }
    }
}
