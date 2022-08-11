# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from pendulum import datetime

from parsec.api.protocol import (
    packb,
    RealmID,
    VlobID,
    RealmRole,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
    vlob_list_versions_serializer,
)
from parsec.backend.realm import RealmGrantedRole
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET

from tests.common import freeze_time
from tests.backend.common import vlob_create, vlob_update, vlob_read, vlob_list_versions
from tests.backend.realm.test_roles import realm_generate_certif_and_update_roles_or_fail

# Fixture
realm_generate_certif_and_update_roles_or_fail

VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")


@pytest.mark.trio
async def test_create_and_read(alice, alice_ws, alice2_ws, realm):
    blob = b"Initial commit."
    with freeze_time("2000-01-03"):
        await vlob_create(alice_ws, realm, VLOB_ID, blob)

    rep = await vlob_read(alice2_ws, VLOB_ID)
    assert rep == {
        "status": "ok",
        "version": 1,
        "blob": blob,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 3),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_create_bad_timestamp(alice_ws, realm):
    blob = b"Initial commit."
    d1 = datetime(2000, 1, 1)
    with freeze_time(d1):
        d2 = d1.add(seconds=3600)
        rep = await vlob_create(alice_ws, realm, VLOB_ID, blob, timestamp=d2, check_rep=False)
    assert rep == {
        "status": "bad_timestamp",
        "backend_timestamp": d1,
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
        "client_timestamp": d2,
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"blob": b"...", "bad_field": "foo"},
        {"blob": 42},
        {"blob": None},
        {"id": "<not an uuid>", "blob": b"..."},
        {"id": 42, "blob": b"..."},
    ],
)
@pytest.mark.trio
async def test_create_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "vlob_create", **bad_msg}))
    raw_rep = await alice_ws.receive()
    rep = vlob_create_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_create_but_already_exists(alice_ws, realm):
    blob = b"Initial commit."

    await vlob_create(alice_ws, realm, VLOB_ID, blob)

    rep = await vlob_create(alice_ws, realm, VLOB_ID, blob, check_rep=False)
    assert rep["status"] == "already_exists"


@pytest.mark.trio
async def test_create_but_unknown_realm(alice_ws):
    bad_realm_id = RealmID.new()
    blob = b"Initial commit."

    rep = await vlob_create(alice_ws, bad_realm_id, VLOB_ID, blob, check_rep=False)
    assert rep["status"] == "not_allowed"


