# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, NotRequired, TypedDict
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
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.post(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.post(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


@pytest.mark.parametrize("kind", ("bad_json", "bad_data"))
async def test_bad_data(client: httpx.AsyncClient, backend: Backend, kind: str) -> None:
    url = "http://parsec.invalid/administration/organizations"

    body_args: dict[str, Any]
    match kind:
        case "bad_json":
            body_args = {"content": b"<dummy>"}
        case "bad_data":
            body_args = {"json": {"dummy": "dummy"}}
        case unknown:
            assert False, unknown

    response = await client.post(
        url, headers={"Authorization": f"Bearer {backend.config.administration_token}"}, **body_args
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    org_id = OrganizationID("MyNewOrg")
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "organization_id": org_id.str,
        },
    )
    assert response.status_code == 405, response.content


class CreateOrganizationParams(TypedDict):
    active_user_limit: NotRequired[int | None]
    user_profile_outsider_allowed: NotRequired[bool]


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
    cleanup_organizations: None,
    args: CreateOrganizationParams,
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
    assert response.status_code == 200, response.content
    assert response.json() == {"bootstrap_url": ANY}

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
