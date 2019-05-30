# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
from pendulum import Pendulum

from tests.backend.realm.conftest import vlob_create, vlob_update, vlob_group_check


@pytest.mark.trio
async def test_group_check(bob_backend_sock, alice_backend_sock, vlobs):
    unknown_vlob_id = UUID("0000000000000000000000000000000A")
    placeholder_vlob_id = UUID("0000000000000000000000000000000B")
    bob_vlob_id = UUID("0000000000000000000000000000000C")
    bob_REALM_ID = UUID("0000000000000000000000000000000D")

    await vlob_create(bob_backend_sock, bob_REALM_ID, bob_vlob_id, b"")
    await vlob_update(bob_backend_sock, bob_vlob_id, 2, b"")

    rep = await vlob_group_check(
        alice_backend_sock,
        [
            # Ignore vlob with no read access
            {"vlob_id": bob_vlob_id, "version": 1},
            # Ignore unknown id
            {"vlob_id": unknown_vlob_id, "version": 1},
            # Version 0 is accepted
            {"vlob_id": placeholder_vlob_id, "version": 0},
            {"vlob_id": vlobs[0], "version": 1},
            {"vlob_id": vlobs[1], "version": 1},
        ],
    )
    assert rep == {
        "status": "ok",
        "changed": [
            {"vlob_id": placeholder_vlob_id, "version": 0},
            {"vlob_id": vlobs[0], "version": 2},
        ],
    }


@pytest.mark.trio
async def test_group_check_other_organization(backend, sock_from_other_organization_factory, vlobs):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await vlob_group_check(sock, [{"vlob_id": vlobs[0], "version": 1}])
        assert rep == {"status": "ok", "changed": []}


@pytest.mark.trio
async def test_group_check_during_maintenance(backend, alice, alice_backend_sock, realm, vlobs):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        Pendulum(2000, 1, 2),
    )

    # Realm under maintenance are simply skipped
    rep = await vlob_group_check(alice_backend_sock, [{"vlob_id": vlobs[0], "version": 1}])
    assert rep == {"changed": [], "status": "ok"}
