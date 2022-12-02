# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import ANY

import pytest
import trio

from parsec._parsec import DateTime
from parsec.api.protocol import (
    BlockID,
    HandshakeOrganizationExpired,
    OrganizationID,
    UserProfile,
    VlobID,
)
from parsec.api.rest import organization_stats_rep_serializer
from parsec.backend.backend_events import BackendEvent
from parsec.backend.organization import Organization
from tests.common import customize_fixtures, local_device_to_backend_user


@pytest.mark.trio
@pytest.mark.parametrize("bad_auth_reason", ["no_header", "bad_header"])
async def test_administration_api_auth(backend_asgi_app, coolorg, bad_auth_reason):

    if bad_auth_reason == "bad_header":
        headers = {"authorization": "Bearer dummy"}
    else:
        assert bad_auth_reason == "no_header"
        headers = {}

    client = backend_asgi_app.test_client()

    response = await client.get(
        f"/administration/organizations/{coolorg.organization_id.str}", headers=headers
    )
    assert response.status_code == 403
    assert await response.get_json() == {"error": "not_allowed"}

    response = await client.patch(
        f"/administration/organizations/{coolorg.organization_id.str}", json={}, headers=headers
    )
    assert response.status_code == 403
    assert await response.get_json() == {"error": "not_allowed"}

    response = await client.post(f"/administration/organizations", json={}, headers=headers)
    assert response.status_code == 403
    assert await response.get_json() == {"error": "not_allowed"}


### organization_create ###


@pytest.mark.trio
async def test_organization_create(backend_asgi_app):
    organization_id = OrganizationID("NewOrg")
    client = backend_asgi_app.test_client()

    response = await client.post(
        f"/administration/organizations",
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
        json={"organization_id": organization_id.str},
    )

    response_content = await response.get_json()
    assert response.status_code == 200
    assert response_content == {"bootstrap_token": ANY}

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=response_content["bootstrap_token"],
        is_expired=False,
        created_on=ANY,
        bootstrapped_on=None,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services_certificates=None,
    )


