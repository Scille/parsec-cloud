# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from parsec._parsec import DateTime
from unittest.mock import ANY

from parsec.api.protocol import VlobID, RealmID, RealmRole, UserProfile
from parsec.api.data import RealmRoleCertificate
from parsec.backend.realm import RealmGrantedRole

from tests.common import freeze_time, customize_fixtures
from tests.backend.common import realm_update_roles, realm_get_role_certificates, vlob_create


NOW = DateTime(2000, 1, 1)
VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
REALM_ID = RealmID.from_hex("0000000000000000000000000000000A")


@pytest.mark.trio
async def test_get_roles_not_found(alice_ws):
    rep = await realm_get_role_certificates(alice_ws, REALM_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `0000000000000000000000000000000a` doesn't exist",
    }


async def _realm_get_clear_role_certifs(sock, realm_id):
    rep = await realm_get_role_certificates(sock, realm_id)
    assert rep["status"] == "ok"
    cooked = [RealmRoleCertificate.unsecure_load(certif) for certif in rep["certificates"]]
    return [item for item in sorted(cooked, key=lambda x: x.timestamp)]


@pytest.fixture
def realm_generate_certif_and_update_roles_or_fail(next_timestamp):
    async def _realm_generate_certif_and_update_roles_or_fail(
        ws, author, realm_id, user_id, role, timestamp=None
    ):
        certif = RealmRoleCertificate(
            author=author.device_id,
            timestamp=timestamp or next_timestamp(),
            realm_id=realm_id,
            user_id=user_id,
            role=role,
        ).dump_and_sign(author.signing_key)
        return await realm_update_roles(ws, certif, check_rep=False)

    return _realm_generate_certif_and_update_roles_or_fail


@pytest.fixture
def backend_realm_generate_certif_and_update_roles(next_timestamp):
    async def _backend_realm_generate_certif_and_update_roles(
        backend, author, realm_id, user_id, role, timestamp=None
    ):
        now = timestamp or next_timestamp()
        certif = RealmRoleCertificate(
            author=author.device_id, timestamp=now, realm_id=realm_id, user_id=user_id, role=role
        ).dump_and_sign(author.signing_key)
        await backend.realm.update_roles(
            author.organization_id,
            RealmGrantedRole(
                certificate=certif,
                realm_id=realm_id,
                user_id=user_id,
                role=role,
                granted_by=author.device_id,
                granted_on=now,
            ),
        )
        return certif

    return _backend_realm_generate_certif_and_update_roles


@pytest.mark.trio
async def test_update_roles_not_found(
    alice, bob, alice_ws, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, REALM_ID, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {
        "status": "not_found",
        "reason": "Realm `0000000000000000000000000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_update_roles_bad_user(
    alice, mallory, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, mallory.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "not_found", "reason": "User `mallory` doesn't exist"}


@pytest.mark.trio
async def test_update_roles_cannot_modify_self(
    alice, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, alice.user_id, RealmRole.MANAGER
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Realm role certificate cannot be self-signed.",
    }


@pytest.mark.trio
@customize_fixtures(bob_profile=UserProfile.OUTSIDER)
async def test_update_roles_outsider_is_limited(
    alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    for role, is_allowed in [
        (RealmRole.READER, True),
        (RealmRole.CONTRIBUTOR, True),
        (RealmRole.MANAGER, False),
        (RealmRole.OWNER, False),
    ]:
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        if is_allowed:
            assert rep == {"status": "ok"}
        else:
            assert rep == {
                "status": "incompatible_profile",
                "reason": "User with OUTSIDER profile cannot be MANAGER or OWNER",
            }


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.OUTSIDER)
async def test_update_roles_outsider_cannot_share_with(
    alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.READER
    )
    assert rep == {"status": "not_allowed", "reason": "Outsider user cannot share realm"}


@pytest.mark.trio
@pytest.mark.parametrize("start_with_existing_role", (False, True))
async def test_remove_role_idempotent(
    alice,
    bob,
    alice_ws,
    realm,
    start_with_existing_role,
    realm_generate_certif_and_update_roles_or_fail,
):
    if start_with_existing_role:
        with freeze_time("2000-01-03"):
            rep = await realm_generate_certif_and_update_roles_or_fail(
                alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER
            )
            assert rep == {"status": "ok"}

    with freeze_time("2000-01-04"):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, None
        )
        if start_with_existing_role:
            assert rep == {"status": "ok"}
        else:
            assert rep == {"status": "already_granted"}

    with freeze_time("2000-01-05"):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, None
        )
        assert rep == {"status": "already_granted"}

    certifs = await _realm_get_clear_role_certifs(alice_ws, realm)
    expected_certifs = [
        RealmRoleCertificate(
            author=alice.device_id,
            timestamp=DateTime(2000, 1, 2),
            realm_id=realm,
            user_id=alice.user_id,
            role=RealmRole.OWNER,
        )
    ]
    if start_with_existing_role:
        expected_certifs += [
            RealmRoleCertificate(
                author=alice.device_id,
                timestamp=DateTime(2000, 1, 3),
                realm_id=realm,
                user_id=bob.user_id,
                role=RealmRole.MANAGER,
            ),
            RealmRoleCertificate(
                author=alice.device_id,
                timestamp=DateTime(2000, 1, 4),
                realm_id=realm,
                user_id=bob.user_id,
                role=None,
            ),
        ]
    assert certifs == expected_certifs


