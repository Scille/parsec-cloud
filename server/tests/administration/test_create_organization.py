# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Iterator
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
from parsec.components.organization import (
    OrganizationDump,
    TermsOfService,
    UnsetType,
)
from tests.common import (
    AdminUnauthErrorsTester,
    Backend,
    MinimalorgRpcClients,
    next_organization_id,
)


async def test_create_organization_auth(
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = "http://parsec.invalid/administration/organizations"

    async def do(client: httpx.AsyncClient):
        return await client.post(url)

    await administration_route_unauth_errors_tester(do)


@pytest.mark.parametrize(
    "kind",
    (
        "invalid_json",
        "bad_type_organization_id",
        "bad_value_organization_id",
        "bad_type_user_profile_outsider_allowed",
        "bad_type_active_users_limit",
        "bad_value_active_users_limit",
        "bad_type_minimum_archiving_period",
        "bad_value_minimum_archiving_period",
        "bad_type_tos",
        "bad_value_tos",
        "bad_type_allowed_client_agent",
        "bad_value_allowed_client_agent",
        "bad_type_account_vault_strategy",
        "bad_value_account_vault_strategy",
    ),
)
async def test_bad_data(administration_client: httpx.AsyncClient, kind: str) -> None:
    url = "http://parsec.invalid/administration/organizations"

    body_args: dict[str, Any]
    match kind:
        case "invalid_json":
            body_args = {"content": b"<dummy>"}
        case "bad_type_organization_id":
            body_args = {"json": {"id": 42}}
        case "bad_value_organization_id":
            body_args = {"json": {"id": "<>"}}
        case "bad_type_user_profile_outsider_allowed":
            body_args = {"json": {"id": "CoolOrg", "user_profile_outsider_allowed": "True"}}
        case "bad_type_active_users_limit":
            body_args = {"json": {"id": "CoolOrg", "active_users_limit": "42"}}
        case "bad_value_active_users_limit":
            body_args = {"json": {"id": "CoolOrg", "active_users_limit": -1}}
        case "bad_type_minimum_archiving_period":
            body_args = {"json": {"id": "CoolOrg", "minimum_archiving_period": "42"}}
        case "bad_value_minimum_archiving_period":
            body_args = {"json": {"id": "CoolOrg", "minimum_archiving_period": -1}}
        case "bad_type_tos":
            body_args = {"json": {"id": "CoolOrg", "tos": "{}"}}
        case "bad_value_tos":
            body_args = {"json": {"id": "CoolOrg", "tos": {"fr": 42}}}
        case "bad_type_allowed_client_agent":
            body_args = {"json": {"id": "CoolOrg", "allowed_client_agent": None}}
        case "bad_value_allowed_client_agent":
            body_args = {"json": {"id": "CoolOrg", "allowed_client_agent": "dummy"}}
        case "bad_type_account_vault_strategy":
            body_args = {"json": {"id": "CoolOrg", "account_vault_strategy": 42}}
        case "bad_value_account_vault_strategy":
            body_args = {"json": {"id": "CoolOrg", "account_vault_strategy": "dummy"}}
        case unknown:
            assert False, unknown

    response = await administration_client.post(url, **body_args)
    assert response.status_code == 422, response.content


async def test_bad_method(
    administration_client: httpx.AsyncClient,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    org_id = OrganizationID("MyNewOrg")
    response = await administration_client.patch(
        url,
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
    administration_client: httpx.AsyncClient,
    backend: Backend,
    args: CreateOrganizationParams,
    organization_initial_params: None,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    org_id = next_organization_id(prefix="MyNewOrg")
    response = await administration_client.post(
        url,
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
    match args.get("active_users_limit", UnsetType.Unset):
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

    dump = await backend.organization.test_dump_organizations()
    assert dump[org_id] == OrganizationDump(
        organization_id=org_id,
        bootstrap_token=bootstrap_token,
        is_bootstrapped=False,
        is_expired=False,
        active_users_limit=expected_active_users_limit,
        user_profile_outsider_allowed=expected_user_profile_outsider_allowed,
        minimum_archiving_period=expected_minimum_archiving_period,
        tos=expected_tos,
    )


async def test_overwrite_existing(
    administration_client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    org_id = next_organization_id(prefix="MyNewOrg")
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
    assert dump[org_id] == OrganizationDump(
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

    url = "http://parsec.invalid/administration/organizations"
    response = await administration_client.post(
        url,
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
    assert dump[org_id] == OrganizationDump(
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


async def test_organization_already_bootstrapped(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = "http://parsec.invalid/administration/organizations"
    response = await administration_client.post(
        url,
        json={"organization_id": minimalorg.organization_id.str},
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Organization already exists"}
