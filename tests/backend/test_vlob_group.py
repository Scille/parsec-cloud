# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
import trio
from pendulum import Pendulum

from parsec.api.protocole import (
    VlobGroupRole,
    vlob_group_poll_serializer,
    vlob_group_get_roles_serializer,
    vlob_group_update_roles_serializer,
)

from tests.backend.test_events import events_subscribe, events_listen_nowait
from tests.backend.test_vlob import vlob_update


NOW = Pendulum(2000, 1, 1)
VLOB_ID = UUID("00000000000000000000000000000001")
OTHER_VLOB_ID = UUID("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = UUID("00000000000000000000000000000003")
GROUP_ID = UUID("0000000000000000000000000000000A")
OTHER_GROUP_ID = UUID("0000000000000000000000000000000B")
YET_ANOTHER_GROUP_ID = UUID("0000000000000000000000000000000C")


async def group_get_roles(sock, id):
    raw_rep = await sock.send(
        vlob_group_get_roles_serializer.req_dumps({"cmd": "vlob_group_get_roles", "id": id})
    )
    raw_rep = await sock.recv()
    return vlob_group_get_roles_serializer.rep_loads(raw_rep)


async def group_update_roles(sock, id, user, role):
    raw_rep = await sock.send(
        vlob_group_update_roles_serializer.req_dumps(
            {"cmd": "vlob_group_update_roles", "id": id, "user": user, "role": role}
        )
    )
    raw_rep = await sock.recv()
    return vlob_group_update_roles_serializer.rep_loads(raw_rep)


async def group_poll(sock, id, last_checkpoint):
    raw_rep = await sock.send(
        vlob_group_poll_serializer.req_dumps(
            {"cmd": "vlob_group_poll", "id": id, "last_checkpoint": last_checkpoint}
        )
    )
    raw_rep = await sock.recv()
    return vlob_group_poll_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_vlob_group_lazy_created_by_new_vlob(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    rep = await group_poll(alice_backend_sock, GROUP_ID, 0)
    assert rep == {"status": "ok", "current_checkpoint": 1, "changes": {VLOB_ID: 1}}

    # Make sure author gets OWNER role

    rep = await group_get_roles(alice_backend_sock, GROUP_ID)
    assert rep == {"status": "ok", "users": {"alice": VlobGroupRole.OWNER}}


@pytest.mark.trio
async def test_vlob_group_updated_by_vlob(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")
    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 2, NOW, b"v2")

    for last_checkpoint in (0, 1):
        rep = await group_poll(alice_backend_sock, GROUP_ID, last_checkpoint)
        assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}


@pytest.mark.trio
async def test_vlob_group_poll_checkpoint_up_to_date(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")
    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 2, NOW, b"v2")

    rep = await group_poll(alice_backend_sock, GROUP_ID, 2)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {}}


