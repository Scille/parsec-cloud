# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import Optional, Union
from pendulum import Pendulum
from nacl.exceptions import CryptoError
from nacl.secret import SecretBox
from nacl.public import SealedBox
from nacl.bindings import crypto_sign_BYTES

from parsec.types import DeviceID
from parsec.serde import BaseSchema, fields, SerdeValidationError, SerdePackingError, Serializer
from parsec.crypto_types import PrivateKey, PublicKey, SigningKey, VerifyKey


class DataError(Exception):
    pass


class DataValidationError(SerdeValidationError, DataError):
    pass


class DataSerializationError(SerdePackingError, DataError):
    pass


class BaseSignedDataSchema(BaseSchema):
    author = fields.DeviceID(missing=None)
    timestamp = fields.DateTime(required=True)


class SignedDataMeta(type):
    CLS_ATTR_COOKING = attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True)

    def __new__(cls, name, bases, nmspc):
        if "SCHEMA_CLS" not in nmspc:
            raise RuntimeError("Missing attribute `SCHEMA_CLS` in class definition")
        if not issubclass(nmspc["SCHEMA_CLS"], BaseSignedDataSchema):
            raise RuntimeError(f"Attribute `SCHEMA_CLS` must inherit {BaseSignedDataSchema!r}")

        if "__attrs_attrs__" not in nmspc:
            if "SERIALIZER" in nmspc and bases:
                raise RuntimeError("Attribute `SERIALIZER` is reserved")
            nmspc["SERIALIZER"] = Serializer(
                nmspc["SCHEMA_CLS"], DataValidationError, DataSerializationError
            )
            raw_cls = type.__new__(cls, name, bases, nmspc)
            return cls.CLS_ATTR_COOKING(raw_cls)
        else:
            return type.__new__(cls, name, bases, nmspc)


class BaseSignedData(metaclass=SignedDataMeta):
    SCHEMA_CLS = BaseSignedDataSchema  # Must be overloaded by child class
    # SERIALIZER attribute sets by the metaclass

    author: Optional[DeviceID]  # Set to None if signed by the root key
    timestamp: Pendulum

    def _serialize(self) -> bytes:
        """
        Raises:
            DataError
        """
        return self.SERIALIZER.dumps(self)

    @classmethod
    def _deserialize(cls, raw: bytes) -> "BaseSignedData":
        """
        Raises:
            DataError
        """
        return cls.SERIALIZER.loads(raw)

    def dump_and_sign(self, author_signkey: SigningKey) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            return author_signkey.sign(self._serialize())

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    def dump_sign_and_encrypt(
        self, author_signkey: SigningKey, key: Union[bytes, SecretBox]
    ) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            signed = author_signkey.sign(self._serialize())
            box = key if isinstance(key, SecretBox) else SecretBox(key)
            return box.encrypt(signed)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    def dump_sign_and_encrypt_for(
        self, author_signkey: SigningKey, recipient_pubkey: Union[PublicKey, SealedBox]
    ) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            signed = author_signkey.sign(self._serialize())
            box = (
                recipient_pubkey
                if isinstance(recipient_pubkey, SealedBox)
                else SealedBox(recipient_pubkey)
            )
            return box.encrypt(signed)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    @classmethod
    def unsecure_load(self, signed: bytes) -> "BaseSignedData":
        """
        Raises:
            DataError
        """
        raw = signed[crypto_sign_BYTES:]
        return self._deserialize(raw)

    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: Pendulum = None,
    ) -> "BaseSignedData":
        """
        Raises:
            DataError
        """
        try:
            raw = author_verify_key.verify(signed)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        data = cls._deserialize(raw)
        if data.author != expected_author:
            raise DataError(f"Invalid author: expect `{expected_author}`, got `{data.author}`")
        if expected_timestamp is not None and data.timestamp != expected_timestamp:
            raise DataError(
                f"Invalid timestamp: expect `{expected_timestamp}`, got `{data.timestamp}`"
            )
        return data

    @classmethod
    def decrypt_verify_and_load(
        self,
        encrypted: bytes,
        key: Union[bytes, SecretBox],
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: Pendulum,
    ) -> "BaseSignedData":
        """
        Raises:
            DataError
        """
        try:
            box = key if isinstance(key, SecretBox) else SecretBox(key)
            signed = box.decrypt(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return self.verify_and_load(signed)

    @classmethod
    def decrypt_verify_and_load_for(
        self,
        encrypted: bytes,
        recipient_privkey: Union[PrivateKey, SealedBox],
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: Pendulum,
    ) -> "BaseSignedData":
        """
        Raises:
            DataError
        """
        try:
            box = (
                recipient_privkey
                if isinstance(recipient_privkey, SealedBox)
                else SealedBox(recipient_privkey)
            )
            signed = box.decrypt(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return self.verify_and_load(signed)
