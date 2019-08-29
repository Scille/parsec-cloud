# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import pendulum

from parsec.api.data import DeviceCertificateContent
from parsec.backend.user import INVITATION_VALIDITY
from parsec.api.protocol import packb, device_create_serializer, ping_serializer

from tests.common import freeze_time


@pytest.fixture
def alice_nd(local_device_factory, alice):
    return local_device_factory(f"{alice.user_id}@new_device")


async def device_create(sock, **kwargs):
    await sock.send(device_create_serializer.req_dumps({"cmd": "device_create", **kwargs}))
    raw_rep = await sock.recv()
    rep = device_create_serializer.rep_loads(raw_rep)
    return rep


@pytest.mark.trio
async def test_device_create_ok(backend, backend_sock_factory, alice_backend_sock, alice, alice_nd):
    now = pendulum.now()
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(alice.signing_key)

    with backend.event_bus.listen() as spy:
        rep = await device_create(
            alice_backend_sock, device_certificate=device_certificate, encrypted_answer=b"<good>"
        )
        assert rep == {"status": "ok"}

        # No guarantees this event occurs before the command's return
        await spy.wait_with_timeout(
            "device.created",
            {
                "organization_id": alice_nd.organization_id,
                "device_id": alice_nd.device_id,
                "device_certificate": device_certificate,
                "encrypted_answer": b"<good>",
            },
        )

    # Make sure the new device can connect now
    async with backend_sock_factory(backend, alice_nd) as sock:
        await sock.send(packb({"cmd": "ping", "ping": "Hello world !"}))
        raw_rep = await sock.recv()
        rep = ping_serializer.rep_loads(raw_rep)
        assert rep == {"status": "ok", "pong": "Hello world !"}


@pytest.mark.trio
async def test_device_create_invalid_certified(alice_backend_sock, bob, alice_nd):
    now = pendulum.now()
    device_certificate = DeviceCertificateContent(
        author=bob.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(bob.signing_key)

    rep = await device_create(
        alice_backend_sock, device_certificate=device_certificate, encrypted_answer=b"<good>"
    )
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }


@pytest.mark.trio
async def test_device_create_already_exists(alice_backend_sock, alice, alice2):
    now = pendulum.now()
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=alice2.device_id,
        verify_key=alice2.verify_key,
    ).dump_and_sign(alice.signing_key)

    rep = await device_create(
        alice_backend_sock, device_certificate=device_certificate, encrypted_answer=b"<good>"
    )
    assert rep == {
        "status": "already_exists",
        "reason": f"Device `{alice2.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_create_not_own_user(bob_backend_sock, bob, alice_nd):
    now = pendulum.now()
    device_certificate = DeviceCertificateContent(
        author=bob.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(bob.signing_key)

    rep = await device_create(
        bob_backend_sock, device_certificate=device_certificate, encrypted_answer=b"<good>"
    )
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_create_certify_too_old(alice_backend_sock, alice, alice_nd):
    now = pendulum.Pendulum(2000, 1, 1)
    device_certificate = DeviceCertificateContent(
        author=alice.device_id,
        timestamp=now,
        device_id=alice_nd.device_id,
        verify_key=alice_nd.verify_key,
    ).dump_and_sign(alice.signing_key)

    with freeze_time(now.add(seconds=INVITATION_VALIDITY + 1)):
        rep = await device_create(
            alice_backend_sock, device_certificate=device_certificate, encrypted_answer=b"<good>"
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certification.",
        }
