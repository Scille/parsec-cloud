# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.api.protocol import UserID, DeviceID, DeviceName, OrganizationID, HumanHandle
from parsec.api.data import SASCode, EntryName


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
        "飞" * 10 + "xx@xx" + "飞" * 10,  # 65 bytes long utf8 string
        "X1-_é飞@X1-_é飞",  # Mix-and-match
    ),
)
def test_good_pattern_device_id(data):
    DeviceID(data)


def test_human_handle_compare():
    a = HumanHandle(email="alice@example.com", label="Alice")
    a2 = HumanHandle(email="alice@example.com", label="Whatever")
    b = HumanHandle(email="bob@example.com", label="Bob")
    assert a == a2
    assert a != b
    assert b == b


@pytest.mark.parametrize(
    "email,label",
    (
        ("alice@example.com", "Alice"),
        ("a@x", "A"),  # Smallest size
        (f"{'a' * 64}@{'x' * 185}.com", "x" * 254),  # Max sizes
        (f"{'飞' * 21}@{'飞' * 62}.com", f"{'飞' * 84}xx"),  # Unicode & max size
    ),
)
def test_valid_human_handle(email, label):
    HumanHandle(email, label)


@pytest.mark.parametrize(
    "email,label",
    (
        ("alice@example.com", "x" * 255),
        (f"{'@example.com':a>255}", "Alice"),
        ("alice@example.com", "飞" * 85),  # 255 bytes long utf8 label
        (f"{'飞' * 21}@{'飞' * 63}.x", "Alice"),  # 255 bytes long utf8 email
        ("alice@example.com", ""),  # Empty label
        ("", "Alice"),  # Empty email
    ),
)
def test_invalid_human_handle(email, label):
    with pytest.raises(ValueError):
        HumanHandle(email, label)


def test_sas_code():
    assert SASCode.from_int(0x0) == SASCode("AAAA")
    assert SASCode.from_int(0x1) == SASCode("BAAA")
    # [...]
    assert SASCode.from_int(0x84001) == SASCode("BASS")
    # [...]
    assert SASCode.from_int(0xFFFFE) == SASCode("8999")
    assert SASCode.from_int(0xFFFFF) == SASCode("9999")

    with pytest.raises(ValueError):
        SASCode.from_int(2 ** 20)

    with pytest.raises(ValueError):
        SASCode.from_int(-1)

    for invalid in ["", "AAA", "AAAAA", "aaaa", "AAAI", "AAAO", "AAA0", "AAA1"]:
        with pytest.raises(ValueError):
            SASCode(invalid)


def test_entryname_normalization():
    nfc_str = "àáâäæãåāçćčèéêëēėęîïíīįìłñńôöòóœøōõßśšûüùúūÿžźż"
    nfd_str = (
        "àáâäæãåāçćčèéêëēėęîïíīįìłñńôöòóœøōõßśšûüùúūÿžźż"
    )

    assert nfc_str != nfd_str
    assert EntryName(nfd_str) == nfc_str
    assert EntryName(nfc_str) == nfc_str
    assert EntryName(nfc_str + nfd_str) == nfc_str + nfc_str
