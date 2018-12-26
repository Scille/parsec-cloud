from json import JSONDecodeError

from parsec.utils import ejson_loads, ejson_dumps
from parsec.schema import UnknownCheckedSchema, fields, ValidationError
from parsec.core.devices_manager.cipher import (
    CipherError,
    BaseLocalDeviceEncryptor,
    BaseLocalDeviceDecryptor,
)
from parsec.core.devices_manager.pkcs11_tools import (
    DevicePKCS11Error,
    encrypt_data,
    decrypt_data,
    get_LIB,
)


class PKCS11PayloadSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("PKCS11", required=True)
    ciphertext = fields.Base64Bytes(required=True)


pkcs11_payload_schema = PKCS11PayloadSchema(strict=True)


class PKCS11DeviceEncryptor(BaseLocalDeviceEncryptor):
    def __init__(self, token_id: int, key_id: int):
        self.key_id = key_id
        self.token_id = token_id
        # Force loading to crash early if opensc-pkcs11.so is not available
        get_LIB()

    def encrypt(self, plaintext: bytes) -> bytes:
        """
        Raises:
            CipherError
        """
        try:
            ciphertext = encrypt_data(self.token_id, self.key_id, plaintext)
            return ejson_dumps(pkcs11_payload_schema.dump({"ciphertext": ciphertext}).data).encode(
                "utf8"
            )

        except (DevicePKCS11Error, ValidationError, JSONDecodeError, ValueError) as exc:
            raise CipherError(str(exc)) from exc


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
            CipherError
        """
        try:
            payload = pkcs11_payload_schema.load(ejson_loads(ciphertext.decode("utf8"))).data
            return decrypt_data(self.pin, self.token_id, self.key_id, payload["ciphertext"])

        except (DevicePKCS11Error, ValidationError, JSONDecodeError, ValueError) as exc:
            raise CipherError(str(exc)) from exc

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        try:
            pkcs11_payload_schema.load(ejson_loads(ciphertext.decode("utf8"))).data
            return True

        except (ValidationError, JSONDecodeError, ValueError) as exc:
            return False
