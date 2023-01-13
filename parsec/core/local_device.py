# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from importlib import import_module
from pathlib import Path, PurePath
from types import ModuleType
from typing import Callable, Dict, Tuple

import trio

from parsec._parsec import AvailableDevice, get_available_device
from parsec.api.data import DataError
from parsec.api.protocol import (
    DeviceID,
    DeviceIDField,
    DeviceLabel,
    DeviceLabelField,
    HumanHandle,
    HumanHandleField,
    OrganizationIDField,
    UserProfile,
)
from parsec.core.types import BackendOrganizationAddr, EntryID, LocalDevice
from parsec.crypto import CryptoError, PrivateKey, SecretKey, SigningKey
from parsec.serde import BaseSchema, MsgpackSerializer, OneOfSchema, fields

# .keys files are not supposed to leave the parsec configuration folder,
# so it's ok to have such a common suffix
DEVICE_FILE_SUFFIX = ".keys"
# .psrk stands for ps(parsec)r(recovery)k(key)
RECOVERY_DEVICE_FILE_SUFFIX = ".psrk"


class LocalDeviceError(Exception):
    pass


class LocalDeviceCryptoError(LocalDeviceError):
    pass


class LocalDeviceCertificatePinCodeUnavailableError(LocalDeviceError):
    pass


class LocalDeviceSignatureError(LocalDeviceError):
    pass


class LocalDeviceNotFoundError(LocalDeviceError):
    pass


class LocalDeviceAlreadyExistsError(LocalDeviceError):
    pass


class LocalDeviceValidationError(LocalDeviceError):
    pass


class LocalDevicePackingError(LocalDeviceError):
    pass


class LocalDeviceCertificateNotFoundError(LocalDeviceError):
    """Used in parsec-extensions for smartcard devices."""


class DeviceFileType(Enum):
    PASSWORD = "password"
    SMARTCARD = "smartcard"
    RECOVERY = "recovery"


class LegacyDeviceFileSchema(BaseSchema):
    """Schema for legacy device files where the filename contains complementary information."""

    type = fields.EnumCheckedConstant(DeviceFileType.PASSWORD, required=True)
    salt = fields.Bytes(required=True)
    ciphertext = fields.Bytes(required=True)

    # Since human_handle/device_label has been introduced, device_id is
    # redacted (i.e. user_id and device_name are 2 random uuids), hence
    # those fields have been added to the device file so the login page in
    # the GUI can use them to provide useful information.
    # Added in Parsec v1.14
    human_handle = HumanHandleField(required=False, allow_none=True, missing=None)
    # Added in Parsec v1.14
    device_label = DeviceLabelField(required=False, allow_none=True, missing=None)


class BaseDeviceFileSchema(BaseSchema):
    """Schema for device files that does not rely on the filename for complementary information."""

    ciphertext = fields.Bytes(required=True)

    # Override those fields to make them required (although `None` is still valid)
    human_handle = HumanHandleField(required=True, allow_none=True)
    device_label = DeviceLabelField(required=True, allow_none=True)

    # Store device ID, organization ID and slug in the device file
    # For legacy versions, this information is available in the file name
    device_id = DeviceIDField(required=True)
    organization_id = OrganizationIDField(required=True)
    slug = fields.String(required=True)


class PasswordDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.PASSWORD, required=True)
    salt = fields.Bytes(required=True)


class RecoveryDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.RECOVERY, required=True)


class SmartcardDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.SMARTCARD, required=True)
    encrypted_key = fields.Bytes(required=True)
    certificate_id = fields.String(required=True)
    certificate_sha1 = fields.Bytes(required=True, allow_none=True)


class DeviceFileSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        DeviceFileType.PASSWORD: PasswordDeviceFileSchema(),
        DeviceFileType.RECOVERY: RecoveryDeviceFileSchema(),
        DeviceFileType.SMARTCARD: SmartcardDeviceFileSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> object:
        return obj["type"]


legacy_key_file_serializer = MsgpackSerializer(
    LegacyDeviceFileSchema,
    validation_exc=LocalDeviceValidationError,
    packing_exc=LocalDevicePackingError,
)

