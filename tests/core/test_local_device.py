# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
import pytest
from pathlib import Path
from uuid import UUID, uuid4

from parsec.crypto import SigningKey
from parsec.serde import packb, unpackb
from parsec.api.protocol import OrganizationID, DeviceID, DeviceLabel, HumanHandle
from parsec.core.types import LocalDevice
from parsec.core.local_device import (
    AvailableDevice,
    DeviceFileType,
    get_key_file,
    get_default_key_file,
    get_devices_dir,
    list_available_devices,
    load_device_with_password,
    save_device_with_password_in_config,
    change_device_password,
    LocalDeviceCryptoError,
    LocalDeviceNotFoundError,
    LocalDevicePackingError,
)

from tests.common import customize_fixtures


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        pass


@pytest.fixture
def config_dir(tmpdir):
    return Path(tmpdir)


@pytest.mark.parametrize("path_exists", (True, False))
def test_list_no_devices(path_exists, config_dir):
    config_dir = config_dir if path_exists else config_dir / "dummy"
    devices = list_available_devices(config_dir)
    assert not devices


def test_list_devices(organization_factory, local_device_factory, config_dir):
    org1 = organization_factory("org1")
    org2 = organization_factory("org2")

    o1d11 = local_device_factory("d1@1", org1)
    o1d12 = local_device_factory("d1@2", org1)
    o1d21 = local_device_factory("d2@1", org1)

    o2d11 = local_device_factory("d1@1", org2, has_human_handle=False)
    o2d12 = local_device_factory("d1@2", org2, has_device_label=False)
    o2d21 = local_device_factory("d2@1", org2, has_human_handle=False, has_device_label=False)

    for device in [o1d11, o1d12, o1d21]:
        save_device_with_password_in_config(config_dir, device, "S3Cr37")

    for device in [o2d11, o2d12, o2d21]:
        save_device_with_password_in_config(config_dir, device, "secret")

    # Also add dummy stuff that should be ignored
    device_dir = config_dir / "devices"
    (device_dir / "bad1").touch()
    (device_dir / "373955f566#corp#bob@laptop").mkdir()
    dummy_slug = "a54ed6df3a#corp#alice@laptop"
    (device_dir / dummy_slug).mkdir()
    (device_dir / dummy_slug / f"{dummy_slug}.keys").write_bytes(b"dummy")

    devices = list_available_devices(config_dir)

    expected_devices = {
        AvailableDevice(
            key_file_path=get_key_file(config_dir, d),
            organization_id=d.organization_id,
            device_id=d.device_id,
            human_handle=d.human_handle,
            device_label=d.device_label,
            slug=d.slug,
            type=DeviceFileType.PASSWORD,
        )
        for d in [o1d11, o1d12, o1d21, o2d11, o2d12, o2d21]
    }
    assert set(devices) == expected_devices


def test_list_devices_support_legacy_file_without_labels(config_dir):
    # Craft file data without the labels fields
    key_file_data = packb({"type": "password", "salt": b"12345", "ciphertext": b"whatever"})
    slug = "9d84fbd57a#Org#Zack@PC1"
    key_file_path = get_devices_dir(config_dir) / slug / f"{slug}.keys"
    key_file_path.parent.mkdir(parents=True)
    key_file_path.write_bytes(key_file_data)

    devices = list_available_devices(config_dir)
    expected_device = AvailableDevice(
        key_file_path=key_file_path,
        organization_id=OrganizationID("Org"),
        device_id=DeviceID("Zack@PC1"),
        human_handle=None,
        device_label=None,
        slug=slug,
        type=DeviceFileType.PASSWORD,
    )
    assert devices == [expected_device]


def test_available_device_display(config_dir, alice):
    without_labels = AvailableDevice(
        key_file_path=get_default_key_file(config_dir, alice),
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        human_handle=None,
        device_label=None,
        slug=alice.slug,
        type=DeviceFileType.PASSWORD,
    )

    with_labels = AvailableDevice(
        key_file_path=get_default_key_file(config_dir, alice),
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        human_handle=alice.human_handle,
        device_label=alice.device_label,
        slug=alice.slug,
        type=DeviceFileType.PASSWORD,
    )

    assert without_labels.device_display == alice.device_name.str
    assert without_labels.user_display == alice.user_id.str

    assert with_labels.device_display == alice.device_label.str
    assert with_labels.user_display == alice.human_handle.str


