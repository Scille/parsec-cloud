import pytest
from pathlib import Path
from unittest.mock import patch

from parsec.core.devices_manager.pkcs11_tools import NoKeysFound, DevicePKCS11Error
from parsec.types import DeviceID
from parsec.core.devices_manager import (
    generate_new_device,
    LocalDevicesManager,
    DeviceManagerError,
    DeviceConfigAleadyExists,
    DeviceConfigNotFound,
    PasswordDeviceEncryptor,
    PasswordDeviceDecryptor,
    PKCS11DeviceDecryptor,
    PKCS11DeviceEncryptor,
)


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        pass


@pytest.fixture
def alice(backend_addr, root_key_certifier):
    return generate_new_device(DeviceID("alice@pc1"), backend_addr, root_key_certifier.verify_key)


@pytest.fixture
def bob(backend_addr, root_key_certifier):
    return generate_new_device(DeviceID("bob@pc2"), backend_addr, root_key_certifier.verify_key)


@pytest.fixture
def local_device_manager(tmpdir):
    return LocalDevicesManager(Path(tmpdir))


@pytest.mark.parametrize("path_exists", (True, False))
def test_list_no_devices(path_exists, tmpdir, local_device_manager):
    path = Path(tmpdir if path_exists else tmpdir / "dummy")
    local_device_manager = LocalDevicesManager(path)
    devices = local_device_manager.list_available_devices()
    assert not devices


def test_list_devices(local_device_manager, alice, bob):
    encryptor = PasswordDeviceEncryptor("S3Cr37")
    local_device_manager.save_device(alice, encryptor)
    local_device_manager.save_device(bob, encryptor)

    devices = local_device_manager.list_available_devices()
    assert set(devices) == {alice.device_id, bob.device_id}


@pytest.mark.parametrize("path_exists", (True, False))
def test_password_save_and_load(path_exists, tmpdir, alice):
    path = Path(tmpdir if path_exists else tmpdir / "dummy")
    local_device_manager = LocalDevicesManager(path)
    encryptor = PasswordDeviceEncryptor("S3Cr37")
    local_device_manager.save_device(alice, encryptor)

    decryptor = PasswordDeviceDecryptor("S3Cr37")
    alice_reloaded = local_device_manager.load_device(alice.device_id, decryptor)
    assert alice == alice_reloaded


def test_load_bad_password(local_device_manager, alice):
    encryptor = PasswordDeviceEncryptor("S3Cr37")
    local_device_manager.save_device(alice, encryptor)

    decryptor = PasswordDeviceDecryptor("dummy")
    with pytest.raises(DeviceManagerError):
        local_device_manager.load_device(alice.device_id, decryptor)


def test_password_save_already_existing(local_device_manager, alice):
    encryptor = PasswordDeviceEncryptor("S3Cr37")
    local_device_manager.save_device(alice, encryptor)
    with pytest.raises(DeviceConfigAleadyExists):
        local_device_manager.save_device(alice, encryptor)


def test_password_load_not_found(local_device_manager):
    decryptor = PasswordDeviceDecryptor("S3Cr37")
    with pytest.raises(DeviceConfigNotFound):
        local_device_manager.load_device(DeviceID("waldo@pc1"), decryptor)


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

    with patch(
        "parsec.core.devices_manager.pkcs11_cipher.encrypt_data", new=encrypt_data_mock
    ), patch(
        "parsec.core.devices_manager.pkcs11_cipher.decrypt_data", new=decrypt_data_mock
    ), patch(
        "parsec.core.devices_manager.pkcs11_cipher.get_LIB"
    ):
        yield (PIN, TOKEN_ID, KEY_ID)


def test_pkcs11_save_and_load(mocked_pkcs11, local_device_manager, alice):
    pin, token_id, key_id = mocked_pkcs11
    encryptor = PKCS11DeviceEncryptor(token_id, key_id)
    decryptor = PKCS11DeviceDecryptor(token_id, key_id, pin)

    local_device_manager.save_device(alice, encryptor)

    alice_reloaded = local_device_manager.load_device(alice.device_id, decryptor)

    assert alice_reloaded == alice

    with pytest.raises(DeviceManagerError):
        bad_encryptor = PKCS11DeviceEncryptor(42, key_id)
        local_device_manager.save_device(alice, bad_encryptor)

    with pytest.raises(DeviceManagerError):
        bad_encryptor = PKCS11DeviceEncryptor(token_id, 42)
        local_device_manager.save_device(alice, bad_encryptor)

    with pytest.raises(DeviceManagerError):
        bad_decryptor = PKCS11DeviceDecryptor(token_id, key_id, "foo")
        local_device_manager.load_device(alice.device_id, bad_decryptor)

    with pytest.raises(DeviceManagerError):
        bad_decryptor = PKCS11DeviceDecryptor(42, key_id, pin)
        local_device_manager.load_device(alice.device_id, bad_decryptor)

    with pytest.raises(DeviceManagerError):
        bad_decryptor = PKCS11DeviceDecryptor(token_id, 42, pin)
        local_device_manager.load_device(alice.device_id, bad_decryptor)
