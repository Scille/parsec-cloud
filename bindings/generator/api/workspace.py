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
    class NoAccess:
        pass

    class AlreadyStarted:
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


class WorkspaceEntryInfoError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class NotAllowed:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class BadTimestamp:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class Internal:
        pass


class EntryInfo(Variant):
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


async def workspace_entry_info(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[EntryInfo, WorkspaceEntryInfoError]:
    raise NotImplementedError