def test_available_devices_slughash_uniqueness(
    organization_factory, local_device_factory, config_dir
):
    def _to_available(device):
        return AvailableDevice(
            key_file_path=get_default_key_file(config_dir, device),
            organization_id=device.organization_id,
            device_id=device.device_id,
            human_handle=device.human_handle,
            device_label=device.device_label,
            slug=device.slug,
            type=DeviceFileType.PASSWORD,
        )

    def _assert_different_as_available(d1, d2):
        available_device_d1 = _to_available(d1)
        available_device_d2 = _to_available(d2)
        assert available_device_d1.slughash != available_device_d2.slughash
        # Make sure slughash is consistent between LocalDevice and AvailableDevice
        assert available_device_d1.slughash == d1.slughash
        assert available_device_d2.slughash == d2.slughash

    o1 = organization_factory("org1")
    o2 = organization_factory("org2")

    # Different user id
    o1u1d1 = local_device_factory("u1@d1", o1)
    o1u2d1 = local_device_factory("u2@d1", o1)
    _assert_different_as_available(o1u1d1, o1u2d1)

    # Different device name
    o1u1d2 = local_device_factory("u1@d2", o1)
    _assert_different_as_available(o1u1d1, o1u1d2)

    # Different organization id
    o2u1d1 = local_device_factory("u1@d1", o2)
    _assert_different_as_available(o1u1d1, o2u1d1)

    # Same organization_id, but different root verify key !
    dummy_key = SigningKey.generate().verify_key
    o1u1d1_bad_rvk = o1u1d1.evolve(
        organization_addr=o1u1d1.organization_addr.build(
            backend_addr=o1u1d1.organization_addr.get_backend_addr(),
            organization_id=o1u1d1.organization_addr.organization_id,
            root_verify_key=dummy_key,
        )
    )
    _assert_different_as_available(o1u1d1, o1u1d1_bad_rvk)

    # Finally make sure slughash is stable through save/load
    save_device_with_password_in_config(config_dir, o1u1d1, "S3Cr37")
    key_file = get_key_file(config_dir, o1u1d1)
    o1u1d1_reloaded = load_device_with_password(key_file, "S3Cr37")
    available_device = _to_available(o1u1d1)
    available_device_reloaded = _to_available(o1u1d1_reloaded)
    assert available_device.slughash == available_device_reloaded.slughash


@pytest.mark.parametrize("path_exists", (True, False))
def test_password_save_and_load(path_exists, config_dir, alice):
    config_dir = config_dir if path_exists else config_dir / "dummy"
    save_device_with_password_in_config(config_dir, alice, "S3Cr37")

    key_file = get_key_file(config_dir, alice)
    alice_reloaded = load_device_with_password(key_file, "S3Cr37")
    assert alice == alice_reloaded


def test_load_bad_password(config_dir, alice):
    save_device_with_password_in_config(config_dir, alice, "S3Cr37")

    with pytest.raises(LocalDeviceCryptoError):
        key_file = get_key_file(config_dir, alice)
        load_device_with_password(key_file, "dummy")


def test_load_bad_data(config_dir, alice):
    alice_key = get_default_key_file(config_dir, alice)
    alice_key.parent.mkdir(parents=True)
    alice_key.write_bytes(b"dummy")

    with pytest.raises(LocalDevicePackingError):
        load_device_with_password(alice_key, "S3Cr37")


def test_password_save_already_existing(config_dir, alice, alice2, otheralice):
    save_device_with_password_in_config(config_dir, alice, "S3Cr37")

    # Different devices should not overwrite each other
    save_device_with_password_in_config(config_dir, otheralice, "S3Cr37")
    save_device_with_password_in_config(config_dir, alice2, "S3Cr37")

    # Overwritting self is allowed
    save_device_with_password_in_config(config_dir, alice, "S3Cr37")

    devices = list_available_devices(config_dir)
    assert len(devices) == 3


def test_password_load_not_found(config_dir, alice):
    with pytest.raises(LocalDeviceNotFoundError):
        key_file = get_default_key_file(config_dir, alice)
        load_device_with_password(key_file, "S3Cr37")


def test_same_device_id_different_orginazations(config_dir, alice, otheralice):
    devices = (alice, otheralice)

    for device in devices:
        save_device_with_password_in_config(
            config_dir, device, f"S3Cr37-{device.organization_id.str}"
        )

    for device in devices:
        key_file = get_key_file(config_dir, device)
        device_reloaded = load_device_with_password(
            key_file, f"S3Cr37-{device.organization_id.str}"
        )
        assert device == device_reloaded


def test_change_password(config_dir, alice):
    old_password = "0ldP@ss"
    new_password = "N3wP@ss"

    save_device_with_password_in_config(config_dir, alice, old_password)
    key_file = get_key_file(config_dir, alice)

    change_device_password(key_file, old_password, new_password)

    alice_reloaded = load_device_with_password(key_file, new_password)
    assert alice == alice_reloaded

    with pytest.raises(LocalDeviceCryptoError):
        load_device_with_password(key_file, old_password)


