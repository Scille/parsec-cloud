# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from parsec._parsec import DateTime
import zlib

from parsec.serde import packb, unpackb
from parsec.api.data import (
    DataError,
    UserCertificate,
    DeviceCertificate,
    RevokedUserCertificate,
)
from parsec.api.protocol import UserProfile


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


def test_unsecure_read_device_certificate_bad_data():
    with pytest.raises(DataError):
        DeviceCertificate.unsecure_load(b"dummy")


def test_unsecure_read_revoked_user_certificate_bad_data():
    with pytest.raises(DataError):
        RevokedUserCertificate.unsecure_load(b"dummy")


def test_unsecure_read_user_certificate_bad_data():
    with pytest.raises(DataError):
        UserCertificate.unsecure_load(b"dummy")


def test_build_user_certificate(alice, bob, mallory):
    now = DateTime.now()
    certif = UserCertificate(
        author=alice.device_id,
        timestamp=now,
        user_id=bob.user_id,
        human_handle=bob.human_handle,
        public_key=bob.public_key,
        profile=UserProfile.ADMIN,
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = UserCertificate.unsecure_load(certif)
    assert isinstance(unsecure, UserCertificate)
    assert unsecure.user_id == bob.user_id
    assert unsecure.public_key == bob.public_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id
    assert unsecure.profile == UserProfile.ADMIN

    verified = UserCertificate.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        UserCertificate.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        UserCertificate.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        UserCertificate.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_user=mallory.user_id,
        )
    assert str(exc.value) == "Invalid user ID: expected `mallory`, got `bob`"


def test_user_certificate_supports_legacy_is_admin_field(alice, bob):
    now = DateTime.now()
    certif = UserCertificate(
        author=bob.device_id,
        timestamp=now,
        user_id=alice.user_id,
        human_handle=None,
        public_key=alice.public_key,
        profile=alice.profile,
    )

    # Manually craft a certificate in legacy format
    raw_legacy_certif = {
        "type": "user_certificate",
        "author": bob.device_id.str,
        "timestamp": now,
        "user_id": alice.user_id.str,
        "public_key": alice.public_key.encode(),
        "is_admin": True,
    }
    dumped_legacy_certif = bob.signing_key.sign(zlib.compress(packb(raw_legacy_certif)))

    # Make sure the legacy format can be loaded
    legacy_certif = UserCertificate.verify_and_load(
        dumped_legacy_certif,
        author_verify_key=bob.verify_key,
        expected_author=bob.device_id,
        expected_user=alice.user_id,
        expected_human_handle=None,
    )
    assert legacy_certif == certif

    # Manually decode new format to check it is compatible with legacy
    dumped_certif = certif.dump_and_sign(bob.signing_key)
    raw_certif = unpackb(zlib.decompress(bob.verify_key.verify(dumped_certif)))
    assert raw_certif == {**raw_legacy_certif, "profile": alice.profile.value, "human_handle": None}


def test_build_device_certificate(alice, bob, mallory):
    now = DateTime.now()
    certif = DeviceCertificate(
        author=alice.device_id,
        timestamp=now,
        device_id=bob.device_id,
        device_label=bob.device_label,
        verify_key=bob.verify_key,
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = DeviceCertificate.unsecure_load(certif)
    assert isinstance(unsecure, DeviceCertificate)
    assert unsecure.device_id == bob.device_id
    assert unsecure.verify_key == bob.verify_key
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = DeviceCertificate.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        DeviceCertificate.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        DeviceCertificate.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        DeviceCertificate.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_device=mallory.device_id,
        )
    assert str(exc.value) == "Invalid device ID: expected `mallory@dev1`, got `bob@dev1`"


def test_build_revoked_user_certificate(alice, bob, mallory):
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=alice.device_id, timestamp=now, user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)
    assert isinstance(certif, bytes)

    unsecure = RevokedUserCertificate.unsecure_load(certif)
    assert isinstance(unsecure, RevokedUserCertificate)
    assert unsecure.user_id == bob.user_id
    assert unsecure.timestamp == now
    assert unsecure.author == alice.device_id

    verified = RevokedUserCertificate.verify_and_load(
        certif, author_verify_key=alice.verify_key, expected_author=alice.device_id
    )
    assert verified == unsecure

    with pytest.raises(DataError) as exc:
        RevokedUserCertificate.verify_and_load(
            certif, author_verify_key=alice.verify_key, expected_author=mallory.device_id
        )
    assert str(exc.value) == "Invalid author: expected `mallory@dev1`, got `alice@dev1`"

    with pytest.raises(DataError) as exc:
        RevokedUserCertificate.verify_and_load(
            certif, author_verify_key=mallory.verify_key, expected_author=alice.device_id
        )
    assert str(exc.value) == "Signature was forged or corrupt"

    with pytest.raises(DataError) as exc:
        RevokedUserCertificate.verify_and_load(
            certif,
            author_verify_key=alice.verify_key,
            expected_author=alice.device_id,
            expected_user=mallory.user_id,
        )
    assert str(exc.value) == "Invalid user ID: expected `mallory`, got `bob`"
