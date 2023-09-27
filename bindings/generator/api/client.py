# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    DateTime,
    DeviceID,
    DeviceLabel,
    EntryName,
    Enum,
    EnumItemUnit,
    ErrorVariant,
    Handle,
    HumanHandle,
    OrganizationID,
    Password,
    Path,
    Result,
    Structure,
    UserID,
    UserProfile,
    Variant,
    VlobID,
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
    class DeviceAlreadyRunning:
        pass

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


class ClientInfoError(ErrorVariant):
    class Internal:
        pass


class ClientInfo(Structure):
    organization_id: OrganizationID
    device_id: DeviceID
    user_id: UserID
    device_label: Optional[DeviceLabel]
    human_handle: Optional[HumanHandle]
    current_profile: UserProfile


async def client_info(
    client: Handle,
) -> Result[ClientInfo, ClientInfoError]:
    raise NotImplementedError


class ClientListWorkspacesError(ErrorVariant):
    class Internal:
        pass


async def client_list_workspaces(
    client: Handle,
) -> Result[list[tuple[VlobID, EntryName]], ClientListWorkspacesError]:
    raise NotImplementedError


class ClientCreateWorkspaceError(ErrorVariant):
    class Internal:
        pass


async def client_create_workspace(
    client: Handle,
    name: EntryName,
) -> Result[VlobID, ClientCreateWorkspaceError]:
    raise NotImplementedError


class ClientRenameWorkspaceError(ErrorVariant):
    class UnknownWorkspace:
        pass

    class Internal:
        pass


async def client_rename_workspace(
    client: Handle,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result[None, ClientRenameWorkspaceError]:
    raise NotImplementedError


class ClientShareWorkspaceError(ErrorVariant):
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


async def client_share_workspace(
    client: Handle,
    realm_id: VlobID,
    recipient: UserID,
    role: Optional[RealmRole],
) -> Result[None, ClientShareWorkspaceError]:
    raise NotImplementedError
