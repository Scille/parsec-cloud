# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, NotRequired, TypedDict
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    OrganizationID,
    ParsecOrganizationBootstrapAddr,
)
from parsec.components.organization import OrganizationDump, TermsOfService
from tests.common import Backend, MinimalorgRpcClients


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
        {
            "active_user_limit": 2,
            "user_profile_outsider_allowed": True,
            "minimum_archiving_period": 1,
            "tos": {
                "en_HK": "https://parsec.invalid/tos_en.pdf",
                "cn_HK": "https://parsec.invalid/tos_cn.pdf",
            },
        },
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
    body = response.json()
    assert body == {"bootstrap_url": ANY}
    bootstrap_token = ParsecOrganizationBootstrapAddr.from_url(body["bootstrap_url"]).token

    expected_active_users_limit = ActiveUsersLimit.from_maybe_int(
        args.get("active_users_limit", None)
    )
    expected_user_profile_outsider_allowed = args.get("user_profile_outsider_allowed", True)
    expected_minimum_archiving_period = args.get("minimum_archiving_period", 2592000)
    match args.get("tos", None):
        case None:
            expected_tos = None
        case tos:
            expected_tos = TermsOfService(updated_on=ANY, per_locale_urls=tos)

    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        org_id: OrganizationDump(
            organization_id=org_id,
            bootstrap_token=bootstrap_token,
            is_bootstrapped=False,
            is_expired=False,
            active_users_limit=expected_active_users_limit,
            user_profile_outsider_allowed=expected_user_profile_outsider_allowed,
            minimum_archiving_period=expected_minimum_archiving_period,
            tos=expected_tos,
        )
    }


async def test_overwrite_existing(
    client: httpx.AsyncClient,
    backend: Backend,
    cleanup_organizations: None,
) -> None:
    org_id = OrganizationID("MyNewOrg")
    bootstrap_token = BootstrapToken.new()

    t0 = DateTime.now()
    outcome = await backend.organization.create(
        now=t0,
        id=org_id,
        active_users_limit=ActiveUsersLimit.limited_to(1),
        user_profile_outsider_allowed=False,
        minimum_archiving_period=2,
        tos={"en_HK": "https://parsec.invalid/tos_en.pdf"},
        force_bootstrap_token=bootstrap_token,
    )
    assert isinstance(outcome, BootstrapToken)

    # Sanity check
    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        org_id: OrganizationDump(
            organization_id=org_id,
            bootstrap_token=bootstrap_token,
            is_bootstrapped=False,
            is_expired=False,
            active_users_limit=ActiveUsersLimit.limited_to(1),
            user_profile_outsider_allowed=False,
            minimum_archiving_period=2,
            tos=TermsOfService(
                updated_on=t0, per_locale_urls={"en_HK": "https://parsec.invalid/tos_en.pdf"}
            ),
        )
    }

    url = "http://parsec.invalid/administration/organizations"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "organization_id": org_id.str,
            "active_user_limit": None,
            "user_profile_outsider_allowed": True,
            "minimum_archiving_period": 1000,
            "tos": {"cn_HK": "https://parsec.invalid/tos_cn.pdf"},
        },
    )
    assert response.status_code == 200, response.content
    body = response.json()
    assert body == {"bootstrap_url": ANY}
    new_bootstrap_token = ParsecOrganizationBootstrapAddr.from_url(body["bootstrap_url"]).token
    assert new_bootstrap_token != bootstrap_token.hex

    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        org_id: OrganizationDump(
            organization_id=org_id,
            bootstrap_token=new_bootstrap_token,
            is_bootstrapped=False,
            is_expired=False,
            active_users_limit=ActiveUsersLimit.NO_LIMIT,
            user_profile_outsider_allowed=True,
            minimum_archiving_period=1000,
            tos=TermsOfService(
                updated_on=ANY, per_locale_urls={"cn_HK": "https://parsec.invalid/tos_cn.pdf"}
            ),
        )
    }


async def test_organization_already_bootstrapped(
    client: httpx.AsyncClient,
    backend: Backend,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"organization_id": minimalorg.organization_id.str},
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Organization already exists"}