@customize_fixtures(alice_has_human_handle=False, alice_has_device_label=False)
def test_supports_legacy_is_admin_field(alice):
    # Manually craft a local user in legacy format
    raw_legacy_local_user = {
        "organization_addr": alice.organization_addr.to_url(),
        "device_id": alice.device_id.str,
        "signing_key": alice.signing_key.encode(),
        "private_key": alice.private_key.encode(),
        "is_admin": True,
        "user_manifest_id": UUID(alice.user_manifest_id.str),
        "user_manifest_key": bytes(alice.user_manifest_key.secret),
        "local_symkey": bytes(alice.local_symkey.secret),
    }
    dumped_legacy_local_user = packb(raw_legacy_local_user)

    # Make sure the legacy format can be loaded
    legacy_local_user = LocalDevice.load(dumped_legacy_local_user)
    assert legacy_local_user == alice

    # Manually decode new format to check it is compatible with legacy
    dumped_local_user = alice.dump()
    raw_local_user = unpackb(dumped_local_user)
    assert raw_local_user == {
        **raw_legacy_local_user,
        "profile": alice.profile.str,
        "human_handle": None,
        "device_label": None,
    }


def test_list_devices_support_legacy_file_with_meaningful_name(config_dir):
    # Legacy path might exceed the 256 characters limit in some cases (see issue #1356)
    # So we use the `\\?\` workaround: https://stackoverflow.com/a/57502760/2846140
    if sys.platform == "win32":
        config_dir = Path("\\\\?\\" + str(config_dir.resolve()))

    # Device information
    user_id = uuid4().hex
    device_name = uuid4().hex
    organization_id = "Org"
    rvk_hash = (uuid4().hex)[:10]
    device_id = f"{user_id}@{device_name}"
    slug = f"{rvk_hash}#{organization_id}#{device_id}"
    human_label = "Billy Mc BillFace"
    human_email = "billy@bill.com"
    device_label = "My device"

    # Craft file data without the user_id, organization_id and slug fields
    key_file_data = packb(
        {
            "type": "password",
            "salt": b"12345",
            "ciphertext": b"whatever",
            "human_handle": (human_email.encode(), human_label.encode()),
            "device_label": device_label,
        }
    )

    key_file_path = get_devices_dir(config_dir) / slug / f"{slug}.keys"
    key_file_path.parent.mkdir(parents=True)
    key_file_path.write_bytes(key_file_data)

    devices = list_available_devices(config_dir)
    expected_device = AvailableDevice(
        key_file_path=key_file_path,
        organization_id=OrganizationID(organization_id),
        device_id=DeviceID(device_id),
        human_handle=HumanHandle(human_email, human_label),
        device_label=DeviceLabel(device_label),
        slug=slug,
        type=DeviceFileType.PASSWORD,
    )
    assert devices == [expected_device]
    assert get_key_file(config_dir, expected_device) == key_file_path


@pytest.mark.parametrize("type", ("password", "smartcard"))
def test_list_devices_support_key_file(config_dir, type):
    if type == "password":
        data_extra = {"type": "password", "salt": b"12345"}
        available_device_extra = {"type": DeviceFileType.PASSWORD}

    elif type == "smartcard":
        data_extra = {
            "type": "smartcard",
            "encrypted_key": b"12345",
            "certificate_id": "42",
            "certificate_sha1": b"12345",
        }
        available_device_extra = {"type": DeviceFileType.SMARTCARD}

    # Device information
    user_id = uuid4().hex
    device_name = uuid4().hex
    organization_id = "Org"
    rvk_hash = (uuid4().hex)[:10]
    device_id = f"{user_id}@{device_name}"
    slug = f"{rvk_hash}#{organization_id}#{device_id}"
    human_label = "Billy Mc BillFace"
    human_email = "billy@bill.com"
    device_label = "My device"

    # Craft file data
    key_file_data = packb(
        {
            "ciphertext": b"whatever",
            "human_handle": (human_email.encode(), human_label.encode()),
            "device_label": device_label,
            "device_id": device_id,
            "organization_id": organization_id,
            "slug": slug,
            **data_extra,
        }
    )

    key_file_path = get_devices_dir(config_dir) / "device.keys"
    key_file_path.parent.mkdir(parents=True)
    key_file_path.write_bytes(key_file_data)

    devices = list_available_devices(config_dir)
    expected_device = AvailableDevice(
        key_file_path=key_file_path,
        organization_id=OrganizationID(organization_id),
        device_id=DeviceID(device_id),
        human_handle=HumanHandle(human_email, human_label),
        device_label=DeviceLabel(device_label),
        slug=slug,
        **available_device_extra,
    )
    assert devices == [expected_device]
    assert get_key_file(config_dir, expected_device) == key_file_path


def test_multiple_files_same_device(config_dir, alice):
    path = save_device_with_password_in_config(config_dir, alice, "test")

    # File names contain the slughash
    assert path.stem == alice.slughash

    # .. but are no longer meaningful
    (path.parent / "testing.keys").write_bytes(path.read_bytes())

    # Make sure we don't list duplicates
    devices = list_available_devices(config_dir)
    assert len(devices) == 1
    assert devices[0].device_id == alice.device_id

    # Remove orignal file
    path.unlink()
    devices = list_available_devices(config_dir)
    assert len(devices) == 1
    assert devices[0].device_id == alice.device_id
