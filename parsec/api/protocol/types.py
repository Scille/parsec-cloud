# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import re
from unicodedata import normalize
from typing import Union, TypeVar, Type, NoReturn
from uuid import uuid4
from enum import Enum
from collections import namedtuple
from email.utils import parseaddr

from parsec.serde import fields

UserIDTypeVar = TypeVar("UserIDTypeVar", bound="UserID")
DeviceIDTypeVar = TypeVar("DeviceIDTypeVar", bound="DeviceID")
DeviceNameTypeVar = TypeVar("DeviceNameTypeVar", bound="DeviceName")


def _bytes_size(txt: str) -> int:
    return len(txt.encode("utf8"))


class OrganizationID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __new__(cls, raw: str) -> "OrganizationID":
        raw = normalize("NFC", raw)
        if not cls.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid organization ID")
        return super(OrganizationID, cls).__new__(cls, raw)

    def __repr__(self) -> str:
        return f"<OrganizationID {super().__repr__()}>"


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __new__(cls, raw: str) -> "UserID":
        raw = normalize("NFC", raw)
        if not cls.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid user name")
        return super(UserID, cls).__new__(cls, raw)

    def __repr__(self) -> str:
        return f"<UserID {super().__repr__()}>"

    @classmethod
    def new(cls: Type[UserIDTypeVar]) -> UserIDTypeVar:
        return cls(uuid4().hex)

    def to_device_id(self, device_name: Union[str, "DeviceID"]) -> "DeviceID":
        return DeviceID(f"{self}@{device_name}")


class DeviceName(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __new__(cls, raw: str) -> "DeviceName":
        raw = normalize("NFC", raw)
        if not cls.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid device name")
        return super(DeviceName, cls).__new__(cls, raw)

    def __repr__(self) -> str:
        return f"<DeviceName {super().__repr__()}>"

    @classmethod
    def new(cls: Type[DeviceNameTypeVar]) -> DeviceNameTypeVar:
        return cls(uuid4().hex)


class DeviceID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}@[\w\-]{1,32}$")

    def __new__(cls, raw: str) -> "DeviceID":
        raw = normalize("NFC", raw)
        if not cls.regex.match(raw) or _bytes_size(raw) > 65:
            raise ValueError("Invalid device ID")
        return super(DeviceID, cls).__new__(cls, raw)

    def __repr__(self) -> str:
        return f"<DeviceID {super().__repr__()}>"

    @property
    def user_id(self) -> UserID:
        return UserID(self.split("@")[0])

    @property
    def device_name(self) -> DeviceName:
        return DeviceName(self.split("@")[1])

    @classmethod
    def new(cls: Type[DeviceIDTypeVar]) -> DeviceIDTypeVar:
        return cls(f"{uuid4().hex}@{uuid4().hex}")


OrganizationIDField = fields.str_based_field_factory(OrganizationID)
UserIDField = fields.str_based_field_factory(UserID)
DeviceNameField = fields.str_based_field_factory(DeviceName)
DeviceIDField = fields.str_based_field_factory(DeviceID)


class HumanHandle(namedtuple("HumanHandle", "email label")):
    __slots__ = ()

    def __new__(cls, email: str, label: str) -> "HumanHandle":
        email = normalize("NFC", email)
        label = normalize("NFC", label)

        # TODO: how to check the email  easily ?
        if not 0 < _bytes_size(email) < 255:
            raise ValueError("Invalid email address")

        if not 0 < _bytes_size(label) < 255:
            raise ValueError("Invalid label")

        parsed_label, parsed_email = parseaddr(f"{label} <{email}>")
        if parsed_email != email or parsed_label != label:
            raise ValueError("Invalid email/label couple")

        return super(HumanHandle, cls).__new__(cls, email, label)

    def __repr__(self) -> str:
        return f"<HumanHandle {str(self)} >"

    def __str__(self) -> str:
        return f"{self.label} <{self.email}>"

    def __eq__(self, other: object) -> bool:
        # Ignore label field, as it is only for human redability
        return isinstance(other, HumanHandle) and self.email == other.email

    def __gt__(self, other: object) -> NoReturn:
        raise NotImplementedError

    def __hash__(self) -> int:
        return hash(self.email)


class HumanHandleField(fields.Tuple):
    def __init__(self, **kwargs: object):
        email = fields.String(required=True)
        label = fields.String(required=True)
        super().__init__(email, label, **kwargs)

    def _deserialize(self, *args: object, **kwargs: object) -> HumanHandle:
        result = super()._deserialize(*args, **kwargs)
        return HumanHandle(*result)


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
