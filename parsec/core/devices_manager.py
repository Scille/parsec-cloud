import os
import re
import logging
import attr
from nacl.public import PrivateKey
from nacl.signing import SigningKey
from nacl.pwhash import argon2i
from nacl.secret import SecretBox
import nacl.utils

from parsec.schema import UnknownCheckedSchema, fields, validate


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
    encryption = fields.String(validate=validate.OneOf({"quedalle", "password"}), required=True)
    salt = fields.Base64Bytes()


device_conf_schema = DeviceConfSchema()


def _secret_box_factory(password, salt):
    key = argon2i.kdf(
        SecretBox.KEY_SIZE, password, salt, opslimit=CRYPTO_OPSLIMIT, memlimit=CRYPTO_MEMLIMIT
    )
    return SecretBox(key)


def _generate_salt():
    return nacl.utils.random(argon2i.SALTBYTES)


@attr.s(init=False, slots=True)
class Device:
    id = attr.ib()
    user_privkey = attr.ib()
    device_signkey = attr.ib()
    local_storage_db_path = attr.ib()

    def __init__(self, id, user_privkey, device_signkey, local_storage_db_path):
        assert is_valid_device_id(id)
        self.id = id
        self.local_storage_db_path = local_storage_db_path
        self.user_privkey = PrivateKey(user_privkey)
        self.device_signkey = SigningKey(device_signkey)

    @property
    def user_id(self):
        return self.id.split("@")[0]

    @property
    def device_name(self):
        return self.id.split("@")[1]

    @property
    def user_pubkey(self):
        return self.user_privkey.public_key

    @property
    def device_verifykey(self):
        return self.device_signkey.verify_key


class DevicesManager:

    def __init__(self, devices_conf_path):
        self.devices_conf_path = devices_conf_path

    def list_available_devices(self):
        devices = []
        try:
            device_ids = os.listdir(self.devices_conf_path)
        except FileNotFoundError:
            return devices

        for device_id in device_ids:
            _, errors = self._load_device_conf(device_id)
            if errors:
                device_conf_path = os.path.join(self.devices_conf_path, device_id)
                logger.warning(
                    "Invalid %s device config:\n%s"
                    % (device_conf_path, "\n".join(["- %s: %s" % kv for kv in errors.items()]))
                )
            else:
                devices.append(device_id)
        return devices

    def _load_device_conf(self, device_id):
        errors = {}

        if not is_valid_device_id(device_id):
            errors[device_id] = "Invalid device id"
            return None, errors

        device_key_path = os.path.join(self.devices_conf_path, device_id, "key.json")
        if not os.path.isfile(device_key_path):
            errors[device_key_path] = "Missing key file"
            return None, errors

        with open(device_key_path) as fd:
            device_conf, errors = device_conf_schema.loads(fd.read())
        if errors:
            return None, errors

        return device_conf, errors

    def _ensure_devices_conf_path_exists(self):
        try:
            os.mkdir(self.devices_conf_path)
        except FileExistsError:
            pass

    def register_new_device(self, device_id, user_privkey, device_signkey, password=None):
        self._ensure_devices_conf_path_exists()
        device_conf_path = os.path.join(self.devices_conf_path, device_id)
        if os.path.exists(device_conf_path):
            raise DeviceSavingError("Device config %s already exists" % device_conf_path)

        os.mkdir(device_conf_path)

        device_conf = {"device_id": device_id}
        if password:
            salt = _generate_salt()
            box = _secret_box_factory(password.encode("utf-8"), salt)
            device_conf["salt"] = salt
            device_conf["encryption"] = "password"
            device_conf["device_signkey"] = box.encrypt(device_signkey)
            device_conf["user_privkey"] = box.encrypt(user_privkey)
        else:
            device_conf["encryption"] = "quedalle"
            # Feel dirty just writting this...
            device_conf["device_signkey"] = device_signkey
            device_conf["user_privkey"] = user_privkey

        device_key_path = os.path.join(device_conf_path, "key.json")
        data, errors = device_conf_schema.dumps(device_conf)
        if errors:
            raise DeviceSavingError(
                "Invalid device config to save for %s: %s" % (device_conf_path, errors)
            )

        with open(device_key_path, "w") as fd:
            fd.write(data)

    def load_device(self, device_id, password=None):
        device_conf_path = os.path.join(self.devices_conf_path, device_id)
        local_storage_db_path = os.path.join(device_conf_path, "local_storage.sqlite")

        device_conf, errors = self._load_device_conf(device_id)
        if errors:
            raise DeviceLoadingError(
                "Invalid %s device config:\n%s"
                % (device_conf_path, "\n".join(["- %s: %s" % kv for kv in errors.items()]))
            )

        if password:
            if device_conf["encryption"] != "password":
                raise DeviceLoadingError(
                    "Invalid %s device config: password provided but encryption is %s"
                    % (device_conf_path, device_conf["encryption"])
                )

            box = _secret_box_factory(password.encode("utf-8"), device_conf["salt"])
            try:
                user_privkey = box.decrypt(device_conf["user_privkey"])
                device_signkey = box.decrypt(device_conf["device_signkey"])
            except nacl.exceptions.CryptoError as exc:
                raise DeviceLoadingError(
                    "Invalid %s device config: decryption key failure" % device_conf_path
                ) from exc

        else:
            if device_conf["encryption"] != "quedalle":
                raise DeviceLoadingError(
                    "Invalid %s device config: no password provided but encryption is %s"
                    % (device_conf_path, device_conf["encryption"])
                )

            user_privkey = device_conf["user_privkey"]
            device_signkey = device_conf["device_signkey"]

        return Device(
            id=device_id,
            user_privkey=user_privkey,
            device_signkey=device_signkey,
            local_storage_db_path=local_storage_db_path,
        )
