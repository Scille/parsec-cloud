import pytest
from parsec.utils import generate_sym_key
from parsec.core.backend_connection import (
    backend_connection_factory,
    backend_send_anonymous_cmd,
    BackendNotAvailable,
    HandshakeError,
)
from parsec.core.encryption_manager import (
    encrypt_for_self,
    encrypt_for,
    decrypt_for,
    verify_signature_from,
    encrypt_with_secret_key,
    decrypt_with_secret_key,
    EncryptionManager,
)
from parsec.core.local_storage import LocalStorage
from parsec.utils import to_jsonb64


@pytest.fixture
def mock_bcm_get_user_side_effect_factory():

    def factory(user):
        return [
            {
                "status": "ok",
                "user_id": "bob",
                "broadcast_key": to_jsonb64(user.user_pubkey.encode()),
                "created_by": "<backend-fixture>",
                "created_on": "2000-01-01T00:00:00+00:00",
                "devices": {
                    "test": {
                        "created_on": "2000-01-01T00:00:00+00:00",
                        "revocated_on": None,
                        "verify_key": to_jsonb64(user.device_verifykey.encode()),
                    }
                },
            }
        ]

    return factory


@pytest.fixture
def mock_bcm_get_alice_side_effect(alice):
    return [
        {
            "status": "ok",
            "user_id": "alice",
            "broadcast_key": to_jsonb64(alice.user_pubkey.encode()),
            "created_by": "<backend-fixture>",
            "created_on": "2000-01-01T00:00:00+00:00",
            "devices": {
                "test": {
                    "created_on": "2000-01-01T00:00:00+00:00",
                    "revocated_on": None,
                    "verify_key": to_jsonb64(alice.device_verifykey.encode()),
                }
            },
        }
    ]


def test_encrypt_for_self(alice):
    msg = {"foo": "bar"}
    ciphered_msg = encrypt_for_self(alice, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_for(alice, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice, signed_msg)
    assert returned_msg == msg


def test_encrypt_for_other(alice, bob):
    msg = {"foo": "bar"}
    ciphered_msg = encrypt_for(alice, bob, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_for(bob, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice, signed_msg)
    assert returned_msg == msg


def test_encrypt_with_secret_key(alice):
    msg = {"foo": "bar"}
    key = generate_sym_key()
    ciphered_msg = encrypt_with_secret_key(alice, key, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_with_secret_key(key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice, signed_msg)
    assert returned_msg == msg


# TODO: test corrupted/forged messages


@pytest.mark.trio
async def test_cache_encrypt(mock_bcm, local_storage, alice, mock_bcm_get_user_side_effect_factory):
    encryption_manager = EncryptionManager(alice, mock_bcm, local_storage)
    # Offline
    mock_bcm.send.side_effect = [BackendNotAvailable]
    with pytest.raises(BackendNotAvailable):
        await encryption_manager.encrypt("bob", {"msg": "foo"})
    # Online
    mock_bcm.send.side_effect = mock_bcm_get_user_side_effect_factory(alice)
    await encryption_manager.encrypt("bob", {"msg": "foo"})
    # Offline with cached results
    mock_bcm.send.side_effect = [BackendNotAvailable]
    await encryption_manager.encrypt("bob", {"msg": "foo"})


@pytest.mark.trio
async def test_cache_decrypt(
    mock_bcm, local_storage, alice, bob, mock_bcm_get_user_side_effect_factory
):
    # Alice encrypt a message for bob
    encryption_manager = EncryptionManager(alice, mock_bcm, local_storage)
    mock_bcm.send.side_effect = mock_bcm_get_user_side_effect_factory(bob)
    encrypted = await encryption_manager.encrypt("bob", {"msg": "foo"})

    encryption_manager = EncryptionManager(bob, mock_bcm, local_storage)
    # Offline
    mock_bcm.send.side_effect = [BackendNotAvailable]
    with pytest.raises(BackendNotAvailable):
        await encryption_manager.decrypt(encrypted)
    # Online
    mock_bcm.send.side_effect = mock_bcm_get_user_side_effect_factory(alice)
    await encryption_manager.decrypt(encrypted)
    # # Offline with cached results
    mock_bcm.send.side_effect = [BackendNotAvailable]
    await encryption_manager.decrypt(encrypted)


@pytest.mark.trio
async def test_cache_decrypt_with_secret_key(
    mock_bcm, local_storage, alice, bob, mock_bcm_get_user_side_effect_factory
):
    # Alice encrypt a message with a symmetric key
    symmetric_key = b"1" * 32
    encryption_manager = EncryptionManager(alice, mock_bcm, local_storage)
    mock_bcm.send.side_effect = mock_bcm_get_user_side_effect_factory(bob)
    encrypted = await encryption_manager.encrypt_with_secret_key(symmetric_key, {"msg": "foo"})

    encryption_manager = EncryptionManager(bob, mock_bcm, local_storage)
    # Offline
    mock_bcm.send.side_effect = [BackendNotAvailable]
    with pytest.raises(BackendNotAvailable):
        await encryption_manager.decrypt_with_secret_key(symmetric_key, encrypted)
    # Online
    mock_bcm.send.side_effect = mock_bcm_get_user_side_effect_factory(alice)
    await encryption_manager.decrypt_with_secret_key(symmetric_key, encrypted)
    # # Offline with cached results
    mock_bcm.send.side_effect = [BackendNotAvailable]
    await encryption_manager.decrypt_with_secret_key(symmetric_key, encrypted)
