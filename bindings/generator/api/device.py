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
    SecretKey,
    Structure,
    TOTPOpaqueKeyID,
    UserID,
    Variant,
    VariantItemUnit,
    X509CertificateReference,
)
from .config import ClientConfig


class AvailableDeviceType(Variant):
    Keyring = VariantItemUnit
    Password = VariantItemUnit
    Recovery = VariantItemUnit

    class PKI:
        certificate_ref: X509CertificateReference

    AccountVault = VariantItemUnit

    class OpenBao:
        openbao_preferred_auth_id: str
        openbao_entity_id: str


class DevicePrimaryProtectionStrategy(Variant):
    Keyring = VariantItemUnit()

    class Password:
        password: Password

    class PKI:
        certificate_ref: X509CertificateReference

    class AccountVault:
        account_handle: Handle

    class OpenBao:
        openbao_server_url: str
        openbao_secret_mount_path: str
        openbao_transit_mount_path: str
        openbao_entity_id: str
        openbao_auth_token: str
        openbao_preferred_auth_id: str


class DeviceAccessStrategy(Structure):
    key_file: Path
    totp_protection: tuple[TOTPOpaqueKeyID, SecretKey] | None
    primary_protection: DevicePrimaryProtectionStrategy


class DeviceSaveStrategy(Structure):
    totp_protection: tuple[TOTPOpaqueKeyID, SecretKey] | None
    primary_protection: DevicePrimaryProtectionStrategy


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
    totp_opaque_key_id: TOTPOpaqueKeyID | None
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
    class NoSpaceAvailable:
        pass

    class Internal:
        pass


async def archive_device(
    config_dir: Ref[Path],
    device_path: Ref[Path],
) -> Result[None, ArchiveDeviceError]:
    raise NotImplementedError


class RemoveDeviceDataError(ErrorVariant):
    class FailedToRemoveData: ...


async def remove_device_data(
    config: ClientConfig, device_id: DeviceID
) -> Result[None, RemoveDeviceDataError]:
    raise NotImplementedError


class UpdateDeviceError(ErrorVariant):
    class NoSpaceAvailable:
        pass

    class InvalidPath:
        pass

    class InvalidData:
        pass

    class TOTPDecryptionFailed:
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
