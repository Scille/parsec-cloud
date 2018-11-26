from typing import NewType
import re


UserID = NewType("UserID", str)
DeviceName = NewType("DeviceName", str)
DeviceID = NewType("DeviceID", str)


def parse_user_id(raw):
    if not re.match(r"^\w{1,32}$", raw):
        raise ValueError("Invalid user ID")
    return UserID(raw)


def parse_device_name(raw):
    if not re.match(r"^\w{1,32}$", raw):
        raise ValueError("Invalid device name")
    return DeviceName(raw)


def parse_device_id(raw):
    if not re.match(r"^\w{1,32}@\w{1,32}$", raw):
        raise ValueError("Invalid device ID")
    return DeviceID(raw)


__all__ = (
    "UserID",
    "DeviceName",
    "DeviceID",
    "parse_user_id",
    "parse_device_name",
    "parse_device_id",
)
