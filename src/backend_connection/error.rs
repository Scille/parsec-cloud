// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyException, pyclass};

use libparsec::client_connection;

#[pyclass]
pub(crate) struct CommandError(pub client_connection::CommandError);

crate::binding_utils::create_exception!(CommandError, PyException, client_connection::CommandError);
