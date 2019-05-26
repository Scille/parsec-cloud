# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
import trio
from pendulum import Pendulum

from parsec.api.protocole import RealmRole

from tests.backend.test_events import events_subscribe, events_listen_nowait
from tests.backend.realm.conftest import realm_update_roles, vlob_update, vlob_poll_changes


NOW = Pendulum(2000, 1, 1)
VLOB_ID = UUID("00000000000000000000000000000001")
OTHER_VLOB_ID = UUID("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = UUID("00000000000000000000000000000003")
REALM_ID = UUID("0000000000000000000000000000000A")
OTHER_REALM_ID = UUID("0000000000000000000000000000000B")
YET_ANOTHER_REALM_ID = UUID("0000000000000000000000000000000C")


@pytest.mark.trio
async def test_realm_updated_by_vlob(backend, alice, alice_backend_sock):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=REALM_ID,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )
    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        version=2,
        timestamp=NOW,
        blob=b"v2",
    )

    for last_checkpoint in (0, 1):
        rep = await vlob_poll_changes(alice_backend_sock, REALM_ID, last_checkpoint)
        assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}


@pytest.mark.trio
async def test_vlob_poll_changes_checkpoint_up_to_date(backend, alice, alice_backend_sock):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=REALM_ID,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )
    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        version=2,
        timestamp=NOW,
        blob=b"v2",
    )

    rep = await vlob_poll_changes(alice_backend_sock, REALM_ID, 2)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {}}


@pytest.mark.trio
async def test_vlob_poll_changes_not_found(alice_backend_sock):
    rep = await vlob_poll_changes(alice_backend_sock, REALM_ID, 0)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_poll_changes(backend, alice, bob, alice_backend_sock, bob_backend_sock):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=REALM_ID,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )

    # At first only Alice is allowed

    rep = await vlob_poll_changes(bob_backend_sock, REALM_ID, 2)
    assert rep == {"status": "not_allowed"}

    # Add Bob with read&write rights

    rep = await realm_update_roles(alice_backend_sock, REALM_ID, bob.user_id, RealmRole.CONTRIBUTOR)
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 2, b"v2")
    assert rep == {"status": "ok"}

    rep = await vlob_poll_changes(bob_backend_sock, REALM_ID, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Change Bob with read only right

    rep = await realm_update_roles(alice_backend_sock, REALM_ID, bob.user_id, RealmRole.READER)
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_poll_changes(bob_backend_sock, REALM_ID, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Finally remove all rights from Bob

    rep = await realm_update_roles(alice_backend_sock, REALM_ID, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await vlob_poll_changes(bob_backend_sock, REALM_ID, 2)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_update(bob_backend_sock, VLOB_ID, 3, b"v3", check_rep=False)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_realm_updated_event(backend, alice_backend_sock, alice, alice2):
    # Not listened events
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=REALM_ID,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )

    # Start listening events
    await events_subscribe(alice_backend_sock, realm_vlobs_updated=[REALM_ID, OTHER_REALM_ID])

    # Good events
    with backend.event_bus.listen() as spy:

        await backend.vlob.create(
            organization_id=alice.organization_id,
            author=alice2.device_id,
            realm_id=OTHER_REALM_ID,
            encryption_revision=1,
            vlob_id=OTHER_VLOB_ID,
            timestamp=NOW,
            blob=b"v1",
        )
        await backend.vlob.update(
            organization_id=alice.organization_id,
            author=alice2.device_id,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            version=2,
            timestamp=NOW,
            blob=b"v2",
        )
        await backend.vlob.update(
            organization_id=alice.organization_id,
            author=alice2.device_id,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            version=3,
            timestamp=NOW,
            blob=b"v3",
        )

        with trio.fail_after(1):
            # No guarantees those events occur before the commands' return
            # On top of that, other `realm.vlobs_updated` has been triggered
            # before us (i.g. during alice user vlob creation). In case of slow
            # database those events could pop only now, hence shadowing the ones
            # we are waiting for. To avoid this we have to specify event params.
            await spy.wait_multiple(
                [
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": OTHER_REALM_ID,
                            "checkpoint": 1,
                            "src_id": OTHER_VLOB_ID,
                            "src_version": 1,
                        },
                    ),
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": REALM_ID,
                            "checkpoint": 2,
                            "src_id": VLOB_ID,
                            "src_version": 2,
                        },
                    ),
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": REALM_ID,
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
            "event": "realm.vlobs_updated",
            "realm_id": OTHER_REALM_ID,
            "checkpoint": 1,
            "src_id": OTHER_VLOB_ID,
            "src_version": 1,
        },
        {
            "status": "ok",
            "event": "realm.vlobs_updated",
            "realm_id": REALM_ID,
            "checkpoint": 2,
            "src_id": VLOB_ID,
            "src_version": 2,
        },
        {
            "status": "ok",
            "event": "realm.vlobs_updated",
            "realm_id": REALM_ID,
            "checkpoint": 3,
            "src_id": VLOB_ID,
            "src_version": 3,
        },
        {"status": "no_events"},
    ]

    # Ignore self events

    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        version=4,
        timestamp=NOW,
        blob=b"v4",
    )

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}

    # Realm id not subscribed to

    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice2.device_id,
        realm_id=YET_ANOTHER_REALM_ID,
        encryption_revision=1,
        vlob_id=YET_ANOTHER_VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )
    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice2.device_id,
        encryption_revision=1,
        vlob_id=YET_ANOTHER_VLOB_ID,
        version=2,
        timestamp=NOW,
        blob=b"v2",
    )

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_vlob_poll_changes_during_maintenance(backend, alice, alice_backend_sock, realm):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id, alice.device_id, realm, 2, {alice.user_id: b"whatever"}
    )

    # Realm under maintenance are simply skipped
    rep = await vlob_poll_changes(alice_backend_sock, realm, 1)
    assert rep == {"status": "in_maintenance"}
