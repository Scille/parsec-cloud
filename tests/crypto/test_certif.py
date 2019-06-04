# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import now as pendulum_now

from parsec.crypto import (
    CertifiedUserData,
    CertifiedDeviceData,
    CertifiedRevokedDeviceData,
    CryptoError,
    CryptoWrappedMsgPackingError,
    CryptoSignatureAuthorMismatchError,
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


def test_unsecure_read_device_certificate_bad_data():
    with pytest.raises(CryptoWrappedMsgPackingError):
        unsecure_read_device_certificate(b"dummy")


def test_unsecure_read_revoked_device_certificate_bad_data():
    with pytest.raises(CryptoWrappedMsgPackingError):
        unsecure_read_revoked_device_certificate(b"dummy")


def test_unsecure_read_user_certificate_bad_data():
    with pytest.raises(CryptoWrappedMsgPackingError):
        unsecure_read_user_certificate(b"dummy")


def test_build_user_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_user_certificate(
        alice.device_id, alice.signing_key, bob.user_id, bob.public_key, False, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_user_certificate(certif)
    assert isinstance(unsecure, CertifiedUserData)
    assert unsecure.user_id == bob.user_id
    assert unsecure.public_key == bob.public_key
    assert unsecure.certified_on == now
    assert unsecure.certified_by == alice.device_id
    assert unsecure.is_admin is False

    verified = verify_user_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoSignatureAuthorMismatchError):
        verify_user_certificate(certif, mallory.device_id, alice.verify_key)

    with pytest.raises(CryptoError):
        verify_user_certificate(certif, alice.device_id, mallory.verify_key)


def test_build_device_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, bob.verify_key, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_device_certificate(certif)
    assert isinstance(unsecure, CertifiedDeviceData)
    assert unsecure.device_id == bob.device_id
    assert unsecure.verify_key == bob.verify_key
    assert unsecure.certified_on == now
    assert unsecure.certified_by == alice.device_id

    verified = verify_device_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoSignatureAuthorMismatchError):
        verify_device_certificate(certif, mallory.device_id, alice.verify_key)

    with pytest.raises(CryptoError):
        verify_device_certificate(certif, alice.device_id, mallory.verify_key)


def test_build_revoked_device_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, now
    )
    assert isinstance(certif, bytes)

    unsecure = unsecure_read_revoked_device_certificate(certif)
    assert isinstance(unsecure, CertifiedRevokedDeviceData)
    assert unsecure.device_id == bob.device_id
    assert unsecure.certified_on == now
    assert unsecure.certified_by == alice.device_id

    verified = verify_revoked_device_certificate(certif, alice.device_id, alice.verify_key)
    assert verified == unsecure

    with pytest.raises(CryptoSignatureAuthorMismatchError):
        verify_revoked_device_certificate(certif, mallory.device_id, alice.verify_key)

    with pytest.raises(CryptoError):
        verify_revoked_device_certificate(certif, alice.device_id, mallory.verify_key)
