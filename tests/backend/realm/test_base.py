# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import now as pendulum_now

from parsec.api.protocole import RealmRole
from parsec.crypto import build_realm_role_certificate

from tests.backend.realm.conftest import realm_status, realm_update_roles


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
