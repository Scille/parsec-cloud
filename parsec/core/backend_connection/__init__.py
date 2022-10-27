# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.backend_connection.exceptions import (
    BackendConnectionError,
    BackendProtocolError,
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendInvitationNotFound,
    BackendInvitationAlreadyUsed,
    BackendNotFoundError,
    BackendInvitationOnExistingMember,
    BackendOutOfBallparkError,
)
from parsec.core.backend_connection.authenticated import (
    BackendAuthenticatedCmds,
    BackendConnStatus,
    BackendAuthenticatedConn,
    backend_authenticated_cmds_factory,
)
from parsec.core.backend_connection.invited import (
    BackendInvitedCmds,
    backend_invited_cmds_factory,
)
from parsec.core.backend_connection.apiv1_anonymous import (
    APIV1_BackendAnonymousCmds,
    apiv1_backend_anonymous_cmds_factory,
)
from parsec.core.backend_connection.anonymous import (
    pki_enrollment_submit,
    pki_enrollment_info,
    organization_bootstrap,
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
    "BackendOutOfBallparkError",
    # Authenticated
    "BackendAuthenticatedCmds",
    "BackendConnStatus",
    "BackendAuthenticatedConn",
    "backend_authenticated_cmds_factory",
    # Invited
    "BackendInvitedCmds",
    "backend_invited_cmds_factory",
    # Anonymous
    "organization_bootstrap",
    "pki_enrollment_submit",
    "pki_enrollment_info",
    # APIv1 Anonymous
    "APIV1_BackendAnonymousCmds",
    "apiv1_backend_anonymous_cmds_factory",
)
