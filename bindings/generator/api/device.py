# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import (
    DateTime,
    DeviceID,
    DeviceLabel,
    Enum,
    EnumItemUnit,
    ErrorVariant,
    HumanHandle,
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    Structure,
    UserID,
    Variant,
)


class DeviceFileType(Enum):
    Keyring = EnumItemUnit
    Password = EnumItemUnit
    Recovery = EnumItemUnit
    Smartcard = EnumItemUnit
    Biometrics = EnumItemUnit


class DeviceSaveStrategy(Variant):
    class Keyring:
        pass

    class Password:
        password: Password

    class Smartcard:
        pass

    class Biometrics:
        pass


class AvailableDevice(Structure):
    key_file_path: Path
    created_on: DateTime
    protected_on: DateTime
    server_url: str
    organization_id: OrganizationID
    user_id: UserID
    device_id: DeviceID
    human_handle: HumanHandle
    device_label: DeviceLabel
    ty: DeviceFileType


async def list_available_devices(path: Ref[Path]) -> list[AvailableDevice]:
    raise NotImplementedError


class ArchiveDeviceError(ErrorVariant):
    class Internal:
        pass


async def archive_device(device_path: Ref[Path]) -> Result[None, ArchiveDeviceError]:
    raise NotImplementedError
