# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re

from parsec.serde import fields


def _bytes_size(txt: str) -> int:
    return len(txt.encode("utf8"))


class OrganizationID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid organization ID")

    def __repr__(self):
        return f"<OrganizationID {super().__repr__()}>"


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid user name")

    def __repr__(self):
        return f"<UserID {super().__repr__()}>"


class DeviceName(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 32:
            raise ValueError("Invalid device name")

    def __repr__(self):
        return f"<DeviceName {super().__repr__()}>"


class DeviceID(str):
    __slots__ = ()
    regex = re.compile(r"^[\w\-]{1,32}@[\w\-]{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw) or _bytes_size(raw) > 65:
            raise ValueError("Invalid device ID")

    def __repr__(self):
        return f"<DeviceID {super().__repr__()}>"

    @property
    def user_id(self) -> UserID:
        return UserID(self.split("@")[0])

    @property
    def device_name(self) -> DeviceName:
        return DeviceName(self.split("@")[1])


OrganizationIDField = fields.str_based_field_factory(OrganizationID)
UserIDField = fields.str_based_field_factory(UserID)
DeviceNameField = fields.str_based_field_factory(DeviceName)
DeviceIDField = fields.str_based_field_factory(DeviceID)
