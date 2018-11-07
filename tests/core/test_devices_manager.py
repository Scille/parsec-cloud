import pytest
from unittest.mock import patch

from parsec.core.devices_manager import (
    LocalDevicesManager,
    DeviceSavingAlreadyExists,
    DeviceLoadingError,
)
from parsec.core.fs.utils import new_access
from parsec.pkcs11_encryption_tool import DevicePKCS11Error, NoKeysFound


@pytest.fixture
def fast_crypto():
    # Default crypto is really slow, so hack it just enough to give it a boost
    from nacl.pwhash import argon2i

    with patch("parsec.core.devices_manager.CRYPTO_OPSLIMIT", argon2i.OPSLIMIT_INTERACTIVE), patch(
        "parsec.core.devices_manager.CRYPTO_MEMLIMIT", argon2i.MEMLIMIT_INTERACTIVE
    ):
        yield


@pytest.fixture
def alice_cleartext_device(tmpdir, alice):
    dm = LocalDevicesManager(tmpdir.strpath)
    dm.register_new_device(
        alice.id,
        alice.user_privkey.encode(),
        alice.device_signkey.encode(),
        alice.user_manifest_access,
    )
    return alice.id


@pytest.fixture
def bob_cleartext_device(tmpdir, bob):
    dm = LocalDevicesManager(tmpdir.strpath)
    dm.register_new_device(
        bob.id, bob.user_privkey.encode(), bob.device_signkey.encode(), bob.user_manifest_access
    )
    return bob.id


def test_non_existant_base_path_list_devices(tmpdir):
    dm = LocalDevicesManager(str(tmpdir.join("dummy")))
    devices = dm.list_available_devices()
    assert not devices


def test_list_no_devices(tmpdir):
    dm = LocalDevicesManager(tmpdir.strpath)
    devices = dm.list_available_devices()
    assert not devices


def test_list_devices(tmpdir, alice_cleartext_device, bob_cleartext_device):
    dm = LocalDevicesManager(tmpdir.strpath)
    devices = dm.list_available_devices()
    assert set(devices) == {alice_cleartext_device, bob_cleartext_device}


def test_load_cleartext_device(tmpdir, alice_cleartext_device, alice):
    dm = LocalDevicesManager(tmpdir.strpath)
    device = dm.load_device(alice_cleartext_device)
    assert device.id == alice_cleartext_device
    assert device.user_privkey == alice.user_privkey
    assert device.device_signkey == alice.device_signkey
    assert device.user_manifest_access == alice.user_manifest_access
    assert device.local_db.path == tmpdir.join(alice.id, "local_storage").strpath


def test_register_new_cleartext_device(tmpdir, alice):
    device_id = alice.id
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey
    user_manifest_access = alice.user_manifest_access

    dm1 = LocalDevicesManager(tmpdir.strpath)
    dm1.register_new_device(
        device_id, user_privkey.encode(), device_signkey.encode(), user_manifest_access
    )

    dm2 = LocalDevicesManager(tmpdir.strpath)
    device = dm2.load_device(device_id)

    assert device.id == device_id
    assert device.user_privkey == user_privkey
    assert device.device_signkey == device_signkey
    assert device.user_manifest_access == user_manifest_access
    assert device.local_db.path == tmpdir.join(alice.id, "local_storage").strpath


def test_register_already_exists_device(tmpdir, alice_cleartext_device, alice):
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey
    user_manifest_access = new_access()

    dm = LocalDevicesManager(tmpdir.strpath)
    with pytest.raises(DeviceSavingAlreadyExists):
        dm.register_new_device(
            alice_cleartext_device,
            user_privkey.encode(),
            device_signkey.encode(),
            user_manifest_access,
        )


def test_register_new_encrypted_device(tmpdir, fast_crypto, alice):
    password = "S3Cr37"

    device_id = alice.id
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey
    user_manifest_access = new_access()

    dm1 = LocalDevicesManager(tmpdir.strpath)
    dm1.register_new_device(
        device_id,
        user_privkey.encode(),
        device_signkey.encode(),
        user_manifest_access,
        password=password,
    )

    dm2 = LocalDevicesManager(tmpdir.strpath)
    device = dm2.load_device(device_id, password=password)

    assert device.id == device_id
    assert device.user_privkey == user_privkey
    assert device.device_signkey == device_signkey
    assert device.user_manifest_access == user_manifest_access
    assert device.local_db.path == tmpdir.join(alice.id, "local_storage").strpath

    dm3 = LocalDevicesManager(tmpdir.strpath)
    with pytest.raises(DeviceLoadingError):
        dm3.load_device(device_id, password="bad pwd")


@patch("parsec.pkcs11_encryption_tool.encrypt_data")
@patch("parsec.pkcs11_encryption_tool.decrypt_data")
def test_register_new_pkcs11_device(decrypt_data, encrypt_data, tmpdir, fast_crypto, alice):
    def encrypt_data_mock(token_id, key_id, input_data):
        if token_id != 1 or key_id != 2:
            raise NoKeysFound()
        return b"ENC:" + input_data

    def decrypt_data_mock(pin, token_id, key_id, input_data):
        if token_id != 1 or key_id != 2:
            raise NoKeysFound()
        if pin != "123456":
            raise DevicePKCS11Error()
        return input_data[4:]

    encrypt_data.side_effect = encrypt_data_mock
    decrypt_data.side_effect = decrypt_data_mock

    device_id = alice.id
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey
    user_manifest_access = new_access()

    dm1 = LocalDevicesManager(tmpdir.strpath)
    dm1.register_new_device(
        device_id,
        user_privkey.encode(),
        device_signkey.encode(),
        user_manifest_access,
        use_pkcs11=True,
        pkcs11_token_id=1,
        pkcs11_key_id=2,
    )

    dm2 = LocalDevicesManager(tmpdir.strpath)
    device = dm2.load_device(device_id, pkcs11_pin="123456", pkcs11_token_id=1, pkcs11_key_id=2)

    assert device.id == device_id
    assert device.user_privkey == user_privkey
    assert device.device_signkey == device_signkey
    assert device.user_manifest_access == user_manifest_access
    assert device.local_db.path == tmpdir.join(alice.id, "local_storage").strpath

    dm3 = LocalDevicesManager(tmpdir.strpath)
    with pytest.raises(DevicePKCS11Error):
        dm3.load_device(device_id, pkcs11_pin="bad pin", pkcs11_token_id=1, pkcs11_key_id=2)

    dm3 = LocalDevicesManager(tmpdir.strpath)
    with pytest.raises(DevicePKCS11Error):
        dm3.load_device(device_id, pkcs11_pin="123456", pkcs11_token_id=9, pkcs11_key_id=2)

    dm3 = LocalDevicesManager(tmpdir.strpath)
    with pytest.raises(DevicePKCS11Error):
        dm3.load_device(device_id, pkcs11_pin="123456", pkcs11_token_id=1, pkcs11_key_id=9)
