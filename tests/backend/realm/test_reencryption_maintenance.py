# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.api.protocole import RealmRole

from tests.common import freeze_time
from tests.backend.test_message import message_get
from tests.backend.realm.conftest import (
    realm_start_reencryption_maintenance,
    realm_finish_reencryption_maintenance,
    vlob_read,
    vlob_maintenance_get_reencryption_batch,
    vlob_maintenance_save_reencryption_batch,
)


@pytest.mark.trio
async def test_start_bad_encryption_revision(backend, alice_backend_sock, realm):
    rep = await realm_start_reencryption_maintenance(
        alice_backend_sock, realm, 42, {"alice": b"wathever"}, check_rep=False
    )
    assert rep == {"status": "bad_encryption_revision"}


@pytest.mark.trio
async def test_start_bad_per_participant_message(backend, alice_backend_sock, alice, bob, realm):
    # Add bob for more fun !
    await backend.realm.update_roles(
        alice.organization_id, alice.device_id, realm, bob.user_id, RealmRole.READER
    )

    for msg in [
        {},
        {alice.user_id: b"ok"},
        {alice.user_id: b"ok", bob.user_id: b"ok", "zack": b"dunno this guy"},
    ]:
        rep = await realm_start_reencryption_maintenance(
            alice_backend_sock, realm, 2, {}, check_rep=False
        )
        assert rep == {
            "status": "maintenance_error",
            "reason": "Realm participants and message recipients mismatch",
        }


@pytest.mark.trio
async def test_start_already_in_maintenance(backend, alice_backend_sock, realm):
    await realm_start_reencryption_maintenance(alice_backend_sock, realm, 2, {"alice": b"wathever"})
    # Providing good or bad encryption revision shouldn't change anything
    for encryption_revision in (2, 3):
        rep = await realm_start_reencryption_maintenance(
            alice_backend_sock, realm, encryption_revision, {"alice": b"wathever"}, check_rep=False
        )
        assert rep == {"status": "in_maintenance"}


@pytest.mark.trio
async def test_start_check_access_rights(backend, bob_backend_sock, alice, bob, realm):
    # User not part of the realm
    rep = await realm_start_reencryption_maintenance(
        bob_backend_sock, realm, 2, {"alice": b"wathever"}, check_rep=False
    )
    assert rep == {"status": "not_allowed"}

    # User part of the realm with various role
    for not_allowed_role in (RealmRole.READER, RealmRole.CONTRIBUTOR, RealmRole.MANAGER):
        await backend.realm.update_roles(
            alice.organization_id, alice.device_id, realm, bob.user_id, not_allowed_role
        )

        rep = await realm_start_reencryption_maintenance(
            bob_backend_sock, realm, 2, {"alice": b"foo", "bob": b"bar"}, check_rep=False
        )
        assert rep == {"status": "not_allowed"}

    # Finally, just make sure owner can do it
    await backend.realm.update_roles(
        alice.organization_id, alice.device_id, realm, bob.user_id, RealmRole.OWNER
    )

    rep = await realm_start_reencryption_maintenance(
        bob_backend_sock, realm, 2, {"alice": b"foo", "bob": b"bar"}, check_rep=False
    )
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_start_other_organization(backend, sock_from_other_organization_factory, realm):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await realm_start_reencryption_maintenance(
            sock, realm, 2, {"alice": b"foo"}, check_rep=False
        )
    assert rep == {
        "status": "not_found",
        "reason": "Realm `a0000000-0000-0000-0000-000000000000` doesn't exist",
    }


@pytest.mark.trio
async def test_finish_not_in_maintenance(alice_backend_sock, realm):
    for encryption_revision in (2, 3):
        rep = await realm_finish_reencryption_maintenance(
            alice_backend_sock, realm, encryption_revision, check_rep=False
        )
        assert rep == {
            "status": "maintenance_error",
            "reason": "Realm `a0000000-0000-0000-0000-000000000000` not under maintenance",
        }


