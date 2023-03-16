// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyException, PyErr};

pyo3::create_exception!(_parsec, RemoteDevicesManagerError, PyException);
pyo3::create_exception!(
    _parsec,
    RemoteDevicesManagerBackendOfflineError,
    RemoteDevicesManagerError
);
pyo3::create_exception!(
    _parsec,
    RemoteDevicesManagerNotFoundError,
    RemoteDevicesManagerError
);
pyo3::create_exception!(
    _parsec,
    RemoteDevicesManagerUserNotFoundError,
    RemoteDevicesManagerNotFoundError
);
pyo3::create_exception!(
    _parsec,
    RemoteDevicesManagerDeviceNotFoundError,
    RemoteDevicesManagerNotFoundError
);
pyo3::create_exception!(
    _parsec,
    RemoteDevicesManagerInvalidTrustchainError,
    RemoteDevicesManagerError
);

pub(crate) struct RemoteDevicesManagerExc(libparsec::core::RemoteDevicesManagerError);

impl From<RemoteDevicesManagerExc> for PyErr {
    fn from(err: RemoteDevicesManagerExc) -> Self {
        match err.0 {
            libparsec::core::RemoteDevicesManagerError::BackendOffline { .. } => {
                RemoteDevicesManagerBackendOfflineError::new_err(err.0.to_string())
            }
            libparsec::core::RemoteDevicesManagerError::InvalidTrustchain { .. } => {
                RemoteDevicesManagerInvalidTrustchainError::new_err(err.0.to_string())
            }
            libparsec::core::RemoteDevicesManagerError::DeviceNotFound { .. } => {
                RemoteDevicesManagerDeviceNotFoundError::new_err(err.0.to_string())
            }
            libparsec::core::RemoteDevicesManagerError::UserNotFound { .. } => {
                RemoteDevicesManagerUserNotFoundError::new_err(err.0.to_string())
            }
            _ => RemoteDevicesManagerError::new_err(err.0.to_string()),
        }
    }
}

impl From<libparsec::core::RemoteDevicesManagerError> for RemoteDevicesManagerExc {
    fn from(err: libparsec::core::RemoteDevicesManagerError) -> Self {
        RemoteDevicesManagerExc(err)
    }
}