key_file_serializer = MsgpackSerializer(
    DeviceFileSchema, validation_exc=LocalDeviceValidationError, packing_exc=LocalDevicePackingError
)


def generate_new_device(
    organization_addr: BackendOrganizationAddr,
    device_id: DeviceID | None = None,
    profile: UserProfile = UserProfile.STANDARD,
    human_handle: HumanHandle | None = None,
    device_label: DeviceLabel | None = None,
    signing_key: SigningKey | None = None,
    private_key: PrivateKey | None = None,
) -> LocalDevice:
    return LocalDevice(
        organization_addr=organization_addr,
        device_id=device_id or DeviceID.new(),
        device_label=device_label,
        human_handle=human_handle,
        signing_key=signing_key or SigningKey.generate(),
        private_key=private_key or PrivateKey.generate(),
        profile=profile,
        user_manifest_id=EntryID.new(),
        user_manifest_key=SecretKey.generate(),
        local_symkey=SecretKey.generate(),
    )


def get_key_file(config_dir: Path, slug: str) -> Path:
    return Path(get_available_device(config_dir, slug).key_file_path)


def get_default_key_file(config_dir: Path, device: LocalDevice) -> Path:
    """Return the default keyfile path for a given device.

    Note that the filename does not carry any intrinsic meaning.
    Here, we simply use the slughash to avoid name collision.
    """
    return get_devices_dir(config_dir) / f"{device.slughash}{DEVICE_FILE_SUFFIX}"


def get_recovery_device_file_name(recovery_device: LocalDevice | AvailableDevice) -> str:
    return f"parsec-recovery-{recovery_device.organization_id.str}-{recovery_device.short_user_display}{RECOVERY_DEVICE_FILE_SUFFIX}"


def get_devices_dir(config_dir: Path) -> Path:
    return config_dir / "devices"


def load_device_file(key_file_path: Path) -> AvailableDevice | None:
    try:
        return AvailableDevice.load(key_file_path)
    except LocalDeviceError:
        # Not a valid device file, ignore this
        return None


def _load_device(
    key_file: Path, decrypt_ciphertext: Callable[[dict[str, object]], bytes]
) -> LocalDevice:
    try:
        ciphertext = key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Config file `{key_file}` is missing") from exc

    try:
        data = key_file_serializer.loads(ciphertext)
    except LocalDeviceError:
        data = legacy_key_file_serializer.loads(ciphertext)

    plaintext = decrypt_ciphertext(data)

    try:
        local_device = LocalDevice.load(plaintext)

    except DataError as exc:
        raise LocalDeviceValidationError(f"Cannot load local device: {exc}") from exc

    return local_device


def _save_device(
    key_file: Path,
    device: LocalDevice,
    force: bool,
    encrypt_dump: Callable[[bytes], Tuple[DeviceFileType, bytes, dict[str, bytes]]],
) -> None:
    assert key_file.suffix == DEVICE_FILE_SUFFIX

    if key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    cleartext = device.dump()
    type, ciphertext, extra_args = encrypt_dump(cleartext)
    key_file_content = key_file_serializer.dumps(
        {
            "type": type,
            **extra_args,
            "ciphertext": ciphertext,
            "human_handle": device.human_handle,
            "device_label": device.device_label,
            "organization_id": device.organization_id,
            "device_id": device.device_id,
            "slug": device.slug,
        }
    )

    try:
        key_file.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
        key_file.write_bytes(key_file_content)

    except OSError as exc:
        raise LocalDeviceError(f"Cannot save {key_file}: {exc}") from exc


