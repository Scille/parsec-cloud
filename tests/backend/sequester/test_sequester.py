# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pendulum
import pytest
from parsec.api.protocol.sequester import SequesterServiceType

from parsec.api.protocol.types import DeviceLabel, HumanHandle, OrganizationID
from parsec.core.types.backend_address import BackendOrganizationBootstrapAddr
from parsec.sequester_crypto import (
    load_sequester_public_key,
    create_service_certificate,
    sign_service_certificate,
    verify_sequester,
)
from parsec.core.invite import bootstrap_organization

from pathlib import Path

sign_private_row_key = Path("tests/backend/sequester/ressources/sign_private_key.pem")
sign_public_row_key = Path("tests/backend/sequester/ressources/sign_public_key.pem")
encrypt_private_row_key = Path("tests/backend/sequester/ressources/encrypt_private_key.pem")
encrypt_public_row_key = Path("tests/backend/sequester/ressources/encrypt_puiblic_key.pem")


org_id = OrganizationID("NewOrg")


@pytest.fixture
async def authority_public_key(running_backend, backend):
    # Create organization
    org_token = "123456"
    await backend.organization.create(org_id, org_token)

    organization_addr = BackendOrganizationBootstrapAddr.build(
        running_backend.addr, org_id, org_token
    )
    human_handle = HumanHandle(email="zack@example.com", label="Zack")
    device_label = DeviceLabel("PC1")

    # Load public key
    bytes_public_key = sign_public_row_key.read_bytes()
    public_key = load_sequester_public_key(bytes_public_key)

    # Boostrap organization
    await bootstrap_organization(
        organization_addr,
        human_handle=human_handle,
        device_label=device_label,
        sequester_public_verify_key=public_key,
    )

    return public_key


@pytest.mark.trio
async def test_sequester_new_service(authority_public_key, backend):
    time_ref = pendulum.now()

    bytes_private_key = sign_private_row_key.read_bytes()
    bytes_public_key = encrypt_public_row_key.read_bytes()

    # Create certificate and signature
    certificate = create_service_certificate(bytes_public_key, "sequester_service")
    certirficate_signature = sign_service_certificate(certificate, bytes_private_key)

    verify_sequester(authority_public_key, certificate, certirficate_signature)

    # Send request
    service_id = await backend.sequester.create(
        org_id, SequesterServiceType.SEQUESTRE, certificate, certirficate_signature
    )

    services = await backend.sequester.get_organization_services(org_id)
    assert len(services) == 1
    assert services[0].service_id == service_id

    # Get service
    sequester_service = await backend.sequester.get(org_id, service_id)
    assert sequester_service == services[0]
    assert sequester_service.service_id == service_id
    assert sequester_service.service_type == SequesterServiceType.SEQUESTRE
    assert sequester_service.service_certificate == certificate
    assert sequester_service.service_certificate_signature == certirficate_signature
    assert sequester_service.deleted_on is None
    assert sequester_service.created_on > time_ref

    # Delete
    delete_time_ref = pendulum.now()
    await backend.sequester.delete(org_id, service_id)
    services = await backend.sequester.get_organization_services(org_id)
    assert not services
    sequester_service = await backend.sequester.get(org_id, service_id)
    assert sequester_service.deleted_on > delete_time_ref
