import attr
from typing import List, Tuple
from pathlib import Path
from json import JSONDecodeError

from parsec.types import DeviceID
from parsec.crypto import SigningKey, VerifyKey, PrivateKey, generate_secret_key
from parsec.schema import UnknownCheckedSchema, fields, ValidationError, post_load
from parsec.core.schemas import ManifestAccessSchema
from parsec.core.types import LocalDevice
from parsec.core.devices_manager.cipher import (
    BaseLocalDeviceEncryptor,
    BaseLocalDeviceDecryptor,
    CipherError,
)
from parsec.core.fs.utils import new_access


class LocalDeviceSchema(UnknownCheckedSchema):
    backend_addr = fields.String(required=True)
    root_verify_key = fields.VerifyKey(required=True)
    device_id = fields.DeviceID(required=True)
    signing_key = fields.SigningKey(required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)
    local_symkey = fields.Base64Bytes(required=True)

    @post_load
    def make_obj(self, data):
        return LocalDevice(**data)


local_device_schema = LocalDeviceSchema(strict=True)


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


# TODO: replace backend_addr by a pasec.types.BackendOrganizationAddr
# so we can remove root_verify_key
def generate_new_device(
    device_id: DeviceID, backend_addr: str, root_verify_key: VerifyKey
) -> LocalDevice:
    return LocalDevice(
        backend_addr=backend_addr,
        root_verify_key=root_verify_key,
        device_id=device_id,
        signing_key=SigningKey.generate(),
        private_key=PrivateKey.generate(),
        user_manifest_access=new_access(),
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
            raise DeviceConfigNotFound(str(key_file))

        for decryptor_cls, cipher in (
            (PKCS11DeviceDecryptor, "pkcs11"),
            (PasswordDeviceDecryptor, "password"),
        ):
            if decryptor_cls.can_decrypt(ciphertext):
                return cipher

        raise DeviceLoadingError("Unknown cipher.")

    def load_device(self, device_id: DeviceID, decryptor: BaseLocalDeviceDecryptor) -> LocalDevice:
        """
        Raises:
            DeviceManagerError
        """
        key_file = self._get_key_file(device_id)
        if not key_file.exists():
            raise DeviceConfigNotFound(str(key_file))

        try:
            ciphertext = key_file.read_bytes()
            raw = decryptor.decrypt(ciphertext)
            return local_device_schema.loads(raw.decode("utf8")).data

        except (CipherError, ValidationError, JSONDecodeError, ValueError, OSError) as exc:
            raise DeviceLoadingError(str(exc)) from exc

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
                raise DeviceConfigAleadyExists(str(key_file))

        try:
            raw = local_device_schema.dumps(device).data.encode("utf8")
            ciphertext = encryptor.encrypt(raw)

            key_file.parent.mkdir(exist_ok=True, parents=True)
            key_file.write_bytes(ciphertext)

        except (CipherError, ValidationError, JSONDecodeError, ValueError, OSError) as exc:
            raise DeviceSavingError(str(exc)) from exc
