import pytest

from parsec.crypto import (
    generate_secret_key,
    encrypt_for_self,
    encrypt_for,
    decrypt_for,
    verify_signature_from,
    encrypt_with_secret_key,
    decrypt_with_secret_key,
)


# TODO: test corrupted/forged messages


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


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
    key = generate_secret_key()
    ciphered_msg = encrypt_with_secret_key(alice.id, alice.device_signkey, key, msg)
    assert isinstance(ciphered_msg, bytes)

    user_id, device_name, signed_msg = decrypt_with_secret_key(key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert user_id == alice.user_id
    assert device_name == alice.device_name

    returned_msg = verify_signature_from(alice.device_verifykey, signed_msg)
    assert returned_msg == msg
