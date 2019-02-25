# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
import trio
from itertools import combinations_with_replacement

from parsec.api.protocole import (
    beacon_poll_serializer,
    beacon_get_rights_serializer,
    beacon_set_rights_serializer,
)

from tests.backend.test_events import events_subscribe, events_listen_nowait
from tests.backend.test_vlob import vlob_update


VLOB_ID = UUID("00000000000000000000000000000001")
OTHER_VLOB_ID = UUID("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = UUID("00000000000000000000000000000003")
BEACON_ID = UUID("0000000000000000000000000000000A")
OTHER_BEACON_ID = UUID("0000000000000000000000000000000B")
YET_ANOTHER_BEACON_ID = UUID("0000000000000000000000000000000C")


async def beacon_get_rights(sock, id):
    raw_rep = await sock.send(
        beacon_get_rights_serializer.req_dumps({"cmd": "beacon_get_rights", "id": id})
    )
    raw_rep = await sock.recv()
    return beacon_get_rights_serializer.rep_loads(raw_rep)


async def beacon_set_rights(sock, id, user, admin_access, read_access, write_access):
    raw_rep = await sock.send(
        beacon_set_rights_serializer.req_dumps(
            {
                "cmd": "beacon_set_rights",
                "id": id,
                "user": user,
                "admin_access": admin_access,
                "read_access": read_access,
                "write_access": write_access,
            }
        )
    )
    raw_rep = await sock.recv()
    return beacon_set_rights_serializer.rep_loads(raw_rep)


async def beacon_poll(sock, id, last_checkpoint):
    raw_rep = await sock.send(
        beacon_poll_serializer.req_dumps(
            {"cmd": "beacon_poll", "id": id, "last_checkpoint": last_checkpoint}
        )
    )
    raw_rep = await sock.recv()
    return beacon_poll_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_beacon_lazy_created_by_new_vlob(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    rep = await beacon_poll(alice_backend_sock, BEACON_ID, 0)
    assert rep == {"status": "ok", "current_checkpoint": 1, "changes": {VLOB_ID: 1}}


@pytest.mark.trio
async def test_beacon_updated_by_vlob(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")
    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 2, b"v2")

    for last_checkpoint in (0, 1):
        rep = await beacon_poll(alice_backend_sock, BEACON_ID, last_checkpoint)
        assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}


@pytest.mark.trio
async def test_beacon_poll_checkpoint_up_to_date(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")
    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 2, b"v2")

    rep = await beacon_poll(alice_backend_sock, BEACON_ID, 2)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {}}


@pytest.mark.trio
async def test_beacon_poll_not_found(alice_backend_sock):
    rep = await beacon_poll(alice_backend_sock, BEACON_ID, 0)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_beacon_get_rights_not_found(alice_backend_sock):
    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_beacon_set_rights_not_found(bob, alice_backend_sock):
    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, True, True, True)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_beacon_set_rights_bad_user(backend, alice, mallory, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, mallory.user_id, True, True, True)
    assert rep == {"status": "error", "reason": "Unknown user"}


@pytest.mark.trio
async def test_beacon_remove_rights_idempotent(backend, alice, bob, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, False, False, False)
    assert rep == {"status": "ok"}

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, False, False, False)
    assert rep == {"status": "ok"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {"alice": {"admin_access": True, "read_access": True, "write_access": True}},
    }


@pytest.mark.trio
async def test_beacon_need_admin_to_share(backend, alice, bob, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    # Only read access...
    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, alice.user_id, False, True, True)
    assert rep == {"status": "ok"}

    # ...so we shouldn't be able to allow Bob to write
    for rights in combinations_with_replacement((True, False), 3):
        rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, *rights)
        assert rep == {"status": "not_allowed"}

    # Can no longer re-enable ourselft either
    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, alice.user_id, True, True, True)
    assert rep == {"status": "not_allowed"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {"alice": {"admin_access": False, "read_access": True, "write_access": True}},
    }


@pytest.mark.trio
async def test_beacon_handle_rights(backend, alice, bob, alice_backend_sock, bob_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    # At first only Alice is allowed

    rep = await beacon_poll(bob_backend_sock, BEACON_ID, 2)
    assert rep == {"status": "not_allowed"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {"alice": {"admin_access": True, "read_access": True, "write_access": True}},
    }

    # Now add Bob with write access only

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, False, False, True)
    assert rep == {"status": "ok"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {
            "alice": {"admin_access": True, "read_access": True, "write_access": True},
            "bob": {"admin_access": False, "read_access": False, "write_access": True},
        },
    }

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 2, b"v2")
    assert rep == {"status": "ok"}

    rep = await beacon_poll(bob_backend_sock, BEACON_ID, 1)
    assert rep == {"status": "not_allowed"}

    # Now add Bob with read access

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, False, True, False)
    assert rep == {"status": "ok"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {
            "alice": {"admin_access": True, "read_access": True, "write_access": True},
            "bob": {"admin_access": False, "read_access": True, "write_access": False},
        },
    }

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}

    rep = await beacon_poll(bob_backend_sock, BEACON_ID, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Finally remove all rights from Bob

    rep = await beacon_set_rights(alice_backend_sock, BEACON_ID, bob.user_id, False, False, False)
    assert rep == {"status": "ok"}

    rep = await beacon_poll(bob_backend_sock, BEACON_ID, 2)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}

    rep = await beacon_get_rights(alice_backend_sock, BEACON_ID)
    assert rep == {
        "status": "ok",
        "users": {"alice": {"admin_access": True, "read_access": True, "write_access": True}},
    }


@pytest.mark.trio
async def test_beacon_updated_event(backend, alice_backend_sock, alice, alice2):
    # Not listened events

    await backend.vlob.create(alice.organization_id, alice.device_id, BEACON_ID, VLOB_ID, b"v1")

    # Start listening events

    await events_subscribe(alice_backend_sock, beacon_updated=[BEACON_ID, OTHER_BEACON_ID])

    # Good events

    await backend.vlob.create(
        alice.organization_id, alice2.device_id, OTHER_BEACON_ID, OTHER_VLOB_ID, b"v1"
    )
    await backend.vlob.update(alice.organization_id, alice2.device_id, VLOB_ID, 2, b"v2")
    await backend.vlob.update(alice.organization_id, alice2.device_id, VLOB_ID, 3, b"v3")

    with trio.fail_after(1):
        # No guarantees those events occur before the commands' return
        await backend.event_bus.spy.wait_multiple(
            ["beacon.updated", "beacon.updated", "beacon.updated"]
        )

    reps = [
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
    ]
    assert reps == [
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": OTHER_BEACON_ID,
            "checkpoint": 1,
            "src_id": OTHER_VLOB_ID,
            "src_version": 1,
        },
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": BEACON_ID,
            "checkpoint": 2,
            "src_id": VLOB_ID,
            "src_version": 2,
        },
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": BEACON_ID,
            "checkpoint": 3,
            "src_id": VLOB_ID,
            "src_version": 3,
        },
        {"status": "no_events"},
    ]

    # Ignore self events

    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 4, b"v4")

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}

    # Beacon id not subscribed to

    await backend.vlob.create(
        alice.organization_id, alice2.device_id, YET_ANOTHER_BEACON_ID, YET_ANOTHER_VLOB_ID, b"v1"
    )
    await backend.vlob.update(
        alice.organization_id, alice2.device_id, YET_ANOTHER_VLOB_ID, 2, b"v2"
    )

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}
