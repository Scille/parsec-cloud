# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from parsec._parsec import ActiveUsersLimit, DateTime
from parsec.config import AccountVaultStrategy, AllowedClientAgent
from tests.common import AdminUnauthErrorsTester, Backend, CoolorgRpcClients


async def test_get_organization_auth(
    coolorg: CoolorgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"

    async def do(client: httpx.AsyncClient):
        return await client.get(url)

    await administration_route_unauth_errors_tester(do)


async def test_bad_method(
    administration_client: httpx.AsyncClient, coolorg: CoolorgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await administration_client.post(
        url,
    )
    assert response.status_code == 405, response.content


async def test_ok(
    administration_client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "active_users_limit": None,
        "is_bootstrapped": True,
        "is_expired": False,
        "user_profile_outsider_allowed": True,
        "minimum_archiving_period": 2592000,  # 30 days
        "tos": None,
        "allowed_client_agent": "NATIVE_OR_WEB",
        "account_vault_strategy": "ALLOWED",
    }

    # Also ensure the API reflects the changes

    await backend.organization.update(
        now=DateTime(2020, 1, 1),
        id=coolorg.organization_id,
        is_expired=True,
        active_users_limit=ActiveUsersLimit.limited_to(1),
        user_profile_outsider_allowed=False,
        minimum_archiving_period=10,
        tos={"en_HK": "https://parsec.invalid/tos_en"},
        allowed_client_agent=AllowedClientAgent.NATIVE_ONLY,
        account_vault_strategy=AccountVaultStrategy.FORBIDDEN,
    )

    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "active_users_limit": 1,
        "is_bootstrapped": True,
        "is_expired": True,
        "user_profile_outsider_allowed": False,
        "minimum_archiving_period": 10,
        "tos": {
            "updated_on": "2020-01-01T00:00:00Z",
            "per_locale_urls": {"en_HK": "https://parsec.invalid/tos_en"},
        },
        "allowed_client_agent": "NATIVE_ONLY",
        "account_vault_strategy": "FORBIDDEN",
    }


async def test_404(
    administration_client: httpx.AsyncClient,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy"
    response = await administration_client.get(
        url,
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}
