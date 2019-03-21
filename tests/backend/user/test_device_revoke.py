# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import pendulum

from parsec.crypto import build_revoked_device_certificate
from parsec.backend.user import INVITATION_VALIDITY
from parsec.api.protocole import device_revoke_serializer, HandshakeRevokedDevice

from tests.common import freeze_time


@pytest.fixture
def bob_revocation(alice, bob):
    now = pendulum.now()
    return build_revoked_device_certificate(alice.device_id, alice.signing_key, bob.device_id, now)


async def device_revoke(sock, **kwargs):
    await sock.send(device_revoke_serializer.req_dumps({"cmd": "device_revoke", **kwargs}))
    raw_rep = await sock.recv()
    rep = device_revoke_serializer.rep_loads(raw_rep)
    return rep


@pytest.mark.trio
async def test_device_revoke_ok(
    backend, backend_sock_factory, bob_backend_sock, alice, alice2, bob
):
    await backend.user.set_user_admin(bob.organization_id, bob.user_id, True)

    now = pendulum.Pendulum(2000, 10, 11)
    alice_revocation = build_revoked_device_certificate(
        bob.device_id, bob.signing_key, alice.device_id, now
    )
    alice2_revocation = build_revoked_device_certificate(
        bob.device_id, bob.signing_key, alice2.device_id, now
    )

    # Revoke Alice's first device
    with freeze_time(now):
        rep = await device_revoke(bob_backend_sock, revoked_device_certificate=alice_revocation)
    assert rep == {"status": "ok", "user_revoked_on": None}

    # Alice cannot connect from now on...
    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, alice):
            pass

    # ...but Alice2 can
    async with backend_sock_factory(backend, alice2):
        pass

    # Revoke Alice's second device (should automatically revoke the user)
    with freeze_time(now):
        rep = await device_revoke(bob_backend_sock, revoked_device_certificate=alice2_revocation)
    assert rep == {"status": "ok", "user_revoked_on": now}

    # Alice2 cannot connect from now on...
    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, alice2):
            pass


@pytest.mark.trio
async def test_device_revoke_not_admin(
    backend, backend_sock_factory, alice_backend_sock, alice, bob, bob_revocation
):
    rep = await device_revoke(alice_backend_sock, revoked_device_certificate=bob_revocation)
    assert rep == {"status": "invalid_role", "reason": f"User `{alice.user_id}` is not admin"}


@pytest.mark.trio
async def test_device_revoke_own_device_not_admin(
    backend, backend_sock_factory, alice_backend_sock, alice, alice2
):
    now = pendulum.now()
    alice2_revocation = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, alice2.device_id, now
    )

    rep = await device_revoke(alice_backend_sock, revoked_device_certificate=alice2_revocation)
    assert rep == {"status": "ok", "user_revoked_on": None}

    # Make sure alice2 cannot connect from now on
    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, alice2):
            pass


@pytest.mark.trio
async def test_device_revoke_unknown(backend, alice_backend_sock, alice, mallory):
    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    revoked_device_certificate = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, mallory.device_id, pendulum.now()
    )

    rep = await device_revoke(
        alice_backend_sock, revoked_device_certificate=revoked_device_certificate
    )
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_good_user_bad_device(backend, alice_backend_sock, alice):
    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    revoked_device_certificate = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, f"{alice.user_id}@foo", pendulum.now()
    )

    rep = await device_revoke(
        alice_backend_sock, revoked_device_certificate=revoked_device_certificate
    )
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_revoke_already_revoked(
    backend, alice_backend_sock, alice, bob, bob_revocation
):
    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    rep = await device_revoke(alice_backend_sock, revoked_device_certificate=bob_revocation)
    assert rep["status"] == "ok"

    rep = await device_revoke(alice_backend_sock, revoked_device_certificate=bob_revocation)
    assert rep == {
        "status": "already_revoked",
        "reason": f"Device `{bob.device_id}` already revoked",
    }


@pytest.mark.trio
async def test_device_revoke_invalid_certified(backend, alice_backend_sock, alice2, bob):
    await backend.user.set_user_admin(alice2.organization_id, alice2.user_id, True)

    revoked_device_certificate = build_revoked_device_certificate(
        alice2.device_id, alice2.signing_key, bob.device_id, pendulum.now()
    )

    rep = await device_revoke(
        alice_backend_sock, revoked_device_certificate=revoked_device_certificate
    )
    assert rep == {
        "status": "invalid_certification",
        "reason": "Invalid certification data (Signature was forged or corrupt).",
    }


@pytest.mark.trio
async def test_device_revoke_certify_too_old(backend, alice_backend_sock, alice, bob):
    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    now = pendulum.Pendulum(2000, 1, 1)
    revoked_device_certificate = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, bob.device_id, now
    )

    with freeze_time(now.add(seconds=INVITATION_VALIDITY + 1)):
        rep = await device_revoke(
            alice_backend_sock, revoked_device_certificate=revoked_device_certificate
        )
        assert rep == {
            "status": "invalid_certification",
            "reason": "Invalid timestamp in certification.",
        }


@pytest.mark.trio
async def test_device_revoke_other_organization(
    sock_from_other_organization_factory, backend_sock_factory, backend, alice, bob
):

    # Organizations should be isolated...
    async with sock_from_other_organization_factory(backend, mimick=alice.device_id) as sock:
        # ...even for organization admins !
        await backend.user.set_user_admin(sock.device.organization_id, sock.device.user_id, True)

        revocation = build_revoked_device_certificate(
            sock.device.device_id, sock.device.signing_key, bob.device_id, pendulum.now()
        )

        rep = await device_revoke(sock, revoked_device_certificate=revocation)
        assert rep == {"status": "not_found"}

    # Make sure bob still works
    async with backend_sock_factory(backend, bob):
        pass
