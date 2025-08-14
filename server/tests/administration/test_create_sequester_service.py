# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64encode
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import (
    DateTime,
    SequesterPrivateKeyDer,
    SequesterServiceCertificate,
    SequesterServiceID,
    SequesterSigningKeyDer,
)
from parsec._parsec import (
    testbed as tb,
)
from parsec.components.sequester import (
    StorageSequesterService,
    WebhookSequesterService,
)
from tests.common import (
    AdminUnauthErrorsTester,
    Backend,
    CoolorgRpcClients,
    SequesteredOrgRpcClients,
)


async def test_bad_auth(
    sequestered_org: SequesteredOrgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"

    async def do(client: httpx.AsyncClient):
        return await client.post(url)

    await administration_route_unauth_errors_tester(do)


async def test_bad_method(
    administration_client: httpx.AsyncClient, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await administration_client.patch(
        url,
        json={"service_certificate": "ZHVtbXk=", "config": {"type": "storage"}},
    )
    assert response.status_code == 405, response.content


async def test_unknown_organization(
    administration_client: httpx.AsyncClient,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/sequester/services"
    response = await administration_client.post(
        url,
        json={"service_certificate": "ZHVtbXk=", "config": {"type": "storage"}},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}


@pytest.mark.parametrize("kind", ("storage", "webhook"))
async def test_ok(
    administration_client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    kind: str,
) -> None:
    certif = SequesterServiceCertificate(
        timestamp=DateTime.now(),
        service_id=SequesterServiceID.new(),
        service_label="Sequester Service 3",
        encryption_key_der=SequesterPrivateKeyDer.generate_pair(1024)[1],
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    match kind:
        case "storage":
            config = {"type": "storage"}
            expected_service = StorageSequesterService(
                service_id=certif.service_id,
                service_label=certif.service_label,
                service_certificate=certif_signed,
                created_on=certif.timestamp,
                revoked_on=None,
            )

        case "webhook":
            config = {"type": "webhook", "webhook_url": "https://parsec.invalid/webhook"}
            expected_service = WebhookSequesterService(
                service_id=certif.service_id,
                service_label=certif.service_label,
                service_certificate=certif_signed,
                created_on=certif.timestamp,
                revoked_on=None,
                webhook_url="https://parsec.invalid/webhook",
            )

        case unknown:
            assert False, unknown

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await administration_client.post(
        url,
        json={
            "service_certificate": b64encode(certif_signed).decode(),
            "config": config,
        },
    )
    assert response.status_code == 200, response.content
    assert response.json() == {}

    sequester_services = await backend.sequester.get_organization_services(
        sequestered_org.organization_id
    )
    assert isinstance(sequester_services, list)
    assert sorted(sequester_services, key=lambda s: s.service_label) == [
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
        expected_service,
    ]


@pytest.mark.parametrize("kind", ("serialization", "signature"))
async def test_invalid_certificate(
    administration_client: httpx.AsyncClient,
    sequestered_org: SequesteredOrgRpcClients,
    kind: str,
) -> None:
    match kind:
        case "serialization":
            certif_signed = b"dummy"
        case "signature":
            certif = SequesterServiceCertificate(
                timestamp=DateTime.now(),
                service_id=SequesterServiceID.new(),
                service_label="Sequester Service 3",
                encryption_key_der=SequesterPrivateKeyDer.generate_pair(1024)[1],
            )
            certif_signed = SequesterSigningKeyDer.generate_pair(1024)[0].sign(certif.dump())
        case unknown:
            assert False, unknown

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await administration_client.post(
        url,
        json={
            "service_certificate": b64encode(certif_signed).decode(),
            "config": {"type": "storage"},
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Invalid certificate"}


async def test_not_sequestered_organization(
    administration_client: httpx.AsyncClient,
    sequestered_org: SequesteredOrgRpcClients,
    coolorg: CoolorgRpcClients,
) -> None:
    other_org_new_sequester_service_event = next(
        e
        for e in sequestered_org.testbed_template.events
        if isinstance(e, tb.TestbedEventNewSequesterService)
    )

    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/sequester/services"
    response = await administration_client.post(
        url,
        json={
            "service_certificate": b64encode(
                other_org_new_sequester_service_event.raw_certificate
            ).decode(),
            "config": {"type": "storage"},
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester disabled"}


async def test_sequestered_service_already_exists(
    administration_client: httpx.AsyncClient,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    certif = SequesterServiceCertificate(
        timestamp=DateTime.now(),
        service_id=sequestered_org.sequester_service_2_id,
        service_label="Sequester Service 3",
        encryption_key_der=SequesterPrivateKeyDer.generate_pair(1024)[1],
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await administration_client.post(
        url,
        json={
            "service_certificate": b64encode(certif_signed).decode(),
            "config": {"type": "storage"},
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester service already exists"}


async def test_require_greater_timestamp(
    administration_client: httpx.AsyncClient,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    last_sequester_event = next(
        e
        for e in reversed(sequestered_org.testbed_template.events)
        if isinstance(e, tb.TestbedEventNewSequesterService | tb.TestbedEventRevokeSequesterService)
    )
    certif = SequesterServiceCertificate(
        timestamp=last_sequester_event.timestamp,
        service_id=SequesterServiceID.new(),
        service_label="Sequester Service 3",
        encryption_key_der=SequesterPrivateKeyDer.generate_pair(1024)[1],
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services"
    response = await administration_client.post(
        url,
        json={
            "service_certificate": b64encode(certif_signed).decode(),
            "config": {"type": "storage"},
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {
        "detail": {
            "msg": "Require greater timestamp",
            "strictly_greater_than": "2000-01-18T00:00:00Z",
        }
    }
