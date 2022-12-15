# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    RealmUpdateRolesRepOk,
    VlobCreateRepAlreadyExists,
    VlobCreateRepBadEncryptionRevision,
    VlobCreateRepBadTimestamp,
    VlobCreateRepInMaintenance,
    VlobCreateRepNotAllowed,
    VlobCreateRepOk,
    VlobCreateRepRequireGreaterTimestamp,
    VlobListVersionsRepNotAllowed,
    VlobListVersionsRepNotFound,
    VlobListVersionsRepOk,
    VlobReadRepBadEncryptionRevision,
    VlobReadRepBadVersion,
    VlobReadRepInMaintenance,
    VlobReadRepNotAllowed,
    VlobReadRepNotFound,
    VlobReadRepOk,
    VlobUpdateRepBadEncryptionRevision,
    VlobUpdateRepBadTimestamp,
    VlobUpdateRepBadVersion,
    VlobUpdateRepInMaintenance,
    VlobUpdateRepNotAllowed,
    VlobUpdateRepNotFound,
    VlobUpdateRepOk,
    VlobUpdateRepRequireGreaterTimestamp,
)
from parsec.api.protocol import (
    RealmID,
    RealmRole,
    VlobID,
    packb,
    vlob_create_serializer,
    vlob_list_versions_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)
from parsec.backend.realm import RealmGrantedRole
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
from tests.backend.common import vlob_create, vlob_list_versions, vlob_read, vlob_update
from tests.backend.realm.test_roles import realm_generate_certif_and_update_roles_or_fail
from tests.common import freeze_time

# Fixture
realm_generate_certif_and_update_roles_or_fail

VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")


@pytest.mark.trio
async def test_create_and_read(alice, alice_ws, alice2_ws, realm):
    blob = b"Initial commit."
    with freeze_time("2000-01-03"):
        await vlob_create(alice_ws, realm, VLOB_ID, blob)

    rep = await vlob_read(alice2_ws, VLOB_ID)
    assert rep == VlobReadRepOk(
        version=1,
        blob=blob,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_create_bad_timestamp(alice_ws, realm):
    blob = b"Initial commit."
    d1 = DateTime(2000, 1, 1)
    with freeze_time(d1):
        d2 = d1.add(seconds=3600)
        rep = await vlob_create(alice_ws, realm, VLOB_ID, blob, timestamp=d2, check_rep=False)
    assert rep == VlobCreateRepBadTimestamp(
        reason=None,
        backend_timestamp=d1,
        ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
        ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
        client_timestamp=d2,
    )


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
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_create_but_already_exists(alice_ws, realm):
    blob = b"Initial commit."

    await vlob_create(alice_ws, realm, VLOB_ID, blob)

    rep = await vlob_create(alice_ws, realm, VLOB_ID, blob, check_rep=False)
    assert isinstance(rep, VlobCreateRepAlreadyExists)


@pytest.mark.trio
async def test_create_but_unknown_realm(alice_ws):
    bad_realm_id = RealmID.new()
    blob = b"Initial commit."

    rep = await vlob_create(alice_ws, bad_realm_id, VLOB_ID, blob, check_rep=False)
    assert isinstance(rep, VlobCreateRepNotAllowed)


@pytest.mark.trio
async def test_create_check_access_rights(backend, alice, bob, bob_ws, realm, next_timestamp):
    vlob_id = VlobID.new()

    # User not part of the realm
    rep = await vlob_create(
        bob_ws, realm, vlob_id, b"Initial version.", next_timestamp(), check_rep=False
    )
    assert isinstance(rep, VlobCreateRepNotAllowed)

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
            isinstance(rep, VlobCreateRepOk)

        else:
            isinstance(rep, VlobCreateRepNotAllowed)

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
    assert isinstance(rep, VlobCreateRepNotAllowed)


@pytest.mark.trio
async def test_read_not_found(alice_ws):
    rep = await vlob_read(alice_ws, VLOB_ID)
    # The reason is no longer generated
    assert isinstance(rep, VlobReadRepNotFound)


@pytest.mark.trio
async def test_read_ok(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0])
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:2",
        version=2,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_ok_v1(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], version=1)
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:1",
        version=1,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 2, 1),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_ok_timestamp_after_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=DateTime(2000, 1, 4))
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:2",
        version=2,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_ok_timestamp_is_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=DateTime(2000, 1, 3))
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:2",
        version=2,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 3),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_ok_timestamp_between_v1_and_v2(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=DateTime(2000, 1, 2, 10))
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:1",
        version=1,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 2, 1),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_ok_timestamp_is_v1(alice, alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=DateTime(2000, 1, 2, 1))
    assert rep == VlobReadRepOk(
        blob=b"r:A b:1 v:1",
        version=1,
        author=alice.device_id,
        timestamp=DateTime(2000, 1, 2, 1),
        author_last_role_granted_on=DateTime(2000, 1, 2),
    )


@pytest.mark.trio
async def test_read_before_v1(alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], timestamp=DateTime(2000, 1, 1))
    assert isinstance(rep, VlobReadRepBadVersion)


@pytest.mark.trio
async def test_read_check_access_rights(backend, alice, bob, bob_ws, realm, vlobs, next_timestamp):
    # Not part of the realm
    rep = await vlob_read(bob_ws, vlobs[0])
    assert isinstance(rep, VlobReadRepNotAllowed)

    for role in RealmRole.values():
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
        assert isinstance(rep, VlobReadRepOk)

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
    assert isinstance(rep, VlobReadRepNotAllowed)


@pytest.mark.trio
async def test_read_other_organization(backend_asgi_app, ws_from_other_organization_factory, vlobs):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_read(sock, vlobs[0])
    # The reason is no longer generated
    assert isinstance(rep, VlobReadRepNotFound)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID.hex, "bad_field": "foo"},
        {"id": "<not an uuid>"},
        {"id": VLOB_ID.hex},  # TODO: really bad ?
        {"id": 42},
        {"id": None},
        {"id": VLOB_ID.hex, "version": 0},
        {"id": VLOB_ID.hex, "version": "foo"},
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
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_read_bad_version(alice_ws, vlobs):
    rep = await vlob_read(alice_ws, vlobs[0], version=3)
    assert isinstance(rep, VlobReadRepBadVersion)


@pytest.mark.trio
async def test_update_ok(alice_ws, vlobs):
    await vlob_update(alice_ws, vlobs[0], version=3, blob=b"Next version.")


@pytest.mark.trio
async def test_update_bad_timestamp(alice_ws, vlobs):
    blob = b"Initial commit."
    d1 = DateTime(2000, 1, 1)
    with freeze_time(d1):
        d2 = d1.add(seconds=3600)
        rep = await vlob_update(
            alice_ws, vlobs[0], version=3, blob=blob, timestamp=d2, check_rep=False
        )
    assert rep == VlobUpdateRepBadTimestamp(
        reason=None,
        backend_timestamp=d1,
        ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
        ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
        client_timestamp=d2,
    )


@pytest.mark.trio
async def test_update_not_found(alice_ws):
    rep = await vlob_update(alice_ws, VLOB_ID, version=2, blob=b"Next version.", check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, VlobUpdateRepNotFound)


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
    assert isinstance(rep, VlobUpdateRepNotAllowed)

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
            assert isinstance(rep, VlobUpdateRepOk)
            next_version += 1

        else:
            assert isinstance(rep, VlobUpdateRepNotAllowed)

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
    assert isinstance(rep, VlobUpdateRepNotAllowed)


@pytest.mark.trio
async def test_update_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, vlobs
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_update(sock, vlobs[0], version=3, blob=b"Next version.", check_rep=False)
    # The reason is no longer generated
    assert isinstance(rep, VlobUpdateRepNotFound)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID.hex, "version": 42, "blob": b"...", "bad_field": "foo"},
        {"id": VLOB_ID.hex, "version": 42, "blob": None},
        {"id": VLOB_ID.hex, "version": 42, "blob": 42},
        {"id": VLOB_ID.hex, "version": 42},
        {"id": VLOB_ID.hex, "version": None, "blob": b"..."},
        {"id": VLOB_ID.hex, "version": -1, "blob": b"..."},
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
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(alice_ws, vlobs):
    rep = await vlob_update(alice_ws, vlobs[0], version=4, blob=b"Next version.", check_rep=False)
    assert isinstance(rep, VlobUpdateRepBadVersion)


@pytest.mark.trio
async def test_bad_encryption_revision(alice_ws, realm, vlobs):
    rep = await vlob_create(
        alice_ws, realm, VLOB_ID, blob=b"First version.", encryption_revision=42, check_rep=False
    )
    assert isinstance(rep, VlobCreateRepBadEncryptionRevision)

    rep = await vlob_read(alice_ws, vlobs[0], encryption_revision=42)
    assert isinstance(rep, VlobReadRepBadEncryptionRevision)

    rep = await vlob_update(
        alice_ws,
        vlobs[0],
        version=3,
        blob=b"Next version.",
        encryption_revision=42,
        check_rep=False,
    )
    assert isinstance(rep, VlobUpdateRepBadEncryptionRevision)


@pytest.mark.trio
async def test_list_versions_ok(alice, alice_ws, vlobs):
    rep = await vlob_list_versions(alice_ws, vlobs[0])
    assert rep == VlobListVersionsRepOk(
        {
            1: (DateTime(2000, 1, 2, 1, 0, 0), alice.device_id),
            2: (DateTime(2000, 1, 3, 0, 0, 0), alice.device_id),
        },
    )


@pytest.mark.trio
async def test_list_versions_not_found(alice_ws):
    rep = await vlob_list_versions(alice_ws, VLOB_ID)
    # The reason is no longer generated
    assert isinstance(rep, VlobListVersionsRepNotFound)


@pytest.mark.trio
async def test_list_versions_check_access_rights(
    backend, alice, bob, bob_ws, realm, vlobs, next_timestamp
):
    # Not part of the realm
    rep = await vlob_list_versions(bob_ws, vlobs[0])
    assert isinstance(rep, VlobListVersionsRepNotAllowed)

    for role in RealmRole.values():
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
        assert isinstance(rep, VlobListVersionsRepOk)

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
    assert isinstance(rep, VlobListVersionsRepNotAllowed)


@pytest.mark.trio
async def test_list_versions_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, vlobs
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await vlob_list_versions(sock, vlobs[0])
    # The reason is no longer generated
    assert isinstance(rep, VlobListVersionsRepNotFound)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID.hex, "bad_field": "foo"},
        {"id": "<not an uuid>"},
        {"id": VLOB_ID.hex},  # TODO: really bad ?
        {"id": 42},
        {"id": None},
        {"id": VLOB_ID.hex, "version": 1},
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
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_access_during_maintenance(backend, alice, alice_ws, realm, vlobs):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        DateTime(2000, 1, 2),
    )

    rep = await vlob_create(
        alice_ws, realm, VLOB_ID, blob=b"First version.", encryption_revision=2, check_rep=False
    )
    assert isinstance(rep, VlobCreateRepInMaintenance)

    rep = await vlob_read(alice_ws, vlobs[0], encryption_revision=2)
    assert isinstance(rep, VlobReadRepInMaintenance)

    rep = await vlob_update(
        alice_ws, vlobs[0], version=3, blob=b"Next version.", encryption_revision=2, check_rep=False
    )
    assert isinstance(rep, VlobUpdateRepInMaintenance)


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
    assert isinstance(rep, RealmUpdateRolesRepOk)

    # Now bob writes to the corresponding realm with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await vlob_create(
            bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=timestamp, check_rep=False
        )
        assert rep == VlobCreateRepRequireGreaterTimestamp(ref)

    # Advance ref
    ref = ref.add(seconds=10)

    # Bob successfully write version 1
    rep = await vlob_create(
        bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=ref, check_rep=False
    )
    assert isinstance(rep, VlobCreateRepOk)

    # Now bob writes to the corresponding vlob with a lower timestamp: this should fail
    rep = await vlob_update(
        bob_ws,
        VLOB_ID,
        version=2,
        blob=b"ciphered",
        timestamp=ref.subtract(seconds=1),
        check_rep=False,
    )
    assert rep == VlobUpdateRepRequireGreaterTimestamp(ref)
