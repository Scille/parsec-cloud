# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import pendulum
import pytest
from unittest.mock import ANY

from parsec.api.protocol import (
    VlobID,
    BlockID,
    OrganizationID,
    UserProfile,
    HandshakeOrganizationExpired,
)
from parsec.api.rest import organization_stats_rep_serializer
from parsec.backend.organization import Organization
from parsec.backend.backend_events import BackendEvent

from tests.common import customize_fixtures


@pytest.mark.trio
@pytest.mark.parametrize("bad_auth_reason", ["no_header", "bad_header"])
async def test_administration_api_auth(backend_rest_send, coolorg, bad_auth_reason):
    if bad_auth_reason == "bad_header":
        headers = {"authorization": "Bearer dummy"}
    else:
        assert bad_auth_reason == "no_header"
        headers = {}

    status, _, body = await backend_rest_send(
        f"/administration/organizations/{coolorg.organization_id}",
        method="GET",
        headers=headers,
        with_administration_token=False,
    )
    assert (status, body) == ((403, "Forbidden"), {"error": "not_allowed"})

    status, _, body = await backend_rest_send(
        f"/administration/organizations/{coolorg.organization_id}",
        method="PATCH",
        body={},
        headers=headers,
        with_administration_token=False,
    )
    assert (status, body) == ((403, "Forbidden"), {"error": "not_allowed"})

    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={},
        headers=headers,
        with_administration_token=False,
    )
    assert (status, body) == ((403, "Forbidden"), {"error": "not_allowed"})


### organization_create ###


@pytest.mark.trio
async def test_organization_create(backend, backend_rest_send):
    organization_id = OrganizationID("NewOrg")
    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={"organization_id": str(organization_id)},
    )
    assert (status, body) == ((200, "OK"), {"bootstrap_token": ANY})

    org = await backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=body["bootstrap_token"],
        is_expired=False,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority_key_certificate=None,
    )


@pytest.mark.trio
async def test_organization_create_bad_data(backend_rest_send):
    organization_id = "NewOrg"
    for bad_body in [
        # Bad OrganizationID
        {"organization_id": ""},  # Empty
        {"organization_id": "x" * 33},  # Too long
        {"organization_id": "My!Org"},  # Forbidden characters
        {"organization_id": "C%C3%A9TAC%C3%A9"},  # Unexpected url escape (so forbidden characters)
        # Missing required field
        {"active_users_limit": 10},
        # Bad field value
        {"organization_id": str(organization_id), "active_users_limit": -1},
        {"organization_id": str(organization_id), "active_users_limit": "foo"},
        {"organization_id": str(organization_id), "user_profile_outsider_allowed": 42},
        {"organization_id": str(organization_id), "user_profile_outsider_allowed": "foo"},
    ]:
        status, _, body = await backend_rest_send(
            f"/administration/organizations", method="POST", body=bad_body
        )
        assert (status, body) == ((400, "Bad Request"), {"error": "bad_data", "reason": ANY})


@pytest.mark.trio
@pytest.mark.parametrize("expired", (False, True))
async def test_organization_create_already_exists_not_bootstrapped(
    backend, backend_rest_send, expired
):
    organization_id = OrganizationID("NewOrg")
    original_bootstrap_token = "123"
    await backend.organization.create(id=organization_id, bootstrap_token=original_bootstrap_token)
    if expired:
        await backend.organization.update(id=organization_id, is_expired=True)

    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={"organization_id": str(organization_id)},
    )
    assert (status, body) == ((200, "OK"), {"bootstrap_token": ANY})

    # Token should be regenerated each time, and the configuration should be overwritten
    assert body["bootstrap_token"] != original_bootstrap_token

    org = await backend.organization.get(id=organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=body["bootstrap_token"],
        is_expired=False,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority_key_certificate=None,
    )


@pytest.mark.trio
async def test_organization_create_already_exists_and_bootstrapped(backend_rest_send, coolorg):
    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={"organization_id": str(coolorg.organization_id)},
    )
    assert (status, body) == ((400, "Bad Request"), {"error": "already_exists"})


