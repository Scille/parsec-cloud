# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID
from pendulum import Pendulum

from parsec.api.protocole import RealmRole

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


@pytest.mark.trio
async def test_update_roles_not_found(bob, alice_backend_sock):
    rep = await realm_update_roles(alice_backend_sock, REALM_ID, bob.user_id, RealmRole.MANAGER)
    assert rep == {
        "status": "not_found",
        "reason": "Realm `00000000-0000-0000-0000-00000000000a` doesn't exist",
    }


@pytest.mark.trio
async def test_update_roles_bad_user(backend, mallory, alice_backend_sock, realm):
    rep = await realm_update_roles(alice_backend_sock, realm, mallory.user_id, RealmRole.MANAGER)
    assert rep == {"status": "not_found", "reason": "User `mallory` doesn't exist"}


@pytest.mark.trio
async def test_update_roles_cannot_modify_self(backend, alice, alice_backend_sock, realm):
    rep = await realm_update_roles(alice_backend_sock, realm, alice.user_id, RealmRole.MANAGER)
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
@pytest.mark.parametrize("start_with_existing_role", (False, True))
async def test_remove_role_idempotent(
    backend, bob, alice_backend_sock, realm, start_with_existing_role
):
    if start_with_existing_role:
        rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, RealmRole.MANAGER)
        assert rep == {"status": "ok"}

    rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, None)
    assert rep == {"status": "ok"}

    rep = await realm_get_roles(alice_backend_sock, realm)
    assert rep == {"status": "ok", "users": {"alice": RealmRole.OWNER}}


@pytest.mark.trio
async def test_update_roles_as_owner(
    backend, alice, bob, alice_backend_sock, bob_backend_sock, realm
):
    for role in RealmRole:
        rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, role)
        assert rep == {"status": "ok"}

        rep = await realm_get_roles(bob_backend_sock, realm)
        assert rep == {"status": "ok", "users": {alice.user_id: RealmRole.OWNER, bob.user_id: role}}

    # Now remove role
    rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, None)
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
    await backend.realm.update_roles(
        alice.organization_id, alice.device_id, realm, zack.user_id, RealmRole.OWNER
    )
    await backend.realm.update_roles(
        alice.organization_id, zack.device_id, realm, alice.user_id, RealmRole.MANAGER
    )

    for role in (RealmRole.CONTRIBUTOR, RealmRole.READER):
        rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, role)
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
    rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, None)
    assert rep == {"status": "ok"}
    rep = await realm_get_roles(bob_backend_sock, realm)
    assert rep["status"] == "not_allowed"

    # Cannot give owner or manager role as manager
    rep = await realm_update_roles(alice_backend_sock, realm, zack.user_id, RealmRole.OWNER)
    assert rep == {"status": "not_allowed"}
    rep = await realm_update_roles(alice_backend_sock, realm, zack.user_id, RealmRole.MANAGER)
    assert rep == {"status": "not_allowed"}

    # Also cannot change owner or manager role
    for bob_role in (RealmRole.OWNER, RealmRole.MANAGER):
        await backend.realm.update_roles(
            alice.organization_id, zack.device_id, realm, bob.user_id, role
        )
        rep = await realm_update_roles(
            alice_backend_sock, realm, zack.user_id, RealmRole.CONTRIBUTOR
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
    await backend.realm.update_roles(
        alice.organization_id, alice.device_id, realm, zack.user_id, RealmRole.OWNER
    )
    await backend.realm.update_roles(
        alice.organization_id, zack.device_id, realm, alice.user_id, alice_role
    )

    # Cannot give role
    for role in RealmRole:
        rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, role)
        assert rep == {"status": "not_allowed"}

    # Cannot remove role
    await backend.realm.update_roles(
        alice.organization_id, zack.device_id, realm, bob.user_id, RealmRole.READER
    )
    rep = await realm_update_roles(alice_backend_sock, realm, bob.user_id, None)
    assert rep == {"status": "not_allowed"}
