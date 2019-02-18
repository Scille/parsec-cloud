# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.crypto import (
    CryptoError,
    BadSignatureError,
    generate_secret_key,
    encrypt_for_self,
    encrypt_for,
    decrypt_for,
    verify_signature_from,
    encrypt_with_secret_key,
    decrypt_with_secret_key,
)


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


def test_encrypt_for_self(alice):
    msg = b"Hello world !"
    ciphered_msg = encrypt_for_self(alice.device_id, alice.signing_key, alice.public_key, msg)
    assert isinstance(ciphered_msg, bytes)

    device_id, signed_msg = decrypt_for(alice.private_key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert device_id == alice.device_id

    returned_msg = verify_signature_from(alice.verify_key, signed_msg)
    assert returned_msg == msg


def test_encrypt_for_other(alice, bob):
    msg = b"Hello world !"
    ciphered_msg = encrypt_for(alice.device_id, alice.signing_key, bob.public_key, msg)
    assert isinstance(ciphered_msg, bytes)

    device_id, signed_msg = decrypt_for(bob.private_key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert device_id == alice.device_id

    returned_msg = verify_signature_from(alice.verify_key, signed_msg)
    assert returned_msg == msg


def test_encrypt_with_secret_key(alice):
    msg = b"Hello world !"
    key = generate_secret_key()
    ciphered_msg = encrypt_with_secret_key(alice.device_id, alice.signing_key, key, msg)
    assert isinstance(ciphered_msg, bytes)

    device_id, signed_msg = decrypt_with_secret_key(key, ciphered_msg)
    assert isinstance(signed_msg, bytes)
    assert device_id == alice.device_id

    returned_msg = verify_signature_from(alice.verify_key, signed_msg)
    assert returned_msg == msg


def test_decrypt_bad_secret_key(alice, bob):
    msg = b"Hello world !"
    key = generate_secret_key()
    bad_key = generate_secret_key()

    ciphered_msg = encrypt_with_secret_key(alice.device_id, alice.signing_key, key, msg)

    with pytest.raises(CryptoError):
        decrypt_with_secret_key(bad_key, ciphered_msg)


def test_decrypt_for_other_bad_receiver(alice, bob, mallory):
    msg = b"Hello world !"
    ciphered_msg = encrypt_for(alice.device_id, alice.signing_key, bob.public_key, msg)

    with pytest.raises(CryptoError):
        decrypt_for(mallory.private_key, ciphered_msg)

    device_id, signed_msg = decrypt_for(bob.private_key, ciphered_msg)
    with pytest.raises(BadSignatureError):
        verify_signature_from(mallory.verify_key, signed_msg)
