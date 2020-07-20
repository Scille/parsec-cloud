# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID, uuid4

from tests.backend.common import realm_stats
from tests.backend.common import vlob_create, block_create

REALM_ID_FAKE = UUID("00000000-0000-0000-0000-000000000001")


@pytest.mark.trio
async def test_realm_stats_ok(backend, alice_backend_sock, realm):

    # Create new data
    await block_create(alice_backend_sock, realm_id=realm, block_id=uuid4(), block=b"1234")
    rep = await realm_stats(alice_backend_sock, realm_id=realm)
    assert rep == {"status": "ok", "blocks_size": 4, "vlobs_size": 0}

    # Create new metadata
    await vlob_create(alice_backend_sock, realm_id=realm, vlob_id=uuid4(), blob=b"1234")
    rep = await realm_stats(alice_backend_sock, realm_id=realm)
    assert rep == {"status": "ok", "blocks_size": 4, "vlobs_size": 4}


@pytest.mark.trio
async def test_realm_stats_ko(
    backend, alice_backend_sock, bob_backend_sock, sock_from_other_organization_factory, realm
):
    # test with no access to the realm
    rep = await realm_stats(bob_backend_sock, realm_id=realm)
    assert rep == {"status": "not_allowed"}

    # test with non existant realm
    rep = await realm_stats(alice_backend_sock, realm_id=REALM_ID_FAKE)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `00000000-0000-0000-0000-000000000001` doesn't exist",
    }

    # test with no access to the realm
    rep = await realm_stats(bob_backend_sock, realm_id=realm)
    assert rep == {"status": "not_allowed"}

    # test with device_id but wrong organization
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await realm_stats(sock, realm_id=realm)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `a0000000-0000-0000-0000-000000000000` doesn't exist",
    }
