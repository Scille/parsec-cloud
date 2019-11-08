# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocol import UserID, DeviceID, DeviceName, OrganizationID


@pytest.mark.parametrize("cls", (UserID, DeviceName, OrganizationID))
@pytest.mark.parametrize(
    "data",
    (
        "!x",  # Invalid character
        " x",  # Invalid character
        "x" * 33,  # Too long
        # Sinogram encoded on 3 bytes with utf8, so those 11 characters
        # form a 33 bytes long utf8 string !
        "飞" * 11,
        "😀",  # Not a unicode word
        "",
    ),
)
def test_max_bytes_size(cls, data):
    with pytest.raises(ValueError):
        cls(data)


@pytest.mark.parametrize("cls", (UserID, DeviceName, OrganizationID))
@pytest.mark.parametrize(
    "data", ("x", "x" * 32, "飞" * 10 + "xx", "X1-_é飞")  # 32 bytes long utf8 string  # Mix-and-match
)
def test_good_pattern(cls, data):
    cls(data)


@pytest.mark.parametrize(
    "data",
    (
        "!x@x",  # Invalid character
        "x@ ",  # Invalid character
        "x" * 66,  # Too long
        # Sinogram encoded on 3 bytes with utf8, so those 22 characters
        # form a 66 bytes long utf8 string !
        "飞" * 22,
        "😀@x",  # Not a unicode word
        "x",  # Missing @ separator
        "@x",
        "x@",
        "x" * 62 + "@x",  # Respect overall length but not UserID length
        "x@" + "x" * 62,  # Respect overall length but not DeviceName length
        "",
    ),
)
def test_max_bytes_size_device_id(data):
    with pytest.raises(ValueError):
        DeviceID(data)


@pytest.mark.parametrize(
    "data",
    (
        "x@x",
        "x" * 32 + "@" + "x" * 32,
        "飞" * 10 + "xx@xx" + "飞" * 10,  # 64 bytes long utf8 string
        "X1-_é飞@X1-_é飞",  # Mix-and-match
    ),
)
def test_good_pattern_device_id(data):
    DeviceID(data)
