# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import pendulum

from parsec.backend.user import INVITATION_VALIDITY
from parsec.api.data import UserCertificateContent, DeviceCertificateContent
from parsec.api.protocol import user_create_serializer

from tests.common import freeze_time
from tests.backend.user.test_access import user_get


async def user_create(sock, **kwargs):
    await sock.send(user_create_serializer.req_dumps({"cmd": "user_create", **kwargs}))
    raw_rep = await sock.recv()
    rep = user_create_serializer.rep_loads(raw_rep)
    return rep


@pytest.mark.trio
@pytest.mark.parametrize("is_admin", [True, False])
async def test_user_create_ok(
    backend, backend_sock_factory, alice_backend_sock, alice, mallory, is_admin
):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=is_admin,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)

    with backend.event_bus.listen() as spy:
        rep = await user_create(
            alice_backend_sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
        )
        assert rep == {"status": "ok"}

        # No guarantees this event occurs before the command's return
        await spy.wait_with_timeout(
            "user.created",
            {
                "organization_id": alice.organization_id,
                "user_id": mallory.user_id,
                "first_device_id": mallory.device_id,
                "user_certificate": user_certificate,
                "first_device_certificate": device_certificate,
            },
        )

    # Make sure mallory can connect now
    async with backend_sock_factory(backend, mallory) as sock:
        rep = await user_get(sock, user_id=mallory.user_id)
        assert rep["status"] == "ok"


@pytest.mark.trio
async def test_user_create_invalid_certified(alice_backend_sock, alice, bob, mallory):
    now = pendulum.now()
    good_user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    good_device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    bad_user_certificate = UserCertificateContent(
        author=bob.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(bob.signing_key)
    bad_device_certificate = DeviceCertificateContent(
        author=bob.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(bob.signing_key)

    for cu, cd in [
        (good_user_certificate, bad_device_certificate),
        (bad_user_certificate, good_device_certificate),
        (bad_user_certificate, bad_device_certificate),
    ]:
        rep = await user_create(alice_backend_sock, user_certificate=cu, device_certificate=cd)
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid certification data (Signature was forged or corrupt).",
        }


@pytest.mark.trio
async def test_user_create_not_matching_user_device(alice_backend_sock, alice, bob, mallory):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=bob.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_backend_sock, user_certificate=user_certificate, device_certificate=device_certificate
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Device and User must have the same user ID.",
    }


@pytest.mark.trio
async def test_user_create_already_exists(alice_backend_sock, alice, bob):
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=alice.device_id,
        timestamp=now,
        user_id=bob.user_id,
        public_key=bob.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id, timestamp=now, device_id=bob.device_id, verify_key=bob.verify_key
    ).dump_and_sign(alice.signing_key)

    rep = await user_create(
        alice_backend_sock, user_certificate=user_certificate, device_certificate=device_certificate
    )
    assert rep == {"status": "already_exists", "reason": f"User `{bob.user_id}` already exists"}


@pytest.mark.trio
async def test_user_create_not_matching_certified_on(alice_backend_sock, alice, mallory):
    date1 = pendulum.Pendulum(2000, 1, 1)
    date2 = date1.add(seconds=1)
    cu = UserCertificateContent(
        author=alice.device_id,
        timestamp=date1,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    cd = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=date2,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)
    with freeze_time(date1):
        rep = await user_create(alice_backend_sock, user_certificate=cu, device_certificate=cd)
        assert rep == {
            "status": "invalid_data",
            "reason": "Device and User certifications must have the same timestamp.",
        }


@pytest.mark.trio
async def test_user_create_certify_too_old(alice_backend_sock, alice, mallory):
    too_old = pendulum.Pendulum(2000, 1, 1)
    now = too_old.add(seconds=INVITATION_VALIDITY + 1)
    cu = UserCertificateContent(
        author=alice.device_id,
        timestamp=too_old,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(alice.signing_key)
    cd = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=too_old,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(alice.signing_key)

    with freeze_time(now):
        rep = await user_create(alice_backend_sock, user_certificate=cu, device_certificate=cd)
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certification.",
        }


@pytest.mark.trio
async def test_user_create_author_not_admin(backend, bob_backend_sock, bob, mallory):
    # Unlike alice, bob is not admin
    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=bob.device_id,
        timestamp=now,
        user_id=mallory.user_id,
        public_key=mallory.public_key,
        is_admin=False,
    ).dump_and_sign(bob.signing_key)
    device_certificate = DeviceCertificateContent(
        author=bob.device_id,
        timestamp=now,
        device_id=mallory.device_id,
        verify_key=mallory.verify_key,
    ).dump_and_sign(bob.signing_key)

    rep = await user_create(
        bob_backend_sock, user_certificate=user_certificate, device_certificate=device_certificate
    )
    assert rep == {"status": "not_allowed", "reason": "User `bob` is not admin"}