@pytest.mark.trio
async def test_update_roles_as_owner(
    backend, alice, bob, alice_ws, bob_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    for role in RealmRole:
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "ok"}

        roles = await backend.realm.get_current_roles(alice.organization_id, realm)
        assert roles == {alice.user_id: RealmRole.OWNER, bob.user_id: role}

    # Now remove role
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    rep = await realm_get_role_certificates(bob_ws, realm)
    assert rep["status"] == "not_allowed"


@pytest.mark.trio
async def test_update_roles_as_manager(
    backend_data_binder,
    local_device_factory,
    backend,
    alice,
    bob,
    alice_ws,
    bob_ws,
    realm,
    realm_generate_certif_and_update_roles_or_fail,
    backend_realm_generate_certif_and_update_roles,
):
    # Vlob realm must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice is manager and gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await backend_realm_generate_certif_and_update_roles(
        backend, alice, realm, zack.user_id, RealmRole.OWNER
    )
    await backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, alice.user_id, RealmRole.MANAGER
    )

    for role in (RealmRole.CONTRIBUTOR, RealmRole.READER):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "ok"}

        roles = await backend.realm.get_current_roles(alice.organization_id, realm)
        assert roles == {
            zack.user_id: RealmRole.OWNER,
            alice.user_id: RealmRole.MANAGER,
            bob.user_id: role,
        }

    # Remove role
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}
    rep = await realm_get_role_certificates(bob_ws, realm)
    assert rep["status"] == "not_allowed"

    # Cannot give owner or manager role as manager
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, new_role
        )
        assert rep == {"status": "not_allowed"}

    # Also cannot change owner or manager role
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        await backend_realm_generate_certif_and_update_roles(
            backend, zack, realm, bob.user_id, new_role
        )
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, zack.user_id, RealmRole.CONTRIBUTOR
        )
        assert rep == {"status": "not_allowed"}


@pytest.mark.trio
@pytest.mark.parametrize("alice_role", (RealmRole.CONTRIBUTOR, RealmRole.READER, None))
async def test_role_update_not_allowed(
    backend_data_binder,
    local_device_factory,
    backend,
    alice,
    bob,
    alice_ws,
    realm,
    alice_role,
    realm_generate_certif_and_update_roles_or_fail,
    backend_realm_generate_certif_and_update_roles,
):
    # Vlob realm must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await backend_realm_generate_certif_and_update_roles(
        backend, alice, realm, zack.user_id, RealmRole.OWNER
    )
    await backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, alice.user_id, alice_role
    )

    # Cannot give role
    for role in RealmRole:
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "not_allowed"}

    # Cannot remove role
    await backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, bob.user_id, RealmRole.READER
    )
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_remove_role_dont_change_other_realms(
    backend, alice, bob, alice_ws, realm, bob_realm, realm_generate_certif_and_update_roles_or_fail
):
    # Bob is owner of bob_realm and manager of realm
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "ok"}

    # Remove Bob from realm
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    # Bob should still have access to bob_realm
    roles = await backend.realm.get_current_roles(alice.organization_id, bob_realm)
    assert roles == {bob.user_id: RealmRole.OWNER}


