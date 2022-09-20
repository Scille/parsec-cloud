# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    EventsListenRepNoEvents,
    EventsListenRepOkVlobsUpdated,
    EventsListenRepOkRealmRolesUpdated,
    EventsListenRepOk,
)
from parsec.api.protocol import VlobID, RealmID, RealmRole
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.backend_events import BackendEvent

from tests.backend.common import events_subscribe, events_listen_nowait


NOW = DateTime(2000, 1, 3)
VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
OTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000003")
REALM_ID = RealmID.from_hex("0000000000000000000000000000000A")


@pytest.mark.trio
async def test_vlobs_updated_event_ok(backend, alice_ws, alice, alice2, realm, other_realm):
    # Not listened events
    with backend.event_bus.listen() as spy:
        await backend.vlob.create(
            organization_id=alice.organization_id,
            author=alice.device_id,
            realm_id=realm,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            timestamp=NOW,
            blob=b"v1",
        )
        await spy.wait_with_timeout(BackendEvent.REALM_VLOBS_UPDATED)

    # Start listening events
    await events_subscribe(alice_ws)

    # Good events
    with backend.event_bus.listen() as spy:

        await backend.vlob.create(
            organization_id=alice.organization_id,
            author=alice2.device_id,
            realm_id=other_realm,
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

        # No guarantees those events occur before the commands' return
        # On top of that, other `realm.vlobs_updated` has been triggered
        # before us (i.g. during alice user vlob creation). In case of slow
        # database those events could pop only now, hence shadowing the ones
        # we are waiting for. To avoid this we have to specify event params.
        await spy.wait_multiple_with_timeout(
            [
                EventsListenRepOkVlobsUpdated(other_realm, 1, OTHER_VLOB_ID, 1),
                EventsListenRepOkVlobsUpdated(realm, 2, VLOB_ID, 2),
                EventsListenRepOkVlobsUpdated(realm, 3, VLOB_ID, 3),
            ]
        )

    reps = [
        await events_listen_nowait(alice_ws),
        await events_listen_nowait(alice_ws),
        await events_listen_nowait(alice_ws),
        await events_listen_nowait(alice_ws),
    ]
    print([type(e) for e in reps])
    assert reps == [
        EventsListenRepOk(EventsListenRepOkVlobsUpdated(other_realm, 1, OTHER_VLOB_ID, 1)),
        EventsListenRepOk(EventsListenRepOkVlobsUpdated(realm, 2, VLOB_ID, 2)),
        EventsListenRepOk(EventsListenRepOkVlobsUpdated(realm, 3, VLOB_ID, 3)),
        EventsListenRepNoEvents(),
    ]


@pytest.mark.trio
async def test_vlobs_updated_event_handle_self_events(backend, alice_ws, alice, realm):
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:

        await backend.vlob.create(
            organization_id=alice.organization_id,
            author=alice.device_id,
            realm_id=realm,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            timestamp=NOW,
            blob=b"v1",
        )

        await backend.vlob.create(
            organization_id=alice.organization_id,
            author=alice.device_id,
            realm_id=realm,
            encryption_revision=1,
            vlob_id=OTHER_VLOB_ID,
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

        # Wait for events to be processed by the backend
        await spy.wait_multiple_with_timeout(
            [
                BackendEvent.REALM_VLOBS_UPDATED,
                BackendEvent.REALM_VLOBS_UPDATED,
                BackendEvent.REALM_VLOBS_UPDATED,
            ]
        )

    # Self-events should have been ignored
    rep = await events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
async def test_vlobs_updated_event_not_participant(backend, alice_ws, bob, bob_realm):
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:

        await backend.vlob.create(
            organization_id=bob.organization_id,
            author=bob.device_id,
            realm_id=bob_realm,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            timestamp=NOW,
            blob=b"v1",
        )
        await backend.vlob.update(
            organization_id=bob.organization_id,
            author=bob.device_id,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            version=2,
            timestamp=NOW,
            blob=b"v2",
        )

        # Wait for events to be processed by the backend
        await spy.wait_multiple_with_timeout(
            [BackendEvent.REALM_VLOBS_UPDATED, BackendEvent.REALM_VLOBS_UPDATED]
        )

    rep = await events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
@pytest.mark.parametrize("realm_created_by_self", (True, False))
async def test_vlobs_updated_event_realm_created_after_subscribe(
    backend, alice_ws, alice, alice2, realm_created_by_self
):
    realm_id = RealmID.from_hex("0000000000000000000000000000000A")
    await events_subscribe(alice_ws)

    # New realm, should get events anyway
    with backend.event_bus.listen() as spy:
        realm_creator = alice if realm_created_by_self else alice2
        # Create the realm
        await backend.realm.create(
            organization_id=realm_creator.organization_id,
            self_granted_role=RealmGrantedRole(
                realm_id=realm_id,
                user_id=realm_creator.user_id,
                certificate=b"<dummy>",
                role=RealmRole.OWNER,
                granted_by=realm_creator.device_id,
                granted_on=DateTime(2000, 1, 2),
            ),
        )
        # Create vlob in realm
        await backend.vlob.create(
            organization_id=realm_creator.organization_id,
            author=realm_creator.device_id,
            realm_id=realm_id,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            timestamp=NOW,
            blob=b"v1",
        )
        # Update vlob in realm
        await backend.vlob.update(
            organization_id=alice2.organization_id,
            author=alice2.device_id,
            encryption_revision=1,
            vlob_id=VLOB_ID,
            version=2,
            timestamp=NOW,
            blob=b"v2",
        )

        # Wait for events to be processed by the backend
        await spy.wait_multiple_with_timeout(
            [
                BackendEvent.REALM_ROLES_UPDATED,
                BackendEvent.REALM_VLOBS_UPDATED,
                BackendEvent.REALM_VLOBS_UPDATED,
            ]
        )

    # Realm access granted
    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkRealmRolesUpdated(realm_id, RealmRole.OWNER)

    # Create vlob in realm event
    if not realm_created_by_self:
        rep = await events_listen_nowait(alice_ws)
        assert rep == EventsListenRepOkVlobsUpdated(realm_id, 1, VLOB_ID, 1)

    # Update vlob in realm event
    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkVlobsUpdated(realm_id, 2, VLOB_ID, 2)

    rep = await events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)
