import pytest

from parsec.utils import generate_sym_key
from parsec.core.encryption_manager import (
    encrypt_for_self,
    encrypt_for,
    decrypt_for,
    verify_signature_from,
    encrypt_with_secret_key,
    decrypt_with_secret_key,
)
from parsec.core.backend_connection import BackendNotAvailable

from tests.open_tcp_stream_mock_wrapper import offline
from tests.conftest import realcrypto


@pytest.fixture(scope="module", autouse=True)
def force_realcrypto():
    yield from realcrypto()


def test_encrypt_for_self(alice):
    msg = b"Hello world !"
    ciphered_msg = encrypt_for_self(alice.id, alice.device_signkey, alice.user_pubkey, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_for(alice.user_privkey, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice.device_verifykey, signed_msg)
    assert returned_msg == msg


def test_encrypt_for_other(alice, bob):
    msg = b"Hello world !"
    ciphered_msg = encrypt_for(alice.id, alice.device_signkey, bob.user_pubkey, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_for(bob.user_privkey, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice.device_verifykey, signed_msg)
    assert returned_msg == msg


def test_encrypt_with_secret_key(alice):
    msg = b"Hello world !"
    key = generate_sym_key()
    ciphered_msg = encrypt_with_secret_key(alice.id, alice.device_signkey, key, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_with_secret_key(key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice.device_verifykey, signed_msg)
    assert returned_msg == msg


# TODO: test corrupted/forged messages


@pytest.mark.trio
async def test_encryption_manager_fetch_remote_device_local_cache(
    tcp_stream_spy, running_backend, backend_addr, encryption_manager, bob
):
    with pytest.raises(BackendNotAvailable):
        with offline(backend_addr):
            await encryption_manager.fetch_remote_device(bob.user_id, bob.device_name)

    remote_bob = await encryption_manager.fetch_remote_device(bob.user_id, bob.device_name)
    assert remote_bob.device_verifykey == bob.device_verifykey

    with offline(backend_addr):
        remot_bob_offline = await encryption_manager.fetch_remote_device(
            bob.user_id, bob.device_name
        )
    assert remot_bob_offline == remote_bob


@pytest.mark.trio
async def test_encryption_manager_fetch_remote_user_local_cache(
    tcp_stream_spy, running_backend, backend_addr, encryption_manager, bob
):
    with pytest.raises(BackendNotAvailable):
        with offline(backend_addr):
            await encryption_manager.fetch_remote_user(bob.user_id)

    remote_bob = await encryption_manager.fetch_remote_user(bob.user_id)
    assert remote_bob.user_pubkey == bob.user_pubkey

    with offline(backend_addr):
        remot_bob_offline = await encryption_manager.fetch_remote_user(bob.user_id)
    assert remot_bob_offline == remote_bob


@pytest.mark.trio
async def test_encryption_manager_fetch_self_device_offline(encryption_manager, alice):
    remote_device = await encryption_manager.fetch_remote_device(alice.user_id, alice.device_name)
    assert remote_device.device_verifykey == alice.device_verifykey


@pytest.mark.trio
async def test_encryption_manager_fetch_self_offline(encryption_manager, alice):
    remote_user = await encryption_manager.fetch_remote_user(alice.user_id)
    assert remote_user.user_pubkey == alice.user_pubkey
