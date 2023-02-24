// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyException, PyErr};

pyo3::create_exception!(_parsec, InviteError, PyException);
pyo3::create_exception!(_parsec, InvitePeerResetError, InviteError);
pyo3::create_exception!(_parsec, InviteNotFoundError, InviteError);
pyo3::create_exception!(_parsec, InviteAlreadyUsedError, InviteError);
pyo3::create_exception!(_parsec, InviteActiveUsersLimitReachedError, InviteError);

pub(crate) struct InviteExc(libparsec::core::InviteError);

impl From<InviteExc> for PyErr {
    fn from(err: InviteExc) -> Self {
        match err.0 {
            libparsec::core::InviteError::Backend(..)
            | libparsec::core::InviteError::Command(..)
            | libparsec::core::InviteError::Custom(..) => InviteError::new_err(err.0.to_string()),
            libparsec::core::InviteError::PeerReset => {
                InvitePeerResetError::new_err(err.0.to_string())
            }
            libparsec::core::InviteError::NotFound => {
                InviteNotFoundError::new_err(err.0.to_string())
            }
            libparsec::core::InviteError::AlreadyUsed => {
                InviteAlreadyUsedError::new_err(err.0.to_string())
            }
            libparsec::core::InviteError::ActiveUsersLimitReached => {
                InviteActiveUsersLimitReachedError::new_err(err.0.to_string())
            }
        }
    }
}

impl From<libparsec::core::InviteError> for InviteExc {
    fn from(err: libparsec::core::InviteError) -> Self {
        InviteExc(err)
    }
}
