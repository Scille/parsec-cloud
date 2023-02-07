// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyException;

use libparsec::client_connection;

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
// TODO: hack needed by `LoggedCore.get_user_info`
pyo3::create_exception!(_parsec, BackendNotFoundError, BackendConnectionError);
// TODO: hack needed by `LoggedCore.new_user_invitation`
pyo3::create_exception!(
    _parsec,
    BackendInvitationOnExistingMember,
    BackendConnectionError
);
pyo3::create_exception!(_parsec, BackendOutOfBallparkError, BackendConnectionError);

crate::binding_utils::create_exception!(
    Command,
    BackendConnectionError,
    client_connection::CommandError,
    no_result_type
);
