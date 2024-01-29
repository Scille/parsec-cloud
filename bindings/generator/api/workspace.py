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


class WorkspaceFsOperationError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class EntryExists:
        pass

    class EntryNotFound:
        pass

    class IsAFolder:
        pass

    class CannotRenameRoot:
        pass

    class NotAFolder:
        pass

    class FolderNotEmpty:
        pass

    class NoRealmAccess:
        pass

    class ReadOnlyRealm:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

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
) -> Result[EntryStat, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_rename_entry(
    workspace: Handle,
    path: Ref[FsPath],
    new_name: EntryName,
    overwrite: bool,
) -> Result[None, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_create_folder(
    workspace: Handle, path: Ref[FsPath]
) -> Result[VlobID, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_create_folder_all(
    workspace: Handle, path: Ref[FsPath]
) -> Result[VlobID, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_create_file(
    workspace: Handle, path: Ref[FsPath]
) -> Result[VlobID, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_remove_entry(
    workspace: Handle, path: Ref[FsPath]
) -> Result[None, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_remove_file(
    workspace: Handle, path: Ref[FsPath]
) -> Result[None, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_remove_folder(
    workspace: Handle, path: Ref[FsPath]
) -> Result[None, WorkspaceFsOperationError]:
    raise NotImplementedError


async def workspace_remove_folder_all(
    workspace: Handle, path: Ref[FsPath]
) -> Result[None, WorkspaceFsOperationError]:
    raise NotImplementedError
