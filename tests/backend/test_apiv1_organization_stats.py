# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from uuid import uuid4
from unittest.mock import ANY

from parsec.api.data import UserProfile
from parsec.api.protocol import apiv1_organization_stats_serializer
from tests.backend.common import vlob_create, block_create
from tests.common import freeze_time


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
        device = local_device_factory(
            org=org,
            base_device_id="godfrey@d1",
            base_human_handle="Godfrey Ho <godfrey.ho@ifd.com>",
            profile=UserProfile.ADMIN,
        )
        with freeze_time("2000-01-01"):
            await binder.bind_organization(org, device)
        async with apiv1_backend_sock_factory(backend, "administration") as sock:

            yield binder, org, device, sock


@pytest.mark.trio
async def test_organization_stats_users(access_testbed, core_factory, local_device_factory):
    binder, org, godfrey1, sock = access_testbed
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
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
        "workspaces": 1,
    }

    nick1 = local_device_factory(
        base_device_id="el_murcielago_enmascarado_ii@d1",
        base_human_handle="Rodolfo Guzman Huerta",
        profile=UserProfile.ADMIN,
        org=org,
    )
    await binder.bind_device(nick1, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 2,
        "active_users": 2,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 2,
    }

    await binder.bind_revocation(nick1.user_id, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 2,
        "active_users": 1,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 1},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 2,
    }

    nick2 = local_device_factory(profile=UserProfile.STANDARD, org=org)
    await binder.bind_device(nick2, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 3,
        "active_users": 2,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 1},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 3,
    }

    await binder.bind_revocation(nick2.user_id, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 3,
        "active_users": 1,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 1},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 1},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 3,
    }

    nick3 = local_device_factory(profile=UserProfile.OUTSIDER, org=org)
    await binder.bind_device(nick3, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 4,
        "active_users": 2,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 1},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 1},
            {"profile": UserProfile.OUTSIDER, "active": 1, "revoked": 0},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 4,
    }

    await binder.bind_revocation(nick3.user_id, certifier=godfrey1)
    stats = await organization_stats(sock, org.organization_id)
    assert stats == {
        "status": "ok",
        "users": 4,
        "active_users": 1,
        "users_per_profile_detail": [
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 1},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 1},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 1},
        ],
        "data_size": 0,
        "metadata_size": ANY,
        "workspaces": 4,
    }
