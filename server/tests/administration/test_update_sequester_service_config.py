# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import (
    DateTime,
)
from parsec.components.sequester import (
    BaseSequesterService,
    StorageSequesterService,
    WebhookSequesterService,
)
from tests.common import Backend, CoolorgRpcClients, SequesteredOrgRpcClients


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


async def test_unknown_service(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/config"
    response = await client.put(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "service_id": "460369bc70914615bf73ba30f896957e",
            "config": {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"},
        },
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Sequester service not found"}


async def test_not_sequestered_organization(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    coolorg: CoolorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/sequester/services/config"
    response = await client.put(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "service_id": sequestered_org.sequester_service_2_id.hex,
            "config": {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"},
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester disabled"}


@pytest.mark.parametrize("kind", ("active", "revoked"))
async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    kind: str,
) -> None:
    expected_services: list[BaseSequesterService] = [
        StorageSequesterService(
            service_id=sequestered_org.sequester_service_1_id,
            service_label="Sequester Service 1",
            service_certificate=ANY,
            created_on=DateTime(2000, 1, 11),
            revoked_on=DateTime(2000, 1, 16),
        ),
        StorageSequesterService(
            service_id=sequestered_org.sequester_service_2_id,
            service_label="Sequester Service 2",
            service_certificate=ANY,
            created_on=DateTime(2000, 1, 18),
            revoked_on=None,
        ),
    ]
    match kind:
        case "active":
            service_id = sequestered_org.sequester_service_2_id
            expected_services[1] = WebhookSequesterService(
                service_id=expected_services[1].service_id,
                service_label=expected_services[1].service_label,
                service_certificate=expected_services[1].service_certificate,
                created_on=expected_services[1].created_on,
                revoked_on=expected_services[1].revoked_on,
                webhook_url="https://parsec.invalid/webhook",
            )
        case "revoked":
            service_id = sequestered_org.sequester_service_1_id
            expected_services[0] = WebhookSequesterService(
                service_id=expected_services[0].service_id,
                service_label=expected_services[0].service_label,
                service_certificate=expected_services[0].service_certificate,
                created_on=expected_services[0].created_on,
                revoked_on=expected_services[0].revoked_on,
                webhook_url="https://parsec.invalid/webhook",
            )
        case unknown:
            assert False, unknown

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/config"
    response = await client.put(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "service_id": service_id.hex,
            "config": {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"},
        },
    )
    assert response.status_code == 200, response.content
    assert response.json() == {}

    sequester_services = await backend.sequester.get_organization_services(
        sequestered_org.organization_id
    )
    assert isinstance(sequester_services, list)
    assert sorted(sequester_services, key=lambda s: s.service_label) == expected_services
