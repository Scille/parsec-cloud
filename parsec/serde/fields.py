# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Type
from parsec._parsec import DateTime as RsDateTime
from uuid import UUID as _UUID
from enum import Enum
from collections import Mapping
from marshmallow import ValidationError
from marshmallow.fields import (
    # Republishing
    Int,
    Float,
    String,
    List,
    Dict,
    Nested,
    Integer,
    Boolean,
    Email,
    Field,
)

from parsec.types import FrozenDict as _FrozenDict
from parsec.crypto import (
    SecretKey as _SecretKey,
    HashDigest as _HashDigest,
    SigningKey as _SigningKey,
    VerifyKey as _VerifyKey,
    PrivateKey as _PrivateKey,
    PublicKey as _PublicKey,
)
from parsec.sequester_crypto import (
    SequesterVerifyKeyDer as _SequesterVerifyKeyDer,
    SequesterEncryptionKeyDer as _SequesterEncryptionKeyDer,
)


__all__ = (
    "enum_field_factory",
    "bytes_based_field_factory",
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


class BaseEnumField(Field):
    ENUM: Enum

    def _serialize(self, value: Any, attr: str, obj: object) -> str | None:
        if value is None:
            return None
        if not isinstance(value, tuple([type(x) for x in self.ENUM])):  # type: ignore[attr-defined]
            raise ValidationError(f"Not a {self.ENUM.__name__}")  # type: ignore[attr-defined]

        return value.value

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> Enum:
        if not isinstance(value, str):
            raise ValidationError("Not string")

        for choice in self.ENUM:  # type: ignore[attr-defined]
            if choice.value == value:
                return choice
        else:
            raise ValidationError(f"Invalid role `{value}`")


# Field factories are correctly typed, however mypy consider the
# `MyField == make_field_class()` pattern as too similar to variale assignment and
# disregard factory typing (see: https://github.com/python/typing/discussions/1020)
# Anyway the solution is to add explicit typing when using the type generator:
# `MyField: Type[Field] == make_field_class()`, transitivity ? never heard of it !


# Using inheritance to define enum fields allows for easy instrospection detection
# which is useful when checking api changes in tests (see tests/schemas/builder.py)
def enum_field_factory(enum: Type[Enum]) -> Type[BaseEnumField]:
    return type(f"{enum.__name__}Field", (BaseEnumField,), {"ENUM": enum})


def bytes_based_field_factory(value_type: Any) -> Type[Field]:
    assert isinstance(value_type, type)

    def _serialize(self: Any, value: Any, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value

    def _deserialize(self: Any, value: bytes, attr: str, data: dict[str, object]) -> Field:
        if not isinstance(value, bytes):
            raise ValidationError("Not bytes")

        try:
            return value_type(value)

        except Exception as exc:
            raise ValidationError(str(exc)) from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


def str_based_field_factory(value_type: Any) -> Type[Field]:
    assert isinstance(value_type, type)

    def _serialize(self: Any, value: Any, attr: str, data: object) -> str | None:
        if value is None:
            return None

        assert isinstance(value, value_type)
        if hasattr(value, "str"):
            return value.str
        return str(value)

    def _deserialize(self: Any, value: str, attr: str, data: dict[str, object]) -> Any:

        if not isinstance(value, str):
            raise ValidationError("Not string")

        try:
            return value_type(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


def rust_enum_field_factory(value_type: Any) -> Type[Field]:
    def _serialize(self: Any, value: Any, attr: str, obj: Any) -> str | None:
        if value is None:
            return None

        if isinstance(value, value_type):
            return value.str
        else:
            raise ValidationError(f"Not a InvitationStatus")

    def _deserialize(self: Any, value: object, attr: str, data: dict[str, object]) -> Any:
        if not isinstance(value, str):
            raise ValidationError("Not string")

        try:
            return value_type.from_str(value)
        except ValueError as exc:
            raise ValidationError(f"Invalid type `{value}`") from exc

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_serialize": _serialize, "_deserialize": _deserialize},
    )


def uuid_based_field_factory(value_type: Any) -> Type[Field]:
    assert isinstance(value_type, type)

    def _serialize(self: Any, value: Any, attr: str, data: object) -> bytes | None:
        if value is None:
            return None

        assert isinstance(value, value_type)
        return value.uuid

    def _deserialize(self: Any, value: object, attr: str, data: dict[str, object]) -> Field:
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


class Path(Field):
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


class UUID(Field):
    """UUID already handled by pack/unpack"""

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> _UUID:
        if not isinstance(value, _UUID):
            raise ValidationError("Not an UUID")
        return value


class DateTime(Field):
    """DateTime already handled by pack/unpack"""

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> RsDateTime:
        if not isinstance(value, RsDateTime):
            raise ValidationError("Not a datetime")

        return value


class EnumCheckedConstant(Field):
    """Make sure the value is present during deserialization"""

    def __init__(self, constant: Enum, **kwargs: Any) -> None:
        kwargs.setdefault("default", constant.value)
        super().__init__(**kwargs)
        self.constant = constant

    def _serialize(self, value: Any, attr: str, obj: object) -> str:
        return self.constant.value

    def _deserialize(self, value: str, attr: str, data: dict[str, object]) -> Enum:
        if value != self.constant.value:
            raise ValidationError(f"Invalid value, should be `{self.constant.value}`")

        return self.constant


class CheckedConstant(Field):
    """Make sure the value is present during deserialization"""

    def __init__(self, constant: str | None, **kwargs: Any) -> None:
        kwargs.setdefault("default", constant)
        super().__init__(**kwargs)
        self.constant = constant

    def _serialize(self, value: Any, attr: str, obj: object) -> str | None:
        return self.constant

    def _deserialize(self, value: str | None, attr: str, data: dict[str, object]) -> str | None:
        if value != self.constant:
            raise ValidationError(f"Invalid value, should be `{self.constant}`")

        return value


class Map(Field):
    default_error_messages = {"invalid": "Not a valid mapping type."}

    def __init__(self, key_field: Any, nested_field: Any, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.key_field = key_field
        self.nested_field = nested_field

    def _serialize(
        self, value: Mapping[Any, Any], attr: str, obj: object
    ) -> Mapping[Any, Any] | None:
        if value is None:
            return None

        ret = {}
        for key, val in value.items():
            k = self.key_field._serialize(key, attr, obj)
            v = self.nested_field._serialize(val, key, value)
            ret[k] = v
        return ret

    def _deserialize(
        self, value: Mapping[Any, Any], attr: str, obj: dict[str, object]
    ) -> Mapping[Any, Any]:

        if not isinstance(value, Mapping):
            self.fail("invalid")

        ret = {}
        for key, val in value.items():
            k = self.key_field.deserialize(key)
            v = self.nested_field.deserialize(val)
            ret[k] = v
        return ret


class FrozenMap(Map):
    def _deserialize(self, value: Any, attr: str, obj: dict[str, object]) -> _FrozenDict[Any, Any]:
        return _FrozenDict(super()._deserialize(value, attr, obj))


class FrozenList(List):
    def _deserialize(self, value: Any, attr: str, obj: dict[str, object]) -> tuple[Any, ...]:
        return tuple(super()._deserialize(value, attr, obj))


class FrozenSet(List):
    def _deserialize(self, value: Any, attr: str, obj: dict[str, object]) -> frozenset[Any]:
        return frozenset(super()._deserialize(value, attr, obj))


class Tuple(Field):
    default_error_messages = {"invalid": "Not a valid tuple type."}

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(**kwargs)
        self.args = args

    def _serialize(self, value: Any, attr: str, obj: object) -> tuple[Any, ...] | None:
        if value is None:
            return None

        return tuple(self.args[i]._serialize(v, attr, obj) for i, v in enumerate(value))

    def _deserialize(self, value: Any, attr: str, obj: dict[str, object]) -> Any:
        if not isinstance(value, (list, tuple)) or len(self.args) != len(value):
            self.fail("invalid")
        return tuple(self.args[i].deserialize(v, attr, obj) for i, v in enumerate(value))


class SigningKey(Field):
    def _serialize(self, value: _SigningKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _SigningKey:
        try:
            return _SigningKey(value)

        except Exception:
            raise ValidationError("Invalid signing key.")


class VerifyKey(Field):
    def _serialize(self, value: _VerifyKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _VerifyKey:
        try:
            return _VerifyKey(value)

        except Exception:
            raise ValidationError("Invalid verify key.")


class PrivateKey(Field):
    def _serialize(self, value: _PrivateKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _PrivateKey:
        try:
            return _PrivateKey(value)

        except Exception:
            raise ValidationError("Invalid private key.")


class PublicKey(Field):
    def _serialize(self, value: _PublicKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _PublicKey:
        try:
            return _PublicKey(value)

        except Exception:
            raise ValidationError("Invalid public key.")


Bytes: Type[Field] = bytes_based_field_factory(bytes)


class HashDigestField(Field):
    def _serialize(self, value: _HashDigest | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.digest

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _HashDigest:
        if not isinstance(value, bytes):
            raise ValidationError("Not bytes")

        try:
            return _HashDigest(value)

        except Exception as exc:
            raise ValidationError(str(exc)) from exc


HashDigest = HashDigestField


class SecretKeyField(Field):
    def _serialize(self, value: _SecretKey | None, attr: str, obj: object) -> bytes | None:
        if value is None:
            return None

        return value.secret

    def _deserialize(self, value: bytes, attr: str, data: dict[str, object]) -> _SecretKey:
        if not isinstance(value, bytes):
            raise ValidationError("Not bytes")

        try:
            return _SecretKey(value)

        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SecretKey = SecretKeyField


class SequesterVerifyKeyDerField(Field):
    def _serialize(
        self, value: _SequesterVerifyKeyDer | None, attr: str, obj: object
    ) -> bytes | None:
        if value is None:
            return None

        return value.dump()

    def _deserialize(
        self, value: bytes, attr: str, data: dict[str, object]
    ) -> _SequesterVerifyKeyDer:
        if not isinstance(value, bytes):
            raise ValidationError("Not bytes")

        try:
            return _SequesterVerifyKeyDer(value)

        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SequesterVerifyKeyDer = SequesterVerifyKeyDerField


class SequesterEncryptionKeyDerField(Field):
    def _serialize(
        self, value: _SequesterEncryptionKeyDer | None, attr: Any, obj: Any
    ) -> bytes | None:
        if value is None:
            return None

        return value.dump()

    def _deserialize(
        self, value: bytes, attr: str, data: dict[str, object]
    ) -> _SequesterEncryptionKeyDer:
        if not isinstance(value, bytes):
            raise ValidationError("Not bytes")

        try:
            return _SequesterEncryptionKeyDer(value)

        except Exception as exc:
            raise ValidationError(str(exc)) from exc


SequesterEncryptionKeyDer = SequesterEncryptionKeyDerField
