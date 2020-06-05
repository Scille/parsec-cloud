# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
from pathlib import Path

from parsec.serde import packb, unpackb
from parsec.core.types import LocalDevice
from parsec.core.local_device import (
    get_key_file,
    list_available_devices,
    load_device_with_password,
    save_device_with_password,
    change_device_password,
    LocalDeviceCryptoError,
    LocalDeviceNotFoundError,
    LocalDeviceAlreadyExistsError,
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

    o2d11 = local_device_factory("d1@1", org2)
    o2d12 = local_device_factory("d1@2", org2)
    o2d21 = local_device_factory("d2@1", org2)

    for device in [o1d11, o1d12, o1d21]:
        save_device_with_password(config_dir, device, "S3Cr37")

    for device in [o2d11, o2d12, o2d21]:
        save_device_with_password(config_dir, device, "secret")

    # Also add dummy stuff that should be ignored
    (config_dir / "bad1").touch()
    (config_dir / "373955f566#corp#bob@laptop").mkdir()
    dummy_slug = "a54ed6df3a#corp#alice@laptop"
    (config_dir / dummy_slug).mkdir()
    (config_dir / dummy_slug / f"{dummy_slug}.keys").write_bytes(b"dummy")

    devices = list_available_devices(config_dir)

    assert set(devices) == {
        (o1d11.organization_id, o1d11.device_id, "password", get_key_file(config_dir, o1d11)),
        (o1d12.organization_id, o1d12.device_id, "password", get_key_file(config_dir, o1d12)),
        (o1d21.organization_id, o1d21.device_id, "password", get_key_file(config_dir, o1d21)),
        (o2d11.organization_id, o2d11.device_id, "password", get_key_file(config_dir, o2d11)),
        (o2d12.organization_id, o2d12.device_id, "password", get_key_file(config_dir, o2d12)),
        (o2d21.organization_id, o2d21.device_id, "password", get_key_file(config_dir, o2d21)),
    }


@pytest.mark.parametrize("path_exists", (True, False))
def test_password_save_and_load(path_exists, config_dir, alice):
    config_dir = config_dir if path_exists else config_dir / "dummy"
    save_device_with_password(config_dir, alice, "S3Cr37")

    key_file = get_key_file(config_dir, alice)
    alice_reloaded = load_device_with_password(key_file, "S3Cr37")
    assert alice == alice_reloaded


def test_load_bad_password(config_dir, alice):
    save_device_with_password(config_dir, alice, "S3Cr37")

    with pytest.raises(LocalDeviceCryptoError):
        key_file = get_key_file(config_dir, alice)
        load_device_with_password(key_file, "dummy")


def test_load_bad_data(config_dir, alice):
    alice_key = get_key_file(config_dir, alice)
    alice_key.parent.mkdir(parents=True)
    alice_key.write_bytes(b"dummy")

    with pytest.raises(LocalDevicePackingError):
        key_file = get_key_file(config_dir, alice)
        load_device_with_password(key_file, "S3Cr37")


def test_password_save_already_existing(config_dir, alice):
    save_device_with_password(config_dir, alice, "S3Cr37")

    with pytest.raises(LocalDeviceAlreadyExistsError):
        save_device_with_password(config_dir, alice, "S3Cr37")


def test_password_load_not_found(config_dir, alice):
    with pytest.raises(LocalDeviceNotFoundError):
        key_file = get_key_file(config_dir, alice)
        load_device_with_password(key_file, "S3Cr37")


def test_same_device_id_different_orginazations(config_dir, alice, otheralice):
    devices = (alice, otheralice)

    for device in devices:
        save_device_with_password(config_dir, device, f"S3Cr37-{device.organization_id}")

    for device in devices:
        key_file = get_key_file(config_dir, device)
        device_reloaded = load_device_with_password(key_file, f"S3Cr37-{device.organization_id}")
        assert device == device_reloaded


def test_change_password(config_dir, alice):
    old_password = "0ldP@ss"
    new_password = "N3wP@ss"

    save_device_with_password(config_dir, alice, old_password)
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
        "device_id": str(alice.device_id),
        "signing_key": alice.signing_key.encode(),
        "private_key": alice.private_key.encode(),
        "is_admin": True,
        "user_manifest_id": UUID(alice.user_manifest_id.hex),
        "user_manifest_key": bytes(alice.user_manifest_key),
        "local_symkey": bytes(alice.local_symkey),
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
        "profile": alice.profile.value,
        "human_handle": None,
        "device_label": None,
    }
