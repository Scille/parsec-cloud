# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from importlib import import_module
from pathlib import Path
from types import ModuleType
from typing import Callable, Tuple

import trio

from parsec._parsec import AvailableDevice, DeviceFile, DeviceFileType, get_available_device
from parsec.api.data import DataError
from parsec.core.types import LocalDevice

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


async def get_key_file(config_dir: Path, device: str) -> Path:
    return Path((await get_available_device(config_dir, device)).key_file_path)


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


async def load_device_file(key_file_path: Path) -> AvailableDevice | None:
    try:
        return await AvailableDevice.load(key_file_path)
    except LocalDeviceError:
        # Not a valid device file, ignore this
        return None


def _load_device(key_file: Path, decrypt_ciphertext: Callable[[DeviceFile], bytes]) -> LocalDevice:
    try:
        ciphertext = key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Config file `{key_file}` is missing") from exc

    data = DeviceFile.load(ciphertext)

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
    encrypt_dump: Callable[
        [bytes], Tuple[DeviceFileType, bytes, bytes | None, bytes | None, str | None, bytes | None]
    ],
) -> None:
    assert key_file.suffix == DEVICE_FILE_SUFFIX

    if key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    cleartext = device.dump()
    type, ciphertext, salt, encrypted_key, certificate_id, certificate_sha1 = encrypt_dump(
        cleartext
    )
    key_file_content = DeviceFile(
        type=type,
        ciphertext=ciphertext,
        human_handle=device.human_handle,
        device_label=device.device_label,
        device_id=device.device_id,
        organization_id=device.organization_id,
        slug=device.slug,
        salt=salt,
        encrypted_key=encrypted_key,
        certificate_id=certificate_id,
        certificate_sha1=certificate_sha1,
    ).dump()

    try:
        key_file.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
        key_file.write_bytes(key_file_content)

    except OSError as exc:
        raise LocalDeviceError(f"Cannot save {key_file}: {exc}") from exc


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
