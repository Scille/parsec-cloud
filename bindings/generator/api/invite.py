# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Callable, Optional

from .common import (
    BackendOrganizationBootstrapAddr,
    DateTime,
    DeviceID,
    DeviceLabel,
    ErrorVariant,
    HumanHandle,
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    SequesterVerifyKeyDer,
    Structure,
    Variant,
)
from .config import ClientConfig
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


async def list_available_devices(path: Ref[Path]) -> list[AvailableDevice]:
    ...


class DeviceSaveStrategy(Variant):
    class Password:
        password: Password

    class Smartcard:
        pass


class OnClientEventCallback(Callable[[ClientEvent], None]):
    pass


class BootstrapOrganizationError(ErrorVariant):
    class Offline:
        pass

    class InvalidToken:
        pass

    class AlreadyUsedToken:
        pass

    class BadTimestamp:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class SaveDeviceError:
        pass

    class Internal:
        pass


async def bootstrap_organization(
    config: ClientConfig,
    on_event_callback: OnClientEventCallback,
    bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: Optional[HumanHandle],
    device_label: Optional[DeviceLabel],
    sequester_authority_verify_key: Optional[SequesterVerifyKeyDer],
) -> Result[AvailableDevice, BootstrapOrganizationError]:
    ...
