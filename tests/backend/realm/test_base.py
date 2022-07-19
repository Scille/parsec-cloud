# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from libparsec.types import DateTime

from parsec.api.data import RealmRoleCertificateContent
from parsec.api.protocol import RealmRole

from tests.backend.common import realm_status, realm_update_roles


@pytest.mark.trio
async def test_status(bob_ws, alice_ws, alice, bob, realm):
    rep = await realm_status(alice_ws, realm)
    assert rep == {
        "status": "ok",
        "in_maintenance": False,
        "maintenance_type": None,
        "maintenance_started_by": None,
        "maintenance_started_on": None,
        "encryption_revision": 1,
    }
    # Cheap test on no access
    rep = await realm_status(bob_ws, realm)
    assert rep == {"status": "not_allowed"}
    # Also test lesser role have access
    await realm_update_roles(
        alice_ws,
        RealmRoleCertificateContent(
            author=alice.device_id,
            timestamp=DateTime.now(),
            realm_id=realm,
            user_id=bob.user_id,
            role=RealmRole.READER,
        ).dump_and_sign(alice.signing_key),
    )
    rep = await realm_status(bob_ws, realm)
    assert rep == {
        "status": "ok",
        "in_maintenance": False,
        "maintenance_type": None,
        "maintenance_started_by": None,
        "maintenance_started_on": None,
        "encryption_revision": 1,
    }
