# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Mapping
from enum import Enum
from typing import TYPE_CHECKING, Any, Iterable, Protocol, Type, TypeVar, Union
from uuid import UUID as _UUID

from marshmallow import ValidationError
from marshmallow.fields import (
    Boolean,
    Dict,
    Email,
    Field,  # Republishing
    Float,
    Int,
    Integer,
    List,
    Nested,
    String,
)

from parsec._parsec import (
    DateTime as RsDateTime,
    PkiEnrollmentSubmitPayload as _PkiEnrollmentSubmitPayload,
)
from parsec.crypto import (
    HashDigest as _HashDigest,
    PrivateKey as _PrivateKey,
    PublicKey as _PublicKey,
    SecretKey as _SecretKey,
    SigningKey as _SigningKey,
    VerifyKey as _VerifyKey,
)
from parsec.sequester_crypto import (
    SequesterEncryptionKeyDer as _SequesterEncryptionKeyDer,
    SequesterVerifyKeyDer as _SequesterVerifyKeyDer,
)
from parsec.types import FrozenDict as _FrozenDict

__all__ = (
    "enum_field_factory",
    "str_based_field_factory",
    "uuid_based_field_factory",
    "Int",
    "Float",
    "String",
    "List",
    "Dict",
    "Nested",
    "Integer",
    "Boolean",
    "Email",
    "Field",
    "Path",
    "Bytes",
    "DateTime",
    "CheckedConstant",
    "EnumCheckedConstant",
    "Map",
    "VerifyKey",
    "SigningKey",
    "PublicKey",
    "PrivateKey",
    "SecretKey",
    "HashDigest",
    "FrozenList",
    "FrozenMap",
    "FrozenSet",
)


class WithStrAttribute(Protocol):
    def __init__(self, value: str) -> None:
        ...

    @property
    def str(self) -> str:
        ...


RustEnumSelf = TypeVar("RustEnumSelf", bound="RustEnum")


class RustEnum(Protocol):
    @classmethod
    def from_str(cls: Type[RustEnumSelf], value: str) -> RustEnumSelf:
        ...

    @property
    def str(self) -> str:
        ...


class UUIDProtocol(Protocol):
    @classmethod
    def __init__(self, value: _UUID) -> None:
        ...

    @property
    def uuid(self) -> _UUID:
        ...


T = TypeVar("T")
E = TypeVar("E", bound=Enum)
S = TypeVar("S", bound=WithStrAttribute)
R = TypeVar("R", bound=RustEnum)
U = TypeVar("U", bound=UUIDProtocol)

if not TYPE_CHECKING:
    # Runtime hack to allow us to use a Generic in a stub file
    # See: https://github.com/python/typing/issues/60
    Field.__class_getitem__ = lambda _: Field


