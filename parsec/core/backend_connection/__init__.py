# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.backend_connection.exceptions import (
    BackendConnectionError,
    BackendNotAvailable,
    BackendIncompatibleVersion,
    BackendHandshakeError,
    BackendHandshakeAPIVersionError,
    BackendDeviceRevokedError,
    BackendCmdsInvalidRequest,
    BackendCmdsInvalidResponse,
    BackendCmdsBadResponse,
    BackendCmdsAlreadyExists,
    BackendCmdsNotAllowed,
    BackendCmdsBadVersion,
    BackendCmdsNotFound,
    BackendCmdsInvalidRole,
    BackendCmdsDenied,
    BackendCmdsBadUserId,
    BackendCmdsInvalidCertification,
    BackendCmdsInvalidData,
    BackendCmdsAlreadyBootstrapped,
    BackendCmdsCancelled,
    BackendCmdsNoEvents,
    BackendCmdsTimeout,
    BackendCmdsBadMessage,
)
from parsec.core.backend_connection.transport import (
    anonymous_transport_factory,
    administration_transport_factory,
    authenticated_transport_pool_factory,
    AuthenticatedTransportPool,
)
from parsec.core.backend_connection.event_listener import backend_listen_events
from parsec.core.backend_connection.monitor import (
    monitor_backend_connection,
    current_backend_connection_state,
    BackendState,
)
from parsec.core.backend_connection.porcelain import (
    BackendCmdsPool,
    backend_cmds_pool_factory,
    BackendAnonymousCmds,
    backend_anonymous_cmds_factory,
    BackendAdministrationCmds,
    backend_administration_cmds_factory,
)


__all__ = (
    "BackendConnectionError",
    "BackendNotAvailable",
    "BackendIncompatibleVersion",
    "BackendHandshakeError",
    "BackendHandshakeAPIVersionError",
    "BackendDeviceRevokedError",
    "BackendCmdsInvalidRequest",
    "BackendCmdsInvalidResponse",
    "BackendCmdsBadResponse",
    "BackendCmdsAlreadyExists",
    "BackendCmdsNotAllowed",
    "BackendCmdsBadVersion",
    "BackendCmdsNotFound",
    "BackendCmdsInvalidRole",
    "BackendCmdsDenied",
    "BackendCmdsBadUserId",
    "BackendCmdsInvalidCertification",
    "BackendCmdsInvalidData",
    "BackendCmdsAlreadyBootstrapped",
    "BackendCmdsCancelled",
    "BackendCmdsNoEvents",
    "BackendCmdsTimeout",
    "BackendCmdsBadMessage",
    "anonymous_transport_factory",
    "administration_transport_factory",
    "authenticated_transport_pool_factory",
    "AuthenticatedTransportPool",
    "backend_listen_events",
    "monitor_backend_connection",
    "current_backend_connection_state",
    "BackendState",
    "BackendCmdsInvalidRequest",
    "BackendCmdsInvalidResponse",
    "BackendCmdsBadResponse",
    "BackendCmdsPool",
    "backend_cmds_pool_factory",
    "BackendAnonymousCmds",
    "backend_anonymous_cmds_factory",
    "BackendAdministrationCmds",
    "backend_administration_cmds_factory",
)
