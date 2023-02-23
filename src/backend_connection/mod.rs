// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod authenticated_cmds;
mod error;

pub(crate) use authenticated_cmds::*;
pub(crate) use error::*;

use pyo3::{types::PyModule, PyResult, Python};

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Error
    m.add(
        "BackendConnectionError",
        py.get_type::<BackendConnectionError>(),
    )?;
    m.add(
        "BackendProtocolError",
        py.get_type::<BackendProtocolError>(),
    )?;
    m.add("BackendNotAvailable", py.get_type::<BackendNotAvailable>())?;
    m.add(
        "BackendConnectionRefused",
        py.get_type::<BackendConnectionRefused>(),
    )?;
    m.add(
        "BackendInvitationAlreadyUsed",
        py.get_type::<BackendInvitationAlreadyUsed>(),
    )?;
    m.add(
        "BackendInvitationAlreadyUsed",
        py.get_type::<BackendInvitationAlreadyUsed>(),
    )?;
    m.add(
        "BackendInvitationNotFound",
        py.get_type::<BackendInvitationNotFound>(),
    )?;
    m.add(
        "BackendNotFoundError",
        py.get_type::<BackendNotFoundError>(),
    )?;
    m.add(
        "BackendInvitationOnExistingMember",
        py.get_type::<BackendInvitationOnExistingMember>(),
    )?;
    m.add(
        "BackendOutOfBallparkError",
        py.get_type::<BackendOutOfBallparkError>(),
    )?;
    m.add("CommandError", py.get_type::<CommandError>())?;

    // Cmds
    m.add_class::<AuthenticatedCmds>()?;

    Ok(())
}
