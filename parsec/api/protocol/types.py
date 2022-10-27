# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unicodedata import normalize
from marshmallow import ValidationError
from typing import Union, TypeVar, Optional, Tuple, Pattern
from enum import Enum

from parsec.serde import fields
from parsec._parsec import (
    OrganizationID,
    UserID,
    DeviceID,
    DeviceLabel,
    HumanHandle,
)

UserIDTypeVar = TypeVar("UserIDTypeVar", bound="UserID")
DeviceIDTypeVar = TypeVar("DeviceIDTypeVar", bound="DeviceID")


def _bytes_size(txt: str) -> int:
    return len(txt.encode("utf8"))


class StrBased:
    __slots__ = ("_str",)
    REGEX: Optional[Pattern[str]]
    MAX_BYTE_SIZE: int

    def __init__(self, raw: Union[str, "StrBased"]):
        if isinstance(raw, StrBased):
            raw = raw._str
        raw = normalize("NFC", raw)
        if (self.REGEX and not self.REGEX.match(raw)) or _bytes_size(raw) > self.MAX_BYTE_SIZE:
            raise ValueError("Invalid data")
        self._str: str = raw

    def __str__(self) -> str:
        return self._str

    def __repr__(self) -> str:
        return f"<{type(self).__name__} {self._str}>"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, type(self)):
            return NotImplemented
        else:
            return self._str == other._str

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, type(self)):
            return NotImplemented
        else:
            return self._str < other._str

    def __hash__(self) -> int:
        return self._str.__hash__()

    @property
    def str(self) -> str:
        return self._str


OrganizationIDField = fields.str_based_field_factory(OrganizationID)
UserIDField = fields.str_based_field_factory(UserID)
DeviceIDField = fields.str_based_field_factory(DeviceID)
DeviceLabelField = fields.str_based_field_factory(DeviceLabel)


class HumanHandleField(fields.Field[HumanHandle]):
    email_field = fields.String(required=True)
    label_field = fields.String(required=True)
    args = (email_field, label_field)

    def _serialize(
        self, value: HumanHandle | None, attr: str, data: object
    ) -> Optional[Tuple[str, str]]:
        if value is None:
            return None
        return (value.email, value.label)

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> HumanHandle:
        if not isinstance(value, (list, tuple)):
            raise ValidationError("Expecting list or tuple")
        try:
            email, label = value
        except ValueError:
            raise ValidationError("Expecting two elements")
        return HumanHandle(self.email_field.deserialize(email), self.label_field.deserialize(label))


class UserProfile(Enum):
    """
    Standard user can create new realms and invite new devices for himself.

    Admin can invite and revoke users and on top of what standard user can do.

    Outsider is only able to collaborate on existing realm and can only
    access redacted certificates (i.e. the realms created by an outsider
    cannot be shared and the outsider cannot be OWNER/MANAGER
    on a realm shared with him)
    """

    ADMIN = "ADMIN"
    STANDARD = "STANDARD"
    OUTSIDER = "OUTSIDER"


UserProfileField = fields.enum_field_factory(UserProfile)
