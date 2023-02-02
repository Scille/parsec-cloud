# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Callable, Optional

from .common import *
from .events import ClientEvent


class DeviceFileType(Variant):
    class Password:
        pass

    class Recovery:
        pass

    class Smartcard:
        pass


class AvailableDevice(Structure):
    key_file_path: Path
    organization_id: OrganizationID
    device_id: DeviceID
    human_handle: Optional[HumanHandle]
    device_label: Optional[DeviceLabel]
    slug: str
    ty: DeviceFileType


class CacheSize(I32BasedType):
    pass


class WorkspaceStorageCacheSize(Variant):
    class Default:
        pass

    class Custom:
        size: CacheSize


class ClientConfig(Structure):
    config_dir: Path
    data_base_dir: Path
    mountpoint_base_dir: Path
    preferred_org_creation_backend_addr: BackendAddr
    workspace_storage_cache_size: WorkspaceStorageCacheSize


class DeviceAccessParams(Variant):
    class Password:
        path: Path
        password: str

    class Smartcard:
        path: Path


class OnClientEventCallback(Callable[[ClientEvent], None]):
    event_type = ClientEvent


async def client_list_available_devices(path: Ref[Path]) -> list[AvailableDevice]:
    ...


class ClientLoginError(Variant):
    class DeviceAlreadyLoggedIn:
        pass

    class AccessMethodNotAvailable:
        pass

    class DecryptionFailed:
        pass

    class DeviceInvalidFormat:
        pass


async def client_login(
    load_device_params: DeviceAccessParams,
    config: ClientConfig,
    on_event_callback: OnClientEventCallback,
) -> Result[ClientHandle, ClientLoginError]:
    ...


class ClientGetterError(Variant):
    class Disconnected:
        pass

    class InvalidHandle:
        handle: ClientHandle


async def client_get_device_id(handle: ClientHandle) -> Result[DeviceID, ClientGetterError]:
    ...
