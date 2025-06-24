# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import (
    U64,
    DateTime,
    DeviceID,
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
    Path,
    SequesterServiceID,
)
from .workspace import FileDescriptor
from .client import ClientConfig, DeviceAccessStrategy


#
# Start workspace history
#


class WorkspaceHistory2StartError(ErrorVariant):
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


async def client_start_workspace_history2(
    client: Handle,
    realm_id: VlobID,
) -> Result[Handle, WorkspaceHistory2StartError]:
    raise NotImplementedError


class WorkspaceHistory2RealmExportDecryptor(Variant):
    class User:
        access: DeviceAccessStrategy

    class SequesterService:
        sequester_service_id: SequesterServiceID
        private_key_pem_path: Path


async def workspace_history2_start_with_realm_export(
    config: ClientConfig,
    export_db_path: Path,
    decryptors: list[WorkspaceHistory2RealmExportDecryptor],
) -> Result[Handle, WorkspaceHistory2StartError]:
    raise NotImplementedError


#
# Stop workspace history
#


class WorkspaceHistory2InternalOnlyError(ErrorVariant):
    class Internal:
        pass


def workspace_history2_stop(
    workspace_history: Handle,
) -> Result[None, WorkspaceHistory2InternalOnlyError]:
    raise NotImplementedError


#
# Workspace history FS operations
#


def workspace_history2_get_timestamp_lower_bound(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistory2InternalOnlyError]:
    raise NotImplementedError


def workspace_history2_get_timestamp_higher_bound(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistory2InternalOnlyError]:
    raise NotImplementedError


def workspace_history2_get_timestamp_of_interest(
    workspace_history: Handle,
) -> Result[DateTime, WorkspaceHistory2InternalOnlyError]:
    raise NotImplementedError


class WorkspaceHistory2SetTimestampOfInterestError(ErrorVariant):
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


async def workspace_history2_set_timestamp_of_interest(
    workspace_history: Handle,
    toi: DateTime,
) -> Result[None, WorkspaceHistory2SetTimestampOfInterestError]:
    raise NotImplementedError


class WorkspaceHistory2EntryStat(Variant):
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


class WorkspaceHistory2StatEntryError(ErrorVariant):
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


async def workspace_history2_stat_entry(
    workspace_history: Handle,
    path: Ref[FsPath],
) -> Result[WorkspaceHistory2EntryStat, WorkspaceHistory2StatEntryError]:
    raise NotImplementedError


async def workspace_history2_stat_entry_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result[WorkspaceHistory2EntryStat, WorkspaceHistory2StatEntryError]:
    raise NotImplementedError


class WorkspaceHistory2StatFolderChildrenError(ErrorVariant):
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


async def workspace_history2_stat_folder_children(
    workspace_history: Handle,
    path: Ref[FsPath],
) -> Result[
    list[tuple[EntryName, WorkspaceHistory2EntryStat]], WorkspaceHistory2StatFolderChildrenError
]:
    raise NotImplementedError


async def workspace_history2_stat_folder_children_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result[
    list[tuple[EntryName, WorkspaceHistory2EntryStat]], WorkspaceHistory2StatFolderChildrenError
]:
    raise NotImplementedError


class WorkspaceHistory2OpenFileError(ErrorVariant):
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


async def workspace_history2_open_file(
    workspace_history: Handle,
    path: FsPath,
) -> Result[FileDescriptor, WorkspaceHistory2OpenFileError]:
    raise NotImplementedError


async def workspace_history2_open_file_by_id(
    workspace_history: Handle,
    entry_id: VlobID,
) -> Result[FileDescriptor, WorkspaceHistory2OpenFileError]:
    raise NotImplementedError


async def workspace_history2_open_file_and_get_id(
    workspace_history: Handle,
    path: FsPath,
) -> Result[tuple[FileDescriptor, VlobID], WorkspaceHistory2OpenFileError]:
    raise NotImplementedError


class WorkspaceHistory2FdCloseError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


def workspace_history2_fd_close(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result[None, WorkspaceHistory2FdCloseError]:
    raise NotImplementedError


class WorkspaceHistory2FdReadError(ErrorVariant):
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


async def workspace_history2_fd_read(
    workspace_history: Handle,
    fd: FileDescriptor,
    offset: U64,
    size: U64,
) -> Result[bytes, WorkspaceHistory2FdReadError]:
    raise NotImplementedError


class WorkspaceHistory2FileStat(Structure):
    id: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt


class WorkspaceHistory2FdStatError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


async def workspace_history2_fd_stat(
    workspace_history: Handle,
    fd: FileDescriptor,
) -> Result[WorkspaceHistory2FileStat, WorkspaceHistory2FdStatError]:
    raise NotImplementedError
