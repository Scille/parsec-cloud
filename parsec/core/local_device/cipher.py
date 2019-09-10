# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.crypto import CryptoError, derivate_secret_key_from_password
from parsec.serde import MsgpackSerializer, BaseSchema, fields
from parsec.core.local_device.exceptions import (
    LocalDevicePackingError,
    LocalDeviceValidationError,
    LocalDeviceCryptoError,
)


class BaseLocalDeviceEncryptor:
    def encrypt(self, plain: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        raise NotImplementedError()


class BaseLocalDeviceDecryptor:
    def decrypt(self, ciphered: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        raise NotImplementedError()

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        raise NotImplementedError()


class PasswordPayloadSchema(BaseSchema):
    type = fields.CheckedConstant("password", required=True)
    salt = fields.Bytes(required=True)
    ciphertext = fields.Bytes(required=True)


password_payload_serializer = MsgpackSerializer(
    PasswordPayloadSchema,
    validation_exc=LocalDeviceValidationError,
    packing_exc=LocalDevicePackingError,
)


class PasswordDeviceEncryptor(BaseLocalDeviceEncryptor):
    def __init__(self, password):
        self.password = password

    def encrypt(self, plaintext: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        try:
            key, salt = derivate_secret_key_from_password(self.password)
            ciphertext = key.encrypt(plaintext)

        except CryptoError as exc:
            raise LocalDeviceCryptoError(str(exc)) from exc

        return password_payload_serializer.dumps({"salt": salt, "ciphertext": ciphertext})


class PasswordDeviceDecryptor(BaseLocalDeviceDecryptor):
    def __init__(self, password):
        self.password = password

    def decrypt(self, ciphertext: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        payload = password_payload_serializer.loads(ciphertext)
        try:
            key, _ = derivate_secret_key_from_password(self.password, payload["salt"])
            return key.decrypt(payload["ciphertext"])

        except CryptoError as exc:
            raise LocalDeviceCryptoError(str(exc)) from exc

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        try:
            password_payload_serializer.loads(ciphertext)
            return True

        except (LocalDeviceValidationError, LocalDevicePackingError):
            return False
