# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    DateTime,
    EntryID,
    EntryName,
    ErrorVariant,
    Handle,
    Password,
    Path,
    Result,
    UserID,
    Variant,
    VariantItemUnit,
)
from .config import ClientConfig
from .events import OnClientEventCallback


class RealmRole(Variant):
    Owner = VariantItemUnit
    Manager = VariantItemUnit
    Contributor = VariantItemUnit
    Reader = VariantItemUnit


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
    ...


class ClientStopError(ErrorVariant):
    class Internal:
        pass


async def client_stop(client: Handle) -> Result[None, ClientStopError]:
    ...


class ClientListWorkspacesError(ErrorVariant):
    class Internal:
        pass


async def client_list_workspaces(
    client: Handle,
) -> Result[list[tuple[EntryID, EntryName]], ClientListWorkspacesError]:
    ...


class ClientWorkspaceCreateError(ErrorVariant):
    class Internal:
        pass


async def client_workspace_create(
    client: Handle,
    name: EntryName,
) -> Result[EntryID, ClientWorkspaceCreateError]:
    ...


class UserOpsError(ErrorVariant):
    class UnknownWorkspace:
        pass

    class Internal:
        pass


async def client_workspace_rename(
    client: Handle,
    workspace_id: EntryID,
    new_name: EntryName,
) -> Result[None, UserOpsError]:
    ...


class UserOpsWorkspaceShareError(ErrorVariant):
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
    workspace_id: EntryID,
    recipient: UserID,
    role: Optional[RealmRole],
) -> Result[None, UserOpsWorkspaceShareError]:
    ...
