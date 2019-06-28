# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
from pendulum import Pendulum, now as pendulum_now

from parsec.api.protocole import RealmRole
from parsec.crypto import build_realm_role_certificate

from tests.backend.realm.conftest import (
    realm_status,
    realm_get_roles,
    realm_update_roles,
    vlob_poll_changes,
)


@pytest.mark.trio
async def test_status(backend, bob_backend_sock, alice_backend_sock, alice, bob, realm):
    rep = await realm_status(alice_backend_sock, realm)
    assert rep == {
        "status": "ok",
        "in_maintenance": False,
        "maintenance_type": None,
        "maintenance_started_by": None,
        "maintenance_started_on": None,
        "encryption_revision": 1,
    }
    # Cheap test on no access
    rep = await realm_status(bob_backend_sock, realm)
    assert rep == {"status": "not_allowed"}
    # Also test lesser role have access
    await realm_update_roles(
        alice_backend_sock,
        build_realm_role_certificate(
            alice.device_id, alice.signing_key, realm, bob.user_id, RealmRole.READER, pendulum_now()
        ),
    )
    rep = await realm_status(bob_backend_sock, realm)
    assert rep == {
        "status": "ok",
        "in_maintenance": False,
        "maintenance_type": None,
        "maintenance_started_by": None,
        "maintenance_started_on": None,
        "encryption_revision": 1,
    }


@pytest.mark.trio
@pytest.mark.xfail(reason="no longer true, TODO: remove meeee")
async def test_realm_lazy_created_by_new_vlob(backend, alice, alice_backend_sock):
    NOW = Pendulum(2000, 1, 1)
    VLOB_ID = UUID("00000000000000000000000000000001")
    REALM_ID = UUID("0000000000000000000000000000000A")

    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=REALM_ID,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )

    rep = await vlob_poll_changes(alice_backend_sock, REALM_ID, 0)
    assert rep == {"status": "ok", "current_checkpoint": 1, "changes": {VLOB_ID: 1}}

    # Make sure author gets OWNER role

    rep = await realm_get_roles(alice_backend_sock, REALM_ID)
    assert rep == {"status": "ok", "users": {"alice": RealmRole.OWNER}}
