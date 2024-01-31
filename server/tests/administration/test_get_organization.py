# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from tests.common import Backend, CoolorgRpcClients


async def test_get_organization_auth(client: httpx.AsyncClient, coolorg: CoolorgRpcClients) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    # No Authorization header
    response = await client.get(url)
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.get(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.get(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


async def test_bad_method(
    client: httpx.AsyncClient, backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 405, response.content


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "active_users_limit": None,
        "is_bootstrapped": True,
        "is_expired": False,
        "user_profile_outsider_allowed": True,
    }


async def test_404(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}
