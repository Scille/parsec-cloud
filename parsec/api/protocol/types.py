# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Pattern, Tuple, TypeVar, Union
from unicodedata import normalize

from marshmallow import ValidationError

from parsec._parsec import DeviceID, DeviceLabel, HumanHandle, OrganizationID, UserID, UserProfile
from parsec.serde import fields

UserIDTypeVar = TypeVar("UserIDTypeVar", bound="UserID")
DeviceIDTypeVar = TypeVar("DeviceIDTypeVar", bound="DeviceID")


def _bytes_size(txt: str) -> int:
    return len(txt.encode("utf8"))


class StrBased:
    __slots__ = ("_str",)
    REGEX: Pattern[str] | None
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
    ) -> Tuple[str, str] | None:
        if value is None:
            return None
        return (value.email, value.label)

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> HumanHandle:
        if not isinstance(value, (list, tuple)):
            raise ValidationError("Expecting list or tuple")
        try:
            email_value, label_value = value
        except ValueError:
            raise ValidationError("Expecting two elements")
        email = self.email_field.deserialize(email_value)
        label = self.label_field.deserialize(label_value)
        try:
            return HumanHandle(email, label)
        except ValueError as exc:
            raise ValidationError(str(exc))


UserProfileField = fields.rust_enum_field_factory(UserProfile)
