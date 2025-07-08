# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .common import (
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
    Structure,
    UserID,
    Variant,
    SecretKey,
    VariantItemUnit,
    AccountVaultItemOpaqueKeyID,
)
from .addr import ParsecAddr


class AvailableDeviceType(Variant):
    Keyring = VariantItemUnit
    Password = VariantItemUnit
    Recovery = VariantItemUnit
    Smartcard = VariantItemUnit

    class AccountVault:
        ciphertext_key_id: AccountVaultItemOpaqueKeyID


class DeviceSaveStrategy(Variant):
    class Keyring:
        pass

    class Password:
        password: Password

    class Smartcard:
        pass

    class AccountVault:
        ciphertext_key_id: AccountVaultItemOpaqueKeyID
        ciphertext_key: SecretKey


class DeviceAccessStrategy(Variant):
    class Keyring:
        key_file: Path

    class Password:
        password: Password
        key_file: Path

    class Smartcard:
        key_file: Path

    class AccountVault:
        key_file: Path
        ciphertext_key_id: AccountVaultItemOpaqueKeyID
        ciphertext_key: SecretKey


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
    ty: AvailableDeviceType


class ListAvailableDeviceError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class Internal:
        pass


async def list_available_devices(
    path: Ref[Path],
) -> Result[list[AvailableDevice], ListAvailableDeviceError]:
    raise NotImplementedError


class ArchiveDeviceError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class Internal:
        pass


async def archive_device(
    config_dir: Ref[Path],
    device_path: Ref[Path],
) -> Result[None, ArchiveDeviceError]:
    raise NotImplementedError


class UpdateDeviceError(ErrorVariant):
    class StorageNotAvailable:
        pass

    class InvalidPath:
        pass

    class InvalidData:
        pass

    class DecryptionFailed:
        pass

    class Internal:
        pass


async def update_device_change_authentication(
    config_dir: Ref[Path],
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy,
) -> Result[AvailableDevice, UpdateDeviceError]:
    raise NotImplementedError


async def update_device_overwrite_server_addr(
    config_dir: Ref[Path],
    access: DeviceAccessStrategy,
    new_server_addr: ParsecAddr,
) -> Result[ParsecAddr, UpdateDeviceError]:
    raise NotImplementedError
