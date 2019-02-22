# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types.remote_device import RemoteDevice, RemoteDevicesMapping, RemoteUser
from parsec.trustchain import (
    TrustChainError,
    TrustChainBrokenChainError,
    certified_extract_parts,
    validate_payload_certified_user,
    certify_device,
    certify_device_revocation,
    certify_user,
    validate_payload_certified_device,
    cascade_validate_devices,
    MAX_TS_BALLPARK,
)
from parsec.types import DeviceID

from tests.common import freeze_time


def test_bad_certified_extract_parts():
    with pytest.raises(TrustChainError):
        certified_extract_parts(b"")


def test_certify_user(alice, mallory):
    now = Pendulum(2000, 1, 1)

    with freeze_time(now):
        certification = certify_user(
            alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key
        )
    assert isinstance(certification, bytes)

    certifier, payload = certified_extract_parts(certification)
    assert certifier == alice.device_id
    assert isinstance(payload, bytes)

    data = validate_payload_certified_user(alice.verify_key, payload, now)
    assert data == {
        "type": "user",
        "user_id": mallory.user_id,
        "public_key": mallory.public_key,
        "timestamp": now,
    }


def test_validate_bad_certified_user(alice):
    now = Pendulum(2000, 1, 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_user(alice.verify_key, b"", now)


def test_validate_certified_user_bad_certifier(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_user(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key
    )
    certifier, payload = certified_extract_parts(certification)

    with pytest.raises(TrustChainError):
        validate_payload_certified_user(bob.verify_key, payload, now)


def test_validate_certified_user_too_old(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    with freeze_time(now):
        certification = certify_user(
            alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key
        )

    certifier, payload = certified_extract_parts(certification)
    now = now.add(seconds=MAX_TS_BALLPARK + 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_user(alice.verify_key, payload, now)


def test_certify_device(alice, mallory):
    now = Pendulum(2000, 1, 1)

    with freeze_time(now):
        certification = certify_device(
            alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key
        )
    assert isinstance(certification, bytes)

    certifier, payload = certified_extract_parts(certification)
    assert certifier == alice.device_id
    assert isinstance(payload, bytes)

    data = validate_payload_certified_device(alice.verify_key, payload, now)
    assert data == {
        "type": "device",
        "device_id": mallory.device_id,
        "verify_key": mallory.verify_key,
        "timestamp": now,
    }


def test_validate_bad_certified_device(alice):
    now = Pendulum(2000, 1, 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_device(alice.verify_key, b"", now)


def test_validate_certified_device_bad_certifier(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_device(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.signing_key
    )
    certifier, payload = certified_extract_parts(certification)

    with pytest.raises(TrustChainError):
        validate_payload_certified_device(bob.verify_key, payload, now)


def test_validate_certified_device_too_old(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    with freeze_time(now):
        certification = certify_device(
            alice.device_id, alice.signing_key, mallory.device_id, mallory.signing_key
        )

    certifier, payload = certified_extract_parts(certification)
    now = now.add(seconds=MAX_TS_BALLPARK + 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_device(alice.verify_key, payload, now)


@pytest.mark.trio
async def test_cascade_validate_devices_ok(coolorg, alice, bob, mallory):
    now = Pendulum(2000, 1, 1)

    with freeze_time(now):
        certification_1 = certify_device(
            None, coolorg.root_signing_key, DeviceID("alice@dev1"), alice.verify_key
        )
        certification_2 = certify_device(
            DeviceID("alice@dev1"), alice.signing_key, DeviceID("alice@dev2"), bob.verify_key
        )
        certification_3 = certify_device(
            DeviceID("alice@dev2"), bob.signing_key, DeviceID("alice@dev3"), mallory.verify_key
        )

        user_certification = certify_user(
            certifier_id=None,
            certifier_key=coolorg.root_signing_key,
            user_id=alice.user_id,
            public_key=alice.public_key,
            now=now,
        )

        alice_device = RemoteDevice(
            device_id=DeviceID("alice@dev1"),
            certified_device=certification_1,
            device_certifier=None,
            created_on=now,
        )

        bob_device = RemoteDevice(
            device_id=DeviceID("alice@dev2"),
            certified_device=certification_2,
            device_certifier=DeviceID("alice@dev1"),
            created_on=now,
        )

        mallory_device = RemoteDevice(
            device_id=DeviceID("alice@dev3"),
            certified_device=certification_3,
            device_certifier=DeviceID("alice@dev2"),
            created_on=now,
        )

        alice_user = RemoteUser(
            user_id="alice",
            certified_user=user_certification,
            user_certifier=None,
            devices=RemoteDevicesMapping(alice_device, bob_device, mallory_device),
            created_on=now,
        )

        trustchain = {DeviceID("alice@dev1"): alice_device, DeviceID("alice@dev2"): bob_device}

        result = cascade_validate_devices(
            alice_user, trustchain, coolorg.organization_id, coolorg.root_verify_key
        )
        assert len(result) == 3
        for i in range(3):
            assert result[i].device_id == f"alice@dev{ i + 1}"


# @pytest.mark.trio
# async def test_cascade_validate_devices_broken_chain(coolorg, alice, bob, mallory):
#     now = Pendulum(2000, 1, 1)

#     with freeze_time(now):
#         certification_1 = certify_device(
#             None, coolorg.root_signing_key, DeviceID("alice@dev1"), alice.verify_key
#         )
#         certification_2 = certify_device(
#             DeviceID("alice@dev2"), bob.signing_key, DeviceID("alice@dev2"), bob.verify_key
#         )
#         certification_3 = certify_device(
#             DeviceID("alice@dev2"), bob.signing_key, DeviceID("alice@dev3"), mallory.verify_key
#         )

#         user_certification = certify_user(
#             certifier_id=None,
#             certifier_key=coolorg.root_signing_key,
#             user_id=alice.user_id,
#             public_key=alice.public_key,
#             now=now,
#         )

#         alice_device = RemoteDevice(
#             device_id=DeviceID("alice@dev1"),
#             certified_device=certification_1,
#             device_certifier=None,
#             created_on=now,
#         )

#         bob_device = RemoteDevice(
#             device_id=DeviceID("alice@dev2"),
#             certified_device=certification_2,
#             device_certifier=DeviceID("alice@dev1"),
#             created_on=now,
#         )

#         mallory_device = RemoteDevice(
#             device_id=DeviceID("alice@dev3"),
#             certified_device=certification_3,
#             device_certifier=DeviceID("alice@dev2"),
#             created_on=now,
#         )

#         alice_user = RemoteUser(
#             user_id="alice",
#             certified_user=user_certification,
#             user_certifier=None,
#             devices=RemoteDevicesMapping(alice_device, bob_device, mallory_device),
#             created_on=now,
#         )

#         trustchain = {DeviceID("alice@dev1"): alice_device, DeviceID("alice@dev2"): bob_device}

#         with pytest.raises(TrustChainBrokenChainError):
#             cascade_validate_devices(
#                 alice_user, trustchain, coolorg.organization_id, coolorg.root_verify_key
#             )


# @pytest.mark.trio
# async def test_cascade_validate_devices_with_revocation(coolorg, alice, bob, mallory):
#     now = Pendulum(2000, 1, 1)

#     with freeze_time(now):
#         certification_1 = certify_device(
#             None, coolorg.root_signing_key, DeviceID("alice@dev1"), alice.verify_key
#         )
#         certification_2 = certify_device(
#             DeviceID("alice@dev1"), alice.signing_key, DeviceID("alice@dev2"), bob.verify_key
#         )
#         certification_3 = certify_device(
#             DeviceID("alice@dev2"), bob.signing_key, DeviceID("alice@dev3"), mallory.verify_key
#         )

#         bob_device_revocation = certify_device_revocation(
#             certifier_id=DeviceID("alice@dev1"),
#             certifier_key=alice.signing_key,
#             revoked_device_id=DeviceID("alice@dev2"),
#         )

#         user_certification = certify_user(
#             certifier_id=None,
#             certifier_key=coolorg.root_signing_key,
#             user_id=alice.user_id,
#             public_key=alice.public_key,
#             now=now,
#         )

#         alice_device = RemoteDevice(
#             device_id=DeviceID("alice@dev1"),
#             certified_device=certification_1,
#             device_certifier=None,
#             created_on=now,
#         )

#         bob_device = RemoteDevice(
#             device_id=DeviceID("alice@dev2"),
#             certified_device=certification_2,
#             device_certifier=DeviceID("alice@dev1"),
#             created_on=now,
#             certified_revocation=bob_device_revocation,
#             revocation_certifier=DeviceID("alice@dev1"),
#             revocated_on=now,
#         )

#         mallory_device = RemoteDevice(
#             device_id=DeviceID("alice@dev3"),
#             certified_device=certification_3,
#             device_certifier=DeviceID("alice@dev2"),
#             created_on=now,
#         )

#         alice_user = RemoteUser(
#             user_id="alice",
#             certified_user=user_certification,
#             user_certifier=None,
#             devices=RemoteDevicesMapping(alice_device, bob_device, mallory_device),
#             created_on=now,
#         )

#         trustchain = {DeviceID("alice@dev1"): alice_device, DeviceID("alice@dev2"): bob_device}

#         cascade_validate_devices(
#             alice_user, trustchain, coolorg.organization_id, coolorg.root_verify_key
#         )


# # TODO device revoked
# # TODO user revoked (only one device revoked)
# # TODO with and without top user
# # TODO wrong time


# def test_cascade_validate_devices_wrong(alice, bob, mallory):
#     now = Pendulum(2000, 1, 1)

#     with freeze_time(now):
#         certification_1 = certify_device(
#             alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key
#         )
#         certification_2 = certify_device(
#             bob.device_id, bob.signing_key, mallory.device_id, mallory.verify_key
#         )
#     with pytest.raises(TrustChainBrokenChainError):
#         cascade_validate_devices(
#             [certification_1, certification_2], alice.device_id, alice.verify_key, now
#         )
