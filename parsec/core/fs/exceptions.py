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


class FSWorkspaceNotFoundError(FSError):
    pass


class FSWorkspaceTimestampedTooEarly(FSError):
    pass


class FSOperationError(FSError):
    pass


class FSOperationLocalError(FSOperationError):

    """
    Used to represent "normal" error (e.g. opening a non-existing file,
    removing a non-empty folder etc.)
    """

    pass


class FSOperationRemoteError(FSError):
    """
    Used to represent error in the underlaying layers (e.g. data inconsistency,
    data access refused by the backend etc.)
    """

    ERRNO = errno.EACCES
    NTSTATUS = ntstatus.STATUS_ACCESS_DENIED


class FSInternalError(FSError):
    """
    Errors used internally by the fs module, should not be visible !
    """

    # TODO: internal exceptions types should not be public !
    def __init__(self, *args):
        super(Exception, self).__init__(*args)

    def __str__(self):
        return super(Exception, self).__str__()


# Operation local errors


class FSPermissionError(FSOperationLocalError, PermissionError):
    ERRNO = errno.EACCES
    NTSTATUS = ntstatus.STATUS_ACCESS_DENIED


class FSNotADirectoryError(FSOperationLocalError, NotADirectoryError):
    ERRNO = errno.ENOTDIR
    NTSTATUS = ntstatus.STATUS_NOT_A_DIRECTORY


class FSFileNotFoundError(FSOperationLocalError, FileNotFoundError):
    ERRNO = errno.ENOENT
    NTSTATUS = ntstatus.STATUS_OBJECT_NAME_NOT_FOUND


class FSCrossDeviceError(FSOperationLocalError):
    ERRNO = errno.EXDEV
    NTSTATUS = ntstatus.STATUS_NOT_SAME_DEVICE


class FSFileExistsError(FSOperationLocalError, FileExistsError):
    ERRNO = errno.EEXIST
    NTSTATUS = ntstatus.STATUS_OBJECT_NAME_COLLISION


class FSIsADirectoryError(FSOperationLocalError, IsADirectoryError):
    ERRNO = errno.EISDIR
    NTSTATUS = ntstatus.STATUS_FILE_IS_A_DIRECTORY


class FSDirectoryNotEmptyError(FSOperationLocalError):
    ERRNO = errno.ENOTEMPTY
    NTSTATUS = ntstatus.STATUS_DIRECTORY_NOT_EMPTY


class FSInvalidFileDescriptor(FSOperationLocalError):
    ERRNO = errno.EBADF
    NTSTATUS = ntstatus.STATUS_INVALID_HANDLE


class FSInvalidArgumentError(FSOperationLocalError):
    ERRNO = errno.EINVAL
    NTSTATUS = ntstatus.STATUS_INVALID_PARAMETER


# Operation remote errors


class FSBackendOfflineError(FSOperationRemoteError):
    ERRNO = errno.EHOSTUNREACH
    NTSTATUS = ntstatus.STATUS_HOST_UNREACHABLE


class FSRemoteManifestNotFound(FSOperationRemoteError):
    pass


class FSRemoteManifestNotFoundBadVersion(FSRemoteManifestNotFound):
    pass


class FSRemoteManifestNotFoundBadTimestamp(FSRemoteManifestNotFound):
    pass


class FSRemoteBlockNotFound(FSOperationRemoteError):
    pass


class FSRemoteSyncError(FSOperationRemoteError):
    pass


class FSBadEncryptionRevision(FSOperationRemoteError):
    pass


class FSSharingNotAllowedError(FSOperationRemoteError):
    pass


class FSWorkspaceNoAccess(FSOperationRemoteError, PermissionError):
    pass


class FSWorkspaceNoReadAccess(FSWorkspaceNoAccess):
    pass


class FSWorkspaceNoWriteAccess(FSWorkspaceNoAccess):
    pass


class FSWorkspaceNotInMaintenance(FSOperationRemoteError):
    pass


class FSWorkspaceInMaintenance(FSOperationRemoteError):
    pass


# Internal errors


class FSLocalMissError(FSInternalError):
    def __init__(self, id: EntryID):
        super().__init__(id)
        self.id = id


class FSFileConflictError(FSInternalError):
    pass


class FSReshapingRequiredError(FSInternalError):
    pass


class FSNoSynchronizationRequired(FSInternalError):
    pass
