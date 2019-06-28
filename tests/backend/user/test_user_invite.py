# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from async_generator import asynccontextmanager

from parsec.backend.user import PEER_EVENT_MAX_WAIT
from parsec.api.protocole import user_invite_serializer


@asynccontextmanager
async def user_invite(sock, **kwargs):
    reps = []
    await sock.send(user_invite_serializer.req_dumps({"cmd": "user_invite", **kwargs}))
    yield reps
    raw_rep = await sock.recv()
    rep = user_invite_serializer.rep_loads(raw_rep)
    reps.append(rep)


@pytest.mark.trio
async def test_user_invite(backend, alice_backend_sock, alice, mallory):
    with backend.event_bus.listen() as spy:
        async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep:

            # Waiting for user.claimed event
            await spy.wait_with_timeout("event.connected", kwargs={"event_name": "user.claimed"})

            backend.event_bus.send(
                "user.claimed",
                organization_id=alice.organization_id,
                user_id="foo",
                encrypted_claim=b"<dummy>",
            )
            backend.event_bus.send(
                "user.claimed",
                organization_id=alice.organization_id,
                user_id=mallory.user_id,
                encrypted_claim=b"<good>",
            )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_user_invite_already_exists(backend, alice_backend_sock, alice, bob):
    async with user_invite(alice_backend_sock, user_id=bob.user_id) as prep:
        pass
    assert prep[0] == {"status": "already_exists", "reason": f"User `{bob.user_id}` already exists"}


@pytest.mark.trio
async def test_user_invite_timeout(mock_clock, backend, alice_backend_sock, alice, mallory):
    with backend.event_bus.listen() as spy:
        async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep:

            await spy.wait_with_timeout("event.connected", kwargs={"event_name": "user.claimed"})
            mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    assert prep[0] == {
        "status": "timeout",
        "reason": "Timeout while waiting for new user to be claimed.",
    }


@pytest.mark.trio
async def test_user_invite_not_admin(bob_backend_sock, bob, mallory):
    async with user_invite(bob_backend_sock, user_id=mallory.user_id) as prep:
        pass
    assert prep[0] == {"status": "invalid_role", "reason": f"User `{bob.user_id}` is not admin"}


@pytest.mark.trio
async def test_concurrent_user_invite(
    backend, alice_backend_sock, adam_backend_sock, alice, adam, mallory
):
    with backend.event_bus.listen() as spy:
        async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep1:

            await spy.wait_with_timeout("event.connected", kwargs={"event_name": "user.claimed"})
            async with user_invite(adam_backend_sock, user_id=mallory.user_id) as prep2:

                spy.clear()
                await spy.wait_with_timeout(
                    "event.connected", kwargs={"event_name": "user.claimed"}
                )

                backend.event_bus.send(
                    "user.claimed",
                    organization_id=mallory.organization_id,
                    user_id=mallory.user_id,
                    encrypted_claim=b"<good>",
                )

    assert prep1[0] == {"status": "ok", "encrypted_claim": b"<good>"}
    assert prep2[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_user_invite_same_name_different_organizations(
    backend, alice_backend_sock, otheralice_backend_sock, alice, otheralice, mallory
):
    # Mallory invitation from first organization
    with backend.event_bus.listen() as spy:
        async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep:

            # Waiting for user.claimed event
            await spy.wait_with_timeout("event.connected", kwargs={"event_name": "user.claimed"})

            backend.event_bus.send(
                "user.claimed",
                organization_id=alice.organization_id,
                user_id="foo",
                encrypted_claim=b"<dummy>",
            )
            backend.event_bus.send(
                "user.claimed",
                organization_id=alice.organization_id,
                user_id=mallory.user_id,
                encrypted_claim=b"<good>",
            )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}

    # Mallory invitation from second organization
    with backend.event_bus.listen() as spy:
        async with user_invite(otheralice_backend_sock, user_id=mallory.user_id) as prep:

            # Waiting for user.claimed event
            await spy.wait_with_timeout("event.connected", kwargs={"event_name": "user.claimed"})

            backend.event_bus.send(
                "user.claimed",
                organization_id=otheralice.organization_id,
                user_id="foo",
                encrypted_claim=b"<dummy>",
            )
            backend.event_bus.send(
                "user.claimed",
                organization_id=otheralice.organization_id,
                user_id=mallory.user_id,
                encrypted_claim=b"<good>",
            )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}
