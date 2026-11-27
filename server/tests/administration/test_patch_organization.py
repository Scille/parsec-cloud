# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, NotRequired, TypedDict
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import ActiveUsersLimit, DateTime
from parsec.components.organization import (
    Organization,
    OrganizationDump,
    TermsOfService,
    TosLocale,
    TosUrl,
)
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
        "bad_type_realm_minimum_archiving_period_before_deletion",
        "bad_value_realm_minimum_archiving_period_before_deletion",
        "bad_type_tos",
        "bad_value_tos",
        "bad_type_expired_on",
        "bad_value_expired_on",
        "bad_data_is_expired_and_expired_on",
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
        case "bad_type_realm_minimum_archiving_period_before_deletion":
            body_args = {"json": {"realm_minimum_archiving_period_before_deletion": "42"}}
        case "bad_value_realm_minimum_archiving_period_before_deletion":
            body_args = {"json": {"realm_minimum_archiving_period_before_deletion": -1}}
        case "bad_type_tos":
            body_args = {"json": {"tos": "{}"}}
        case "bad_value_tos":
            body_args = {"json": {"tos": {"fr": 42}}}
        case "bad_type_expired_on":
            # Only a real RFC 3339 date is valid !
            body_args = {"json": {"expired_on": "not a date"}}
        case "bad_value_expired_on":
            body_args = {"json": {"expired_on": 42}}
        case "bad_data_is_expired_and_expired_on":
            # `is_expired` and `expired_on` are mutually exclusive
            body_args = {"json": {"is_expired": True, "expired_on": "2026-01-01T00:00:00Z"}}
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
    expired_on: NotRequired[str | None]
    active_user_limit: NotRequired[int]
    user_profile_outsider_allowed: NotRequired[bool]
    tos: NotRequired[dict[TosLocale, TosUrl]]


@pytest.mark.parametrize(
    "params",
    (
        {},
        {"is_expired": True},
        {"expired_on": "2026-01-01T00:00:00Z"},
        {"expired_on": None},
        {"active_users_limit": 1},
        {"user_profile_outsider_allowed": False},
        {"realm_minimum_archiving_period_before_deletion": 1},
        {
            "is_expired": False,
            "active_users_limit": None,
            "user_profile_outsider_allowed": True,
            "realm_minimum_archiving_period_before_deletion": 0,
            "tos": {"en_HK": "https://parsec.invalid/tos_en"},
        },
        {
            "expired_on": None,
            "active_users_limit": None,
            "user_profile_outsider_allowed": True,
            "realm_minimum_archiving_period_before_deletion": 0,
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

    dump = await backend.organization.list_organizations()
    org = await backend.organization.get(coolorg.organization_id)
    assert isinstance(org, Organization)
    # The two fields are mutually exclusive, and `is_expired: true` is equivalent
    # to `expired_on: <now>`, while `expired_on: None` / `is_expired: false` cancels
    # the expiration.
    expected_expired_on: DateTime | None
    if params.get("is_expired", False):
        assert "expired_on" not in params
        assert org.expired_on is not None
        expected_expired_on = org.expired_on
    elif "expired_on" in params:
        raw_expected = params["expired_on"]
        if raw_expected is None:
            expected_expired_on = None
            assert org.expired_on is None
        else:
            expected_expired_on = DateTime.from_rfc3339(raw_expected)
            assert org.expired_on == expected_expired_on
    else:
        assert org.expired_on is None
        expected_expired_on = None
    expected_is_expired = expected_expired_on is not None and expected_expired_on <= DateTime.now()
    assert dump[coolorg.organization_id].is_expired(DateTime.now()) is expected_is_expired
    assert dump[coolorg.organization_id] == OrganizationDump(
        organization_id=coolorg.organization_id,
        created_on=org.created_on,
        bootstrap_token=ANY,
        is_bootstrapped=True,
        bootstrapped_on=org.bootstrapped_on,
        expired_on=org.expired_on,
        active_users_limit=ActiveUsersLimit.from_maybe_int(params.get("active_users_limit", None)),
        user_profile_outsider_allowed=params.get("user_profile_outsider_allowed", True),
        realm_minimum_archiving_period_before_deletion=params.get(
            "realm_minimum_archiving_period_before_deletion", 2592000
        ),
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

        dump = await backend.organization.list_organizations()
        assert dump[coolorg.organization_id].is_expired(DateTime.now()) is True

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

        dump = await backend.organization.list_organizations()
        assert dump[coolorg.organization_id].is_expired(DateTime.now()) is True

        # Cancel expiration (using the legacy `is_expired: false` spelling)

        response = await administration_client.patch(
            url,
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.list_organizations()
        assert dump[coolorg.organization_id].is_expired(DateTime.now()) is False

        # Re-cancel expiration, should be a no-op

        response = await administration_client.patch(
            url,
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.list_organizations()
        assert dump[coolorg.organization_id].is_expired(DateTime.now()) is False

        # Schedule a future-dated expiration: the organization is not expired
        # yet (no event), and auto-expires when the date becomes due

        scheduled_on = DateTime.now().add(minutes=1)
        response = await administration_client.patch(
            url,
            json={"expired_on": scheduled_on.to_rfc3339()},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        org = await backend.organization.get(coolorg.organization_id)
        assert isinstance(org, Organization)
        assert org.expired_on == scheduled_on
        dump = await backend.organization.list_organizations()
        assert dump[coolorg.organization_id].is_expired(DateTime.now()) is False

        # Once due, the organization is considered expired (without any new
        # PATCH, the derivation is date-based)

        now_after_due = scheduled_on.add(seconds=1)
        assert dump[coolorg.organization_id].is_expired(now_after_due) is True

        # Cancel the scheduled expiration

        response = await administration_client.patch(
            url,
            json={"expired_on": None},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        org = await backend.organization.get(coolorg.organization_id)
        assert isinstance(org, Organization)
        assert org.expired_on is None


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

        dump = await backend.organization.list_organizations()
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

        dump = await backend.organization.list_organizations()
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

        dump = await backend.organization.list_organizations()
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
