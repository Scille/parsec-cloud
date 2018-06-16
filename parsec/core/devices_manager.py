import re
import logging
from pathlib import Path
from nacl.public import PrivateKey
from nacl.signing import SigningKey
from nacl.pwhash import argon2i
from nacl.secret import SecretBox
import nacl.utils

from parsec.schema import UnknownCheckedSchema, fields, validate
from parsec.core.local_db import LocalDB


logger = logging.getLogger("parsec")


# TODO: SENSITIVE is really slow which is not good for unittests...
# CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_SENSITIVE
# CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_SENSITIVE
CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_INTERACTIVE
CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_INTERACTIVE


class DeviceLoadingError(Exception):
    pass


class DeviceSavingError(Exception):
    pass


USER_ID_PATTERN = r"^[0-9a-zA-Z\-_.]+$"
DEVICE_NAME_PATTERN = r"^[0-9a-zA-Z\-_.]+$"
DEVICE_ID_PATTERN = r"^[0-9a-zA-Z\-_.]+@[0-9a-zA-Z\-_.]+$"


def is_valid_user_id(tocheck):
    return bool(re.match(USER_ID_PATTERN, tocheck))


def is_valid_device_name(tocheck):
    return bool(re.match(DEVICE_NAME_PATTERN, tocheck))


def is_valid_device_id(tocheck):
    return bool(re.match(DEVICE_ID_PATTERN, tocheck))


class DeviceConfSchema(UnknownCheckedSchema):
    device_id = fields.String(validate=validate.Regexp(DEVICE_ID_PATTERN), required=True)
    user_privkey = fields.Base64Bytes(required=True)
    device_signkey = fields.Base64Bytes(required=True)
    user_manifest_access = fields.Base64Bytes(required=True)
    local_symkey = fields.Base64Bytes(required=True)
    encryption = fields.String(validate=validate.OneOf({"quedalle", "password"}), required=True)
    salt = fields.Base64Bytes()


class UserManifestAccessSchema(UnknownCheckedSchema):
    id = fields.String(required=True)
    rts = fields.String(required=True)
    wts = fields.String(required=True)
    key = fields.Base64Bytes(required=True, validate=validate.Length(min=32, max=32))


device_conf_schema = DeviceConfSchema()
user_manifest_access_schema = UserManifestAccessSchema()


def dumps_user_manifest_access(access):
    user_manifest_access_raw, errors = user_manifest_access_schema.dumps(access)
    assert not errors
    return user_manifest_access_raw.encode()


def loads_user_manifest_access(data):
    user_manifest_access, errors = user_manifest_access_schema.loads(data.decode())
    assert not errors
    return user_manifest_access


def _secret_box_factory(password, salt):
    key = argon2i.kdf(
        SecretBox.KEY_SIZE, password, salt, opslimit=CRYPTO_OPSLIMIT, memlimit=CRYPTO_MEMLIMIT
    )
    return SecretBox(key)


def _generate_salt():
    return nacl.utils.random(argon2i.SALTBYTES)


class Device:
    def __repr__(self):
        return f"<{type(self).__name__}(id={self.id!r}, local_db={self.local_db!r})>"

    def __init__(
        self, id, user_privkey, device_signkey, local_symkey, user_manifest_access, local_db
    ):
        assert is_valid_device_id(id)
        self.id = id
        self.user_id, self.device_name = id.split("@")
        self.user_privkey = PrivateKey(user_privkey)
        self.device_signkey = SigningKey(device_signkey)
        self.local_symkey = local_symkey
        self.user_manifest_access = user_manifest_access
        self.local_db = local_db

    @property
    def user_pubkey(self):
        return self.user_privkey.public_key

    @property
    def device_verifykey(self):
        return self.device_signkey.verify_key


