# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

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
    FSNameTooLongError,
    # Remote operation errors
    FSBackendOfflineError,
    FSRemoteManifestNotFound,
    FSRemoteManifestNotFoundBadVersion,
    FSRemoteBlockNotFound,
    FSRemoteSyncError,
    FSBadEncryptionRevision,
    FSSharingNotAllowedError,
    FSWorkspaceNoAccess,
    FSWorkspaceNoReadAccess,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceNotInMaintenance,
    FSWorkspaceInMaintenance,
    FSUserNotFoundError,
    FSDeviceNotFoundError,
    FSInvalidTrustchainError,
)
from parsec.core.fs.path import FsPath, AnyPath
from parsec.core.fs.workspacefs import WorkspaceFS, WorkspaceFSTimestamped

__all__ = (
    "UserFS",
    "WorkspaceFS",
    "WorkspaceFSTimestamped",
    # Path
    "FsPath",
    "AnyPath",
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
    "FSNameTooLongError",
    # Remote operation error
    "FSBackendOfflineError",
    "FSRemoteManifestNotFound",
    "FSRemoteManifestNotFoundBadVersion",
    "FSRemoteBlockNotFound",
    "FSRemoteSyncError",
    "FSBadEncryptionRevision",
    "FSSharingNotAllowedError",
    "FSWorkspaceNoAccess",
    "FSWorkspaceNoReadAccess",
    "FSWorkspaceNoWriteAccess",
    "FSWorkspaceNotInMaintenance",
    "FSWorkspaceInMaintenance",
    "FSUserNotFoundError",
    "FSDeviceNotFoundError",
    "FSInvalidTrustchainError",
)
