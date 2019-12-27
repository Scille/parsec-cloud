# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.backend_connection.exceptions import (
    BackendConnectionError,
    BackendProtocolError,
    BackendNotAvailable,
    BackendConnectionRefused,
)
from parsec.core.backend_connection.authenticated import (
    BackendAuthenticatedCmds,
    BackendConnStatus,
    BackendAuthenticatedConn,
    backend_authenticated_cmds_factory,
)
from parsec.core.backend_connection.annonymous import (
    BackendAnonymousCmds,
    backend_anonymous_cmds_factory,
)
from parsec.core.backend_connection.administration import (
    BackendAdministrationCmds,
    backend_administration_cmds_factory,
)


__all__ = (
    # Exceptions
    "BackendConnectionError",
    "BackendProtocolError",
    "BackendNotAvailable",
    "BackendConnectionRefused",
    # Authenticated
    "BackendAuthenticatedCmds",
    "BackendConnStatus",
    "BackendAuthenticatedConn",
    "backend_authenticated_cmds_factory",
    # Annonymous
    "BackendAnonymousCmds",
    "backend_anonymous_cmds_factory",
    # Administration
    "BackendAdministrationCmds",
    "backend_administration_cmds_factory",
)