@pytest.mark.trio
async def test_organization_create_with_custom_initial_config(backend, backend_rest_send):
    organization_id = OrganizationID("NewOrg")
    original_bootstrap_token = "123"
    await backend.organization.create(id=organization_id, bootstrap_token=original_bootstrap_token)

    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={
            "organization_id": str(organization_id),
            "user_profile_outsider_allowed": False,
            "active_users_limit": None,
        },
    )
    assert (status, body) == ((200, "OK"), {"bootstrap_token": ANY})

    org = await backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=body["bootstrap_token"],
        is_expired=False,
        root_verify_key=None,
        user_profile_outsider_allowed=False,
        active_users_limit=None,
        sequester_authority_key_certificate=None,
    )

    # New custom initial config should be taken into account each time the org is recreated
    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={
            "organization_id": str(organization_id),
            "user_profile_outsider_allowed": True,
            "active_users_limit": 10,
        },
    )
    assert (status, body) == ((200, "OK"), {"bootstrap_token": ANY})

    org = await backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=body["bootstrap_token"],
        is_expired=False,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=10,
        sequester_authority_key_certificate=None,
    )

    # Default initial config should also be used if org is recreated without custom config
    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={"organization_id": str(organization_id)},
    )
    assert (status, body) == ((200, "OK"), {"bootstrap_token": ANY})

    org = await backend.organization.get(organization_id)
    assert org == Organization(
        organization_id=organization_id,
        bootstrap_token=body["bootstrap_token"],
        is_expired=False,
        root_verify_key=None,
        user_profile_outsider_allowed=True,
        active_users_limit=None,
        sequester_authority_key_certificate=None,
    )


### organization_config ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_config_not_found(backend_rest_send, type):
    org = "dummy" if type == "unknown" else "x" * 33
    status, _, body = await backend_rest_send(f"/administration/organizations/{org}")
    assert (status, body) == ((404, "Not Found"), {"error": "not_found"})


@pytest.mark.trio
@pytest.mark.parametrize("bootstrapped", (True, False))
async def test_organization_config_ok(backend, backend_rest_send, coolorg, bootstrapped):
    if not bootstrapped:
        organization_id = OrganizationID("NewOrg")
        await backend.organization.create(id=organization_id, bootstrap_token="123")
    else:
        organization_id = coolorg.organization_id

    status, _, body = await backend_rest_send(f"/administration/organizations/{organization_id}")
    assert (status, body) == (
        (200, "OK"),
        {
            "active_users_limit": None,
            "is_bootstrapped": bootstrapped,
            "is_expired": False,
            "user_profile_outsider_allowed": True,
        },
    )

    # Ensure config change is taken into account
    await backend.organization.update(id=organization_id, active_users_limit=42, is_expired=True)
    status, _, body = await backend_rest_send(f"/administration/organizations/{organization_id}")
    assert (status, body) == (
        (200, "OK"),
        {
            "active_users_limit": 42,
            "is_bootstrapped": bootstrapped,
            "is_expired": True,
            "user_profile_outsider_allowed": True,
        },
    )


### organization_update ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_update_not_found(backend_rest_send, type):
    org = "dummy" if type == "unknown" else "x" * 33
    status, _, body = await backend_rest_send(
        f"/administration/organizations/{org}", method="PATCH", body={"bootstrap_token": "123"}
    )
    assert (status, body) == ((404, "Not Found"), {"error": "not_found"})

    # Empty update is an interesting case
    status, _, body = await backend_rest_send(
        f"/administration/organizations/dummy", method="PATCH", body={}
    )
    assert (status, body) == ((404, "Not Found"), {"error": "not_found"})


@pytest.mark.trio
@pytest.mark.parametrize("bootstrapped", (True, False))
async def test_organization_update_ok(backend, backend_rest_send, coolorg, bootstrapped):
    if not bootstrapped:
        organization_id = OrganizationID("NewOrg")
        await backend.organization.create(id=organization_id, bootstrap_token="123")
    else:
        organization_id = coolorg.organization_id

    with backend.event_bus.listen() as spy:

        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}",
            method="PATCH",
            body={"user_profile_outsider_allowed": False, "active_users_limit": 10},
        )
        assert (status, body) == ((200, "OK"), {})

        org = await backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit == 10

        # Partial update
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}",
            method="PATCH",
            body={"active_users_limit": None},
        )
        assert (status, body) == ((200, "OK"), {})

        org = await backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit is None

        # Partial update with unknown field
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}",
            method="PATCH",
            body={"dummy": "whatever"},
        )
        assert (status, body) == ((200, "OK"), {})

        # Empty update
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}", method="PATCH", body={}
        )
        assert (status, body) == ((200, "OK"), {})

        org = await backend.organization.get(organization_id)
        assert org.user_profile_outsider_allowed is False
        assert org.active_users_limit is None

    # No BackendEvent.ORGANIZATION_EXPIRED should have occured
    await trio.testing.wait_all_tasks_blocked()
    assert spy.events == []