async def load_recovery_device(key_file: PurePath, passphrase: str) -> LocalDevice:
    """
    Raises:
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    key_file = trio.Path(key_file)
    try:
        ciphertext = await key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Recovery file `{key_file}` is missing") from exc

    try:
        data = key_file_serializer.loads(ciphertext)
    except LocalDevicePackingError as exc:
        raise LocalDeviceValidationError("Not a device recovery file") from exc

    if data["type"] != DeviceFileType.RECOVERY:
        raise LocalDeviceValidationError("Not a device recovery file")

    try:
        key = SecretKey.from_recovery_passphrase(passphrase)
    except CryptoError as exc:
        # Not really a crypto operation, but it is more coherent for the caller
        raise LocalDeviceCryptoError("Invalid passphrase") from exc

    try:
        plaintext = key.decrypt(data["ciphertext"])
    except CryptoError as exc:
        raise LocalDeviceCryptoError(str(exc)) from exc

    try:
        return LocalDevice.load(plaintext)

    except DataError as exc:
        raise LocalDeviceValidationError(f"Cannot load local device: {exc}") from exc


async def save_recovery_device(key_file: PurePath, device: LocalDevice, force: bool = False) -> str:
    """
    Return the recovery passphrase
    """
    assert key_file.suffix == RECOVERY_DEVICE_FILE_SUFFIX
    key_file = trio.Path(key_file)

    if await key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    passphrase, key = SecretKey.generate_recovery_passphrase()

    try:
        ciphertext = key.encrypt(device.dump())

    except (CryptoError, DataError) as exc:
        raise LocalDeviceValidationError(f"Cannot dump local device: {exc}") from exc

    key_file_content = key_file_serializer.dumps(
        {
            "type": DeviceFileType.RECOVERY,
            "ciphertext": ciphertext,
            "human_handle": device.human_handle,
            "device_label": device.device_label,
            "organization_id": device.organization_id,
            "device_id": device.device_id,
            "slug": device.slug,
        }
    )

    try:
        await key_file.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
        await key_file.write_bytes(key_file_content)

    except OSError as exc:
        raise LocalDeviceError(f"Cannot save {key_file}: {exc}") from exc

    return passphrase


def _load_smartcard_extension() -> ModuleType:
    try:
        return import_module("parsec_ext.smartcard")
    except ModuleNotFoundError as exc:
        raise ModuleNotFoundError("Parsec smartcard extension not available") from exc


def is_smartcard_extension_available() -> bool:
    try:
        _load_smartcard_extension()
        return True
    except ModuleNotFoundError:
        return False


def load_device_with_smartcard_sync(key_file: Path) -> LocalDevice:
    """
    LocalDeviceError
    LocalDeviceNotFoundError
    LocalDeviceCryptoError
    LocalDeviceValidationError
    LocalDevicePackingError
    LocalDeviceCertificatePinCodeUnavailableError
    """
    return _load_smartcard_extension().load_device_with_smartcard(key_file)


async def load_device_with_smartcard(key_file: Path) -> LocalDevice:
    """
    LocalDeviceError
    LocalDeviceNotFoundError
    LocalDeviceCryptoError
    LocalDeviceValidationError
    LocalDevicePackingError
    LocalDeviceCertificatePinCodeUnavailableError
    """
    return await trio.to_thread.run_sync(load_device_with_smartcard_sync, key_file)


async def save_device_with_smartcard_in_config(
    config_dir: Path,
    device: LocalDevice,
    certificate_id: str | None = None,
    certificate_sha1: bytes | None = None,
) -> Path:
    """
    Raises:
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
        LocalDeviceCertificatePinCodeUnavailableError
    """
    key_file = get_default_key_file(config_dir, device)
    # Why do we use `force=True` here ?
    # Key file name is per-device unique (given it contains the device slughash),
    # hence there is no risk to overwrite another device.
    # So if we are overwriting a key file it could be by:
    # - the same device object, hence overwriting has no effect
    # - a device object with same slughash but different device/user keys
    #   This would mean the device enrollment has been replayed (which is
    #   not possible in theory, but could occur in case of a rollback in the
    #   Parsec server), in this case the old device object is now invalid
    #   and it's a good thing to replace it.
    await trio.to_thread.run_sync(
        lambda: _load_smartcard_extension().save_device_with_smartcard(
            key_file,
            device,
            force=True,
            certificate_id=certificate_id,
            certificate_sha1=certificate_sha1,
        )
    )
    return key_file
