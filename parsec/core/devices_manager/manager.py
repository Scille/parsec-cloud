import attr
from typing import List, Tuple
from pathlib import Path

from parsec.types import DeviceID, BackendOrganizationAddr
from parsec.crypto import SigningKey, PrivateKey, generate_secret_key
from parsec.serde import Serializer, SerdeError, UnknownCheckedSchema, fields, post_load
from parsec.core.types import LocalDevice
from parsec.core.types.access import ManifestAccessSchema, ManifestAccess
from parsec.core.devices_manager.cipher import (
    BaseLocalDeviceEncryptor,
    BaseLocalDeviceDecryptor,
    CipherError,
)


class LocalDeviceSchema(UnknownCheckedSchema):
    backend_addr = fields.BackendOrganizationAddr(required=True)
    device_id = fields.DeviceID(required=True)
    signing_key = fields.SigningKey(required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)
    local_symkey = fields.Bytes(required=True)

    @post_load
    def make_obj(self, data):
        return LocalDevice(**data)


local_device_serializer = Serializer(LocalDeviceSchema)


class DeviceManagerError(Exception):
    pass


class DeviceConfigNotFound(DeviceManagerError):
    pass


class DeviceConfigAleadyExists(DeviceManagerError):
    pass


class DeviceLoadingError(DeviceManagerError):
    pass


class DeviceSavingError(DeviceManagerError):
    pass


def generate_new_device(device_id: DeviceID, backend_addr: BackendOrganizationAddr) -> LocalDevice:
    return LocalDevice(
        backend_addr=backend_addr,
        device_id=device_id,
        signing_key=SigningKey.generate(),
        private_key=PrivateKey.generate(),
        user_manifest_access=ManifestAccess(),
        local_symkey=generate_secret_key(),
    )


# TODO: remove this class: helper functions are easier to work with
@attr.s(frozen=True, slots=True, auto_attribs=True)
class LocalDevicesManager:
    config_path: Path

    @property
    def devices_config_path(self):
        return self.config_path / "devices"

    def _get_key_file(self, device_id: DeviceID):
        return self.devices_config_path / device_id / f"{device_id}.keys"

    def list_available_devices(self) -> List[Tuple[DeviceID, str]]:
        try:
            candidate_pathes = list(self.devices_config_path.iterdir())
        except FileNotFoundError:
            return []

        # Sanity checks
        devices = []
        for device_path in candidate_pathes:
            try:
                device_id = DeviceID(device_path.name)
            except ValueError:
                continue

            try:
                cipher = self.get_cipher_info(device_id)
                devices.append((device_id, cipher))

            except DeviceManagerError:
                continue

        return devices

    def get_cipher_info(self, device_id: DeviceID) -> str:
        from .pkcs11_cipher import PKCS11DeviceDecryptor
        from .cipher import PasswordDeviceDecryptor

        key_file = self._get_key_file(device_id)
        try:
            ciphertext = key_file.read_bytes()
        except OSError as exc:
            raise DeviceConfigNotFound(f"Config file {key_file} is missing")

        for decryptor_cls, cipher in (
            (PKCS11DeviceDecryptor, "pkcs11"),
            (PasswordDeviceDecryptor, "password"),
        ):
            if decryptor_cls.can_decrypt(ciphertext):
                return cipher

        raise DeviceLoadingError(f"Unknown cipher for {key_file}")

    def load_device(self, device_id: DeviceID, decryptor: BaseLocalDeviceDecryptor) -> LocalDevice:
        """
        Raises:
            DeviceManagerError
        """
        key_file = self._get_key_file(device_id)

        try:
            ciphertext = key_file.read_bytes()
        except OSError as exc:
            raise DeviceConfigNotFound(f"Config file {key_file} is missing") from exc

        try:
            raw = decryptor.decrypt(ciphertext)
            return local_device_serializer.loads(raw)

        except (CipherError, SerdeError) as exc:
            raise DeviceLoadingError(f"Cannot load {key_file}: {exc}") from exc

    def save_device(
        self, device: LocalDevice, encryptor: BaseLocalDeviceEncryptor, force: bool = False
    ) -> None:
        """
        Raises:
            DeviceManagerError
        """
        key_file = self._get_key_file(device.device_id)
        if key_file.exists():
            if force:
                key_file.unlink()
            else:
                raise DeviceConfigAleadyExists(f"Device {device.device_id} already exists")

        try:
            raw = local_device_serializer.dumps(device)
            ciphertext = encryptor.encrypt(raw)

            key_file.parent.mkdir(exist_ok=True, parents=True)
            key_file.write_bytes(ciphertext)

        except (CipherError, SerdeError, OSError) as exc:
            raise DeviceSavingError(f"Cannot save {key_file}: {exc}") from exc
