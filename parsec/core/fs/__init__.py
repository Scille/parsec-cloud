# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.fs.userfs import UserFS
from parsec.core.fs.exceptions import (
    FSError,
    FSEntryNotFound,
    FSValidationError,
    FSPackingError,
    FSWorkspaceNotFoundError,
    FSWorkspaceInMaintenance,
    FSWorkspaceNotInMaintenance,
    FSBackendOfflineError,
    FSSharingNotAllowedError,
    FSBadEncryptionRevision,
)
from parsec.core.fs.workspacefs import WorkspaceFS, WorkspaceFSTimestamped

# TODO: refactor local storage exceptions
from parsec.core.local_storage import FSInvalidFileDescriptor


__all__ = (
    "UserFS",
    "FSError",
    "FSEntryNotFound",
    "FSValidationError",
    "FSPackingError",
    "FSWorkspaceNotFoundError",
    "FSWorkspaceInMaintenance",
    "FSWorkspaceNotInMaintenance",
    "FSBackendOfflineError",
    "FSSharingNotAllowedError",
    "FSBadEncryptionRevision",
    "FSInvalidFileDescriptor",
    "WorkspaceFS",
    "WorkspaceFSTimestamped",
)
