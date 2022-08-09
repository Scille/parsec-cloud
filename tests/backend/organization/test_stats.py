# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from parsec._parsec import DateTime
from unittest.mock import ANY

from parsec.api.protocol import VlobID, BlockID, UserProfile

from tests.common import customize_fixtures
from tests.backend.common import organization_stats


@pytest.mark.trio
async def test_organization_stats_data(alice_ws, realm, realm_factory, alice, backend):
    stats = await organization_stats(alice_ws)
    assert stats == {
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
        "realms": 4,
    }
    initial_metadata_size = stats["metadata_size"]

    # Create new metadata
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime.now(),
        blob=b"1234",
    )
    stats = await organization_stats(alice_ws)
    assert stats == {
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
        "realms": 4,
    }

    # Create new data
    await backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        block_id=BlockID.new(),
        realm_id=realm,
        block=b"1234",
    )
    stats = await organization_stats(alice_ws)
    assert stats == {
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
        "realms": 4,
    }

    # create new workspace
    await realm_factory(backend, alice)
    stats = await organization_stats(alice_ws)
    assert stats == {
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
        "realms": 5,
    }


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_organization_stats_users(
    backend_asgi_app,
    backend_data_binder_factory,
    organization_factory,
    local_device_factory,
    otherorg,
    backend_authenticated_ws_factory,
):
    binder = backend_data_binder_factory(backend_asgi_app.backend)
    org = organization_factory("IFD")
    godfrey1 = local_device_factory(
        org=org,
        base_device_id="godfrey@d1",
        base_human_handle="Godfrey Ho <godfrey.ho@ifd.com>",
        profile=UserProfile.ADMIN,
    )
    await binder.bind_organization(org, godfrey1, initial_user_manifest="not_synced")

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
        "realms": 0,
    }

    async with backend_authenticated_ws_factory(backend_asgi_app, godfrey1) as sock:
        for profile in UserProfile:
            i = [
                i
                for i, v in enumerate(expected_stats["users_per_profile_detail"])
                if v["profile"] == profile
            ][0]
            device = local_device_factory(profile=profile, org=org)
            await binder.bind_device(device, certifier=godfrey1, initial_user_manifest="not_synced")
            expected_stats["users"] += 1
            expected_stats["active_users"] += 1
            expected_stats["users_per_profile_detail"][i]["active"] += 1
            stats = await organization_stats(sock)
            assert stats == expected_stats

            await binder.bind_revocation(device.user_id, certifier=godfrey1)
            expected_stats["active_users"] -= 1
            expected_stats["users_per_profile_detail"][i]["active"] -= 1
            expected_stats["users_per_profile_detail"][i]["revoked"] += 1
            stats = await organization_stats(sock)
            assert stats == expected_stats

    # Also make sure stats are isolated between organizations
    otherorg_device = local_device_factory(org=otherorg, profile=UserProfile.ADMIN)
    await binder.bind_organization(otherorg, otherorg_device, initial_user_manifest="not_synced")
    async with backend_authenticated_ws_factory(backend_asgi_app, otherorg_device) as sock:
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
        "metadata_size": 0,
        "realms": 0,
    }
