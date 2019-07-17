# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
import pytest
import trio

from parsec.api.protocole import user_create_serializer
from parsec.backend.user import INVITATION_VALIDITY
from parsec.crypto import build_device_certificate, build_user_certificate
from tests.backend.user.test_access import user_get
from tests.common import freeze_time


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
    user_certificate = build_user_certificate(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, is_admin, now
    )
    device_certificate = build_device_certificate(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key, now
    )

    with backend.event_bus.listen() as spy:
        rep = await user_create(
            alice_backend_sock,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
        )
        assert rep == {"status": "ok"}

        with trio.fail_after(1):
            # No guarantees this event occurs before the command's return
            await spy.wait(
                "user.created",
                kwargs={
                    "organization_id": alice.organization_id,
                    "user_id": mallory.user_id,
                    "first_device_id": mallory.device_id,
                },
            )

    # Make sure mallory can connect now
    async with backend_sock_factory(backend, mallory) as sock:
        rep = await user_get(sock, user_id=mallory.user_id)
        assert rep["status"] == "ok"


@pytest.mark.trio
async def test_user_create_invalid_certified(alice_backend_sock, alice, bob, mallory):
    now = pendulum.now()
    good_user_certificate = build_user_certificate(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, False, now
    )
    good_device_certificate = build_device_certificate(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key, now
    )
    bad_user_certificate = build_user_certificate(
        bob.device_id, bob.signing_key, mallory.user_id, mallory.public_key, False, now
    )
    bad_device_certificate = build_device_certificate(
        bob.device_id, bob.signing_key, mallory.device_id, mallory.verify_key, now
    )

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
    user_certificate = build_user_certificate(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, False, now
    )
    device_certificate = build_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, mallory.verify_key, now
    )

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
    user_certificate = build_user_certificate(
        alice.device_id, alice.signing_key, bob.user_id, bob.public_key, False, now
    )
    device_certificate = build_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, bob.verify_key, now
    )

    rep = await user_create(
        alice_backend_sock, user_certificate=user_certificate, device_certificate=device_certificate
    )
    assert rep == {"status": "already_exists", "reason": f"User `{bob.user_id}` already exists"}


@pytest.mark.trio
async def test_user_create_not_matching_certified_on(alice_backend_sock, alice, mallory):
    date1 = pendulum.Pendulum(2000, 1, 1)
    date2 = date1.add(seconds=1)
    cu = build_user_certificate(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, False, date1
    )
    cd = build_device_certificate(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key, date2
    )
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
    cu = build_user_certificate(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, False, too_old
    )
    cd = build_device_certificate(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key, too_old
    )

    with freeze_time(now):
        rep = await user_create(alice_backend_sock, user_certificate=cu, device_certificate=cd)
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certification.",
        }
