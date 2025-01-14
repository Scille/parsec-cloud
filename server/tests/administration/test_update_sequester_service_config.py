# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from tests.common import Backend, SequesteredOrgRpcClients


async def test_bad_auth(
    client: httpx.AsyncClient, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/config"
    # No Authorization header
    response = await client.put(url)
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.put(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.put(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


async def test_bad_method(
    client: httpx.AsyncClient, backend: Backend, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/config"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "service_id": "460369bc70914615bf73ba30f896957e",
            "config": {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"},
        },
    )
    assert response.status_code == 405, response.content


async def test_unknown_organization(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/sequester/services/config"
    response = await client.put(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "service_id": "460369bc70914615bf73ba30f896957e",
            "config": {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"},
        },
    )
    assert response.status_code == 404, response.content