@pytest.mark.trio
async def test_create_check_access_rights(backend, alice, bob, bob_ws, realm, next_timestamp):
    vlob_id = VlobID.new()

    # User not part of the realm
    rep = await vlob_create(
        bob_ws, realm, vlob_id, b"Initial version.", next_timestamp(), check_rep=False
    )
    assert rep == {"status": "not_allowed"}

    # User part of the realm with various role
    for role, access_granted in [
        (RealmRole.READER, False),
        (RealmRole.CONTRIBUTOR, True),
        (RealmRole.MANAGER, True),
        (RealmRole.OWNER, True),
    ]:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"dummy",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        vlob_id = VlobID.new()
        rep = await vlob_create(
            bob_ws, realm, vlob_id, b"Initial version.", next_timestamp(), check_rep=False
        )
        if access_granted:
            assert rep == {"status": "ok"}

        else:
            assert rep == {"status": "not_allowed"}

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await vlob_create(
        bob_ws, realm, vlob_id, b"Initial version.", next_timestamp(), check_rep=False
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_read_not_found(alice_ws):
    rep = await vlob_read(alice_ws, VLOB_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000000000000000000000000001` doesn't exist",
    }


@pytest.mark.trio
async def test_read_ok(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0])
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:2",
        "version": 2,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 3),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_ok_v1(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], version=1)
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:1",
        "version": 1,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 2, 1),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_ok_timestamp_after_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=datetime(2000, 1, 4))
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:2",
        "version": 2,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 3),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_ok_timestamp_is_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=datetime(2000, 1, 3))
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:2",
        "version": 2,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 3),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_ok_timestamp_between_v1_and_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=datetime(2000, 1, 2, 10))
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:1",
        "version": 1,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 2, 1),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_ok_timestamp_is_v1(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=datetime(2000, 1, 2, 1))
    assert rep == {
        "status": "ok",
        "blob": b"r:A b:1 v:1",
        "version": 1,
        "author": alice.device_id,
        "timestamp": datetime(2000, 1, 2, 1),
        "author_last_role_granted_on": datetime(2000, 1, 2),
    }


@pytest.mark.trio
async def test_read_before_v1(alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=datetime(2000, 1, 1))
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_read_check_access_rights(backend, alice, bob, bob_ws, realm, vlobs, next_timestamp):
    # Not part of the realm
    rep = await vlob_read(bob_ws, vlobs[0])
    assert rep == {"status": "not_allowed"}

    for role in RealmRole:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"dummy",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        rep = await vlob_read(bob_ws, vlobs[0])
        assert rep["status"] == "ok"

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await vlob_read(bob_ws, vlobs[0])
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_read_other_organization(backend_asgi_app, ws_from_other_organization_factory, vlobs):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_read(sock, vlobs[0])
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `10000000000000000000000000000000` doesn't exist",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(VLOB_ID), "bad_field": "foo"},
        {"id": "<not an uuid>"},
        {"id": str(VLOB_ID)},  # TODO: really bad ?
        {"id": 42},
        {"id": None},
        {"id": str(VLOB_ID), "version": 0},
        {"id": str(VLOB_ID), "version": "foo"},
        {},
    ],
)
@pytest.mark.trio
async def test_read_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "vlob_read", **bad_msg}))
    raw_rep = await alice_ws.receive()
    rep = vlob_read_serializer.rep_loads(raw_rep)
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_read_bad_version(alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], version=3)
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_update_ok(alice_ws, vlobs):
    await vlob_update(alice_ws, vlobs[0], version=3, blob=b"Next version.")


@pytest.mark.trio
async def test_update_bad_timestamp(alice_ws, vlobs):
    blob = b"Initial commit."
    d1 = datetime(2000, 1, 1)
    with freeze_time(d1):
        d2 = d1.add(seconds=3600)
        rep = await vlob_update(
            alice_ws, vlobs[0], version=3, blob=blob, timestamp=d2, check_rep=False
        )
    assert rep == {
        "status": "bad_timestamp",
        "backend_timestamp": d1,
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
        "client_timestamp": d2,
    }


@pytest.mark.trio
async def test_update_not_found(alice_ws):
    rep = await vlob_update(alice_ws, VLOB_ID, version=2, blob=b"Next version.", check_rep=False)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000000000000000000000000001` doesn't exist",
    }


@pytest.mark.trio
async def test_update_check_access_rights(
    backend, alice, bob, bob_ws, realm, vlobs, next_timestamp
):
    # User not part of the realm
    rep = await vlob_update(
        bob_ws,
        vlobs[0],
        version=3,
        blob=b"Next version.",
        timestamp=next_timestamp(),
        check_rep=False,
    )
    assert rep == {"status": "not_allowed"}

    # User part of the realm with various role
    next_version = 3
    for role, access_granted in [
        (RealmRole.READER, False),
        (RealmRole.CONTRIBUTOR, True),
        (RealmRole.MANAGER, True),
        (RealmRole.OWNER, True),
    ]:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"dummy",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        rep = await vlob_update(
            bob_ws,
            vlobs[0],
            version=next_version,
            blob=b"Next version.",
            timestamp=next_timestamp(),
            check_rep=False,
        )
        if access_granted:
            assert rep == {"status": "ok"}
            next_version += 1

        else:
            assert rep == {"status": "not_allowed"}

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await vlob_update(
        bob_ws,
        vlobs[0],
        version=next_version,
        blob=b"Next version.",
        timestamp=next_timestamp(),
        check_rep=False,
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_update_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, vlobs
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_update(sock, vlobs[0], version=3, blob=b"Next version.", check_rep=False)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `10000000000000000000000000000000` doesn't exist",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(VLOB_ID), "version": 42, "blob": b"...", "bad_field": "foo"},
        {"id": str(VLOB_ID), "version": 42, "blob": None},
        {"id": str(VLOB_ID), "version": 42, "blob": 42},
        {"id": str(VLOB_ID), "version": 42},
        {"id": str(VLOB_ID), "version": None, "blob": b"..."},
        {"id": str(VLOB_ID), "version": -1, "blob": b"..."},
        {"id": 42, "version": 42, "blob": b"..."},
        {"id": None, "version": 42, "blob": b"..."},
        {"version": 42, "blob": b"..."},
        {},
    ],
)
@pytest.mark.trio
async def test_update_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "vlob_update", **bad_msg}))
    raw_rep = await alice_ws.receive()
    rep = vlob_update_serializer.rep_loads(raw_rep)
    # Id and version are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(alice_ws, vlobs):
    rep = await vlob_update(alice_ws, vlobs[0], version=4, blob=b"Next version.", check_rep=False)
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_bad_encryption_revision(alice_ws, realm, vlobs):
    rep = await vlob_create(
        alice_ws, realm, VLOB_ID, blob=b"First version.", encryption_revision=42, check_rep=False
    )
    assert rep == {"status": "bad_encryption_revision"}

    rep = await vlob_read(alice_ws, vlobs[0], encryption_revision=42)
    assert rep == {"status": "bad_encryption_revision"}

    rep = await vlob_update(
        alice_ws,
        vlobs[0],
        version=3,
        blob=b"Next version.",
        encryption_revision=42,
        check_rep=False,
    )
    assert rep == {"status": "bad_encryption_revision"}


