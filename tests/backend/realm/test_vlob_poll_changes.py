# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from libparsec.types import DateTime

from parsec.api.data import RealmRoleCertificateContent
from parsec.api.protocol import VlobID, RealmID, RealmRole

from tests.backend.common import realm_update_roles, vlob_update, vlob_poll_changes


NOW = DateTime(2000, 1, 3)
VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
OTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000002")
YET_ANOTHER_VLOB_ID = VlobID.from_hex("00000000000000000000000000000003")
UNKNOWN_REALM_ID = RealmID.from_hex("0000000000000000000000000000000F")


@pytest.fixture
def realm_generate_certif_and_update_roles_or_fail(next_timestamp):
    async def _realm_generate_certif_and_update_roles_or_fail(ws, author, realm_id, user_id, role):
        certif = RealmRoleCertificateContent(
            author=author.device_id,
            timestamp=next_timestamp(),
            realm_id=realm_id,
            user_id=user_id,
            role=role,
        ).dump_and_sign(author.signing_key)
        return await realm_update_roles(ws, certif, check_rep=False)

    return _realm_generate_certif_and_update_roles_or_fail


@pytest.mark.trio
async def test_realm_updated_by_vlob(backend, alice, alice_ws, realm):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
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
        rep = await vlob_poll_changes(alice_ws, realm, last_checkpoint)
        assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}


@pytest.mark.trio
async def test_vlob_poll_changes_checkpoint_up_to_date(backend, alice, alice_ws, realm):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
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

    rep = await vlob_poll_changes(alice_ws, realm, 2)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {}}


@pytest.mark.trio
async def test_vlob_poll_changes_not_found(alice_ws):
    rep = await vlob_poll_changes(alice_ws, UNKNOWN_REALM_ID, 0)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `0000000000000000000000000000000f` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_poll_changes(
    backend,
    alice,
    bob,
    alice_ws,
    bob_ws,
    realm,
    next_timestamp,
    realm_generate_certif_and_update_roles_or_fail,
):
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VLOB_ID,
        timestamp=NOW,
        blob=b"v1",
    )

    # At first only Alice is allowed

    rep = await vlob_poll_changes(bob_ws, realm, 2)
    assert rep == {"status": "not_allowed"}

    # Add Bob with read&write rights

    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR
    )
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_ws, VLOB_ID, 2, b"v2", next_timestamp())
    assert rep == {"status": "ok"}

    rep = await vlob_poll_changes(bob_ws, realm, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Change Bob with read only right

    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.READER
    )
    assert rep == {"status": "ok"}

    rep = await vlob_update(bob_ws, VLOB_ID, 3, b"v3", next_timestamp(), check_rep=False)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_poll_changes(bob_ws, realm, 1)
    assert rep == {"status": "ok", "current_checkpoint": 2, "changes": {VLOB_ID: 2}}

    # Finally remove all rights from Bob

    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    rep = await vlob_poll_changes(bob_ws, realm, 2)
    assert rep == {"status": "not_allowed"}

    rep = await vlob_update(bob_ws, VLOB_ID, 3, b"v3", next_timestamp(), check_rep=False)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_vlob_poll_changes_during_maintenance(backend, alice, alice_ws, realm):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        DateTime(2000, 1, 2),
    )

    # It's ok to poll changes while the workspace is being reencrypted
    rep = await vlob_poll_changes(alice_ws, realm, 1)
    assert rep["status"] == "ok"