@pytest.mark.trio
async def test_reencrypt_and_finish_check_access_rights(
    backend, alice_backend_sock, bob_backend_sock, alice, bob, realm, vlobs
):
    encryption_revision = 1
    start_reencryption_msgs = {"alice": b"foo"}

    # Changing realm roles is not possible during maintenance,
    # hence those helpers to easily jump in/out of maintenance

    async def _ready_to_finish():
        nonlocal encryption_revision
        encryption_revision += 1
        await realm_start_reencryption_maintenance(
            alice_backend_sock, realm, encryption_revision, start_reencryption_msgs
        )
        updated_batch = [
            {
                "vlob_id": vlob_id,
                "version": version,
                "blob": f"{vlob_id}::{version}::{encryption_revision}".encode(),
            }
            for vlob_id, version in {(vlobs[0], 1), (vlobs[0], 2), (vlobs[1], 1)}
        ]
        await vlob_maintenance_save_reencryption_batch(
            alice_backend_sock, realm, encryption_revision, updated_batch
        )

    async def _finish():
        await realm_finish_reencryption_maintenance(alice_backend_sock, realm, encryption_revision)

    async def _assert_bob_access(allowed):
        if allowed:
            expected_rep = {"status": "ok"}
        else:
            expected_rep = {"status": "not_allowed"}
        rep = await vlob_maintenance_save_reencryption_batch(
            bob_backend_sock, realm, encryption_revision, [], check_rep=False
        )
        assert rep == expected_rep
        rep = await realm_finish_reencryption_maintenance(
            bob_backend_sock, realm, encryption_revision, check_rep=False
        )
        assert rep == expected_rep

    # User not part of the realm
    await _ready_to_finish()
    await _assert_bob_access(allowed=False)
    await _finish()

    # User part of the realm with various role
    start_reencryption_msgs["bob"] = b"bar"
    for not_allowed_role in (RealmRole.READER, RealmRole.CONTRIBUTOR, RealmRole.MANAGER):
        await backend.realm.update_roles(
            alice.organization_id, alice.device_id, realm, bob.user_id, not_allowed_role
        )
        await _ready_to_finish()
        await _assert_bob_access(allowed=False)
        await _finish()

    # Finally, just make sure owner can do it
    await backend.realm.update_roles(
        alice.organization_id, alice.device_id, realm, bob.user_id, RealmRole.OWNER
    )
    await _ready_to_finish()
    await _assert_bob_access(allowed=True)


@pytest.mark.trio
async def test_reencryption_batch_not_during_maintenance(alice_backend_sock, realm):
    rep = await vlob_maintenance_get_reencryption_batch(alice_backend_sock, realm, 1)
    assert rep == {
        "status": "maintenance_error",
        "reason": "Realm `a0000000-0000-0000-0000-000000000000` not under maintenance",
    }

    rep = await vlob_maintenance_save_reencryption_batch(
        alice_backend_sock, realm, 1, [], check_rep=False
    )
    assert rep == {
        "status": "maintenance_error",
        "reason": "Realm `a0000000-0000-0000-0000-000000000000` not under maintenance",
    }

    rep = await realm_finish_reencryption_maintenance(alice_backend_sock, realm, 1, check_rep=False)
    assert rep == {
        "status": "maintenance_error",
        "reason": "Realm `a0000000-0000-0000-0000-000000000000` not under maintenance",
    }


@pytest.mark.trio
async def test_reencryption_batch_bad_revisison(alice_backend_sock, realm):
    await realm_start_reencryption_maintenance(alice_backend_sock, realm, 2, {"alice": b"foo"})

    rep = await vlob_maintenance_get_reencryption_batch(alice_backend_sock, realm, 1)
    assert rep == {"status": "bad_encryption_revision"}

    rep = await realm_finish_reencryption_maintenance(alice_backend_sock, realm, 1, check_rep=False)
    assert rep == {"status": "bad_encryption_revision"}


@pytest.mark.trio
async def test_reencryption(alice, alice_backend_sock, realm, vlobs, vlob_atoms):
    with freeze_time("2000-01-02"):
        await realm_start_reencryption_maintenance(alice_backend_sock, realm, 2, {"alice": b"foo"})

    # Each participant should have received a message
    rep = await message_get(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "messages": [
            {
                "count": 1,
                "body": b"foo",
                "timestamp": Pendulum(2000, 1, 2),
                "sender": alice.device_id,
            }
        ],
    }

    async def _reencrypt_with_batch_of_2(expected_size):
        rep = await vlob_maintenance_get_reencryption_batch(alice_backend_sock, realm, 2, size=2)
        assert rep["status"] == "ok"
        assert len(rep["batch"]) == expected_size
        for entry in rep["batch"]:
            entry["blob"] = f"{entry['vlob_id']}::{entry['version']} reencrypted".encode()
        await vlob_maintenance_save_reencryption_batch(alice_backend_sock, realm, 2, rep["batch"])

    # Should have 2 batch to reencrypt
    await _reencrypt_with_batch_of_2(expected_size=2)
    await _reencrypt_with_batch_of_2(expected_size=1)
    await _reencrypt_with_batch_of_2(expected_size=0)

    # Finish the reencryption
    await realm_finish_reencryption_maintenance(alice_backend_sock, realm, 2)

    # Check the vlob have changed
    for vlob_id, version in vlob_atoms:
        rep = await vlob_read(alice_backend_sock, vlob_id, version, encryption_revision=2)
        assert rep["blob"] == f"{vlob_id}::{version} reencrypted".encode()
