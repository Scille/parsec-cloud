# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any, NotRequired, TypedDict
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import ActiveUsersLimit
from parsec.components.organization import OrganizationDump, TermsOfService
from parsec.config import AccountVaultStrategy, AllowedClientAgent
from parsec.events import EventOrganizationExpired, EventOrganizationTosUpdated
from tests.common import Backend, CoolorgRpcClients


async def test_get_organization_auth(client: httpx.AsyncClient, coolorg: CoolorgRpcClients) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    # No Authorization header
    response = await client.patch(url, json={"is_expired": True})
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.patch(
        url,
        headers={
            "Authorization": "DUMMY",
        },
        json={"is_expired": True},
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.patch(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
        json={"is_expired": True},
    )
    assert response.status_code == 403, response.content


@pytest.mark.parametrize("kind", ("bad_json", "bad_data"))
async def test_bad_data(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients, kind: str
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    body_args: dict[str, Any]
    match kind:
        case "bad_json":
            body_args = {"content": b"<dummy>"}
        case "bad_data":
            body_args = {"json": {"is_expired": "dummy"}}
        case unknown:
            assert False, unknown

    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        **body_args,
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={},
    )
    assert response.status_code == 405, response.content


class PatchOrganizationParams(TypedDict):
    is_expired: NotRequired[bool]
    active_user_limit: NotRequired[int]
    user_profile_outsider_allowed: NotRequired[bool]


@pytest.mark.parametrize(
    "params",
    (
        {},
        {"is_expired": True},
        {"active_user_limit": 1},
        {"user_profile_outsider_allowed": False},
        {"minimum_archiving_period": 1},
        {
            "is_expired": False,
            "active_user_limit": None,
            "user_profile_outsider_allowed": True,
            "minimum_archiving_period": 0,
            "tos": {"en_HK": "https://parsec.invalid/tos_en"},
            "allowed_client_agent": "NATIVE_ONLY",
            "account_vault_strategy": "FORBIDDEN",
        },
    ),
)
async def test_ok(
    params: PatchOrganizationParams,
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json=params,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {}

    dump = await backend.organization.test_dump_organizations()
    assert dump == {
        coolorg.organization_id: OrganizationDump(
            organization_id=coolorg.organization_id,
            bootstrap_token=ANY,
            is_bootstrapped=True,
            is_expired=params.get("is_expired", False),
            active_users_limit=ActiveUsersLimit.from_maybe_int(
                params.get("active_users_limit", None)
            ),
            user_profile_outsider_allowed=params.get("user_profile_outsider_allowed", True),
            minimum_archiving_period=params.get("minimum_archiving_period", 2592000),
            tos=TermsOfService(updated_on=ANY, per_locale_urls=params["tos"])
            if "tos" in params
            else None,
            allowed_client_agent=AllowedClientAgent(
                params.get("allowed_client_agent", "NATIVE_OR_WEB")
            ),
            account_vault_strategy=AccountVaultStrategy(
                params.get("account_vault_strategy", "ALLOWED")
            ),
        )
    }


async def test_expire_and_cancel_expire(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
):
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    with backend.event_bus.spy() as spy:
        # Expire

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is False

        # Re-cancel expiration, should be a no-op

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
            json={"is_expired": False},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        # Cancelling the expiration doesn't trigger any event

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].is_expired is False


async def test_set_unset_tos(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
):
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    # Set

    with backend.event_bus.spy() as spy:
        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
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

        response = await client.patch(
            url,
            headers={"Authorization": f"Bearer {backend.config.administration_token}"},
            json={"tos": None},
        )
        assert response.status_code == 200, response.content
        assert response.json() == {}

        await spy.wait_event_occurred(
            EventOrganizationTosUpdated(organization_id=coolorg.organization_id)
        )

        dump = await backend.organization.test_dump_organizations()
        assert dump[coolorg.organization_id].tos is None


async def test_404(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"is_expired": True},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}
