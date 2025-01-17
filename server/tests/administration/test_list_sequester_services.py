# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from parsec.components.sequester import SequesterServiceType
from tests.common import Backend, CoolorgRpcClients, SequesteredOrgRpcClients


async def test_bad_auth(
    client: httpx.AsyncClient, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
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
    client: httpx.AsyncClient, backend: Backend, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 405, response.content


async def test_unknown_organization(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/sequester/services"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}


async def test_not_sequestered_organization(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/sequester/services"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester disabled"}


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "services": [
            {
                "service_id": sequestered_org.sequester_service_1_id.hex,
                "service_label": "Sequester Service 1",
                "created_on": "2000-01-11T00:00:00Z",
                "revoked_on": "2000-01-16T00:00:00Z",
                "type": "storage",
            },
            {
                "service_id": sequestered_org.sequester_service_2_id.hex,
                "service_label": "Sequester Service 2",
                "created_on": "2000-01-18T00:00:00Z",
                "revoked_on": None,
                "type": "storage",
            },
        ],
    }

    # Update service config

    outcome = await backend.sequester.update_config_for_service(
        organization_id=sequestered_org.organization_id,
        service_id=sequestered_org.sequester_service_2_id,
        config=(SequesterServiceType.WEBHOOK, "https://parsec.invalid/webhook"),
    )
    assert outcome is None

    response = await client.get(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
    )
    assert response.status_code == 200, response.content
    assert response.json() == {
        "services": [
            {
                "service_id": sequestered_org.sequester_service_1_id.hex,
                "service_label": "Sequester Service 1",
                "created_on": "2000-01-11T00:00:00Z",
                "revoked_on": "2000-01-16T00:00:00Z",
                "type": "storage",
            },
            {
                "service_id": sequestered_org.sequester_service_2_id.hex,
                "service_label": "Sequester Service 2",
                "created_on": "2000-01-18T00:00:00Z",
                "revoked_on": None,
                "type": "webhook",
                "webhook_url": "https://parsec.invalid/webhook",
            },
        ],
    }
