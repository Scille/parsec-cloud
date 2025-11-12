# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, NotRequired, TypedDict
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import ActiveUsersLimit
from parsec.components.organization import OrganizationDump, TermsOfService, TosLocale, TosUrl
from parsec.events import EventOrganizationExpired, EventOrganizationTosUpdated
from tests.common import AdminUnauthErrorsTester, Backend, CoolorgRpcClients


async def test_get_organization_auth(
    client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    async def do(client: httpx.AsyncClient) -> httpx.Response:
        return await client.patch(url, json={"is_expired": True})

    await administration_route_unauth_errors_tester(do)


@pytest.mark.parametrize(
    "kind",
    (
        "invalid_json",
        "bad_type_is_expired",
        "bad_type_user_profile_outsider_allowed",
        "bad_type_active_users_limit",
        "bad_value_active_users_limit",
        "bad_type_minimum_archiving_period",
        "bad_value_minimum_archiving_period",
        "bad_type_tos",
        "bad_value_tos",
    ),
)
async def test_bad_data(
    coolorg: CoolorgRpcClients,
    kind: str,
    administration_client: httpx.AsyncClient,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    body_args: dict[str, Any]
    match kind:
        case "invalid_json":
            body_args = {"content": b"<dummy>"}
        case "bad_type_is_expired":
            body_args = {"json": {"is_expired": "False"}}  # Only real bool is valid !
        case "bad_type_user_profile_outsider_allowed":
            body_args = {"json": {"user_profile_outsider_allowed": "True"}}
        case "bad_type_active_users_limit":
            body_args = {"json": {"active_users_limit": "42"}}
        case "bad_value_active_users_limit":
            body_args = {"json": {"active_users_limit": -1}}
        case "bad_type_minimum_archiving_period":
            body_args = {"json": {"minimum_archiving_period": "42"}}
        case "bad_value_minimum_archiving_period":
            body_args = {"json": {"minimum_archiving_period": -1}}
        case "bad_type_tos":
            body_args = {"json": {"tos": "{}"}}
        case "bad_value_tos":
            body_args = {"json": {"tos": {"fr": 42}}}
        case unknown:
            assert False, unknown

    response = await administration_client.patch(
        url,
        **body_args,
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await administration_client.post(
        url,
        json={},
    )
    assert response.status_code == 405, response.content


class PatchOrganizationParams(TypedDict):
    is_expired: NotRequired[bool]
    active_user_limit: NotRequired[int]
    user_profile_outsider_allowed: NotRequired[bool]
    tos: NotRequired[dict[TosLocale, TosUrl]]


@pytest.mark.parametrize(
    "params",
    (
        {},
        {"is_expired": True},
        {"active_users_limit": 1},
        {"user_profile_outsider_allowed": False},
        {"minimum_archiving_period": 1},
        {
            "is_expired": False,
            "active_users_limit": None,
            "user_profile_outsider_allowed": True,
            "minimum_archiving_period": 0,
            "tos": {"en_HK": "https://parsec.invalid/tos_en"},
        },
    ),
)
async def test_ok(
    params: PatchOrganizationParams,
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await administration_client.patch(
        url,
        json=params,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {}

    dump = await backend.organization.test_dump_organizations()
    assert dump[coolorg.organization_id] == OrganizationDump(
        organization_id=coolorg.organization_id,
        bootstrap_token=ANY,
        is_bootstrapped=True,
        is_expired=params.get("is_expired", False),
        active_users_limit=ActiveUsersLimit.from_maybe_int(params.get("active_users_limit", None)),
        user_profile_outsider_allowed=params.get("user_profile_outsider_allowed", True),
        minimum_archiving_period=params.get("minimum_archiving_period", 2592000),
        tos=TermsOfService(updated_on=ANY, per_locale_urls=params["tos"])
        if "tos" in params
        else None,
    )


async def test_expire_and_cancel_expire(
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
):
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    with backend.event_bus.spy() as spy:
        # Expire

        response = await administration_client.patch(
            url,
            json={"is_expired": True},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        await spy.wait_event_occurred(
            EventOrganizationExpired(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is True

        # Re-expire, should be a no-op

        response = await administration_client.patch(
            url,
            json={"is_expired": True},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Note that the event is triggered again even if it's a no-op (this is no big
        # deal and simplifies implementation)
        await spy.wait_event_occurred(
            EventOrganizationExpired(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is True

        # Cancel expiration

        response = await administration_client.patch(
            url,
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is False

        # Re-cancel expiration, should be a no-op

        response = await administration_client.patch(
            url,
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is False


async def test_set_unset_tos(
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
):
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    # Set

    with backend.event_bus.spy() as spy:
        response = await administration_client.patch(
            url,
            json={"tos": {"fr_CA": "https://parsec.invalid/tos_fr1"}},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        await spy.wait_event_occurred(
            EventOrganizationTosUpdated(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].tos == TermsOfService(
            updated_on=ANY, per_locale_urls={"fr_CA": "https://parsec.invalid/tos_fr1"}
        )

        # Update

        response = await administration_client.patch(
            url,
            json={
                "tos": {
                    "fr_CA": "https://parsec.invalid/tos_fr2",
                    "en_CA": "https://parsec.invalid/tos_en1",
                }
            },
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        await spy.wait_event_occurred(
            EventOrganizationTosUpdated(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].tos == TermsOfService(
            updated_on=ANY,
            per_locale_urls={
                "fr_CA": "https://parsec.invalid/tos_fr2",
                "en_CA": "https://parsec.invalid/tos_en1",
            },
        )

        # Unset

        response = await administration_client.patch(
            url,
            json={"tos": None},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        await spy.wait_event_occurred(
            EventOrganizationTosUpdated(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].tos is None


async def test_unknown_organization(
    administration_client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy"
    response = await administration_client.patch(
        url,
        json={"is_expired": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}
