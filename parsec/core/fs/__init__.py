# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.fs.userfs import UserFS
from parsec.core.fs.exceptions import (
    # Generic classes
    FSError,
    FSOperationError,
    FSLocalOperationError,
    FSRemoteOperationError,
    # Misc errors
    FSWorkspaceNotFoundError,
    FSWorkspaceTimestampedTooEarly,
    # Local operation errors
    FSPermissionError,
    FSNoAccessError,
    FSReadOnlyError,
    FSNotADirectoryError,
    FSFileNotFoundError,
    FSCrossDeviceError,
    FSFileExistsError,
    FSIsADirectoryError,
    FSDirectoryNotEmptyError,
    FSInvalidFileDescriptor,
    FSInvalidArgumentError,
    FSEndOfFileError,
    # Remote operation errors
    FSBackendOfflineError,
    FSRemoteManifestNotFound,
    FSRemoteManifestNotFoundBadVersion,
    FSRemoteManifestNotFoundBadTimestamp,
    FSRemoteBlockNotFound,
    FSRemoteSyncError,
    FSBadEncryptionRevision,
    FSSharingNotAllowedError,
    FSWorkspaceNoAccess,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceNotInMaintenance,
    FSWorkspaceInMaintenance,
)
from parsec.core.fs.workspacefs import WorkspaceFS, WorkspaceFSTimestamped


__all__ = (
    "UserFS",
    "WorkspaceFS",
    "WorkspaceFSTimestamped",
    # Generic error classes
    "FSError",
    "FSOperationError",
    "FSLocalOperationError",
    "FSRemoteOperationError",
    # Misc errors
    "FSWorkspaceNotFoundError",
    "FSWorkspaceTimestampedTooEarly",
    # Local operation errors
    "FSPermissionError",
    "FSNoAccessError",
    "FSReadOnlyError",
    "FSNotADirectoryError",
    "FSFileNotFoundError",
    "FSCrossDeviceError",
    "FSFileExistsError",
    "FSIsADirectoryError",
    "FSDirectoryNotEmptyError",
    "FSInvalidFileDescriptor",
    "FSInvalidArgumentError",
    "FSEndOfFileError",
    # Remote operation error
    "FSBackendOfflineError",
    "FSRemoteManifestNotFound",
    "FSRemoteManifestNotFoundBadVersion",
    "FSRemoteManifestNotFoundBadTimestamp",
    "FSRemoteBlockNotFound",
    "FSRemoteSyncError",
    "FSBadEncryptionRevision",
    "FSSharingNotAllowedError",
    "FSWorkspaceNoAccess",
    "FSWorkspaceNoReadAccess",
    "FSWorkspaceNoWriteAccess",
    "FSWorkspaceNotInMaintenance",
    "FSWorkspaceInMaintenance",
)
