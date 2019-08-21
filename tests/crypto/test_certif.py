# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import now as pendulum_now

from parsec.api.data import (
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedDeviceCertificateContent,
)
from parsec.crypto import (
    CryptoError,
    verify_device_certificate,
    verify_revoked_device_certificate,
    verify_user_certificate,
    unsecure_read_device_certificate,
    unsecure_read_revoked_device_certificate,
    unsecure_read_user_certificate,
    build_device_certificate,
    build_revoked_device_certificate,
    build_user_certificate,
)


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


# TODO: rework those exceptions


def test_unsecure_read_device_certificate_bad_data():
    with pytest.raises(CryptoError):
        unsecure_read_device_certificate(b"dummy")


def test_unsecure_read_revoked_device_certificate_bad_data():
    with pytest.raises(CryptoError):
        unsecure_read_revoked_device_certificate(b"dummy")


def test_unsecure_read_user_certificate_bad_data():
    with pytest.raises(CryptoError):
        unsecure_read_user_certificate(b"dummy")


def test_build_user_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_user_certificate(
        alice.device_id, alice.signing_key, bob.user_id, bob.public_key, False, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_user_certificate(certif)
    assert isinstance(unsecure, UserCertificateContent)
    assert unsecure.user_id == bob.user_id
    assert unsecure.public_key == bob.public_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id
    assert unsecure.is_admin is False

    verified = verify_user_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoError) as exc:
        verify_user_certificate(certif, mallory.device_id, alice.verify_key)
    assert str(exc.value) == "Invalid author: expect `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(CryptoError) as exc:
        verify_user_certificate(certif, alice.device_id, mallory.verify_key)
    assert str(exc.value) == "Signature was forged or corrupt"


def test_build_device_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, bob.verify_key, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_device_certificate(certif)
    assert isinstance(unsecure, DeviceCertificateContent)
    assert unsecure.device_id == bob.device_id
    assert unsecure.verify_key == bob.verify_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = verify_device_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoError) as exc:
        verify_device_certificate(certif, mallory.device_id, alice.verify_key)
    assert str(exc.value) == "Invalid author: expect `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(CryptoError) as exc:
        verify_device_certificate(certif, alice.device_id, mallory.verify_key)
    assert str(exc.value) == "Signature was forged or corrupt"


def test_build_revoked_device_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_revoked_device_certificate(certif)
    assert isinstance(unsecure, RevokedDeviceCertificateContent)
    assert unsecure.device_id == bob.device_id
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = verify_revoked_device_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoError) as exc:
        verify_revoked_device_certificate(certif, mallory.device_id, alice.verify_key)
    assert str(exc.value) == "Invalid author: expect `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(CryptoError) as exc:
        verify_revoked_device_certificate(certif, alice.device_id, mallory.verify_key)
    assert str(exc.value) == "Signature was forged or corrupt"
