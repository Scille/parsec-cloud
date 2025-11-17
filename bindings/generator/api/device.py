# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .addr import ParsecAddr
from .common import (
    DateTime,
    DeviceID,
    DeviceLabel,
    ErrorVariant,
    Handle,
    HumanHandle,
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    Structure,
    UserID,
    Variant,
    VariantItemUnit,
    X509CertificateReference,
)


class AvailableDeviceType(Variant):
    Keyring = VariantItemUnit
    Password = VariantItemUnit
    Recovery = VariantItemUnit
    Smartcard = VariantItemUnit
    AccountVault = VariantItemUnit

    class OpenBao:
        openbao_preferred_auth_id: str
        openbao_entity_id: str


class DeviceSaveStrategy(Variant):
    class Keyring:
        pass

    class Password:
        password: Password

    class Smartcard:
        certificate_reference: X509CertificateReference

    class AccountVault:
        account_handle: Handle

    class OpenBao:
        openbao_server_url: str
        openbao_secret_mount_path: str
        openbao_entity_id: str
        openbao_auth_token: str
        openbao_preferred_auth_id: str


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
        account_handle: Handle

    class OpenBao:
        key_file: Path
        openbao_server_url: str
        openbao_secret_mount_path: str
        openbao_entity_id: str
        openbao_auth_token: str


class AvailableDevice(Structure):
    key_file_path: Path
    created_on: DateTime
    protected_on: DateTime
    server_addr: ParsecAddr
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

    class RemoteOpaqueKeyOperationOffline:
        pass

    class RemoteOpaqueKeyOperationFailed:
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
