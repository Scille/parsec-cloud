# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, Iterable, Tuple, Type, TypeVar

import attr

from parsec._parsec import DataError
from parsec.crypto import CryptoError, PrivateKey, PublicKey, SecretKey
from parsec.serde import BaseSchema, BaseSerializer, SerdePackingError, SerdeValidationError


class DataValidationError(SerdeValidationError, DataError):
    pass


class DataSerializationError(SerdePackingError, DataError):
    pass


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
