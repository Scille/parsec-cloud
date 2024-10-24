# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    U64,
    DateTime,
    EntryName,
    ErrorVariant,
    FsPath,
    Handle,
    Ref,
    Result,
    SizeInt,
    Variant,
    VersionInt,
    VlobID,
    Structure,
)
from .workspace import FileDescriptor


#
# Workspace history FS operations
#


class WorkspaceHistoryGetWorkspaceManifestV1TimestampError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_history_get_workspace_manifest_v1_timestamp(
    workspace: Handle,
) -> Result[Optional[DateTime], WorkspaceHistoryGetWorkspaceManifestV1TimestampError]:
    raise NotImplementedError


class WorkspaceHistoryEntryStat(Variant):
    class File:
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        version: VersionInt
        size: SizeInt

    class Folder:
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        version: VersionInt


class WorkspaceHistoryStatEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_history_stat_entry(
    workspace: Handle,
    at: DateTime,
    path: Ref[FsPath],
) -> Result[WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError]:
    raise NotImplementedError


async def workspace_history_stat_entry_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result[WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError]:
    raise NotImplementedError


class WorkspaceHistoryStatFolderChildrenError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class EntryIsFile:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_history_stat_folder_children(
    workspace: Handle,
    at: DateTime,
    path: Ref[FsPath],
) -> Result[
    list[tuple[EntryName, WorkspaceHistoryEntryStat]], WorkspaceHistoryStatFolderChildrenError
]:
    raise NotImplementedError


async def workspace_history_stat_folder_children_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result[
    list[tuple[EntryName, WorkspaceHistoryEntryStat]], WorkspaceHistoryStatFolderChildrenError
]:
    raise NotImplementedError


class WorkspaceHistoryOpenFileError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class NoRealmAccess:
        pass

    class EntryNotFound:
        pass

    class EntryNotAFile:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_history_open_file(
    workspace: Handle,
    at: DateTime,
    path: FsPath,
) -> Result[FileDescriptor, WorkspaceHistoryOpenFileError]:
    raise NotImplementedError


async def workspace_history_open_file_by_id(
    workspace: Handle,
    at: DateTime,
    entry_id: VlobID,
) -> Result[FileDescriptor, WorkspaceHistoryOpenFileError]:
    raise NotImplementedError


class WorkspaceHistoryFdCloseError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


def workspace_history_fd_close(
    workspace: Handle, fd: FileDescriptor
) -> Result[None, WorkspaceHistoryFdCloseError]:
    raise NotImplementedError


class WorkspaceHistoryFdReadError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class NoRealmAccess:
        pass

    class BlockNotFound:
        pass

    class InvalidBlockAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class Internal:
        pass


async def workspace_history_fd_read(
    workspace: Handle,
    fd: FileDescriptor,
    offset: U64,
    size: U64,
) -> Result[bytes, WorkspaceHistoryFdReadError]:
    raise NotImplementedError


class WorkspaceHistoryFdStatError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


class WorkspaceHistoryFileStat(Structure):
    id: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt


async def workspace_history_fd_stat(
    workspace: Handle,
    fd: FileDescriptor,
) -> Result[WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError]:
    raise NotImplementedError
