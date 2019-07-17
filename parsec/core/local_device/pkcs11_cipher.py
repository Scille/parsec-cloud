# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.local_device.cipher import BaseLocalDeviceDecryptor, BaseLocalDeviceEncryptor
from parsec.core.local_device.exceptions import (
    LocalDeviceCryptoError,
    LocalDevicePackingError,
    LocalDeviceValidationError,
)
from parsec.core.local_device.pkcs11_tools import (
    DevicePKCS11Error,
    decrypt_data,
    encrypt_data,
    get_LIB,
)
from parsec.serde import Serializer, UnknownCheckedSchema, fields


class PKCS11PayloadSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("PKCS11", required=True)
    ciphertext = fields.Bytes(required=True)


pkcs11_payload_serializer = Serializer(
    PKCS11PayloadSchema,
    validation_exc=LocalDeviceValidationError,
    packing_exc=LocalDevicePackingError,
)


class PKCS11DeviceEncryptor(BaseLocalDeviceEncryptor):
    def __init__(self, token_id: int, key_id: int):
        self.key_id = key_id
        self.token_id = token_id
        # Force loading to crash early if opensc-pkcs11.so is not available
        get_LIB()

    def encrypt(self, plaintext: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        try:
            ciphertext = encrypt_data(self.token_id, self.key_id, plaintext)

        except DevicePKCS11Error as exc:
            raise LocalDeviceCryptoError(str(exc)) from exc

        return pkcs11_payload_serializer.dumps({"ciphertext": ciphertext})


class PKCS11DeviceDecryptor(BaseLocalDeviceDecryptor):
    def __init__(self, token_id: int, key_id: int, pin: str):
        self.key_id = key_id
        self.token_id = token_id
        self.pin = pin
        # Force loading to crash early if opensc-pkcs11.so is not available
        get_LIB()

    def decrypt(self, ciphertext: bytes) -> bytes:
        """
        Raises:
            LocalDeviceCryptoError
            LocalDeviceValidationError
            LocalDevicePackingError
        """
        payload = pkcs11_payload_serializer.loads(ciphertext)
        try:
            return decrypt_data(self.pin, self.token_id, self.key_id, payload["ciphertext"])

        except DevicePKCS11Error as exc:
            raise LocalDeviceCryptoError(str(exc)) from exc

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        try:
            pkcs11_payload_serializer.loads(ciphertext)
            return True

        except (LocalDeviceValidationError, LocalDevicePackingError):
            return False
