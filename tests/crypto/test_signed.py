# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import now as pendulum_now

from parsec.crypto import (
    CryptoError,
    CryptoSignatureTimestampMismatchError,
    CryptoSignatureAuthorMismatchError,
    generate_secret_key,
    build_signed_msg,
    verify_signed_msg,
    encrypt_signed_msg_for,
    decrypt_signed_msg_for,
    decrypt_and_verify_signed_msg_for,
    encrypt_signed_msg_with_secret_key,
    decrypt_signed_msg_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
)


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


def test_encrypt_signed_msg_for(alice, bob):
    msg = b"Hello world !"
    now = pendulum_now()

    ciphered_msg = encrypt_signed_msg_for(
        alice.device_id, alice.signing_key, bob.public_key, msg, now
    )
    assert isinstance(ciphered_msg, bytes)

    expected_author_id, expected_timestamp, signed_msg = decrypt_signed_msg_for(
        bob.private_key, ciphered_msg
    )
    assert expected_author_id == alice.device_id
    assert expected_timestamp == now

    returned_msg = verify_signed_msg(signed_msg, alice.device_id, alice.verify_key, now)
    assert returned_msg == msg

    returned_msg = decrypt_and_verify_signed_msg_for(
        bob.private_key, ciphered_msg, alice.device_id, alice.verify_key, now
    )
    assert returned_msg == msg


def test_encrypt_signed_msg_with_secret_key(alice):
    msg = b"Hello world !"
    now = pendulum_now()
    key = generate_secret_key()

    ciphered_msg = encrypt_signed_msg_with_secret_key(
        alice.device_id, alice.signing_key, key, msg, now
    )
    assert isinstance(ciphered_msg, bytes)

    expected_author_id, expected_timestamp, signed_msg = decrypt_signed_msg_with_secret_key(
        key, ciphered_msg
    )
    assert expected_author_id == alice.device_id
    assert expected_timestamp == now

    returned_msg = verify_signed_msg(signed_msg, alice.device_id, alice.verify_key, now)
    assert returned_msg == msg

    returned_msg = decrypt_and_verify_signed_msg_with_secret_key(
        key, ciphered_msg, alice.device_id, alice.verify_key, now
    )
    assert returned_msg == msg


def test_signed_decrypt_bad_secret_key(alice, bob):
    msg = b"Hello world !"
    now = pendulum_now()
    key = generate_secret_key()
    bad_key = generate_secret_key()

    ciphered_msg = encrypt_signed_msg_with_secret_key(
        alice.device_id, alice.signing_key, key, msg, now
    )

    with pytest.raises(CryptoError):
        decrypt_signed_msg_with_secret_key(bad_key, ciphered_msg)

    with pytest.raises(CryptoError):
        decrypt_and_verify_signed_msg_with_secret_key(
            bad_key, ciphered_msg, alice.device_id, alice.verify_key, now
        )


def test_signed_decrypt_bad_receiver(alice, bob, mallory):
    msg = b"Hello world !"
    now = pendulum_now()

    ciphered_msg = encrypt_signed_msg_for(
        alice.device_id, alice.signing_key, bob.public_key, msg, now
    )
    assert isinstance(ciphered_msg, bytes)

    with pytest.raises(CryptoError):
        decrypt_signed_msg_for(mallory.private_key, ciphered_msg)

    with pytest.raises(CryptoError):
        decrypt_and_verify_signed_msg_for(
            mallory.private_key, ciphered_msg, alice.device_id, alice.verify_key, now
        )


def test_signed_decrypt_bad_timestamp(alice, bob, mallory):
    msg = b"Hello world !"
    now = pendulum_now()
    signed_msg = build_signed_msg(alice.device_id, alice.signing_key, msg, now)
    assert isinstance(signed_msg, bytes)

    # Bad device_id
    with pytest.raises(CryptoSignatureAuthorMismatchError):
        verify_signed_msg(signed_msg, bob.device_id, alice.verify_key, now)

    # Bad verify_key
    with pytest.raises(CryptoError):
        verify_signed_msg(signed_msg, alice.device_id, bob.verify_key, now)

    # Bad timestamp
    with pytest.raises(CryptoSignatureTimestampMismatchError):
        verify_signed_msg(signed_msg, alice.device_id, alice.verify_key, now.add(1))
