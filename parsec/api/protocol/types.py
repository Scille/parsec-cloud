# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
from typing import Union, TypeVar, Type, NoReturn
from uuid import uuid4
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

    def __init__(self, raw: str):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid organization ID")

    def __repr__(self) -> str:
        return f"<OrganizationID {super().__repr__()}>"


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __init__(self, raw: str):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid user name")

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

    def __init__(self, raw: str):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid device name")

    def __repr__(self) -> str:
        return f"<DeviceName {super().__repr__()}>"

    @classmethod
    def new(cls: Type[DeviceNameTypeVar]) -> DeviceNameTypeVar:
        return cls(uuid4().hex)


class DeviceID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}@[\w\-]{1,32}$")

    def __init__(self, raw: str):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 65:
            raise ValueError("Invalid device ID")

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

    def __init__(self, email: str, label: str):
        # TODO: how to check the email  easily ?
        if not isinstance(email, str) or not 0 < _bytes_size(email) < 255:
            raise ValueError("Invalid email address")

        if not isinstance(label, str) or not 0 < _bytes_size(label) < 255:
            raise ValueError("Invalid label")

        parsed_label, parsed_email = parseaddr(str(self))
        if parsed_email != email:
            raise ValueError("Invalid email address")
        if parsed_label != label:
            raise ValueError("Invalid label")

        # No need to call super().__init__ given namedtuple set attributes during __new__
        super().__init__()

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
