# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .client import ClientConfig, DeviceAccessStrategy
from .common import (
    U64,
    DateTime,
    DeviceID,
    EntryName,
    ErrorVariant,
    FsPath,
    Handle,
    Path,
    Ref,
    Result,
    SequesterServiceID,
    SizeInt,
    Structure,
    Variant,
    VersionInt,
    VlobID,
)
from .workspace import FileDescriptor

#
# Start workspace history
#


class WorkspaceHistoryStartError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class NoHistory:
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

    class CannotOpenRealmExportDatabase:
        pass

    class InvalidRealmExportDatabase:
        pass

    class UnsupportedRealmExportDatabaseVersion:
        pass

    class IncompleteRealmExportDatabase:
        pass


async def client_start_workspace_history(
    client: Handle,
    realm_id: VlobID,
) -> Result[Handle, WorkspaceHistoryStartError]:
    raise NotImplementedError


class WorkspaceHistoryRealmExportDecryptor(Variant):
    class User:
        access: DeviceAccessStrategy

    class SequesterService:
        sequester_service_id: SequesterServiceID
        private_key_pem_path: Path


async def workspace_history_start_with_realm_export(
    config: ClientConfig,
    export_db_path: Path,
    decryptors: list[WorkspaceHistoryRealmExportDecryptor],
) -> Result[Handle, WorkspaceHistoryStartError]:
    raise NotImplementedError


#
# Stop workspace history
#


class WorkspaceHistoryInternalOnlyError(ErrorVariant):
    class Internal:
        pass


def workspace_history_stop(
    workspace_history: Handle,
) -> Result[None, WorkspaceHistoryInternalOnlyError]:
    raise NotImplementedError


#
# Workspace history FS operations
#


def workspace_history_get_timestamp_lower_bound(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistoryInternalOnlyError]:
    raise NotImplementedError


def workspace_history_get_timestamp_higher_bound(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistoryInternalOnlyError]:
    raise NotImplementedError


def workspace_history_get_timestamp_of_interest(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistoryInternalOnlyError]:
    raise NotImplementedError


class WorkspaceHistorySetTimestampOfInterestError(ErrorVariant):
    class OlderThanLowerBound:
        pass

    class NewerThanHigherBound:
        pass

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

    class InvalidHistory:
        pass

    class Internal:
        pass


async def workspace_history_set_timestamp_of_interest(
    workspace_history: Handle,
    toi: DateTime,
) -> Result[None, WorkspaceHistorySetTimestampOfInterestError]:
    raise NotImplementedError


class WorkspaceHistoryEntryStat(Variant):
    class File:
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        version: VersionInt
        size: SizeInt
        last_updater: DeviceID

    class Folder:
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        version: VersionInt
        last_updater: DeviceID


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

    class InvalidHistory:
        pass

    class Internal:
        pass


async def workspace_history_stat_entry(
    workspace_history: Handle,
    path: Ref[FsPath],
) -> Result[WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError]:
    raise NotImplementedError


async def workspace_history_stat_entry_by_id(
    workspace_history: Handle,
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

    class InvalidHistory:
        pass

    class Internal:
        pass


async def workspace_history_stat_folder_children(
    workspace_history: Handle,
    path: Ref[FsPath],
) -> Result[
    list[tuple[EntryName, WorkspaceHistoryEntryStat]], WorkspaceHistoryStatFolderChildrenError
]:
    raise NotImplementedError


async def workspace_history_stat_folder_children_by_id(
    workspace_history: Handle,
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

    class InvalidHistory:
        pass

    class Internal:
        pass


async def workspace_history_open_file(
    workspace_history: Handle,
    path: FsPath,
) -> Result[FileDescriptor, WorkspaceHistoryOpenFileError]:
    raise NotImplementedError


async def workspace_history_open_file_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result[FileDescriptor, WorkspaceHistoryOpenFileError]:
    raise NotImplementedError


async def workspace_history_open_file_and_get_id(
    workspace_history: Handle,
    path: FsPath,
) -> Result[tuple[FileDescriptor, VlobID], WorkspaceHistoryOpenFileError]:
    raise NotImplementedError


class WorkspaceHistoryFdCloseError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


def workspace_history_fd_close(
    workspace_history: Handle,
    fd: FileDescriptor,
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

    class ServerBlockstoreUnavailable:
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
    workspace_history: Handle,
    fd: FileDescriptor,
    offset: U64,
    size: U64,
) -> Result[bytes, WorkspaceHistoryFdReadError]:
    raise NotImplementedError


class WorkspaceHistoryFileStat(Structure):
    id: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt


class WorkspaceHistoryFdStatError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


async def workspace_history_fd_stat(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result[WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError]:
    raise NotImplementedError