@pytest.mark.trio
async def test_role_access_during_maintenance(
    backend, alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        DateTime(2000, 1, 2),
    )

    # Get roles allowed...
    roles = await backend.realm.get_current_roles(alice.organization_id, realm)
    assert roles == {alice.user_id: RealmRole.OWNER}

    rep = await realm_get_role_certificates(alice_ws, realm)
    assert rep == {"status": "ok", "certificates": [ANY]}

    # ...buit not update role
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "in_maintenance"}


@pytest.mark.trio
async def test_get_role_certificates_multiple(
    backend, alice, bob, adam, bob_ws, realm, backend_realm_generate_certif_and_update_roles
):
    # Realm is created on 2000-01-02

    with freeze_time("2000-01-03"):
        c3 = await backend_realm_generate_certif_and_update_roles(
            backend, alice, realm, bob.user_id, RealmRole.OWNER
        )

    with freeze_time("2000-01-04"):
        c4 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, adam.user_id, RealmRole.MANAGER
        )

    with freeze_time("2000-01-05"):
        c5 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, RealmRole.READER
        )

    with freeze_time("2000-01-06"):
        c6 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, None
        )

    rep = await realm_get_role_certificates(bob_ws, realm)
    assert rep == {"status": "ok", "certificates": [ANY, c3, c4, c5, c6]}


@pytest.mark.trio
async def test_get_role_certificates_no_longer_allowed(
    backend, alice, bob, alice_ws, realm, backend_realm_generate_certif_and_update_roles
):
    # Realm is created on 2000-01-02

    with freeze_time("2000-01-03"):
        await backend_realm_generate_certif_and_update_roles(
            backend, alice, realm, bob.user_id, RealmRole.OWNER
        )

    with freeze_time("2000-01-04"):
        await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, None
        )

    rep = await realm_get_role_certificates(alice_ws, realm)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_update_roles_causality_checks(
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

    # Now try to change bob's role with the same timestamp or lower, this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, timestamp
        )
        assert rep == {"status": "require_greater_timestamp", "strictly_greater_than": ref}

    # Advance ref
    ref = ref.add(seconds=10)

    # Now bob invites someone at timestamp ref
    rep = await realm_generate_certif_and_update_roles_or_fail(
        bob_ws, bob, realm, adam.user_id, RealmRole.CONTRIBUTOR, ref
    )
    assert rep == {"status": "ok"}

    # Now try to remove bob's management rights with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, timestamp
        )
        assert rep == {"status": "require_greater_timestamp", "strictly_greater_than": ref}

    # Advance ref
    ref = ref.add(seconds=10)

    # Now bob writes to the corresponding realm
    rep = await vlob_create(
        bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=ref, check_rep=False
    )
    assert rep == {"status": "ok"}

    # Now try to remove bob's write rights with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.READER, timestamp
        )
        assert rep == {"status": "require_greater_timestamp", "strictly_greater_than": ref}


@pytest.mark.trio
async def test_update_roles_for_revoked_user(
    backend,
    alice,
    bob,
    alice_ws,
    realm,
    realm_generate_certif_and_update_roles_or_fail,
    next_timestamp,
    backend_data_binder,
):
    # Grant a role to bob
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER, next_timestamp()
    )
    assert rep == {"status": "ok"}

    # Revoke Bob
    await backend_data_binder.bind_revocation(bob.user_id, certifier=alice)

    # Now try to change bob's role, this should fail
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, next_timestamp()
    )
    assert rep == {"status": "user_revoked"}

    # Even removing access should fail
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None, next_timestamp()
    )
    assert rep == {"status": "user_revoked"}
