# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    OrganizationID,
)
from parsec.components.organization import OrganizationDump
from tests.common import Backend


async def test_create_organization_auth(client: httpx.AsyncClient) -> None:
    url = "http://parsec.invalid/administration/organizations"
    # No Authorization header
    response = await client.post(url)
    assert response.status_code == 403
    # Invalid Authorization header
    response = await client.post(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403
    # Bad bearer token
    response = await client.post(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403


@pytest.mark.parametrize(
    "args",
    (
        {},
        {"active_user_limit": 2, "user_profile_outsider_allowed": True},
        {"active_user_limit": None, "user_profile_outsider_allowed": False},
    ),
)
async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    args: dict,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    org_id = OrganizationID("MyNewOrg")
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "organization_id": org_id.str,
            **args,
        },
    )
    assert response.status_code == 200
    assert response.json() == {"bootstrap_token": ANY}

    expected_active_users_limit = ActiveUsersLimit.from_maybe_int(
        args.get("active_users_limit", None)
    )
    expected_user_profile_outsider_allowed = args.get("user_profile_outsider_allowed", True)
    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        org_id: OrganizationDump(
            organization_id=org_id,
            is_bootstrapped=False,
            is_expired=False,
            active_users_limit=expected_active_users_limit,
            user_profile_outsider_allowed=expected_user_profile_outsider_allowed,
        )
    }
