# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
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
)


class ClientStartWorkspaceError(ErrorVariant):
    class WorkspaceNotFound:
        pass

    class Internal:
        pass


async def client_start_workspace(
    client: Handle, realm_id: VlobID
) -> Result[Handle, ClientStartWorkspaceError]:
    raise NotImplementedError


class WorkspaceStopError(ErrorVariant):
    class Internal:
        pass


async def workspace_stop(workspace: Handle) -> Result[None, WorkspaceStopError]:
    raise NotImplementedError


class WorkspaceCreateFileError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class ParentNotFound:
        pass

    class ParentIsFile:
        pass

    class EntryExists:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceCreateFolderError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class ParentNotFound:
        pass

    class ParentIsFile:
        pass

    class EntryExists:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceRemoveEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class CannotRemoveRoot:
        pass

    class EntryNotFound:
        pass

    class NoRealmAccess:
        pass

    class EntryIsFile:
        pass

    class EntryIsFolder:
        pass

    class EntryIsNonEmptyFolder:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceRenameEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class CannotRenameRoot:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class DestinationExists:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceStatEntryError(ErrorVariant):
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


class EntryStat(Variant):
    class File:
        confinement_point: Optional[VlobID]
        id: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        size: SizeInt

    class Folder:
        confinement_point: Optional[VlobID]
        id: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        children: list[EntryName]


async def workspace_stat_entry(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[EntryStat, WorkspaceStatEntryError]:
    raise NotImplementedError


async def workspace_rename_entry(
    workspace: Handle,
    path: FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result[None, WorkspaceRenameEntryError]:
    raise NotImplementedError


async def workspace_create_folder(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFolderError]:
    raise NotImplementedError


async def workspace_create_folder_all(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFolderError]:
    raise NotImplementedError


async def workspace_create_file(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFileError]:
    raise NotImplementedError


async def workspace_remove_entry(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_file(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_folder(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_folder_all(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError
