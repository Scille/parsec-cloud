# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from .common import (
    EntryID,
    EntryName,
    ErrorVariant,
    Handle,
    Password,
    Path,
    Result,
    Variant,
)
from .config import ClientConfig
from .events import OnClientEventCallback


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


async def client_stop(handle: Handle) -> Result[None, ClientStopError]:
    ...


class ClientListWorkspacesError(ErrorVariant):
    class Internal:
        pass


async def client_list_workspaces(
    handle: Handle,
) -> Result[list[tuple[EntryID, EntryName]], ClientListWorkspacesError]:
    ...
