# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import now as pendulum_now

from parsec.api.data import (
    DataError,
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedUserCertificateContent,
)


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


def test_unsecure_read_device_certificate_bad_data():
    with pytest.raises(DataError):
        DeviceCertificateContent.unsecure_load(b"dummy")


def test_unsecure_read_revoked_user_certificate_bad_data():
    with pytest.raises(DataError):
        RevokedUserCertificateContent.unsecure_load(b"dummy")


def test_unsecure_read_user_certificate_bad_data():
    with pytest.raises(DataError):
        UserCertificateContent.unsecure_load(b"dummy")


def test_build_user_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=bob.user_id,
        public_key=bob.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = UserCertificateContent.unsecure_load(certif)
    assert isinstance(unsecure, UserCertificateContent)
    assert unsecure.user_id == bob.user_id
    assert unsecure.public_key == bob.public_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id
    assert unsecure.is_admin is False

    verified = UserCertificateContent.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        UserCertificateContent.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        UserCertificateContent.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        UserCertificateContent.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_user=mallory.user_id,
        )
    assert str(exc.value) == "Invalid user ID: expected `mallory`, got `bob`"


def test_build_device_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = DeviceCertificateContent(
        author=alice.device_id, timestamp=now, device_id=bob.device_id, verify_key=bob.verify_key
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = DeviceCertificateContent.unsecure_load(certif)
    assert isinstance(unsecure, DeviceCertificateContent)
    assert unsecure.device_id == bob.device_id
    assert unsecure.verify_key == bob.verify_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = DeviceCertificateContent.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        DeviceCertificateContent.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        DeviceCertificateContent.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        DeviceCertificateContent.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_device=mallory.device_id,
        )
    assert str(exc.value) == "Invalid device ID: expected `mallory@dev1`, got `bob@dev1`"


def test_build_revoked_user_certificate(alice, bob, mallory):
    now = pendulum_now()
    certif = RevokedUserCertificateContent(
        author=alice.device_id, timestamp=now, user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = RevokedUserCertificateContent.unsecure_load(certif)
    assert isinstance(unsecure, RevokedUserCertificateContent)
    assert unsecure.user_id == bob.user_id
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = RevokedUserCertificateContent.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        RevokedUserCertificateContent.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        RevokedUserCertificateContent.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        RevokedUserCertificateContent.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_user=mallory.user_id,
        )
    assert str(exc.value) == "Invalid user ID: expected `mallory`, got `bob`"
