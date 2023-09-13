# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    DateTime,
    EntryName,
    Enum,
    EnumItemUnit,
    ErrorVariant,
    Handle,
    Password,
    Path,
    RealmID,
    Result,
    UserID,
    Variant,
)
from .config import ClientConfig
from .events import OnClientEventCallback


class RealmRole(Enum):
    Owner = EnumItemUnit
    Manager = EnumItemUnit
    Contributor = EnumItemUnit
    Reader = EnumItemUnit


class DeviceAccessStrategy(Variant):
    class Password:
        password: Password
        key_file: Path

    class Smartcard:
        key_file: Path


class ClientStartError(ErrorVariant):
    class LoadDeviceInvalidPath:
        pass

    class LoadDeviceInvalidData:
        pass

    class LoadDeviceDecryptionFailed:
        pass

    class Internal:
        pass


async def client_start(
    config: ClientConfig, on_event_callback: OnClientEventCallback, access: DeviceAccessStrategy
) -> Result[Handle, ClientStartError]:
    raise NotImplementedError


class ClientStopError(ErrorVariant):
    class Internal:
        pass


async def client_stop(client: Handle) -> Result[None, ClientStopError]:
    raise NotImplementedError


class ClientListWorkspacesError(ErrorVariant):
    class Internal:
        pass


async def client_list_workspaces(
    client: Handle,
) -> Result[list[tuple[RealmID, EntryName]], ClientListWorkspacesError]:
    raise NotImplementedError


class ClientWorkspaceCreateError(ErrorVariant):
    class Internal:
        pass


async def client_workspace_create(
    client: Handle,
    name: EntryName,
) -> Result[RealmID, ClientWorkspaceCreateError]:
    raise NotImplementedError


class ClientWorkspaceRenameError(ErrorVariant):
    class UnknownWorkspace:
        pass

    class Internal:
        pass


async def client_workspace_rename(
    client: Handle,
    realm_id: RealmID,
    new_name: EntryName,
) -> Result[None, ClientWorkspaceRenameError]:
    raise NotImplementedError


class ClientWorkspaceShareError(ErrorVariant):
    class ShareToSelf:
        pass

    class UnknownWorkspace:
        pass

    class UnknownRecipient:
        pass

    class UnknownRecipientOrWorkspace:
        pass

    class RevokedRecipient:
        pass

    class WorkspaceInMaintenance:
        pass

    class NotAllowed:
        pass

    class OutsiderCannotBeManagerOrOwner:
        pass

    class Offline:
        pass

    class BadTimestamp:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class Internal:
        pass


async def client_workspace_share(
    client: Handle,
    realm_id: RealmID,
    recipient: UserID,
    role: Optional[RealmRole],
) -> Result[None, ClientWorkspaceShareError]:
    raise NotImplementedError
