from json import JSONDecodeError

from parsec.schema import UnknownCheckedSchema, fields, ValidationError
from parsec.crypto import (
    CryptoError,
    derivate_secret_key_from_password,
    encrypt_raw_with_secret_key,
    decrypt_raw_with_secret_key,
)


class CipherError(Exception):
    pass


class BaseLocalDeviceEncryptor:
    def encrypt(self, plain: bytes) -> bytes:
        raise NotImplementedError()


class BaseLocalDeviceDecryptor:
    def decrypt(self, ciphered: bytes) -> bytes:
        raise NotImplementedError()

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        raise NotImplementedError()


class PasswordPayloadSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("password", required=True)
    salt = fields.Base64Bytes(required=True)
    ciphertext = fields.Base64Bytes(required=True)


password_payload_schema = PasswordPayloadSchema(strict=True)


class PasswordDeviceEncryptor(BaseLocalDeviceEncryptor):
    def __init__(self, password):
        self.password = password

    def encrypt(self, plaintext: bytes) -> bytes:
        """
        Raises:
            CipherError
        """
        try:
            key, salt = derivate_secret_key_from_password(self.password)

            ciphertext = encrypt_raw_with_secret_key(key, plaintext)
            return password_payload_schema.dumps(
                {"salt": salt, "ciphertext": ciphertext}
            ).data.encode("utf8")

        except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
            raise CipherError(str(exc)) from exc


class PasswordDeviceDecryptor(BaseLocalDeviceDecryptor):
    def __init__(self, password):
        self.password = password

    def decrypt(self, ciphertext: bytes) -> bytes:
        """
        Raises:
            CipherError
        """
        try:
            payload = password_payload_schema.loads(ciphertext.decode("utf8")).data
            key, _ = derivate_secret_key_from_password(self.password, payload["salt"])
            return decrypt_raw_with_secret_key(key, payload["ciphertext"])

        except (CryptoError, ValidationError, JSONDecodeError, ValueError) as exc:
            raise CipherError(str(exc)) from exc

    @staticmethod
    def can_decrypt(ciphertext: bytes) -> bool:
        try:
            password_payload_schema.loads(ciphertext.decode("utf8")).data
            return True

        except (ValidationError, JSONDecodeError, ValueError) as exc:
            return False
