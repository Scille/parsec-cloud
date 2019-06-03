# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pendulum import Pendulum
from uuid import UUID as _UUID
from collections import Mapping
from marshmallow import ValidationError
from marshmallow.fields import (
    # Republishing
    Int,
    String,
    List,
    Dict,
    Nested,
    Integer,
    Boolean,
    Field,
)
import re

from parsec.types import (
    DeviceID as _DeviceID,
    UserID as _UserID,
    DeviceName as _DeviceName,
    BackendOrganizationAddr as _BackendOrganizationAddr,
    OrganizationID as _OrganizationID,
)
from parsec.crypto_types import (
    SigningKey as _SigningKey,
    VerifyKey as _VerifyKey,
    PrivateKey as _PrivateKey,
    PublicKey as _PublicKey,
)


__all__ = (
    "enum_field_factory",
    "bytes_based_field_factory",
    "str_based_field_factory",
    "uuid_based_field_factory",
    "Int",
    "String",
    "List",
    "Dict",
    "Nested",
    "Integer",
    "Boolean",
    "Field",
    "Path",
    "Bytes",
    "DateTime",
    "CheckedConstant",
    "Map",
    "VerifyKey",
    "SigningKey",
    "PublicKey",
    "PrivateKey",
    "SecretKey",
    "DeviceID",
    "UserID",
    "DeviceName",
    "SemVer",
)


def enum_field_factory(enum):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        if not isinstance(value, enum):
            raise ValidationError(f"Not a {enum.__name__}")

        return value.value

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if not isinstance(value, str):
            raise ValidationError("Not string")

        for choice in enum:
            if choice.value == value:
                return choice
        else:
            raise ValidationError(f"Invalid role `{value}`")

    return type(
        f"{enum.__name__}Field", (Field,), {"_serialize": _serialize, "_deserialize": _deserialize}
    )


def bytes_based_field_factory(value_type):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        return value

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

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


def str_based_field_factory(value_type):
    def _deserialize(self, value, attr, data):
        try:
            return value_type(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return str(value)

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_deserialize": _deserialize, "_serialize": _serialize},
    )


def uuid_based_field_factory(value_type):
    def _deserialize(self, value, attr, data):
        try:
            return value_type(str(value))
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value

    return type(
        f"{value_type.__name__}Field",
        (Field,),
        {"_deserialize": _deserialize, "_serialize": _serialize},
    )


# TODO: test this field and use it everywhere in the api !


class Path(Field):
    """Absolute path"""

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if not value.startswith("/"):
            raise ValidationError("Path must be absolute")
        if value != "/":
            for item in value.split("/")[1:]:
                if item in (".", "..", ""):
                    raise ValidationError("Invalid path")
        return value


class UUID(Field):
    """UUID already handled by pack/unpack"""

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if not isinstance(value, _UUID):
            raise ValidationError("Not an UUID")
        return value


class DateTime(Field):
    """DateTime already handled by pack/unpack"""

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if not isinstance(value, Pendulum):
            raise ValidationError("Not a datetime")

        return value


class CheckedConstant(Field):
    """Make sure the value is present during deserialization"""

    def __init__(self, constant, **kwargs):
        kwargs.setdefault("default", constant)
        super().__init__(**kwargs)
        self.constant = constant

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if value != self.constant:
            raise ValidationError("Invalid value, should be `%s`" % self.constant)

        return value


class Map(Field):
    default_error_messages = {"invalid": "Not a valid mapping type."}

    def __init__(self, key_field, nested_field, **kwargs):
        super().__init__(**kwargs)
        self.key_field = key_field
        self.nested_field = nested_field

    def _deserialize(self, value, attr, obj):

        if not isinstance(value, Mapping):
            self.fail("invalid")

        ret = {}
        for key, val in value.items():
            k = self.key_field.deserialize(key)
            v = self.nested_field.deserialize(val)
            ret[k] = v
        return ret

    def _serialize(self, value, attr, obj):
        ret = {}
        for key, val in value.items():
            k = self.key_field._serialize(key, attr, obj)
            v = self.nested_field._serialize(val, key, value)
            ret[k] = v
        return ret


class SigningKey(Field):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        try:
            return _SigningKey(value)

        except Exception:
            raise ValidationError("Invalid signing key.")


class VerifyKey(Field):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        try:
            return _VerifyKey(value)

        except Exception:
            raise ValidationError("Invalid verify key.")


class PrivateKey(Field):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        try:
            return _PrivateKey(value)

        except Exception:
            raise ValidationError("Invalid secret key.")


class PublicKey(Field):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        return value.encode()

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        try:
            return _PublicKey(value)

        except Exception:
            raise ValidationError("Invalid verify key.")


class SemVer(Field):
    default_error_messages = {"no_string": "Not a string.", "regex_failed": "String not a SemVer"}

    def _serialize(self, value, attr, obj):
        if not isinstance(value, str):
            self.fail("no_string")
        if not re.match("(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)", value):
            self.fail("regex_failed")
        return value

    _deserialize = _serialize


SecretKey = bytes_based_field_factory(bytes)
Bytes = bytes_based_field_factory(bytes)
DeviceID = str_based_field_factory(_DeviceID)
UserID = str_based_field_factory(_UserID)
OrganizationID = str_based_field_factory(_OrganizationID)
DeviceName = str_based_field_factory(_DeviceName)
BackendOrganizationAddr = str_based_field_factory(_BackendOrganizationAddr)