class BaseEnumField(Field[E]):
    ENUM: Type[E]

    def _serialize(self, value: E | None, attr: str, obj: object) -> str | None:
        if value is None:
            return None
        assert isinstance(value, self.ENUM)
        return value.value

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> E:
        if not isinstance(value, str):
            raise ValidationError("Not string")
        try:
            return self.ENUM(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


# Using inheritance to define enum fields allows for easy instrospection detection
# which is useful when checking api changes in tests (see tests/schemas/builder.py)
def enum_field_factory(enum: Type[E]) -> Type[BaseEnumField[E]]:
    return type(f"{enum.__name__}Field", (BaseEnumField,), {"ENUM": enum})


# The name looks weird for legacy reasons
class bytesField(Field[bytes]):
    def _serialize(self: Field[bytes], value: bytes | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, bytes)
        return value

    def _deserialize(
        self: Field[bytes], value: object, attr: str, data: dict[str, object]
    ) -> bytes:
        if not isinstance(value, bytes):
            raise ValidationError(f"expected 'bytes' for got '{type(value)}'")
        try:
            return bytes(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


Bytes = bytesField


def str_based_field_factory(value_type: Type[S]) -> Type[Field[S]]:
    assert isinstance(value_type, type)

    def _serialize(self: Field[S], value: S | None, attr: str, data: object) -> str | None:
        if value is None:
            return None
        assert isinstance(value, value_type)
        return value.str

    def _deserialize(self: Field[S], value: object, attr: str, data: dict[str, object]) -> S:
        if not isinstance(value, str):
            raise ValidationError("Not a string")
        try:
            return value_type(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


def rust_enum_field_factory(value_type: Type[R]) -> Type[Field[R]]:
    def _serialize(self: Field[R], value: R | None, attr: str, obj: Any) -> str | None:
        if value is None:
            return None
        assert isinstance(value, value_type)
        return value.str

    def _deserialize(self: Field[R], value: object, attr: str, data: dict[str, object]) -> R:
        if not isinstance(value, str):
            raise ValidationError(f"expected 'str' for got '{type(value)}'")
        try:
            return value_type.from_str(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


def uuid_based_field_factory(value_type: Type[U]) -> Type[Field[U]]:
    assert isinstance(value_type, type)

    def _serialize(self: Field[U], value: U | None, attr: str, data: object) -> _UUID | None:
        if value is None:
            return None
        assert isinstance(value, value_type)
        return value.uuid

    def _deserialize(self: Field[U], value: object, attr: str, data: dict[str, object]) -> U:
        if not isinstance(value, _UUID):
            raise ValidationError("Not an UUID")
        try:
            return value_type(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


# TODO: test this field and use it everywhere in the api !


class Path(Field[str]):
    """Absolute path"""

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> str:
        if not isinstance(value, str):
            raise ValidationError(f"expected 'str' but got {type(value)}")
        if not value.startswith("/"):
            raise ValidationError("Path must be absolute")
        if value != "/":
            for item in value.split("/")[1:]:
                if item in (".", "..", ""):
                    raise ValidationError("Invalid path")
        return value


class UUID(Field[_UUID]):
    """UUID already handled by pack/unpack"""

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _UUID:
        if not isinstance(value, _UUID):
            raise ValidationError("Not an UUID")
        return value


class DateTime(Field[RsDateTime]):
    """DateTime already handled by pack/unpack"""

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> RsDateTime:
        if not isinstance(value, RsDateTime):
            raise ValidationError("Not a datetime")

        return value


class EnumCheckedConstant(Field[E]):
    """Make sure the value is present during deserialization"""

    def __init__(self, constant: E, **kwargs: Any) -> None:
        kwargs.setdefault("default", constant.value)
        super().__init__(**kwargs)
        self.constant = constant

    def _serialize(self, value: E | None, attr: str, obj: object) -> str:
        return self.constant.value

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> E:
        if value != self.constant.value:
            raise ValidationError(f"Invalid value, should be `{self.constant.value}`")
        return self.constant


class CheckedConstant(Field[Union[str, None]]):
    """Make sure the value is present during deserialization"""

    def __init__(self, constant: str | None, **kwargs: Any) -> None:
        kwargs.setdefault("default", constant)
        super().__init__(**kwargs)
        self.constant = constant

    def _serialize(self, value: str | None, attr: str, obj: object) -> str | None:
        return self.constant

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> str | None:
        if value != self.constant:
            raise ValidationError(f"Invalid value, should be `{self.constant}`")
        return self.constant


class Map(Field[Mapping[str, T]]):
    default_error_messages = {"invalid": "Not a valid mapping type."}

    def __init__(self, key_field: Field[str], nested_field: Field[T], **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.key_field = key_field
        self.nested_field = nested_field

    def _serialize(
        self, value: Mapping[str, T] | None, attr: str, obj: object
    ) -> dict[str, T] | None:
        if value is None:
            return None
        assert isinstance(value, Mapping)
        ret = {}
        for key, val in value.items():
            k = self.key_field._serialize(key, attr, obj)
            v = self.nested_field._serialize(val, key, value)
            ret[k] = v
        return ret

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> dict[str, T]:

        if not isinstance(value, Mapping):
            self.fail("invalid")
            assert False

        ret = {}
        for key, val in value.items():
            k = self.key_field.deserialize(key)
            v = self.nested_field.deserialize(val)
            ret[k] = v
        return ret


class FrozenMap(Map[T]):
    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> _FrozenDict[str, T]:
        return _FrozenDict(super()._deserialize(value, attr, obj))


class FrozenList(List):
    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> tuple[Any, ...]:
        return tuple(super()._deserialize(value, attr, obj))


class FrozenSet(List):
    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> frozenset[Any]:
        return frozenset(super()._deserialize(value, attr, obj))


class Tuple(Field[Iterable[Any]]):
    default_error_messages = {"invalid": "Not a valid tuple type."}

    def __init__(self, *args: Field[Any], **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.args: tuple[Field[Any], ...] = args

    def _serialize(
        self, value: Iterable[Any] | None, attr: str, obj: object
    ) -> tuple[Any, ...] | None:
        if value is None:
            return None
        assert isinstance(value, Iterable)
        return tuple(self.args[i]._serialize(v, attr, obj) for i, v in enumerate(value))

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> tuple[Any, ...]:
        if not isinstance(value, (list, tuple)) or len(self.args) != len(value):
            self.fail("invalid")
            assert False
        return tuple(self.args[i].deserialize(v) for i, v in enumerate(value))


class SigningKey(Field[_SigningKey]):
    def _serialize(self, value: _SigningKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _SigningKey)
        return value.encode()

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _SigningKey:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _SigningKey(value)
        except Exception:
            raise ValidationError("Invalid signing key.")


class VerifyKey(Field[_VerifyKey]):
    def _serialize(self, value: _VerifyKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _VerifyKey)
        return value.encode()

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _VerifyKey:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _VerifyKey(value)
        except Exception:
            raise ValidationError("Invalid verify key.")


class PrivateKey(Field[_PrivateKey]):
    def _serialize(self, value: _PrivateKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _PrivateKey)
        return value.encode()

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _PrivateKey:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _PrivateKey(value)
        except Exception:
            raise ValidationError("Invalid private key.")


class PublicKey(Field[_PublicKey]):
    def _serialize(self, value: _PublicKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _PublicKey)
        return value.encode()

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _PublicKey:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _PublicKey(value)
        except Exception:
            raise ValidationError("Invalid public key.")


class HashDigestField(Field[_HashDigest]):
    def _serialize(self, value: _HashDigest | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _HashDigest)
        return value.digest

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _HashDigest:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _HashDigest(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


HashDigest = HashDigestField


class SecretKeyField(Field[_SecretKey]):
    def _serialize(self, value: _SecretKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _SecretKey)
        return value.secret

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _SecretKey:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _SecretKey(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SecretKey = SecretKeyField


class SequesterVerifyKeyDerField(Field[_SequesterVerifyKeyDer]):
    def _serialize(
        self, value: _SequesterVerifyKeyDer | None, attr: str, obj: object
    ) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _SequesterVerifyKeyDer)
        return value.dump()

    def _deserialize(
        self, value: object, attr: str, data: dict[str, object]
    ) -> _SequesterVerifyKeyDer:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _SequesterVerifyKeyDer(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SequesterVerifyKeyDer = SequesterVerifyKeyDerField


class SequesterEncryptionKeyDerField(Field[_SequesterEncryptionKeyDer]):
    def _serialize(
        self, value: _SequesterEncryptionKeyDer | None, attr: Any, obj: Any
    ) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _SequesterEncryptionKeyDer)
        return value.dump()

    def _deserialize(
        self, value: object, attr: str, data: dict[str, object]
    ) -> _SequesterEncryptionKeyDer:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _SequesterEncryptionKeyDer(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SequesterEncryptionKeyDer = SequesterEncryptionKeyDerField


class PkiEnrollmentSubmitPayloadField(Field[_PkiEnrollmentSubmitPayload]):
    def _serialize(
        self, value: _PkiEnrollmentSubmitPayload | None, attr: Any, obj: Any
    ) -> bytes | None:
        if value is None:
            return None
        assert isinstance(value, _PkiEnrollmentSubmitPayload)
        return value.dump()

    def _deserialize(
        self, value: object, attr: str, data: dict[str, object]
    ) -> _PkiEnrollmentSubmitPayload:
        if not isinstance(value, bytes):
            raise ValidationError("Expecting bytes")
        try:
            return _PkiEnrollmentSubmitPayload.load(value)
        except Exception as exc:
            raise ValidationError(str(exc)) from exc
