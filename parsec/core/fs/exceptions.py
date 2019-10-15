# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import errno
from parsec.core.types import EntryID
from parsec.core.fs.utils import ntstatus


# Base class for all file system errors


class FSError(OSError):
    ERRNO = None
    WINERROR = None
    NTSTATUS = None

    def __init__(self, message=None, filename=None, filename2=None):
        # Get the actual message and save it
        if message is None and self.ERRNO is not None:
            message = os.strerror(self.ERRNO)
        self.message = str(message)

        # Error with no standard errno
        if self.ERRNO is None:
            return super().__init__(message)

        # Cast filename arguments
        if filename is not None:
            filename = str(filename)
        if filename2 is not None:
            filename2 = str(filename)

        # Full OS error initialization
        self.ntstatus = self.NTSTATUS
        super().__init__(self.ERRNO, self.message, filename, self.WINERROR, filename2)

    def __str__(self):
        if self.filename2:
            return f"{self.message}: {self.filename} -> {self.filename2}"
        if self.filename:
            return f"{self.message}: {self.filename}"
        return self.message


# Base errors


class FSPermissionError(FSError, PermissionError):
    ERRNO = errno.EACCES
    NTSTATUS = ntstatus.STATUS_ACCESS_DENIED


class FSNotADirectoryError(FSError, NotADirectoryError):
    ERRNO = errno.ENOTDIR
    NTSTATUS = ntstatus.STATUS_NOT_A_DIRECTORY


class FSFileNotFoundError(FSError, FileNotFoundError):
    ERRNO = errno.ENOENT
    NTSTATUS = ntstatus.STATUS_OBJECT_NAME_NOT_FOUND


class FSCrossDeviceError(FSError):
    ERRNO = errno.EXDEV
    NTSTATUS = ntstatus.STATUS_NOT_SAME_DEVICE


class FSFileExistsError(FSError, FileExistsError):
    ERRNO = errno.EEXIST
    NTSTATUS = ntstatus.STATUS_OBJECT_NAME_COLLISION


class FSIsADirectoryError(FSError, IsADirectoryError):
    ERRNO = errno.EISDIR
    NTSTATUS = ntstatus.STATUS_FILE_IS_A_DIRECTORY


class FSDirectoryNotEmptyError(FSError):
    ERRNO = errno.ENOTEMPTY
    NTSTATUS = ntstatus.STATUS_DIRECTORY_NOT_EMPTY


class FSInvalidFileDescriptor(FSError):
    ERRNO = errno.EBADF
    NTSTATUS = ntstatus.STATUS_INVALID_HANDLE


class FSNetworkError(FSError):
    ERRNO = errno.EHOSTUNREACH
    NTSTATUS = ntstatus.STATUS_HOST_UNREACHABLE


class FSEntryNotFound(FSError):
    ERRNO = errno.ENOENT
    NTSTATUS = ntstatus.STATUS_OBJECT_NAME_NOT_FOUND


class FSInvalidArgumentError(FSError):
    ERRNO = errno.EINVAL
    NTSTATUS = ntstatus.STATUS_INVALID_PARAMETER


class FSInternalError(FSError):
    def __init__(self, *args):
        super(Exception, self).__init__(*args)


# Protocol errors


class FSValidationError(FSInternalError):
    pass


class FSPackingError(FSInternalError):
    pass


# Remote errors


class FSRemoteManifestNotFound(FSInternalError):
    pass


class FSRemoteManifestNotFoundBadVersion(FSRemoteManifestNotFound):
    pass


class FSRemoteBlockNotFound(FSInternalError):
    pass


class FSRemoteSyncError(FSInternalError):
    pass


class FSBadEncryptionRevision(FSInternalError):
    pass


# Local miss errors


class FSLocalMissError(FSInternalError):
    def __init__(self, id: EntryID):
        super().__init__(id)
        self.id = id


class FSWorkspaceNotFoundError(FSLocalMissError):
    pass


# Connection errors


class FSBackendOfflineError(FSNetworkError):
    pass


# Rights errors


class FSSharingNotAllowedError(FSPermissionError):
    pass


class FSWorkspaceNoAccess(FSPermissionError):
    pass


class FSWorkspaceNoReadAccess(FSWorkspaceNoAccess):
    pass


class FSWorkspaceNoWriteAccess(FSWorkspaceNoAccess):
    pass


# Maintenance errors


class FSWorkspaceNotInMaintenance(FSInternalError):
    pass


class FSWorkspaceInMaintenance(FSPermissionError):
    pass


class FSMaintenanceNotAllowedError(FSPermissionError):
    pass


# Workspace internal errors


class FSFileConflictError(FSInternalError):
    pass


class FSReshapingRequiredError(FSInternalError):
    pass


class FSNoSynchronizationRequired(FSInternalError):
    pass


# Timestamping errors


class FSWorkspaceTimestampedTooEarly(FSInternalError):
    pass