@pytest.mark.trio
async def test_organization_create_bad_data(backend_asgi_app):
    organization_id = OrganizationID("NewOrg")
    for bad_body in [
        # Bad OrganizationID
        {"organization_id": ""},  # Empty
        {"organization_id": "x" * 33},  # Too long
        {"organization_id": "My!Org"},  # Forbidden characters
        {"organization_id": "C%C3%A9TAC%C3%A9"},  # Unexpected url escape (so forbidden characters)
        # Missing required field
        {"active_users_limit": 10},
        # Bad field value
        {"organization_id": organization_id.str, "active_users_limit": -1},
        {"organization_id": organization_id.str, "active_users_limit": "foo"},
        {"organization_id": organization_id.str, "user_profile_outsider_allowed": 42},
        {"organization_id": organization_id.str, "user_profile_outsider_allowed": "foo"},
    ]:
        client = backend_asgi_app.test_client()
        response = await client.post(
            "/administration/organizations",
            json=bad_body,
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 400
        assert await response.get_json() == {"error": "bad_data", "reason": ANY}


@pytest.mark.trio
@pytest.mark.parametrize("expired", (False, True))
async def test_organization_create_already_exists_not_bootstrapped(backend_asgi_app, expired):
    organization_id = OrganizationID("NewOrg")
    original_bootstrap_token = "123"
    await backend_asgi_app.backend.organization.create(
        id=organization_id, bootstrap_token=original_bootstrap_token
    )
    if expired:
        await backend_asgi_app.backend.organization.update(id=organization_id, is_expired=True)

    client = backend_asgi_app.test_client()

    response = await client.post(
        f"/administration/organizations",
        json={"organization_id": organization_id.str},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    response_content = await response.get_json()
    assert response_content == {"bootstrap_token": ANY}

    # Token should be regenerated each time, and the configuration should be overwritten
    assert response_content["bootstrap_token"] != original_bootstrap_token

    org = await backend_asgi_app.backend.organization.get(id=organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=response_content["bootstrap_token"],
        is_expired=False,
        created_on=ANY,
        bootstrapped_on=None,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services_certificates=None,
    )


@pytest.mark.trio
async def test_organization_create_already_exists_and_bootstrapped(backend_asgi_app, coolorg):
    client = backend_asgi_app.test_client()

    response = await client.post(
        f"/administration/organizations",
        json={"organization_id": coolorg.organization_id.str},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 400
    assert await response.get_json() == {"error": "already_exists"}


@pytest.mark.trio
async def test_organization_create_with_custom_initial_config(backend_asgi_app):
    organization_id = OrganizationID("NewOrg")
    original_bootstrap_token = "123"

    await backend_asgi_app.backend.organization.create(
        id=organization_id, bootstrap_token=original_bootstrap_token
    )
    client = backend_asgi_app.test_client()
    response = await client.post(
        f"/administration/organizations",
        json={
            "organization_id": organization_id.str,
            "user_profile_outsider_allowed": False,
            "active_users_limit": None,
        },
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    response_content = await response.get_json()
    assert response_content == {"bootstrap_token": ANY}

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=response_content["bootstrap_token"],
        is_expired=False,
        created_on=ANY,
        bootstrapped_on=None,
        root_verify_key=None,
        user_profile_outsider_allowed=False,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services_certificates=None,
    )

    # New custom initial config should be taken into account each time the org is recreated
    response = await client.post(
        f"/administration/organizations",
        json={
            "organization_id": organization_id.str,
            "user_profile_outsider_allowed": True,
            "active_users_limit": 10,
        },
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    response_content = await response.get_json()
    assert response.status_code == 200
    assert response_content == {"bootstrap_token": ANY}

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=response_content["bootstrap_token"],
        is_expired=False,
        created_on=ANY,
        bootstrapped_on=None,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=10,
        sequester_authority=None,
        sequester_services_certificates=None,
    )

    # Default initial config should also be used if org is recreated without custom config
    response = await client.post(
        f"/administration/organizations",
        json={"organization_id": organization_id.str},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    response_content = await response.get_json()
    assert response_content == {"bootstrap_token": ANY}

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=response_content["bootstrap_token"],
        is_expired=False,
        created_on=ANY,
        bootstrapped_on=None,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services_certificates=None,
    )


### organization_config ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_config_not_found(backend_asgi_app, type):
    org = "dummy" if type == "unknown" else "x" * 33
    client = backend_asgi_app.test_client()
    response = await client.get(
        f"/administration/organizations/{org}",
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 404
    assert await response.get_json() == {"error": "not_found"}


@pytest.mark.trio
@pytest.mark.parametrize("bootstrapped", (True, False))
async def test_organization_config_ok(backend_asgi_app, coolorg, bootstrapped):
    if not bootstrapped:
        organization_id = OrganizationID("NewOrg")
        await backend_asgi_app.backend.organization.create(
            id=organization_id, bootstrap_token="123"
        )
    else:
        organization_id = coolorg.organization_id

    client = backend_asgi_app.test_client()

    response = await client.get(
        f"/administration/organizations/{organization_id.str}",
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    assert await response.get_json() == {
        "active_users_limit": None,
        "is_bootstrapped": bootstrapped,
        "is_expired": False,
        "user_profile_outsider_allowed": True,
    }

    # Ensure config change is taken into account
    await backend_asgi_app.backend.organization.update(
        id=organization_id, active_users_limit=42, is_expired=True
    )
    response = await client.get(
        f"/administration/organizations/{organization_id.str}",
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    assert await response.get_json() == {
        "active_users_limit": 42,
        "is_bootstrapped": bootstrapped,
        "is_expired": True,
        "user_profile_outsider_allowed": True,
    }


### organization_update ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_update_not_found(backend_asgi_app, type):
    org = "dummy" if type == "unknown" else "x" * 33
    client = backend_asgi_app.test_client()
    response = await client.patch(
        f"/administration/organizations/{org}",
        json={"bootstrap_token": "123"},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 404
    assert await response.get_json() == {"error": "not_found"}

    # Empty update is an interesting case
    response = await client.patch(
        f"/administration/organizations/dummy",
        json={},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 404
    assert await response.get_json() == {"error": "not_found"}


@pytest.mark.trio
@pytest.mark.parametrize("bootstrapped", (True, False))
async def test_organization_update_ok(backend_asgi_app, coolorg, bootstrapped):
    if not bootstrapped:
        organization_id = OrganizationID("NewOrg")
        await backend_asgi_app.backend.organization.create(
            id=organization_id, bootstrap_token="123"
        )
    else:
        organization_id = coolorg.organization_id

    client = backend_asgi_app.test_client()

    with backend_asgi_app.backend.event_bus.listen() as spy:
        response = await client.patch(
            f"/administration/organizations/{organization_id.str}",
            json={"user_profile_outsider_allowed": False, "active_users_limit": 10},
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        assert await response.get_json() == {}

        org = await backend_asgi_app.backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit == 10

        # Partial update
        response = await client.patch(
            f"/administration/organizations/{organization_id.str}",
            json={"active_users_limit": None},
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        assert await response.get_json() == {}

        org = await backend_asgi_app.backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit is None

        # Partial update with unknown field
        response = await client.patch(
            f"/administration/organizations/{organization_id.str}",
            json={"dummy": "whatever"},
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        assert await response.get_json() == {}

        # Empty update
        response = await client.patch(
            f"/administration/organizations/{organization_id.str}",
            json={},
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        assert await response.get_json() == {}

        org = await backend_asgi_app.backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit is None

    # No BackendEvent.ORGANIZATION_EXPIRED should have occured
    await trio.testing.wait_all_tasks_blocked()
    assert spy.events == []


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_bootstsrap_expired_organization(backend_asgi_app, backend, alice, coolorg):
    bootstrap_token = "123"
    await backend_asgi_app.backend.organization.create(
        id=coolorg.organization_id, bootstrap_token=bootstrap_token
    )
    await backend_asgi_app.backend.organization.update(id=coolorg.organization_id, is_expired=True)

    # Bootstrap should go fine
    backend_user, backend_first_device = local_device_to_backend_user(alice, coolorg)
    await backend.organization.bootstrap(
        id=coolorg.organization_id,
        user=backend_user,
        first_device=backend_first_device,
        bootstrap_token=bootstrap_token,
        root_verify_key=coolorg.root_verify_key,
    )

    # Once bootstrapped, the organization is still expired
    org = await backend.organization.get(id=coolorg.organization_id)
    assert org.is_expired is True


@pytest.mark.trio
async def test_organization_update_expired_field(
    backend_asgi_app, coolorg, alice, backend_authenticated_ws_factory
):
    organization_id = coolorg.organization_id

    client = backend_asgi_app.test_client()

    with backend_asgi_app.backend.event_bus.listen() as spy:
        response = await client.patch(
            f"/administration/organizations/{organization_id.str}",
            json={"is_expired": True},
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        assert await response.get_json() == {}
        await spy.wait_with_timeout(
            BackendEvent.ORGANIZATION_EXPIRED, {"organization_id": organization_id}
        )

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org.is_expired is True

    # Not longer allowed to use the organization
    with pytest.raises(HandshakeOrganizationExpired):
        async with backend_authenticated_ws_factory(backend_asgi_app, alice):
            pass

    # Re-enable the organization
    response = await client.patch(
        f"/administration/organizations/{organization_id.str}",
        json={"is_expired": False},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200
    assert await response.get_json() == {}

    org = await backend_asgi_app.backend.organization.get(organization_id)
    assert org.is_expired is False

    # Now device can connect to the organization

    async with backend_authenticated_ws_factory(backend_asgi_app, alice):
        pass


# `active_users_limit` is already tested in test/backend/user/test_user_create.py


@pytest.mark.trio
async def test_organization_update_bad_data(backend_asgi_app, coolorg):
    client = backend_asgi_app.test_client()
    for bad_body in [
        # Bad field value
        {"active_users_limit": -1},
        {"active_users_limit": "foo"},
        {"user_profile_outsider_allowed": 42},
        {"user_profile_outsider_allowed": "foo"},
    ]:
        response = await client.patch(
            f"/administration/organizations/{coolorg.organization_id.str}",
            json=bad_body,
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 400
        assert await response.get_json() == {"error": "bad_data", "reason": ANY}


### organization_stats ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_stats_not_found(backend_asgi_app, type):
    org = "dummy" if type == "unknown" else "x" * 33
    client = backend_asgi_app.test_client()
    response = await client.get(
        f"/administration/organizations/{org}/stats",
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 404
    assert await response.get_json() == {"error": "not_found"}


@pytest.mark.trio
async def test_organization_stats_data(backend_asgi_app, realm, realm_factory, alice):
    client = backend_asgi_app.test_client()

    async def organization_stats():
        response = await client.get(
            f"/administration/organizations/{alice.organization_id.str}/stats",
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        return organization_stats_rep_serializer.load(await response.get_json())

    rep = await organization_stats()
    assert rep == {
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
    initial_metadata_size = rep["metadata_size"]

    # Create new metadata
    await backend_asgi_app.backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=DateTime.now(),
        blob=b"1234",
    )
    rep = await organization_stats()
    assert rep == {
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
    await backend_asgi_app.backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        block_id=BlockID.new(),
        realm_id=realm,
        block=b"1234",
    )
    rep = await organization_stats()
    assert rep == {
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
    await realm_factory(backend_asgi_app.backend, alice)
    rep = await organization_stats()
    assert rep == {
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
):
    client = backend_asgi_app.test_client()

    async def organization_stats(organization_id):
        response = await client.get(
            f"/administration/organizations/{organization_id.str}/stats",
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200
        return organization_stats_rep_serializer.load(await response.get_json())

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

    for profile in UserProfile.values():
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
        stats = await organization_stats(org.organization_id)
        assert stats == expected_stats

        await binder.bind_revocation(device.user_id, certifier=godfrey1)
        expected_stats["active_users"] -= 1
        expected_stats["users_per_profile_detail"][i]["active"] -= 1
        expected_stats["users_per_profile_detail"][i]["revoked"] += 1
        stats = await organization_stats(org.organization_id)
        assert stats == expected_stats

    # Also make sure stats are isolated between organizations
    otherorg_device = local_device_factory(org=otherorg, profile=UserProfile.ADMIN)
    await binder.bind_organization(otherorg, otherorg_device, initial_user_manifest="not_synced")
    stats = await organization_stats(otherorg_device.organization_id)
    assert stats == {
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


@pytest.mark.trio
async def test_handles_escaped_path(backend_asgi_app):
    organization_id = "CéTACé"
    escaped_organization_id = "C%C3%A9TAC%C3%A9"
    bad_escaped_organization_id = "C%C3%A9TAC%+C3%A9"

    ROUTES_PATTERN = (
        "/administration/organizations/{organization_id}",
        "/administration/organizations/{organization_id}/stats",
    )

    client = backend_asgi_app.test_client()

    # Not found
    for route_pattern in ROUTES_PATTERN:
        route = route_pattern.format(organization_id=escaped_organization_id)
        response = await client.get(
            route,
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 404, route
        assert await response.get_json() == {"error": "not_found"}, route

    # Now create the org
    response = await client.post(
        f"/administration/organizations",
        json={"organization_id": organization_id},
        headers={"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"},
    )
    assert response.status_code == 200

    # Found !
    for route_pattern in ROUTES_PATTERN:
        route = route_pattern.format(organization_id=escaped_organization_id)
        response = await client.get(
            route,
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 200, route

        route = route_pattern.format(organization_id=bad_escaped_organization_id)
        response = await client.get(
            route,
            headers={
                "Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"
            },
        )
        assert response.status_code == 404, route
