# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BackendEventRealmRolesUpdated,
    BackendEventRealmVlobsUpdated,
    DateTime,
)
from parsec.api.protocol import (
    ApiV2V3_APIEventRealmRolesUpdated,
    ApiV2V3_APIEventRealmVlobsUpdated,
    ApiV2V3_EventsListenRepNoEvents,
    ApiV2V3_EventsListenRepOk,
    RealmRole,
    VlobID,
)
from parsec.backend.realm import RealmGrantedRole
from tests.backend.common import apiv2v3_events_listen_nowait, apiv2v3_events_subscribe

NOW = DateTime(2000, 1, 3)
VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
OTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000003")
REALM_ID = VlobID.from_hex("0000000000000000000000000000000A")


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
        await spy.wait_with_timeout(BackendEventRealmVlobsUpdated)

    # Start listening events
    await apiv2v3_events_subscribe(alice_ws)

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
                BackendEventRealmVlobsUpdated(
                    organization_id=alice.organization_id,
                    author=alice2.device_id,
                    realm_id=other_realm,
                    checkpoint=1,
                    src_id=OTHER_VLOB_ID,
                    src_version=1,
                ),
                BackendEventRealmVlobsUpdated(
                    organization_id=alice.organization_id,
                    author=alice2.device_id,
                    realm_id=realm,
                    checkpoint=2,
                    src_id=VLOB_ID,
                    src_version=2,
                ),
                BackendEventRealmVlobsUpdated(
                    organization_id=alice.organization_id,
                    author=alice2.device_id,
                    realm_id=realm,
                    checkpoint=3,
                    src_id=VLOB_ID,
                    src_version=3,
                ),
            ]
        )

    reps = [
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
        await apiv2v3_events_listen_nowait(alice_ws),
    ]

    assert reps == [
        ApiV2V3_EventsListenRepOk(
            ApiV2V3_APIEventRealmVlobsUpdated(other_realm, 1, OTHER_VLOB_ID, 1)
        ),
        ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventRealmVlobsUpdated(realm, 2, VLOB_ID, 2)),
        ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventRealmVlobsUpdated(realm, 3, VLOB_ID, 3)),
        ApiV2V3_EventsListenRepNoEvents(),
    ]


@pytest.mark.trio
async def test_vlobs_updated_event_handle_self_events(backend, alice_ws, alice, realm):
    await apiv2v3_events_subscribe(alice_ws)

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
                BackendEventRealmVlobsUpdated,
                BackendEventRealmVlobsUpdated,
                BackendEventRealmVlobsUpdated,
            ]
        )

    # Self-events should have been ignored
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert isinstance(rep, ApiV2V3_EventsListenRepNoEvents)


@pytest.mark.trio
async def test_vlobs_updated_event_not_participant(backend, alice_ws, bob, bob_realm):
    await apiv2v3_events_subscribe(alice_ws)

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
            [BackendEventRealmVlobsUpdated, BackendEventRealmVlobsUpdated]
        )

    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert isinstance(rep, ApiV2V3_EventsListenRepNoEvents)


@pytest.mark.trio
@pytest.mark.parametrize("realm_created_by_self", (True, False))
async def test_vlobs_updated_event_realm_created_after_subscribe(
    backend, alice_ws, alice, alice2, realm_created_by_self
):
    realm_id = VlobID.from_hex("0000000000000000000000000000000A")
    await apiv2v3_events_subscribe(alice_ws)

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
                BackendEventRealmRolesUpdated,
                BackendEventRealmVlobsUpdated,
                BackendEventRealmVlobsUpdated,
            ]
        )

    # Realm access granted
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == ApiV2V3_EventsListenRepOk(
        ApiV2V3_APIEventRealmRolesUpdated(realm_id, RealmRole.OWNER)
    )

    # Create vlob in realm event
    if not realm_created_by_self:
        rep = await apiv2v3_events_listen_nowait(alice_ws)
        assert rep == ApiV2V3_EventsListenRepOk(
            ApiV2V3_APIEventRealmVlobsUpdated(realm_id, 1, VLOB_ID, 1)
        )

    # Update vlob in realm event
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == ApiV2V3_EventsListenRepOk(
        ApiV2V3_APIEventRealmVlobsUpdated(realm_id, 2, VLOB_ID, 2)
    )

    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert isinstance(rep, ApiV2V3_EventsListenRepNoEvents)
