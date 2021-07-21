# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from uuid import uuid4
from unittest.mock import ANY

from parsec.api.data import UserProfile
from parsec.api.protocol import apiv1_organization_stats_serializer
from tests.backend.common import vlob_create, block_create


async def organization_stats(sock, organization_id):
    raw_rep = await sock.send(
        apiv1_organization_stats_serializer.req_dumps(
            {"cmd": "organization_stats", "organization_id": organization_id}
        )
    )
    raw_rep = await sock.recv()
    return apiv1_organization_stats_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_stats_data(
    coolorg, alice_backend_sock, administration_backend_sock, realm, realm_factory, alice, backend
):
    rep = await organization_stats(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "data_size": 0,
        "metadata_size": ANY,
        "users": 3,
        "active_users": 3,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "workspaces": 4,
    }
    initial_metadata_size = rep["metadata_size"]

    # Create new metadata
    await vlob_create(alice_backend_sock, realm_id=realm, vlob_id=uuid4(), blob=b"1234")
    rep = await organization_stats(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "data_size": 0,
        "metadata_size": initial_metadata_size + 4,
        "users": 3,
        "active_users": 3,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "workspaces": 4,
    }

    # Create new data
    await block_create(alice_backend_sock, realm_id=realm, block_id=uuid4(), block=b"1234")
    rep = await organization_stats(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "data_size": 4,
        "metadata_size": initial_metadata_size + 4,
        "users": 3,
        "active_users": 3,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "workspaces": 4,
    }

    # create new workspace
    await realm_factory(backend, alice)
    rep = await organization_stats(administration_backend_sock, coolorg.organization_id)
    assert rep == {
        "status": "ok",
        "data_size": 4,
        "metadata_size": initial_metadata_size + 4,
        "users": 3,
        "active_users": 3,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "workspaces": 5,
    }


@pytest.mark.trio
async def test_stats_unknown_organization(administration_backend_sock):
    rep = await organization_stats(administration_backend_sock, organization_id="dummy")
    assert rep == {"status": "not_found"}


@pytest.fixture
async def access_testbed(
    backend_factory,
    backend_data_binder_factory,
    apiv1_backend_sock_factory,
    organization_factory,
    local_device_factory,
):

    async with backend_factory(populated=False) as backend:
        binder = backend_data_binder_factory(backend)

        org = organization_factory("IFD")
        device = local_device_factory(org=org, profile=UserProfile.ADMIN)
        await binder.bind_organization(org, device, initial_user_manifest_in_v0=True)
        async with apiv1_backend_sock_factory(backend, "administration") as sock:

            yield binder, org, device, sock


@pytest.mark.trio
async def test_organization_stats_users(
    access_testbed, core_factory, local_device_factory, otherorg
):
    binder, org, godfrey1, sock = access_testbed
    expected_stats = {
        "status": "ok",
        "users": 1,
        "active_users": 1,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 0,
    }

    for profile in UserProfile:
        i = [
            i
            for i, v in enumerate(expected_stats["users_per_profile_detail"])
            if v["profile"] == profile
        ][0]
        device = local_device_factory(profile=profile, org=org)
        await binder.bind_device(device, certifier=godfrey1, initial_user_manifest_in_v0=True)
        expected_stats["users"] += 1
        expected_stats["active_users"] += 1
        expected_stats["users_per_profile_detail"][i]["active"] += 1
        stats = await organization_stats(sock, org.organization_id)
        assert stats == expected_stats

        await binder.bind_revocation(device.user_id, certifier=godfrey1)
        expected_stats["active_users"] -= 1
        expected_stats["users_per_profile_detail"][i]["active"] -= 1
        expected_stats["users_per_profile_detail"][i]["revoked"] += 1
        stats = await organization_stats(sock, org.organization_id)
        assert stats == expected_stats
    # Also make sure stats are isolated between organizations
    device = local_device_factory(org=otherorg, profile=UserProfile.ADMIN)
    await binder.bind_organization(otherorg, device, initial_user_manifest_in_v0=True)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == expected_stats
