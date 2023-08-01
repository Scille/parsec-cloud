# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BackendEventRealmRolesUpdated,
    DateTime,
)
from parsec.api.data import RealmRoleCertificate
from parsec.api.protocol import (
    ApiV2V3_APIEventRealmRolesUpdated,
    ApiV2V3_EventsListenRepNoEvents,
    ApiV2V3_EventsListenRepOk,
    RealmCreateRepOk,
    RealmID,
    RealmRole,
    RealmUpdateRolesRepOk,
)
from tests.backend.common import (
    apiv2v3_events_listen_nowait,
    apiv2v3_events_subscribe,
    realm_create,
    realm_update_roles,
)


@pytest.mark.trio
async def test_realm_create(backend, alice, alice_ws):
    await apiv2v3_events_subscribe(alice_ws)

    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    certif = RealmRoleCertificate.build_realm_root_certif(
        author=alice.device_id, timestamp=DateTime.now(), realm_id=realm_id
    ).dump_and_sign(alice.signing_key)
    with backend.event_bus.listen() as spy:
        rep = await realm_create(alice_ws, certif)
        assert isinstance(rep, RealmCreateRepOk)
        await spy.wait_with_timeout(BackendEventRealmRolesUpdated)


@pytest.mark.trio
async def test_roles_updated_for_participant(
    backend, alice, bob, alice_ws, bob_ws, realm, next_timestamp
):
    async def _update_role_and_check_events(role):
        with backend.event_bus.listen() as spy:
            certif = RealmRoleCertificate(
                author=alice.device_id,
                timestamp=next_timestamp(),
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
            ).dump_and_sign(alice.signing_key)
            rep = await realm_update_roles(alice_ws, certif, check_rep=False)
            assert isinstance(rep, RealmUpdateRolesRepOk)

            await spy.wait_with_timeout(
                BackendEventRealmRolesUpdated(
                    organization_id=alice.organization_id,
                    author=alice.device_id,
                    realm_id=realm,
                    user=bob.user_id,
                    role=role,
                )
            )

        # Check events propagated to the client
        rep = await apiv2v3_events_listen_nowait(bob_ws)
        assert rep == ApiV2V3_EventsListenRepOk(ApiV2V3_APIEventRealmRolesUpdated(realm, role))
        rep = await apiv2v3_events_listen_nowait(bob_ws)
        assert isinstance(rep, ApiV2V3_EventsListenRepNoEvents)

    # 0) Init event listening on the socket
    await apiv2v3_events_subscribe(bob_ws)

    # 1) New participant
    await _update_role_and_check_events(RealmRole.MANAGER)

    # 2) Change participant role
    await _update_role_and_check_events(RealmRole.READER)

    # 3) Stop sharing with participant
    await _update_role_and_check_events(None)
