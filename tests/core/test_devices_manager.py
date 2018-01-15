import pytest
from unittest.mock import patch
import json

from parsec.core.devices_manager import (
    DevicesManager, DeviceSavingError, DeviceLoadingError)
from parsec.utils import to_jsonb64


def cleartext_device(basedir, device_id, user_privkey, device_signkey):
    conf_path = basedir.mkdir(device_id)
    conf = {
        'device_id': device_id,
        'encryption': 'quedalle',
        'user_privkey': to_jsonb64(user_privkey),
        'device_signkey': to_jsonb64(device_signkey),
    }
    with open(conf_path.join('key.json'), 'w') as fd:
        json.dump(conf, fd)
    return device_id


@pytest.fixture
def fast_crypto():
    # Default crypto is really slow, so hack it just enough to give it a boost
    from nacl.pwhash import argon2i
    with patch('parsec.core.devices_manager.CRYPTO_OPSLIMIT', argon2i.OPSLIMIT_INTERACTIVE), \
            patch('parsec.core.devices_manager.CRYPTO_MEMLIMIT', argon2i.MEMLIMIT_INTERACTIVE):
        yield


@pytest.fixture
def alice_cleartext_device(tmpdir, alice):
    return cleartext_device(
        tmpdir,
        alice.id,
        alice.user_privkey.encode(),
        alice.device_signkey.encode()
    )


@pytest.fixture
def bob_cleartext_device(tmpdir, bob):
    return cleartext_device(
        tmpdir,
        bob.id,
        bob.user_privkey.encode(),
        bob.device_signkey.encode()
    )


def test_non_existant_base_path_list_devices(tmpdir):
    dm = DevicesManager(str(tmpdir.join('dummy')))
    devices = dm.list_available_devices()
    assert not devices


def test_list_no_devices(tmpdir):
    dm = DevicesManager(str(tmpdir))
    devices = dm.list_available_devices()
    assert not devices


def test_list_devices(tmpdir, alice_cleartext_device, bob_cleartext_device):
    dm = DevicesManager(str(tmpdir))
    devices = dm.list_available_devices()
    assert set(devices) == {
        alice_cleartext_device,
        bob_cleartext_device
    }


def test_load_cleartext_device(tmpdir, alice_cleartext_device, alice):
    dm = DevicesManager(str(tmpdir))
    device = dm.load_device(alice_cleartext_device)
    assert device.id == alice_cleartext_device
    assert alice.user_privkey == device.user_privkey
    assert alice.device_signkey == device.device_signkey
    assert device.local_storage_db_path == tmpdir.join('alice@test', 'local_storage.sqlite')


def test_register_new_cleartext_device(tmpdir, alice):
    device_id = alice.id
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey

    dm1 = DevicesManager(str(tmpdir))
    dm1.register_new_device(
        device_id,
        user_privkey.encode(),
        device_signkey.encode()
    )

    dm2 = DevicesManager(str(tmpdir))
    device = dm2.load_device(device_id)

    assert device.id == device_id
    assert device.user_privkey == user_privkey
    assert device.device_signkey == device_signkey
    assert device.local_storage_db_path == tmpdir.join('alice@test', 'local_storage.sqlite')


def test_register_already_exists_device(tmpdir, alice_cleartext_device, alice):
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey

    dm = DevicesManager(str(tmpdir))
    with pytest.raises(DeviceSavingError):
        dm.register_new_device(
            alice_cleartext_device,
            user_privkey.encode(),
            device_signkey.encode()
        )


def test_register_new_encrypted_device(tmpdir, fast_crypto, alice):
    password = 'S3Cr37'

    device_id = alice.id
    device_signkey = alice.device_signkey
    user_privkey = alice.user_privkey

    dm1 = DevicesManager(str(tmpdir))
    dm1.register_new_device(
        device_id,
        user_privkey.encode(),
        device_signkey.encode(),
        password=password
    )

    dm2 = DevicesManager(str(tmpdir))
    device = dm2.load_device(device_id, password=password)

    assert device.id == device_id
    assert device.user_privkey == user_privkey
    assert device.device_signkey == device_signkey
    assert device.local_storage_db_path == tmpdir.join('alice@test', 'local_storage.sqlite')

    dm2 = DevicesManager(str(tmpdir))
    with pytest.raises(DeviceLoadingError):
        dm2.load_device(device_id, password='bad pwd')
