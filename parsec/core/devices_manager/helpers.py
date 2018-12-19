from typing import List, Tuple
from pathlib import Path

from parsec.types import DeviceID
from parsec.core.types import LocalDevice
from parsec.core.devices_manager.manager import LocalDevicesManager
from parsec.core.devices_manager.cipher import PasswordDeviceDecryptor, PasswordDeviceEncryptor
from parsec.core.devices_manager.pkcs11_cipher import PKCS11DeviceDecryptor, PKCS11DeviceEncryptor


def get_cipher_info(config_dir: Path, device_id: DeviceID) -> str:
    return LocalDevicesManager(config_dir).get_cipher_info(device_id)


def list_available_devices(config_dir: Path) -> List[Tuple[DeviceID, str]]:
    return LocalDevicesManager(config_dir).list_available_devices()


def load_device_with_password(config_dir: Path, device_id: DeviceID, password: str) -> LocalDevice:
    decryptor = PasswordDeviceDecryptor(password)
    return LocalDevicesManager(config_dir).load_device(device_id, decryptor)


def save_device_with_password(
    config_dir: Path, device: LocalDevice, password: str, force: bool = False
) -> None:
    encryptor = PasswordDeviceEncryptor(password)
    LocalDevicesManager(config_dir).save_device(device, encryptor, force)


def load_device_with_pkcs11(
    config_dir: Path, device_id: DeviceID, token_id: int, key_id: int, pin: str
) -> LocalDevice:
    decryptor = PKCS11DeviceDecryptor(token_id, key_id, pin)
    return LocalDevicesManager(config_dir).load_device(device_id, decryptor)


def save_device_with_pkcs11(
    config_dir: Path, device: LocalDevice, token_id: int, key_id: int
) -> None:
    encryptor = PKCS11DeviceEncryptor(token_id, key_id)
    LocalDevicesManager(config_dir).save_device(device, encryptor)
