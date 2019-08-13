# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import pendulum
from uuid import UUID

from parsec.api.protocol import RealmRole
from parsec.crypto import build_realm_self_role_certificate, build_realm_role_certificate

from tests.backend.test_events import events_subscribe, events_listen_nowait
from tests.backend.realm.conftest import realm_create, realm_update_roles


@pytest.mark.trio
async def test_realm_create(backend, alice, alice_backend_sock):
    await events_subscribe(alice_backend_sock)

    realm_id = UUID("C0000000000000000000000000000000")
    certif = build_realm_self_role_certificate(
        alice.device_id, alice.signing_key, realm_id, pendulum.now()
    )
    with backend.event_bus.listen() as spy:
        await realm_create(alice_backend_sock, certif)
        await spy.wait_with_timeout("realm.roles_updated")


@pytest.mark.trio
async def test_roles_updated_for_participant(
    backend, alice, bob, alice_backend_sock, bob_backend_sock, realm
):
    async def _update_role_and_check_events(role):

        with backend.event_bus.listen() as spy:
            certif = build_realm_role_certificate(
                alice.device_id, alice.signing_key, realm, bob.user_id, role, pendulum.now()
            )
            rep = await realm_update_roles(alice_backend_sock, certif, check_rep=False)
            assert rep == {"status": "ok"}

            await spy.wait_with_timeout(
                "realm.roles_updated",
                {
                    "organization_id": alice.organization_id,
                    "author": alice.device_id,
                    "realm_id": realm,
                    "user": bob.user_id,
                    "role": role,
                },
            )

        # Check events propagated to the client
        rep = await events_listen_nowait(bob_backend_sock)
        assert rep == {
            "status": "ok",
            "event": "realm.roles_updated",
            "realm_id": realm,
            "role": role,
        }
        rep = await events_listen_nowait(bob_backend_sock)
        assert rep == {"status": "no_events"}

    # 0) Init event listening on the socket
    await events_subscribe(bob_backend_sock)

    # 1) New participant
    await _update_role_and_check_events(RealmRole.MANAGER)

    # 2) Change participant role
    await _update_role_and_check_events(RealmRole.READER)

    # 3) Stop sharing with participant
    await _update_role_and_check_events(None)
