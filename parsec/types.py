import re


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not self.regex.match(raw):
            raise ValueError("Invalid user ID")


class DeviceName(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not self.regex.match(raw):
            raise ValueError("Invalid device name")


class DeviceID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}@\w{1,32}$")

    def __init__(self, raw):
        if not self.regex.match(raw):
            raise ValueError("Invalid device ID")

    @property
    def user_id(self) -> UserID:
        return UserID(self.split("@")[0])

    @property
    def device_name(self) -> DeviceName:
        return DeviceName(self.split("@")[1])


__all__ = ("UserID", "DeviceName", "DeviceID")
