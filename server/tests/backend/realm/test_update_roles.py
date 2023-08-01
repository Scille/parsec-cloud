# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
)
from parsec.api.data import RealmRoleCertificate
from parsec.api.protocol import (
    RealmID,
    RealmRole,
    RealmUpdateRolesRepAlreadyGranted,
    RealmUpdateRolesRepIncompatibleProfile,
    RealmUpdateRolesRepInMaintenance,
    RealmUpdateRolesRepInvalidData,
    RealmUpdateRolesRepNotAllowed,
    RealmUpdateRolesRepNotFound,
    RealmUpdateRolesRepOk,
    RealmUpdateRolesRepRequireGreaterTimestamp,
    RealmUpdateRolesRepUserRevoked,
    UserProfile,
    VlobCreateRepOk,
    VlobID,
)
from parsec.backend.realm import RealmGrantedRole
from tests.backend.common import realm_update_roles, vlob_create
from tests.common import customize_fixtures, freeze_time

VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
REALM_ID = RealmID.from_hex("0000000000000000000000000000000A")


async def _realm_get_clear_role_certifs(backend, device, realm_id):
    certificates = await backend.realm.get_role_certificates(
        organization_id=device.organization_id, author=device.device_id, realm_id=realm_id
    )
    cooked = [RealmRoleCertificate.unsecure_load(certif) for certif in certificates]
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
    # The reason is no longer generated
    assert isinstance(rep, RealmUpdateRolesRepNotFound)


@pytest.mark.trio
async def test_update_roles_bad_user(
    alice, mallory, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, mallory.user_id, RealmRole.MANAGER
    )
    # The reason is no longer generated
    assert isinstance(rep, RealmUpdateRolesRepNotFound)


@pytest.mark.trio
async def test_update_roles_cannot_modify_self(
    alice, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, alice.user_id, RealmRole.MANAGER
    )
    # The reason is no longer generated
    assert isinstance(rep, RealmUpdateRolesRepInvalidData)


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
            assert isinstance(rep, RealmUpdateRolesRepOk)
        else:
            # The reason is no longer generated
            assert isinstance(rep, RealmUpdateRolesRepIncompatibleProfile)


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.OUTSIDER)
async def test_update_roles_outsider_cannot_share_with(
    alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.READER
    )
    # The reason is no longer generated
    assert isinstance(rep, RealmUpdateRolesRepNotAllowed)


@pytest.mark.trio
@pytest.mark.parametrize("start_with_existing_role", (False, True))
async def test_remove_role_idempotent(
    backend,
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
            assert isinstance(rep, RealmUpdateRolesRepOk)

    with freeze_time("2000-01-04"):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, None
        )
        if start_with_existing_role:
            assert isinstance(rep, RealmUpdateRolesRepOk)
        else:
            assert isinstance(rep, RealmUpdateRolesRepAlreadyGranted)

    with freeze_time("2000-01-05"):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, None
        )
        assert isinstance(rep, RealmUpdateRolesRepAlreadyGranted)

    certifs = await _realm_get_clear_role_certifs(backend, alice, realm)
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
    backend, alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    for role in RealmRole.VALUES:
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        assert isinstance(rep, RealmUpdateRolesRepOk)

        roles = await backend.realm.get_current_roles(alice.organization_id, realm)
        assert roles == {alice.user_id: RealmRole.OWNER, bob.user_id: role}

    # Now remove role
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert isinstance(rep, RealmUpdateRolesRepOk)

    roles = await backend.realm.get_current_roles(bob.organization_id, realm)
    assert roles == {alice.user_id: RealmRole.OWNER}


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
        assert isinstance(rep, RealmUpdateRolesRepOk)

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
    assert isinstance(rep, RealmUpdateRolesRepOk)

    roles = await backend.realm.get_current_roles(alice.organization_id, realm)
    assert roles == {
        zack.user_id: RealmRole.OWNER,
        alice.user_id: RealmRole.MANAGER,
    }

    # Cannot give owner or manager role as manager
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, new_role
        )
        assert isinstance(rep, RealmUpdateRolesRepNotAllowed)

    # Also cannot change owner or manager role
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        await backend_realm_generate_certif_and_update_roles(
            backend, zack, realm, bob.user_id, new_role
        )
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, zack.user_id, RealmRole.CONTRIBUTOR
        )
        assert isinstance(rep, RealmUpdateRolesRepNotAllowed)


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
    for role in RealmRole.VALUES:
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, role
        )
        assert isinstance(rep, RealmUpdateRolesRepNotAllowed)

    # Cannot remove role
    await backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, bob.user_id, RealmRole.READER
    )
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert isinstance(rep, RealmUpdateRolesRepNotAllowed)


@pytest.mark.trio
async def test_remove_role_dont_change_other_realms(
    backend, alice, bob, alice_ws, realm, bob_realm, realm_generate_certif_and_update_roles_or_fail
):
    # Bob is owner of bob_realm and manager of realm
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert isinstance(rep, RealmUpdateRolesRepOk)

    # Remove Bob from realm
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None
    )
    assert isinstance(rep, RealmUpdateRolesRepOk)

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

    # Update role is not allowed
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert isinstance(rep, RealmUpdateRolesRepInMaintenance)


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
    assert isinstance(rep, RealmUpdateRolesRepOk)

    # Now try to change bob's role with the same timestamp or lower, this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, timestamp
        )
        assert rep == RealmUpdateRolesRepRequireGreaterTimestamp(ref)

    # Advance ref
    ref = ref.add(seconds=10)

    # Now bob invites someone at timestamp ref
    rep = await realm_generate_certif_and_update_roles_or_fail(
        bob_ws, bob, realm, adam.user_id, RealmRole.CONTRIBUTOR, ref
    )
    assert isinstance(rep, RealmUpdateRolesRepOk)

    # Now try to remove bob's management rights with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, timestamp
        )
        assert rep == RealmUpdateRolesRepRequireGreaterTimestamp(ref)

    # Advance ref
    ref = ref.add(seconds=10)

    # Now bob writes to the corresponding realm
    rep = await vlob_create(
        bob_ws, realm, VLOB_ID, blob=b"ciphered", timestamp=ref, check_rep=False
    )
    assert isinstance(rep, VlobCreateRepOk)

    # Now try to remove bob's write rights with the same timestamp or lower: this should fail
    for timestamp in (ref, ref.subtract(seconds=1)):
        rep = await realm_generate_certif_and_update_roles_or_fail(
            alice_ws, alice, realm, bob.user_id, RealmRole.READER, timestamp
        )
        assert rep == RealmUpdateRolesRepRequireGreaterTimestamp(ref)


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
    assert isinstance(rep, RealmUpdateRolesRepOk)

    # Revoke Bob
    await backend_data_binder.bind_revocation(bob.user_id, certifier=alice)

    # Now try to change bob's role, this should fail
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, RealmRole.CONTRIBUTOR, next_timestamp()
    )
    assert isinstance(rep, RealmUpdateRolesRepUserRevoked)

    # Even removing access should fail
    rep = await realm_generate_certif_and_update_roles_or_fail(
        alice_ws, alice, realm, bob.user_id, None, next_timestamp()
    )
    assert isinstance(rep, RealmUpdateRolesRepUserRevoked)
