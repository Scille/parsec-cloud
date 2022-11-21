# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import Dict, Iterable, Tuple, Type, TypeVar
from parsec._parsec import DateTime

from parsec.serde import (
    BaseSchema,
    fields,
    SerdeValidationError,
    SerdePackingError,
    BaseSerializer,
    ZipMsgpackSerializer,
)
from parsec.crypto import CryptoError, PrivateKey, PublicKey, SigningKey, VerifyKey, SecretKey
from parsec.api.protocol import DeviceID, DeviceIDField


class DataError(Exception):
    pass


class DataValidationError(SerdeValidationError, DataError):
    pass


class DataSerializationError(SerdePackingError, DataError):
    pass


class BaseSignedDataSchema(BaseSchema):
    author = DeviceIDField(required=True, allow_none=False)
    timestamp = fields.DateTime(required=True)


class DataMeta(type):

    SERIALIZER: BaseSerializer
    BASE_SCHEMA_CLS = BaseSchema

    def __new__(cls, name: str, bases: Tuple[type, ...], nmspc: Dict[str, object]) -> DataMeta:
        # Sanity checks
        SCHEMA_CLS = nmspc.get("SCHEMA_CLS")
        if SCHEMA_CLS is None:
            raise RuntimeError("Missing attribute `SCHEMA_CLS` in class definition")
        if not isinstance(SCHEMA_CLS, type):
            raise RuntimeError("SCHEMA_CLS is not a type")
        if not issubclass(SCHEMA_CLS, cls.BASE_SCHEMA_CLS):
            raise RuntimeError(f"Attribute `SCHEMA_CLS` must inherit {cls.BASE_SCHEMA_CLS!r}")

        raw_cls = type.__new__(cls, name, bases, nmspc)

        # Sanity checks: class SCHEMA_CLS needs to define parents SCHEMA_CLS fields
        schema_cls_fields = set(SCHEMA_CLS._declared_fields)
        for base in bases:
            BASE_SCHEMA_CLS = getattr(base, "SCHEMA_CLS", None)
            if BASE_SCHEMA_CLS is not None:
                assert issubclass(BASE_SCHEMA_CLS, BaseSchema)
                assert BASE_SCHEMA_CLS._declared_fields.keys() <= schema_cls_fields

        # Sanity check: attr fields need to be defined in SCHEMA_CLS
        attrs = nmspc.get("__attrs_attrs__")
        if attrs is not None:
            assert isinstance(attrs, Iterable)
            assert {att.name for att in attrs} <= schema_cls_fields

        serializer_cls = getattr(raw_cls, "SERIALIZER_CLS", None)
        if serializer_cls is None:
            raise RuntimeError("Missing attribute `SERIALIZER_CLS` in class definition")

        if not issubclass(serializer_cls, BaseSerializer):
            raise RuntimeError(f"Attribute `SERIALIZER_CLS` must inherit {BaseSerializer!r}")

        raw_cls.SERIALIZER = serializer_cls(
            nmspc["SCHEMA_CLS"], DataValidationError, DataSerializationError
        )

        return raw_cls


class SignedDataMeta(DataMeta):
    BASE_SCHEMA_CLS = BaseSignedDataSchema


BaseSignedDataTypeVar = TypeVar("BaseSignedDataTypeVar", bound="BaseSignedData")


