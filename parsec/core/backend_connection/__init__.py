# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.core.backend_connection.exceptions import (
    BackendConnectionError,
    BackendProtocolError,
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendInvitationNotFound,
    BackendInvitationAlreadyUsed,
    BackendNotFoundError,
    BackendInvitationOnExistingMember,
)
from parsec.core.backend_connection.authenticated import (
    BackendAuthenticatedCmds,
    BackendConnStatus,
    BackendAuthenticatedConn,
    backend_authenticated_cmds_factory,
)
from parsec.core.backend_connection.invited import BackendInvitedCmds, backend_invited_cmds_factory
from parsec.core.backend_connection.apiv1_annonymous import (
    APIV1_BackendAnonymousCmds,
    apiv1_backend_anonymous_cmds_factory,
)
from parsec.core.backend_connection.apiv1_administration import (
    APIV1_BackendAdministrationCmds,
    apiv1_backend_administration_cmds_factory,
)


__all__ = (
    # Exceptions
    "BackendConnectionError",
    "BackendProtocolError",
    "BackendNotAvailable",
    "BackendConnectionRefused",
    "BackendInvitationNotFound",
    "BackendInvitationAlreadyUsed",
    "BackendNotFoundError",
    "BackendInvitationOnExistingMember",
    # Authenticated
    "BackendAuthenticatedCmds",
    "BackendConnStatus",
    "BackendAuthenticatedConn",
    "backend_authenticated_cmds_factory",
    # Invited
    "BackendInvitedCmds",
    "backend_invited_cmds_factory",
    # APIv1 Annonymous
    "APIV1_BackendAnonymousCmds",
    "apiv1_backend_anonymous_cmds_factory",
    # APIv1 Administration
    "APIV1_BackendAdministrationCmds",
    "apiv1_backend_administration_cmds_factory",
)