class DevicesManager:
    def __init__(self, devices_conf_path):
        self.devices_conf_path = Path(devices_conf_path)

    def list_available_devices(self):
        try:
            candidate_pathes = list(self.devices_conf_path.iterdir())
        except FileNotFoundError:
            return []

        # Sanity check
        devices = []
        for device_path in candidate_pathes:
            _, errors = self._load_device_conf(device_path.name)
            if errors:
                logger.warning(f"Invalid `{device_path}` device config: {errors}")
            else:
                devices.append(device_path.name)
        return devices

    def _load_device_conf(self, device_id):
        errors = {}

        if not is_valid_device_id(device_id):
            errors[device_id] = "Invalid device id"
            return None, errors

        device_key_path = self.devices_conf_path / device_id / "key.json"
        if not device_key_path.is_file():
            errors[device_key_path] = "Missing key file"
            return None, errors

        device_conf, errors = device_conf_schema.loads(device_key_path.read_text())
        if errors:
            return None, errors

        return device_conf, errors

    def register_new_device(
        self, device_id, user_privkey, device_signkey, user_manifest_access, password=None
    ):
        device_conf_path = self.devices_conf_path / device_id
        try:
            device_conf_path.mkdir(parents=True)
        except FileExistsError as exc:
            raise DeviceSavingError(f"Device config `{device_conf_path}` already exists") from exc

        user_manifest_access_raw, _ = user_manifest_access_schema.dumps(user_manifest_access)
        device_conf = {"device_id": device_id}
        local_symkey = nacl.utils.random(nacl.secret.SecretBox.KEY_SIZE)
        if password:
            salt = _generate_salt()
            box = _secret_box_factory(password.encode("utf-8"), salt)
            device_conf["salt"] = salt
            device_conf["encryption"] = "password"
            device_conf["device_signkey"] = box.encrypt(device_signkey)
            device_conf["user_privkey"] = box.encrypt(user_privkey)
            device_conf["local_symkey"] = box.encrypt(local_symkey)
            device_conf["user_manifest_access"] = box.encrypt(user_manifest_access_raw.encode())
        else:
            device_conf["encryption"] = "quedalle"
            # Feel dirty just writting this...
            device_conf["device_signkey"] = device_signkey
            device_conf["user_privkey"] = user_privkey
            device_conf["local_symkey"] = local_symkey
            device_conf["user_manifest_access"] = user_manifest_access_raw.encode()

        device_key_path = device_conf_path / "key.json"
        data, errors = device_conf_schema.dumps(device_conf)
        if errors:
            raise DeviceSavingError(
                f"Invalid device config to save for `{self.devices_conf_path}`: {errors}"
            )

        device_key_path.write_text(data)

    def load_device(self, device_id, password=None):
        device_conf_path = self.devices_conf_path / device_id

        device_conf, errors = self._load_device_conf(device_id)
        if errors:
            raise DeviceLoadingError(f"Invalid {device_conf_path} device config: {errors}")

        if password:
            if device_conf["encryption"] != "password":
                raise DeviceLoadingError(
                    f"Invalid `{self.devices_conf_path}` device config: password "
                    f"provided but encryption is `{device_conf['encryption']}`"
                )

            box = _secret_box_factory(password.encode("utf-8"), device_conf["salt"])
            try:
                user_privkey = box.decrypt(device_conf["user_privkey"])
                device_signkey = box.decrypt(device_conf["device_signkey"])
                local_symkey = box.decrypt(device_conf["local_symkey"])
                user_manifest_access_raw = box.decrypt(device_conf["user_manifest_access"]).decode()
                user_manifest_access, errors = user_manifest_access_schema.loads(
                    user_manifest_access_raw
                )
                # TODO: improve data validation
                assert not errors
            except nacl.exceptions.CryptoError as exc:
                raise DeviceLoadingError(
                    f"Invalid `{device_conf_path}` device config: decryption key failure"
                ) from exc

        else:
            if device_conf["encryption"] != "quedalle":
                raise DeviceLoadingError(
                    f"Invalid `{device_conf_path}` device config: no password "
                    f"provided but encryption is `{device_conf['encryption']}`"
                )

            user_privkey = device_conf["user_privkey"]
            device_signkey = device_conf["device_signkey"]
            local_symkey = device_conf["local_symkey"]
            user_manifest_access, errors = user_manifest_access_schema.loads(
                device_conf["user_manifest_access"]
            )
            assert not errors

        return Device(
            id=device_id,
            user_privkey=user_privkey,
            device_signkey=device_signkey,
            local_symkey=local_symkey,
            local_db=LocalDB(device_conf_path / "local_storage"),
            user_manifest_access=user_manifest_access,
        )
