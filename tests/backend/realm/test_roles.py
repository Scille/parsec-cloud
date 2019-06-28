# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
from pendulum import Pendulum, now as pendulum_now

from parsec.api.protocole import RealmRole
from parsec.backend.realm import RealmGrantedRole
from parsec.crypto import build_realm_role_certificate

from tests.backend.realm.conftest import realm_get_roles, realm_update_roles


NOW = Pendulum(2000, 1, 1)
VLOB_ID = UUID("00000000000000000000000000000001")
REALM_ID = UUID("0000000000000000000000000000000A")


@pytest.mark.trio
async def test_get_roles_not_found(alice_backend_sock):
    rep = await realm_get_roles(alice_backend_sock, REALM_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


async def _realm_generate_certif_and_update_roles_or_fail(
    backend_sock, author, realm_id, user_id, role
):
    certif = build_realm_role_certificate(
        author.device_id, author.signing_key, realm_id, user_id, role, pendulum_now()
    )
    return await realm_update_roles(backend_sock, certif, check_rep=False)


async def _backend_realm_generate_certif_and_update_roles(backend, author, realm_id, user_id, role):
    now = pendulum_now()
    certif = build_realm_role_certificate(
        author.device_id, author.signing_key, realm_id, user_id, role, now
    )
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


@pytest.mark.trio
async def test_update_roles_not_found(alice, bob, alice_backend_sock):
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, REALM_ID, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {
        "status": "not_found",
        "reason": "Realm `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_update_roles_bad_user(backend, alice, mallory, alice_backend_sock, realm):
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, mallory.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "not_found", "reason": "User `mallory` doesn't exist"}


@pytest.mark.trio
async def test_update_roles_cannot_modify_self(backend, alice, alice_backend_sock, realm):
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, alice.user_id, RealmRole.MANAGER
    )
    assert rep == {
        "status": "invalid_data",
        "reason": "Realm role certificate cannot be self-signed.",
    }


@pytest.mark.trio
@pytest.mark.parametrize("start_with_existing_role", (False, True))
async def test_remove_role_idempotent(
    backend, alice, bob, alice_backend_sock, realm, start_with_existing_role
):
    if start_with_existing_role:
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, bob.user_id, RealmRole.MANAGER
        )
        assert rep == {"status": "ok"}

    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    rep = await realm_get_roles(alice_backend_sock, realm)
    assert rep == {"status": "ok", "users": {"alice": RealmRole.OWNER}}


@pytest.mark.trio
async def test_update_roles_as_owner(
    backend, alice, bob, alice_backend_sock, bob_backend_sock, realm
):
    for role in RealmRole:
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "ok"}

        rep = await realm_get_roles(bob_backend_sock, realm)
        assert rep == {"status": "ok", "users": {alice.user_id: RealmRole.OWNER, bob.user_id: role}}

    # Now remove role
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    rep = await realm_get_roles(bob_backend_sock, realm)
    assert rep["status"] == "not_allowed"


@pytest.mark.trio
async def test_update_roles_as_manager(
    backend_data_binder,
    local_device_factory,
    backend,
    alice,
    bob,
    alice_backend_sock,
    bob_backend_sock,
    realm,
):
    # Vlob realm must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice is manager and gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await _backend_realm_generate_certif_and_update_roles(
        backend, alice, realm, zack.user_id, RealmRole.OWNER
    )
    await _backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, alice.user_id, RealmRole.MANAGER
    )

    for role in (RealmRole.CONTRIBUTOR, RealmRole.READER):
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "ok"}

        rep = await realm_get_roles(bob_backend_sock, realm)
        assert rep == {
            "status": "ok",
            "users": {
                zack.user_id: RealmRole.OWNER,
                alice.user_id: RealmRole.MANAGER,
                bob.user_id: role,
            },
        }

    # Remove role
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}
    rep = await realm_get_roles(bob_backend_sock, realm)
    assert rep["status"] == "not_allowed"

    # Cannot give owner or manager role as manager
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, bob.user_id, new_role
        )
        assert rep == {"status": "not_allowed"}

    # Also cannot change owner or manager role
    for new_role in (RealmRole.OWNER, RealmRole.MANAGER):
        await _backend_realm_generate_certif_and_update_roles(
            backend, zack, realm, bob.user_id, new_role
        )
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, zack.user_id, RealmRole.CONTRIBUTOR
        )
        assert rep == {"status": "not_allowed"}


@pytest.mark.trio
@pytest.mark.parametrize("alice_role", (RealmRole.CONTRIBUTOR, RealmRole.READER))
async def test_role_update_not_allowed(
    backend_data_binder,
    local_device_factory,
    backend,
    alice,
    bob,
    alice_backend_sock,
    realm,
    alice_role,
):
    # Vlob realm must have at least one owner, so we need 3 users in total
    # (Zack is owner, Alice gives role to Bob)
    zack = local_device_factory("zack@dev1")
    await backend_data_binder.bind_device(zack)
    await _backend_realm_generate_certif_and_update_roles(
        backend, alice, realm, zack.user_id, RealmRole.OWNER
    )
    await _backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, alice.user_id, alice_role
    )

    # Cannot give role
    for role in RealmRole:
        rep = await _realm_generate_certif_and_update_roles_or_fail(
            alice_backend_sock, alice, realm, bob.user_id, role
        )
        assert rep == {"status": "not_allowed"}

    # Cannot remove role
    await _backend_realm_generate_certif_and_update_roles(
        backend, zack, realm, bob.user_id, RealmRole.READER
    )
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_remove_role_dont_change_other_realms(
    backend, alice, bob, alice_backend_sock, bob_backend_sock, realm, bob_realm
):
    # Bob is owner of bob_realm and manager of realm
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "ok"}

    # Remove Bob from realm
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, None
    )
    assert rep == {"status": "ok"}

    # Bob should still have access to bob_realm
    rep = await realm_get_roles(bob_backend_sock, bob_realm)
    assert rep == {"status": "ok", "users": {"bob": RealmRole.OWNER}}


@pytest.mark.trio
async def test_role_access_during_maintenance(
    backend, alice, bob, alice_backend_sock, realm, vlobs
):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        Pendulum(2000, 1, 2),
    )

    # Get roles allowed...
    rep = await realm_get_roles(alice_backend_sock, realm)
    assert rep == {"status": "ok", "users": {"alice": RealmRole.OWNER}}

    # ...buit not update role
    rep = await _realm_generate_certif_and_update_roles_or_fail(
        alice_backend_sock, alice, realm, bob.user_id, RealmRole.MANAGER
    )
    assert rep == {"status": "in_maintenance"}


# TODO: add tests on realm_get_role_certificates