@pytest.mark.trio
async def test_vlob_group_poll_not_found(alice_backend_sock):
    rep = await group_poll(alice_backend_sock, GROUP_ID, 0)
    assert rep == {
        "status": "not_found",
        "reason": "Group `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_group_get_roles_not_found(alice_backend_sock):
    rep = await group_get_roles(alice_backend_sock, GROUP_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Group `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_group_update_roles_not_found(bob, alice_backend_sock):
    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, VlobGroupRole.MANAGER)
    assert rep == {
        "status": "not_found",
        "reason": "Group `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_group_update_roles_bad_user(backend, alice, mallory, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    rep = await group_update_roles(
        alice_backend_sock, GROUP_ID, mallory.user_id, VlobGroupRole.MANAGER
    )
    assert rep == {"status": "not_found", "reason": "User `mallory` doesn't exist"}


@pytest.mark.trio
async def test_vlob_group_update_roles_cannot_modify_self(backend, alice, alice_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    rep = await group_update_roles(
        alice_backend_sock, GROUP_ID, alice.user_id, VlobGroupRole.MANAGER
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
@pytest.mark.parametrize("start_with_existing_role", (False, True))
async def test_vlob_group_remove_role_idempotent(
    backend, alice, bob, alice_backend_sock, start_with_existing_role
):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    if start_with_existing_role:
        rep = await group_update_roles(
            alice_backend_sock, GROUP_ID, bob.user_id, VlobGroupRole.MANAGER
        )
        assert rep == {"status": "ok"}

    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await group_get_roles(alice_backend_sock, GROUP_ID)
    assert rep == {"status": "ok", "users": {"alice": VlobGroupRole.OWNER}}


@pytest.mark.trio
async def test_vlob_group_update_roles_as_owner(
    backend, alice, bob, alice_backend_sock, bob_backend_sock
):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    for role in VlobGroupRole:
        rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, role)
        assert rep == {"status": "ok"}

        rep = await group_get_roles(bob_backend_sock, GROUP_ID)
        assert rep == {
            "status": "ok",
            "users": {alice.user_id: VlobGroupRole.OWNER, bob.user_id: role},
        }

    # Now remove role
    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await group_get_roles(bob_backend_sock, GROUP_ID)
    assert rep["status"] == "not_allowed"


@pytest.mark.trio
async def test_vlob_group_update_roles_as_manager(
    backend_data_binder,
    local_device_factory,
    backend,
    alice,
    bob,
    alice_backend_sock,
    bob_backend_sock,
):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")
    # Vlob group must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice is manager and gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await backend.vlob.update_group_roles(
        alice.organization_id, alice.user_id, GROUP_ID, zack.user_id, VlobGroupRole.OWNER
    )
    await backend.vlob.update_group_roles(
        alice.organization_id, zack.user_id, GROUP_ID, alice.user_id, VlobGroupRole.MANAGER
    )

    for role in (VlobGroupRole.CONTRIBUTOR, VlobGroupRole.READER):
        rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, role)
        assert rep == {"status": "ok"}

        rep = await group_get_roles(bob_backend_sock, GROUP_ID)
        assert rep == {
            "status": "ok",
            "users": {
                zack.user_id: VlobGroupRole.OWNER,
                alice.user_id: VlobGroupRole.MANAGER,
                bob.user_id: role,
            },
        }

    # Remove role
    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "ok"}
    rep = await group_get_roles(bob_backend_sock, GROUP_ID)
    assert rep["status"] == "not_allowed"

    # Cannot give owner or manager role as manager
    rep = await group_update_roles(alice_backend_sock, GROUP_ID, zack.user_id, VlobGroupRole.OWNER)
    assert rep == {"status": "not_allowed"}
    rep = await group_update_roles(
        alice_backend_sock, GROUP_ID, zack.user_id, VlobGroupRole.MANAGER
    )
    assert rep == {"status": "not_allowed"}

    # Also cannot change owner or manager role
    for bob_role in (VlobGroupRole.OWNER, VlobGroupRole.MANAGER):
        await backend.vlob.update_group_roles(
            alice.organization_id, zack.user_id, GROUP_ID, bob.user_id, role
        )
        rep = await group_update_roles(
            alice_backend_sock, GROUP_ID, zack.user_id, VlobGroupRole.CONTRIBUTOR
        )
        assert rep == {"status": "not_allowed"}


@pytest.mark.trio
@pytest.mark.parametrize("alice_role", (VlobGroupRole.CONTRIBUTOR, VlobGroupRole.READER))
async def test_vlob_group_update_not_allowed(
    backend_data_binder, local_device_factory, backend, alice, bob, alice_backend_sock, alice_role
):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")
    # Vlob group must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await backend.vlob.update_group_roles(
        alice.organization_id, alice.user_id, GROUP_ID, zack.user_id, VlobGroupRole.OWNER
    )
    await backend.vlob.update_group_roles(
        alice.organization_id, zack.user_id, GROUP_ID, alice.user_id, alice_role
    )

    # Cannot give role
    for role in VlobGroupRole:
        rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, role)
        assert rep == {"status": "not_allowed"}

    # Cannot remove role
    await backend.vlob.update_group_roles(
        alice.organization_id, zack.user_id, GROUP_ID, bob.user_id, VlobGroupRole.READER
    )
    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_vlob_group_poll(backend, alice, bob, alice_backend_sock, bob_backend_sock):
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    # At first only Alice is allowed

    rep = await group_poll(bob_backend_sock, GROUP_ID, 2)
    assert rep == {"status": "not_allowed"}

    # Add Bob with read&write rights

    rep = await group_update_roles(
        alice_backend_sock, GROUP_ID, bob.user_id, VlobGroupRole.CONTRIBUTOR
    )
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 2, b"v2")
    assert rep == {"status": "ok"}

    rep = await group_poll(bob_backend_sock, GROUP_ID, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Change Bob with read only right

    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, VlobGroupRole.READER)
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}

    rep = await group_poll(bob_backend_sock, GROUP_ID, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Finally remove all rights from Bob

    rep = await group_update_roles(alice_backend_sock, GROUP_ID, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await group_poll(bob_backend_sock, GROUP_ID, 2)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_vlob_group_updated_event(backend, alice_backend_sock, alice, alice2):
    # Not listened events
    await backend.vlob.create(alice.organization_id, alice.device_id, VLOB_ID, GROUP_ID, NOW, b"v1")

    # Start listening events
    await events_subscribe(alice_backend_sock, vlob_group_updated=[GROUP_ID, OTHER_GROUP_ID])

    # Good events
    with backend.event_bus.listen() as spy:

        await backend.vlob.create(
            alice.organization_id, alice2.device_id, OTHER_VLOB_ID, OTHER_GROUP_ID, NOW, b"v1"
        )
        await backend.vlob.update(alice.organization_id, alice2.device_id, VLOB_ID, 2, NOW, b"v2")
        await backend.vlob.update(alice.organization_id, alice2.device_id, VLOB_ID, 3, NOW, b"v3")

        with trio.fail_after(1):
            # No guarantees those events occur before the commands' return
            # On top of that, other `vlob_group.updated` has been triggered
            # before us (i.g. during alice user vlob creation). In case of slow
            # database those events could pop only now, hence shadowing the ones
            # we are waiting for. To avoid this we have to specify event params.
            await spy.wait_multiple(
                [
                    (
                        "vlob_group.updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "id": OTHER_GROUP_ID,
                            "checkpoint": 1,
                            "src_id": OTHER_VLOB_ID,
                            "src_version": 1,
                        },
                    ),
                    (
                        "vlob_group.updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "id": GROUP_ID,
                            "checkpoint": 2,
                            "src_id": VLOB_ID,
                            "src_version": 2,
                        },
                    ),
                    (
                        "vlob_group.updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "id": GROUP_ID,
                            "checkpoint": 3,
                            "src_id": VLOB_ID,
                            "src_version": 3,
                        },
                    ),
                ]
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
            "event": "vlob_group.updated",
            "id": OTHER_GROUP_ID,
            "checkpoint": 1,
            "src_id": OTHER_VLOB_ID,
            "src_version": 1,
        },
        {
            "status": "ok",
            "event": "vlob_group.updated",
            "id": GROUP_ID,
            "checkpoint": 2,
            "src_id": VLOB_ID,
            "src_version": 2,
        },
        {
            "status": "ok",
            "event": "vlob_group.updated",
            "id": GROUP_ID,
            "checkpoint": 3,
            "src_id": VLOB_ID,
            "src_version": 3,
        },
        {"status": "no_events"},
    ]

    # Ignore self events

    await backend.vlob.update(alice.organization_id, alice.device_id, VLOB_ID, 4, NOW, b"v4")

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}

    # Group id not subscribed to

    await backend.vlob.create(
        alice.organization_id,
        alice2.device_id,
        YET_ANOTHER_VLOB_ID,
        YET_ANOTHER_GROUP_ID,
        NOW,
        b"v1",
    )
    await backend.vlob.update(
        alice.organization_id, alice2.device_id, YET_ANOTHER_VLOB_ID, 2, NOW, b"v2"
    )

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


# TODO: add per-role test on vlob_group_poll