@attr.s(slots=True, frozen=True, kw_only=True, eq=False)
class BaseSignedData(metaclass=SignedDataMeta):
    """
    Most data within the api should inherit this class. The goal is to have
    immutable data (thanks to attr frozen) that can be easily (de)serialize
    with encryption/signature support.
    """

    # Must be overloaded by child classes
    SCHEMA_CLS = BaseSignedDataSchema
    SERIALIZER_CLS = BaseSerializer
    SERIALIZER: BaseSerializer  # Configured by the metaclass

    author: DeviceID = attr.ib()
    timestamp: DateTime = attr.ib()

    def __eq__(self, other: object) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self: BaseSignedDataTypeVar, **kwargs: object) -> BaseSignedDataTypeVar:
        return attr.evolve(self, **kwargs)

    def _serialize(self) -> bytes:
        """
        Raises:
            DataError
        """
        return self.SERIALIZER.dumps(self)

    @classmethod
    def _deserialize(cls: Type[BaseSignedDataTypeVar], raw: bytes) -> BaseSignedDataTypeVar:
        """
        Raises:
            DataError
        """
        try:
            return cls.SERIALIZER.loads(raw)
        except DataError:
            raise

    def dump_and_sign(self, author_signkey: SigningKey) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            return author_signkey.sign(self._serialize())

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    def dump_sign_and_encrypt(self, author_signkey: SigningKey, key: SecretKey) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            signed = author_signkey.sign(self._serialize())
            return key.encrypt(signed)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    def dump_sign_and_encrypt_for(
        self, author_signkey: SigningKey, recipient_pubkey: PublicKey
    ) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            signed = author_signkey.sign(self._serialize())
            return recipient_pubkey.encrypt_for_self(signed)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    @classmethod
    def unsecure_load(cls: Type[BaseSignedDataTypeVar], signed: bytes) -> BaseSignedDataTypeVar:
        """
        Raises:
            DataError
        """
        raw = VerifyKey.unsecure_unwrap(signed)
        return cls._deserialize(raw)

    @classmethod
    def verify_and_load(
        cls: Type[BaseSignedDataTypeVar],
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID | None,
        expected_timestamp: DateTime | None = None,
        **kwargs: object,
    ) -> BaseSignedDataTypeVar:
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
            repr_expected_author = expected_author or "<root key>"
            raise DataError(
                f"Invalid author: expected `{repr_expected_author}`, got `{data.author}`"
            )
        if expected_timestamp is not None and data.timestamp != expected_timestamp:
            raise DataError(
                f"Invalid timestamp: expected `{expected_timestamp}`, got `{data.timestamp}`"
            )
        return data

    @classmethod
    def decrypt_verify_and_load(
        cls: Type[BaseSignedDataTypeVar],
        encrypted: bytes,
        key: SecretKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        **kwargs: object,
    ) -> BaseSignedDataTypeVar:
        """
        Raises:
            DataError
        """
        try:
            signed = key.decrypt(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return cls.verify_and_load(
            signed,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
            expected_timestamp=expected_timestamp,
            **kwargs,
        )

    @classmethod
    def decrypt_verify_and_load_for(
        cls: Type[BaseSignedDataTypeVar],
        encrypted: bytes,
        recipient_privkey: PrivateKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
        **kwargs: object,
    ) -> BaseSignedDataTypeVar:
        """
        Raises:
            DataError
        """
        try:
            signed = recipient_privkey.decrypt_from_self(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return cls.verify_and_load(
            signed,
            author_verify_key=author_verify_key,
            expected_author=expected_author,
            expected_timestamp=expected_timestamp,
            **kwargs,
        )


BaseDataTypeVar = TypeVar("BaseDataTypeVar", bound="BaseData")


@attr.s(slots=True, frozen=True, kw_only=True, eq=False)
class BaseData(metaclass=DataMeta):
    """
    Some data within the api don't have to be signed (e.g. claim info) and
    should inherit this class.
    The goal is to have immutable data (thanks to attr frozen) that can be
    easily (de)serialize with encryption support.
    """

    # Must be overloaded by child classes
    SCHEMA_CLS = BaseSchema
    SERIALIZER_CLS = BaseSerializer
    SERIALIZER: BaseSerializer  # Configured by the metaclass

    def __eq__(self, other: object) -> bool:
        if isinstance(other, type(self)):
            return attr.astuple(self).__eq__(attr.astuple(other))
        return NotImplemented

    def evolve(self: BaseDataTypeVar, **kwargs: object) -> BaseDataTypeVar:
        return attr.evolve(self, **kwargs)

    def dump(self) -> bytes:
        """
        Raises:
            DataError
        """
        return self.SERIALIZER.dumps(self)

    @classmethod
    def load(cls: Type[BaseDataTypeVar], raw: bytes, **kwargs: object) -> BaseDataTypeVar:
        """
        Raises:
            DataError
        """
        return cls.SERIALIZER.loads(raw)

    def dump_and_encrypt(self, key: SecretKey) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            raw = self.dump()
            return key.encrypt(raw)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    def dump_and_encrypt_for(self, recipient_pubkey: PublicKey) -> bytes:
        """
        Raises:
            DataError
        """
        try:
            raw = self.dump()
            return recipient_pubkey.encrypt_for_self(raw)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

    @classmethod
    def decrypt_and_load(
        cls: Type[BaseDataTypeVar], encrypted: bytes, key: SecretKey, **kwargs: object
    ) -> BaseDataTypeVar:
        """
        Raises:
            DataError
        """
        try:
            raw = key.decrypt(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return cls.load(raw, **kwargs)

    @classmethod
    def decrypt_and_load_for(
        cls: Type[BaseDataTypeVar],
        encrypted: bytes,
        recipient_privkey: PrivateKey,
        **kwargs: object,
    ) -> BaseDataTypeVar:
        """
        Raises:
            DataError
        """
        try:
            raw = recipient_privkey.decrypt_from_self(encrypted)

        except CryptoError as exc:
            raise DataError(str(exc)) from exc

        return cls.load(raw, **kwargs)


# Data class with serializers


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseAPISignedData(BaseSignedData):
    """Signed and compressed base class for API data"""

    SCHEMA_CLS = BaseSignedDataSchema
    SERIALIZER_CLS = ZipMsgpackSerializer


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class BaseAPIData(BaseData):
    """Unsigned and compressed base class for API data"""

    SCHEMA_CLS = BaseSchema
    SERIALIZER_CLS = ZipMsgpackSerializer