@pytest.mark.trio
async def test_list_versions_ok(alice, alice_ws, vlobs):
    rep = await vlob_list_versions(alice_ws, vlobs[0])
    assert rep == {
        "status": "ok",
        "versions": {
            1: (datetime(2000, 1, 2, 1, 0, 0), alice.device_id),
            2: (datetime(2000, 1, 3, 0, 0, 0), alice.device_id),
        },
    }


@pytest.mark.trio
async def test_list_versions_not_found(alice_ws):
    rep = await vlob_list_versions(alice_ws, VLOB_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000000000000000000000000001` doesn't exist",
    }


@pytest.mark.trio
async def test_list_versions_check_access_rights(
    backend, alice, bob, bob_ws, realm, vlobs, next_timestamp
):
    # Not part of the realm
    rep = await vlob_list_versions(bob_ws, vlobs[0])
    assert rep == {"status": "not_allowed"}

    for role in RealmRole:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"dummy",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        rep = await vlob_list_versions(bob_ws, vlobs[0])
        assert rep["status"] == "ok"

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await vlob_list_versions(bob_ws, vlobs[0])
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_list_versions_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, vlobs
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_list_versions(sock, vlobs[0])
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `10000000000000000000000000000000` doesn't exist",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(VLOB_ID), "bad_field": "foo"},
        {"id": "<not an uuid>"},
        {"id": str(VLOB_ID)},  # TODO: really bad ?
        {"id": 42},
        {"id": None},
        {"id": str(VLOB_ID), "version": 1},
        {},
    ],
)
@pytest.mark.trio
async def test_list_versions_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "vlob_list_versions", **bad_msg}))
    raw_rep = await alice_ws.receive()
    rep = vlob_list_versions_serializer.rep_loads(raw_rep)
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_access_during_maintenance(backend, alice, alice_ws, realm, vlobs):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        datetime(2000, 1, 2),
    )

    rep = await vlob_create(
        alice_ws, realm, VLOB_ID, blob=b"First version.", encryption_revision=2, check_rep=False
    )
    assert rep == {"status": "in_maintenance"}

    rep = await vlob_read(alice_ws, vlobs[0], encryption_revision=2)
    assert rep == {"status": "in_maintenance"}

    rep = await vlob_update(
        alice_ws, vlobs[0], version=3, blob=b"Next version.", encryption_revision=2, check_rep=False
    )
    assert rep == {"status": "in_maintenance"}


@pytest.mark.trio
async def test_vlob_updates_causality_checks(
    backend,
    alice,
    bob,
    adam,
    alice_ws,
    bob_ws,
    realm,
    realm_generate_certif_and_update_roles_or_fail,
    next_timestamp,
):
    # Use this timestamp as reference
    ref = next_timestamp()

    # Grant a role to bob
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER, ref
    )
    assert rep == {"status": "ok"}

    # Now bob writes to the corresponding realm with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await vlob_create(
            bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=timestamp, check_rep=False
        )
        assert rep == {"status": "require_greater_timestamp", "strictly_greater_than": ref}

    # Advance ref
    ref = ref.add(seconds=10)

    # Bob successfuly write version 1
    rep = await vlob_create(
        bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=ref, check_rep=False
    )
    assert rep == {"status": "ok"}

    # Now bob writes to the corresponding vlob with a lower timestamp: this should fail
    rep = await vlob_update(
        bob_ws,
        VLOB_ID,
        version=2,
        blob=b"ciphered",
        timestamp=ref.subtract(seconds=1),
        check_rep=False,
    )
    assert rep == {"status": "require_greater_timestamp", "strictly_greater_than": ref}
