# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from .addr import ParsecAddr
from .common import (
    AccountVaultItemOpaqueKeyID,
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
    SecretKey,
    Structure,
    UserID,
    Variant,
    VariantItemUnit,
)
from .pki import X509CertificateReference


class OpenBaoAuthType(Enum):
    Hexagone = EnumItemUnit
    AgentConnect = EnumItemUnit


class AvailableDeviceType(Variant):
    Keyring = VariantItemUnit
    Password = VariantItemUnit
    Recovery = VariantItemUnit
    Smartcard = VariantItemUnit

    class AccountVault:
        ciphertext_key_id: AccountVaultItemOpaqueKeyID

    class OpenBao:
        openbao_url: str
        openbao_ciphertext_key_path: str
        openbao_auth_path: str
        openbao_auth_type: OpenBaoAuthType


class DeviceSaveStrategy(Variant):
    class Keyring:
        pass

    class Password:
        password: Password

    class Smartcard:
        certificate_reference: X509CertificateReference

    class AccountVault:
        ciphertext_key_id: AccountVaultItemOpaqueKeyID
        ciphertext_key: SecretKey

    class OpenBao:
        openbao_url: str
        openbao_ciphertext_key_path: str
        openbao_auth_path: str
        openbao_auth_type: OpenBaoAuthType
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

    class OpenBao:
        key_file: Path
        openbao_url: str
        openbao_ciphertext_key_path: str
        openbao_auth_path: str
        openbao_auth_type: OpenBaoAuthType
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
