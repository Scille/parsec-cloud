# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import re
from unicodedata import normalize
from typing import Union, TypeVar, Type, NoReturn, TYPE_CHECKING, Optional, Tuple, Pattern
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


class OrganizationID(StrBased):
    __slots__ = ()
    REGEX = re.compile(r"^[\w\-]{1,32}$")
    MAX_BYTE_SIZE = 32


_PyOrganizationID = OrganizationID
if not TYPE_CHECKING:
    try:
        from libparsec.types import OrganizationID as _RsOrganizationID
    except:
        pass
    else:
        OrganizationID = _RsOrganizationID


class UserID(StrBased):
    __slots__ = ()
    REGEX = re.compile(r"^[\w\-]{1,32}$")
    MAX_BYTE_SIZE = 32

    @classmethod
    def new(cls: Type[UserIDTypeVar]) -> UserIDTypeVar:
        return cls(uuid4().hex)

    def to_device_id(self, device_name: Union[str, "DeviceID"]) -> "DeviceID":
        return DeviceID(f"{self._str}@{device_name}")


_PyUserID = UserID
if not TYPE_CHECKING:
    try:
        from libparsec.types import UserID as _RsUserID
    except:
        pass
    else:
        UserID = _RsUserID


class DeviceName(StrBased):
    __slots__ = ()
    REGEX = re.compile(r"^[\w\-]{1,32}$")
    MAX_BYTE_SIZE = 32

    @classmethod
    def new(cls: Type[DeviceNameTypeVar]) -> DeviceNameTypeVar:
        return cls(uuid4().hex)


_PyDeviceName = DeviceName
if not TYPE_CHECKING:
    try:
        from libparsec.types import DeviceName as _RsDeviceName
    except:
        pass
    else:
        DeviceName = _RsDeviceName


class DeviceID(StrBased):
    __slots__ = ()
    REGEX = re.compile(r"^[\w\-]{1,32}@[\w\-]{1,32}$")
    MAX_BYTE_SIZE = 65

    @property
    def user_id(self) -> UserID:
        return UserID(self._str.split("@")[0])

    @property
    def device_name(self) -> DeviceName:
        return DeviceName(self._str.split("@")[1])

    @classmethod
    def new(cls: Type[DeviceIDTypeVar]) -> DeviceIDTypeVar:
        return cls(f"{uuid4().hex}@{uuid4().hex}")


_PyDeviceID = DeviceID
if not TYPE_CHECKING:
    try:
        from libparsec.types import DeviceID as _RsDeviceID
    except:
        pass
    else:
        DeviceID = _RsDeviceID


class DeviceLabel(StrBased):
    REGEX = re.compile(r"^.+$")  # At least 1 character
    MAX_BYTE_SIZE = 255


_PyDeviceLabel = DeviceLabel
if not TYPE_CHECKING:
    try:
        from libparsec.types import DeviceLabel as _RsDeviceLabel
    except:
        pass
    else:
        DeviceLabel = _RsDeviceLabel

OrganizationIDField = fields.str_based_field_factory(OrganizationID)
UserIDField = fields.str_based_field_factory(UserID)
DeviceNameField = fields.str_based_field_factory(DeviceName)
DeviceIDField = fields.str_based_field_factory(DeviceID)
DeviceLabelField = fields.str_based_field_factory(DeviceLabel)


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

        return super(_PyHumanHandle, cls).__new__(cls, email, label)

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


_PyHumanHandle = HumanHandle
if not TYPE_CHECKING:
    try:
        from libparsec.types import HumanHandle as _RsHumanHandle
    except ImportError:
        pass
    else:
        HumanHandle = _RsHumanHandle


class HumanHandleField(fields.Tuple):
    def __init__(self, **kwargs: object):
        email = fields.String(required=True)
        label = fields.String(required=True)
        super().__init__(email, label, **kwargs)

    def _serialize(
        self, value: HumanHandle, attr: object, data: object
    ) -> Optional[Tuple[str, str]]:
        if value is None:
            return None
        return (value.email, value.label)

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
