// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyException, PyErr};

pyo3::create_exception!(_parsec, BackendConnectionError, PyException);
pyo3::create_exception!(_parsec, BackendProtocolError, BackendConnectionError);
pyo3::create_exception!(_parsec, BackendNotAvailable, BackendConnectionError);
pyo3::create_exception!(_parsec, BackendConnectionRefused, BackendConnectionError);
pyo3::create_exception!(
    _parsec,
    BackendInvitationAlreadyUsed,
    BackendConnectionRefused
);
pyo3::create_exception!(_parsec, BackendInvitationNotFound, BackendConnectionRefused);
// TODO: `LoggedCore.get_user_info` raise `BackendNotFoundError`
// It was convenient to have it inherit `BackendConnectionError` but it would be best to have a specific set of `exception` for it.
pyo3::create_exception!(_parsec, BackendNotFoundError, BackendConnectionError);
// TODO: `LoggedCore.new_user_invitation` raise `BackendInvitationOnExistingMember`
// It was convenient to have it inherit `BackendConnectionError` but it would be best to have a specific set of `exception` for it.
pyo3::create_exception!(
    _parsec,
    BackendInvitationOnExistingMember,
    BackendConnectionError
);
pyo3::create_exception!(_parsec, BackendOutOfBallparkError, BackendConnectionError);

pub(crate) struct CommandExc(libparsec::client_connection::CommandError);

impl From<CommandExc> for PyErr {
    fn from(err: CommandExc) -> Self {
        match err.0 {
            libparsec::client_connection::CommandError::NoResponse { .. } => {
                BackendNotAvailable::new_err(err.0.to_string())
            }
            libparsec::client_connection::CommandError::InvalidResponseStatus(status, ..)
                if status == 500 =>
            {
                BackendNotAvailable::new_err(err.0.to_string())
            }
            libparsec::client_connection::CommandError::InvalidResponseStatus(status, ..)
                if (status == 401 || status == 404) =>
            {
                BackendConnectionRefused::new_err("Invalid handshake information")
            }
            libparsec::client_connection::CommandError::RevokedUser
            | libparsec::client_connection::CommandError::ExpiredOrganization => {
                BackendConnectionRefused::new_err(err.0.to_string())
            }
            _ => BackendConnectionError::new_err(err.0.to_string()),
        }
    }
}

impl From<libparsec::client_connection::CommandError> for CommandExc {
    fn from(err: libparsec::client_connection::CommandError) -> Self {
        CommandExc(err)
    }
}
