# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pathlib import Path
from unittest.mock import patch

from parsec.core.local_device import (
    get_key_file,
    list_available_devices,
    load_device_with_password,
    save_device_with_password,
    load_device_with_pkcs11,
    save_device_with_pkcs11,
    LocalDeviceCryptoError,
    LocalDeviceNotFoundError,
    LocalDeviceAlreadyExistsError,
    LocalDevicePackingError,
)
from parsec.core.local_device.pkcs11_tools import NoKeysFound, DevicePKCS11Error


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


def test_list_devices(mocked_pkcs11, organization_factory, local_device_factory, config_dir):
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
        _, token_id, key_id = mocked_pkcs11
        save_device_with_pkcs11(config_dir, device, token_id, key_id)

    # Also add dummy stuff that should be ignored
    (config_dir / "bad1").touch()
    (config_dir / "bad2#user@dev").mkdir()
    bad_dir = config_dir / "bad3#user@dev"
    bad_dir.mkdir()
    (bad_dir / "bad3#user@dev.keys").write_bytes(b"dummy")

    devices = list_available_devices(config_dir)
    assert set(devices) == {
        (o1d11.organization_id, o1d11.device_id, "password"),
        (o1d12.organization_id, o1d12.device_id, "password"),
        (o1d21.organization_id, o1d21.device_id, "password"),
        (o2d11.organization_id, o2d11.device_id, "pkcs11"),
        (o2d12.organization_id, o2d12.device_id, "pkcs11"),
        (o2d21.organization_id, o2d21.device_id, "pkcs11"),
    }


@pytest.mark.parametrize("path_exists", (True, False))
def test_password_save_and_load(path_exists, config_dir, alice):
    config_dir = config_dir if path_exists else config_dir / "dummy"
    save_device_with_password(config_dir, alice, "S3Cr37")

    alice_reloaded = load_device_with_password(
        config_dir, alice.organization_id, alice.device_id, "S3Cr37"
    )
    assert alice == alice_reloaded


def test_load_bad_password(config_dir, alice):
    save_device_with_password(config_dir, alice, "S3Cr37")

    with pytest.raises(LocalDeviceCryptoError):
        load_device_with_password(config_dir, alice.organization_id, alice.device_id, "dummy")


def test_load_bad_data(config_dir, alice):
    alice_key = get_key_file(config_dir, alice.organization_id, alice.device_id)
    alice_key.parent.mkdir()
    alice_key.write_bytes(b"dummy")

    with pytest.raises(LocalDevicePackingError):
        load_device_with_password(config_dir, alice.organization_id, alice.device_id, "S3Cr37")


def test_password_save_already_existing(config_dir, alice):
    save_device_with_password(config_dir, alice, "S3Cr37")

    with pytest.raises(LocalDeviceAlreadyExistsError):
        save_device_with_password(config_dir, alice, "S3Cr37")


def test_password_load_not_found(config_dir, alice):
    with pytest.raises(LocalDeviceNotFoundError):
        load_device_with_password(config_dir, alice.organization_id, alice.device_id, "S3Cr37")


def test_same_device_id_different_orginazations(config_dir, alice, otheralice):
    devices = (alice, otheralice)

    for device in devices:
        save_device_with_password(config_dir, device, f"S3Cr37-{device.organization_id}")

    for device in devices:
        device_reloaded = load_device_with_password(
            config_dir, device.organization_id, device.device_id, f"S3Cr37-{device.organization_id}"
        )
        assert device == device_reloaded


@pytest.fixture
def mocked_pkcs11():
    PIN = "123456"
    TOKEN_ID = 1
    KEY_ID = 2

    def encrypt_data_mock(token_id, key_id, input_data):
        if token_id != TOKEN_ID or key_id != KEY_ID:
            raise NoKeysFound()
        return b"ENC:" + input_data

    def decrypt_data_mock(pin, token_id, key_id, input_data):
        if token_id != TOKEN_ID or key_id != KEY_ID:
            raise NoKeysFound()
        if pin != PIN:
            raise DevicePKCS11Error()
        return input_data[4:]

    with patch("parsec.core.local_device.pkcs11_cipher.encrypt_data", new=encrypt_data_mock), patch(
        "parsec.core.local_device.pkcs11_cipher.decrypt_data", new=decrypt_data_mock
    ), patch("parsec.core.local_device.pkcs11_cipher.get_LIB"):
        yield (PIN, TOKEN_ID, KEY_ID)


def test_pkcs11_save_and_load(mocked_pkcs11, config_dir, alice, bob):
    pin, token_id, key_id = mocked_pkcs11

    save_device_with_pkcs11(config_dir, alice, token_id, key_id)

    alice_reloaded = load_device_with_pkcs11(
        config_dir, alice.organization_id, alice.device_id, token_id, key_id, pin
    )
    assert alice_reloaded == alice

    with pytest.raises(LocalDeviceCryptoError):
        save_device_with_pkcs11(config_dir, bob, 42, key_id)

    with pytest.raises(LocalDeviceCryptoError):
        save_device_with_pkcs11(config_dir, bob, token_id, 42)

    with pytest.raises(LocalDeviceCryptoError):
        load_device_with_pkcs11(
            config_dir, alice.organization_id, alice.device_id, token_id, key_id, "foo"
        )

    with pytest.raises(LocalDeviceCryptoError):
        load_device_with_pkcs11(config_dir, alice.organization_id, alice.device_id, 42, key_id, pin)

    with pytest.raises(LocalDeviceCryptoError):
        load_device_with_pkcs11(
            config_dir, alice.organization_id, alice.device_id, token_id, 42, pin
        )
