# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.fs.exceptions import (
    # Remote operation errors
    FSBackendOfflineError,
    FSBadEncryptionRevision,
    FSCrossDeviceError,
    FSDeviceNotFoundError,
    FSDirectoryNotEmptyError,
    FSEndOfFileError,
    # Generic classes
    FSError,
    FSFileExistsError,
    FSFileNotFoundError,
    FSInvalidArgumentError,
    FSInvalidFileDescriptor,
    FSInvalidTrustchainError,
    FSIsADirectoryError,
    FSLocalOperationError,
    FSNameTooLongError,
    FSNoAccessError,
    FSNotADirectoryError,
    FSOperationError,
    # Local operation errors
    FSPermissionError,
    FSReadOnlyError,
    FSRemoteBlockNotFound,
    FSRemoteManifestNotFound,
    FSRemoteManifestNotFoundBadVersion,
    FSRemoteOperationError,
    FSRemoteSyncError,
    FSSharingNotAllowedError,
    FSUserNotFoundError,
    FSWorkspaceInMaintenance,
    FSWorkspaceNoAccess,
    FSWorkspaceNoReadAccess,
    # Misc errors
    FSWorkspaceNotFoundError,
    FSWorkspaceNotInMaintenance,
    FSWorkspaceNoWriteAccess,
    FSWorkspaceTimestampedTooEarly,
)
from parsec.core.fs.path import AnyPath, FsPath
from parsec.core.fs.userfs import UserFS
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
