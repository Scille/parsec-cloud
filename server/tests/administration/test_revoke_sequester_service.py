# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64encode
from unittest.mock import ANY

import httpx
import pytest

from parsec._parsec import (
    DateTime,
    SequesterRevokedServiceCertificate,
    SequesterSigningKeyDer,
)
from parsec._parsec import (
    testbed as tb,
)
from parsec.components.sequester import (
    StorageSequesterService,
)
from tests.common import Backend, CoolorgRpcClients, SequesteredOrgRpcClients


async def test_bad_auth(
    client: httpx.AsyncClient, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    # No Authorization header
    response = await client.post(url)
    assert response.status_code == 403, response.content
    # Invalid Authorization header
    response = await client.post(
        url,
        headers={
            "Authorization": "DUMMY",
        },
    )
    assert response.status_code == 403, response.content
    # Bad bearer token
    response = await client.post(
        url,
        headers={
            "Authorization": "Bearer BADTOKEN",
        },
    )
    assert response.status_code == 403, response.content


async def test_bad_method(
    client: httpx.AsyncClient, backend: Backend, sequestered_org: SequesteredOrgRpcClients
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    response = await client.patch(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"revoked_service_certificate": "ZHVtbXk="},
    )
    assert response.status_code == 405, response.content


async def test_unknown_organization(
    client: httpx.AsyncClient,
    backend: Backend,
) -> None:
    url = "http://parsec.invalid/administration/organizations/Dummy/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={"revoked_service_certificate": "ZHVtbXk="},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}


async def test_ok(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    certif = SequesterRevokedServiceCertificate(
        timestamp=DateTime.now(),
        service_id=sequestered_org.sequester_service_2_id,
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "revoked_service_certificate": b64encode(certif_signed).decode(),
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
            revoked_on=certif.timestamp,
        ),
    ]


@pytest.mark.parametrize("kind", ("serialization", "signature"))
async def test_invalid_certificate(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    kind: str,
) -> None:
    match kind:
        case "serialization":
            certif_signed = b"dummy"
        case "signature":
            certif = SequesterRevokedServiceCertificate(
                timestamp=DateTime.now(),
                service_id=sequestered_org.sequester_service_2_id,
            )
            certif_signed = SequesterSigningKeyDer.generate_pair(1024)[0].sign(certif.dump())
        case unknown:
            assert False, unknown

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "revoked_service_certificate": b64encode(certif_signed).decode(),
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Invalid certificate"}


async def test_not_sequestered_organization(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
    coolorg: CoolorgRpcClients,
) -> None:
    other_org_revoke_sequester_service_event = next(
        e
        for e in sequestered_org.testbed_template.events
        if isinstance(e, tb.TestbedEventRevokeSequesterService)
    )

    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "revoked_service_certificate": b64encode(
                other_org_revoke_sequester_service_event.raw_certificate
            ).decode(),
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester disabled"}


async def test_sequestered_service_already_revoked(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    certif = SequesterRevokedServiceCertificate(
        timestamp=DateTime.now(),
        service_id=sequestered_org.sequester_service_1_id,
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "revoked_service_certificate": b64encode(certif_signed).decode(),
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Sequester service already revoked"}


async def test_require_greater_timestamp(
    client: httpx.AsyncClient,
    backend: Backend,
    sequestered_org: SequesteredOrgRpcClients,
) -> None:
    last_sequester_event = next(
        e
        for e in reversed(sequestered_org.testbed_template.events)
        if isinstance(e, tb.TestbedEventNewSequesterService | tb.TestbedEventRevokeSequesterService)
    )
    certif = SequesterRevokedServiceCertificate(
        timestamp=last_sequester_event.timestamp,
        service_id=sequestered_org.sequester_service_2_id,
    )
    certif_signed = sequestered_org.sequester_authority_signing_key.sign(certif.dump())

    url = f"http://parsec.invalid/administration/organizations/{sequestered_org.organization_id.str}/sequester/services/revoke"
    response = await client.post(
        url,
        headers={"Authorization": f"Bearer {backend.config.administration_token}"},
        json={
            "revoked_service_certificate": b64encode(certif_signed).decode(),
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
