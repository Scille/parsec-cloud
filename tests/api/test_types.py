# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import zlib
from unicodedata import normalize

from parsec.serde import packb
from parsec.crypto import SecretKey
from parsec.api.protocol import UserID, DeviceID, DeviceName, OrganizationID, HumanHandle
from parsec.api.data import (
    DataError,
    SASCode,
    EntryName,
    EntryNameTooLongError,
    FileManifest as RemoteFileManifest,
    FolderManifest as RemoteFolderManifest,
    WorkspaceManifest as RemoteWorkspaceManifest,
    UserManifest as RemoteUserManifest,
)
from parsec.core.types import (
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
)
from parsec._parsec import LocalDevice


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
def test_normalization(cls):
    nfc_str = normalize("NFC", "àæßšūÿź")
    nfd_str = normalize("NFD", nfc_str)

    assert nfc_str != nfd_str
    assert str(cls(nfd_str)) == nfc_str
    assert str(cls(nfc_str)) == nfc_str
    assert str(cls(nfc_str + nfd_str)) == nfc_str + nfc_str


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
        ("john.doe@example.com", "J.D."),
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
        ("", "Alice <alice@example.com>"),  # Empty email and misleading label
        ("Alice <alice@example.com>", ""),  # Empty label and misleading label
        ("Alice <@example.com>", "Alice"),  # Missing local part in email
    ),
)
def test_invalid_human_handle(email, label):
    with pytest.raises(ValueError):
        HumanHandle(email, label)


def test_human_handle_normalization():
    nfc_label = normalize("NFC", "àæßšūÿź")
    nfd_label = normalize("NFD", nfc_label)
    nfc_email = normalize("NFC", "àæßš@ūÿ.ź")
    nfd_email = normalize("NFD", nfc_email)
    assert nfc_label != nfd_label
    assert nfc_email != nfd_email

    hh = HumanHandle(nfd_email, nfd_label)
    assert hh.email == nfc_email
    assert hh.label == nfc_label

    hh = HumanHandle(nfc_email, nfc_label)
    assert hh.email == nfc_email
    assert hh.label == nfc_label


def test_sas_code():
    assert SASCode.from_int(0x0) == SASCode("AAAA")
    assert SASCode.from_int(0x1) == SASCode("BAAA")
    # [...]
    assert SASCode.from_int(0x84001) == SASCode("BASS")
    # [...]
    assert SASCode.from_int(0xFFFFE) == SASCode("8999")
    assert SASCode.from_int(0xFFFFF) == SASCode("9999")

    with pytest.raises(ValueError):
        SASCode.from_int(2**20)

    # OverflowError for Rust binding
    with pytest.raises((ValueError, OverflowError)):
        SASCode.from_int(-1)

    for invalid in ["", "AAA", "AAAAA", "aaaa", "AAAI", "AAAO", "AAA0", "AAA1"]:
        with pytest.raises(ValueError):
            SASCode(invalid)


@pytest.mark.parametrize(
    "data",
    (
        "foo",
        "foo.txt",
        "x" * 255,  # Max size
        "飞" * 85,  # Unicode & max size
        "X1-_é飞",
        "🌍☄️==🦕🦖💀",  # Probably a bad name for a real folder...
        ".a",  # Dot and dot-dot are allowed if they are not alone
        "..a",
        "a..",
        "a.",
    ),
)
def test_valid_entry_name(data):
    EntryName(data)


@pytest.mark.parametrize("data", ("x" * 256, "飞" * 85 + "x"))
def test_entry_name_too_long(data):
    with pytest.raises(EntryNameTooLongError):
        EntryName(data)


@pytest.mark.parametrize(
    "data",
    (
        ".",  # Not allowed
        "..",  # Not allowed
        "/x",  # Slash not allowed
        "x/x",
        "x/",
        "/",
        "\x00x",  # Null-byte not allowed
        "x\x00x",
        "x\x00",
        "\x00",
    ),
)
def test_invalid_entry_name(data):
    with pytest.raises(ValueError):
        EntryName(data)


def test_entry_name_normalization():
    nfc_str = normalize("NFC", "àáâäæãåāçćčèéêëēėęîïíīįìłñńôöòóœøōõßśšûüùúūÿžźż")
    nfd_str = normalize("NFD", nfc_str)

    assert nfc_str != nfd_str
    assert EntryName(nfd_str).str == nfc_str
    assert EntryName(nfc_str).str == nfc_str
    assert EntryName(nfc_str + nfd_str).str == nfc_str + nfc_str


def test_local_manifests_load_invalid_data():
    key = SecretKey.generate()
    valid_msgpack_but_bad_fields = packb({"foo": 42})
    valid_zip_bud_bad_msgpack = zlib.compress(b"dummy")
    invalid_zip = b"\x42" * 10

    for cls in (
        LocalFileManifest,
        LocalFolderManifest,
        LocalWorkspaceManifest,
        LocalUserManifest,
    ):
        with pytest.raises(DataError):
            cls.decrypt_and_load(b"", key=key)

        with pytest.raises(DataError):
            cls.decrypt_and_load(invalid_zip, key=key)

        with pytest.raises(DataError):
            cls.decrypt_and_load(valid_zip_bud_bad_msgpack, key=key)

        # Valid to deserialize, invalid fields
        with pytest.raises(DataError):
            cls.decrypt_and_load(valid_msgpack_but_bad_fields, key=key)


def test_remote_manifests_load_invalid_data(alice: LocalDevice):
    key = SecretKey.generate()
    valid_zip_msgpack_but_bad_fields = zlib.compress(packb({"foo": 42}))
    valid_zip_bud_bad_msgpack = zlib.compress(b"dummy")
    invalid_zip = b"\x42" * 10

    for cls in (
        RemoteFileManifest,
        RemoteFolderManifest,
        RemoteWorkspaceManifest,
        RemoteUserManifest,
    ):
        print(f"Testing class {cls.__name__}")
        with pytest.raises(DataError):
            cls.decrypt_verify_and_load(
                b"",
                key=key,
                author_verify_key=alice.verify_key,
                expected_author=alice.device_id,
                expected_timestamp=alice.timestamp(),
            )

        with pytest.raises(DataError):
            cls.decrypt_verify_and_load(
                invalid_zip,
                key=key,
                author_verify_key=alice.verify_key,
                expected_author=alice.device_id,
                expected_timestamp=alice.timestamp(),
            )

        with pytest.raises(DataError):
            cls.decrypt_verify_and_load(
                valid_zip_bud_bad_msgpack,
                key=key,
                author_verify_key=alice.verify_key,
                expected_author=alice.device_id,
                expected_timestamp=alice.timestamp(),
            )

        # Valid to deserialize, invalid fields
        with pytest.raises(DataError):
            cls.decrypt_verify_and_load(
                valid_zip_msgpack_but_bad_fields,
                key=key,
                author_verify_key=alice.verify_key,
                expected_author=alice.device_id,
                expected_timestamp=alice.timestamp(),
            )
