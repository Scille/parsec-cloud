# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, Iterator, NotRequired, TypedDict
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
from parsec.components.organization import (
    OrganizationDump,
    TermsOfService,
    Unset,
    UnsetType,
)
from parsec.config import AllowedClientAgent
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
    tos: NotRequired[dict[str, str]]
    minimum_archiving_period: NotRequired[int]


@pytest.fixture(params=("default", "custom"))
def organization_initial_params(backend: Backend, request) -> Iterator[None]:
    match request.param:
        case "default":
            pass
        case "custom":
            backend.config.organization_initial_active_users_limit = ActiveUsersLimit.limited_to(10)
            backend.config.organization_initial_user_profile_outsider_allowed = False
            backend.config.organization_initial_minimum_archiving_period = 1000
            backend.config.organization_initial_tos = {
                "fr_FR": "https://parsec.invalid/tos_fr.pdf",
                "en_US": "https://parsec.invalid/tos_en.pdf",
            }
            backend.config.organization_initial_allowed_client_agent = (
                AllowedClientAgent.NATIVE_ONLY
            )
        case unknown:
            assert False, unknown
    yield


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
    organization_initial_params: None,
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

    # Expected active_users_limit
    match args.get("active_users_limit", Unset):
        case UnsetType.Unset:
            expected_active_users_limit = backend.config.organization_initial_active_users_limit
        case None:
            expected_active_users_limit = ActiveUsersLimit.NO_LIMIT
        case active_user_limit:
            expected_active_users_limit = ActiveUsersLimit.limited_to(active_user_limit)

    # Expected user_profile_outsider_allowed
    expected_user_profile_outsider_allowed = args.get(
        "user_profile_outsider_allowed",
        backend.config.organization_initial_user_profile_outsider_allowed,
    )

    # Expected minimum_archiving_period
    expected_minimum_archiving_period = args.get(
        "minimum_archiving_period", backend.config.organization_initial_minimum_archiving_period
    )

    # Expected tos
    match args.get("tos", UnsetType.Unset):
        case UnsetType.Unset:
            match backend.config.organization_initial_tos:
                case None:
                    expected_tos = None
                case tos:
                    expected_tos = TermsOfService(updated_on=ANY, per_locale_urls=tos)
        case tos:
            expected_tos = TermsOfService(updated_on=ANY, per_locale_urls=tos)

    # Expected allowed_client_agent
    expected_allowed_client_agent = args.get(
        "allowed_client_agent", backend.config.organization_initial_allowed_client_agent
    )

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
            allowed_client_agent=expected_allowed_client_agent,
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
        allowed_client_agent=AllowedClientAgent.NATIVE_ONLY,
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
            allowed_client_agent=AllowedClientAgent.NATIVE_ONLY,
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
            "allowed_client_agent": "NATIVE_OR_WEB",
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
            allowed_client_agent=AllowedClientAgent.NATIVE_OR_WEB,
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