@pytest.mark.trio
async def test_organization_update_expired_field(
    backend, backend_rest_send, coolorg, alice, backend_sock_factory
):
    organization_id = coolorg.organization_id

    with backend.event_bus.listen() as spy:
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}",
            method="PATCH",
            body={"is_expired": True},
        )
        assert (status, body) == ((200, "OK"), {})
        await spy.wait_with_timeout(
            BackendEvent.ORGANIZATION_EXPIRED, {"organization_id": organization_id}
        )

    org = await backend.organization.get(organization_id)
    assert org.is_expired is True

    # Not longer allowed to use the organization
    with pytest.raises(HandshakeOrganizationExpired):
        async with backend_sock_factory(backend, alice):
            pass

    # Re-enable the organization

    status, _, body = await backend_rest_send(
        f"/administration/organizations/{organization_id}",
        method="PATCH",
        body={"is_expired": False},
    )
    assert (status, body) == ((200, "OK"), {})

    org = await backend.organization.get(organization_id)
    assert org.is_expired is False

    # Now device can connect to the organization

    async with backend_sock_factory(backend, alice):
        pass


# `active_users_limit` is already tested in test/backend/user/test_user_create.py


@pytest.mark.trio
async def test_organization_update_bad_data(backend_rest_send, coolorg):
    for bad_body in [
        # Bad field value
        {"active_users_limit": -1},
        {"active_users_limit": "foo"},
        {"user_profile_outsider_allowed": 42},
        {"user_profile_outsider_allowed": "foo"},
    ]:
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{coolorg.organization_id}",
            method="PATCH",
            body=bad_body,
        )
        assert (status, body) == ((400, "Bad Request"), {"error": "bad_data", "reason": ANY})


### organization_stats ###


@pytest.mark.trio
@pytest.mark.parametrize("type", ("unknown", "invalid"))
async def test_organization_stats_not_found(backend_rest_send, type):
    org = "dummy" if type == "unknown" else "x" * 33
    status, _, body = await backend_rest_send(f"/administration/organizations/{org}/stats")
    assert (status, body) == ((404, "Not Found"), {"error": "not_found"})


@pytest.mark.trio
async def test_organization_stats_data(backend_rest_send, realm, realm_factory, alice, backend):
    async def organization_stats():
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{alice.organization_id}/stats"
        )
        assert status == (200, "OK")
        return organization_stats_rep_serializer.load(body)

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
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=VlobID.new(),
        timestamp=pendulum.now(),
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
    await backend.block.create(
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
    await realm_factory(backend, alice)
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
    backend,
    backend_data_binder_factory,
    organization_factory,
    local_device_factory,
    otherorg,
    backend_rest_send,
):
    async def organization_stats(organization_id):
        status, _, body = await backend_rest_send(
            f"/administration/organizations/{organization_id}/stats"
        )
        assert status == (200, "OK")
        return organization_stats_rep_serializer.load(body)

    binder = backend_data_binder_factory(backend)
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
async def test_handles_escaped_path(backend_rest_send):
    organization_id = "CéTACé"
    escaped_organization_id = "C%C3%A9TAC%C3%A9"
    bad_escaped_organization_id = "C%C3%A9TAC%+C3%A9"

    ROUTES_PATTERN = (
        "/administration/organizations/{organization_id}",
        "/administration/organizations/{organization_id}/stats",
    )

    # Not found
    for route_pattern in ROUTES_PATTERN:
        route = route_pattern.format(organization_id=escaped_organization_id)
        status, _, body = await backend_rest_send(route)
        assert (status, body) == ((404, "Not Found"), {"error": "not_found"}), route

    # Now create the org
    status, _, body = await backend_rest_send(
        f"/administration/organizations",
        method="POST",
        body={"organization_id": str(organization_id)},
    )

    # Found !
    for route_pattern in ROUTES_PATTERN:
        route = route_pattern.format(organization_id=escaped_organization_id)
        status, _, _ = await backend_rest_send(route)
        assert status == (200, "OK"), route

        route = route_pattern.format(organization_id=bad_escaped_organization_id)
        status, _, _ = await backend_rest_send(route)
        assert status == (404, "Not Found"), route
