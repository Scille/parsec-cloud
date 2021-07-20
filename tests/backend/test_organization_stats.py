# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from uuid import uuid4
from unittest.mock import ANY
from pendulum import now as pendulum_now
from tests.common import freeze_time

from parsec.api.protocol import organization_stats_serializer
from parsec.core.backend_connection import BackendConnectionError
from parsec.api.data import UserProfile
from parsec.core.logged_core import OrganizationStats


@pytest.mark.trio
async def test_organization_stats_data(
    running_backend, backend, realm, alice, alice_core, bob_core, otheralice_core
):
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail=[
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        data_size=0,
        metadata_size=ANY,
    )
    initial_metadata_size = organization_stats.metadata_size

    # Create new metadata
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        timestamp=pendulum_now(),
        realm_id=realm,
        vlob_id=uuid4(),
        blob=b"1234",
    )
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail=[
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        data_size=0,
        metadata_size=initial_metadata_size + 4,
    )

    # Create new data
    await backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        block_id=uuid4(),
        block=b"1234",
    )
    organization_stats = await alice_core.get_organization_stats()
    assert organization_stats == OrganizationStats(
        users=3,
        active_users=3,
        users_per_profile_detail=[
            {"profile": UserProfile.ADMIN, "active": 2, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 1, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        data_size=4,
        metadata_size=initial_metadata_size + 4,
    )

    # Bob is not admin, it should fail
    with pytest.raises(BackendConnectionError) as exc:
        await bob_core.get_organization_stats()
    assert (
        str(exc.value)
        == "Backend error: {'reason': 'User `bob` is not admin', 'status': 'not_allowed'}"
    )

    # Ensure organization isolation
    other_organization_stats = await otheralice_core.get_organization_stats()
    assert other_organization_stats == OrganizationStats(
        users=1,
        active_users=1,
        users_per_profile_detail=[
            {"profile": UserProfile.ADMIN, "active": 1, "revoked": 0},
            {"profile": UserProfile.STANDARD, "active": 0, "revoked": 0},
            {"profile": UserProfile.OUTSIDER, "active": 0, "revoked": 0},
        ],
        data_size=0,
        metadata_size=ANY,
    )


@pytest.fixture
async def access_testbed(
    backend_factory,
    backend_data_binder_factory,
    backend_sock_factory,
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

        async with backend_sock_factory(backend, device) as sock:

            yield binder, org, device, sock


async def organization_stats(sock, **kwargs):
    await sock.send(
        organization_stats_serializer.req_dumps({"cmd": "organization_stats", **kwargs})
    )
    raw_rep = await sock.recv()
    return organization_stats_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_organization_stats_users(access_testbed, core_factory, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    stats = await organization_stats(sock)
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
    }

    nick1 = local_device_factory(
        base_device_id="el_murcielago_enmascarado_ii@d1",
        base_human_handle="Rodolfo Guzman Huerta",
        profile=UserProfile.ADMIN,
        org=org,
    )
    await binder.bind_device(nick1, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }

    await binder.bind_revocation(nick1.user_id, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }

    nick2 = local_device_factory(profile=UserProfile.STANDARD, org=org)
    await binder.bind_device(nick2, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }

    await binder.bind_revocation(nick2.user_id, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }

    nick3 = local_device_factory(profile=UserProfile.OUTSIDER, org=org)
    await binder.bind_device(nick3, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }

    await binder.bind_revocation(nick3.user_id, certifier=godfrey1)
    stats = await organization_stats(sock)
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
    }
