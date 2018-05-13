from parsec.utils import generate_sym_key
from parsec.core.encryption_manager import (
    RemoteDevice,
    RemoteUser,
    encrypt_for_self,
    encrypt_for,
    decrypt_for,
    verify_signature_from,
    encrypt_with_secret_key,
    decrypt_with_secret_key,
)


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
