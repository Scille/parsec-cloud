# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pendulum import now as pendulum_now, datetime

from parsec.api.protocol import OrganizationID, SequesterServiceID
from parsec.backend.sequester import (
    SequesterOrganizationNotFoundError,
    SequesterServiceNotFoundError,
    SequesterDisabledError,
    SequesterCertificateValidationError,
    SequesterServiceAlreadyExists,
    SequesterCertificateOutOfBallparkError,
    SequesterServiceAlreadyDeletedError,
)

from tests.common import (
    OrganizationFullData,
    customize_fixtures,
    sequester_authority_factory,
    sequester_service_factory,
)


# Sequester service modification is not exposed as a command API, so we only
# test the internal component API


@pytest.mark.trio
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_create_delete_services(
    coolorg: OrganizationFullData, otherorg: OrganizationFullData, backend
):
    service = sequester_service_factory("Test Service", coolorg.sequester_authority)

    # 1) Service creation

    # Unknown organization ID
    with pytest.raises(SequesterOrganizationNotFoundError):
        await backend.sequester.create_service(
            organization_id=OrganizationID("DummyOrg"), service=service.backend_service
        )

    # Try to create service in a non sequestered organization
    with pytest.raises(SequesterDisabledError):
        await backend.sequester.create_service(
            organization_id=otherorg.organization_id, service=service.backend_service
        )

    # Invalid service certificate
    with pytest.raises(SequesterCertificateValidationError):
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id,
            service=service.backend_service.evolve(service_certificate=b"<dummy>"),
        )

    # Valid service certificate, but with invalid signature
    bad_authority = sequester_authority_factory(coolorg.root_signing_key)
    service_signed_by_bad_authority = sequester_service_factory("Test Service", bad_authority)
    with pytest.raises(SequesterCertificateValidationError):
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id,
            service=service_signed_by_bad_authority.backend_service,
        )

    # Service certificate timestamp out of ballpark
    service_bad_timestamp = sequester_service_factory(
        "sequester_service_1", coolorg.sequester_authority, timestamp=datetime(2000, 1, 1)
    )
    with pytest.raises(SequesterCertificateOutOfBallparkError):
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service_bad_timestamp.backend_service
        )

    # Successful service creation
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=service.backend_service
    )

    # Service ID already exists
    with pytest.raises(SequesterServiceAlreadyExists):
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )

    # Retreive service list
    services = await backend.sequester.get_organization_services(
        organization_id=coolorg.organization_id
    )
    assert services == [service.backend_service]

    # Cannot retreive service list on non sequestered organization
    with pytest.raises(SequesterDisabledError):
        services = await backend.sequester.get_organization_services(
            organization_id=otherorg.organization_id
        )

    # 2) Service deletion
    deleted_on = pendulum_now()

    # Unknown organization ID
    with pytest.raises(SequesterOrganizationNotFoundError):
        await backend.sequester.delete_service(
            organization_id=OrganizationID("DummyOrg"),
            service_id=service.service_id,
            deleted_on=deleted_on,
        )

    # Unknown sequestre service
    with pytest.raises(SequesterServiceNotFoundError):
        await backend.sequester.delete_service(
            organization_id=coolorg.organization_id,
            service_id=SequesterServiceID.new(),
            deleted_on=deleted_on,
        )

    # Try deletion in a non sequestered organization
    with pytest.raises(SequesterDisabledError):
        await backend.sequester.delete_service(
            organization_id=otherorg.organization_id,
            service_id=service.service_id,
            deleted_on=deleted_on,
        )

    # Successful deletion
    await backend.sequester.delete_service(
        organization_id=coolorg.organization_id,
        service_id=service.service_id,
        deleted_on=deleted_on,
    )

    # Already deleted
    with pytest.raises(SequesterServiceAlreadyDeletedError):
        await backend.sequester.delete_service(
            organization_id=coolorg.organization_id,
            service_id=service.service_id,
            deleted_on=deleted_on,
        )

    # Retreive service list
    services = await backend.sequester.get_organization_services(
        organization_id=coolorg.organization_id
    )
    assert services == [service.backend_service.evolve(deleted_on=deleted_on)]

    # 3) Bonus: Create after deletion

    # Cannot recreate deleted service
    with pytest.raises(SequesterServiceAlreadyExists):
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )

    # Successful service creation
    backend_service2 = service.backend_service.evolve(service_id=SequesterServiceID.new())
    await backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=backend_service2
    )

    # 4) Bonus: list services

    # Retreive service list
    services = await backend.sequester.get_organization_services(
        organization_id=coolorg.organization_id
    )
    expected_backend_service1 = service.backend_service.evolve(deleted_on=deleted_on)
    assert services == [expected_backend_service1, backend_service2]

    # Retreive single service
    retreived_backend_service1 = await backend.sequester.get_service(
        organization_id=coolorg.organization_id, service_id=service.service_id
    )
    assert retreived_backend_service1 == expected_backend_service1

    # Retreive single service unknown organization ID
    with pytest.raises(SequesterOrganizationNotFoundError):
        await backend.sequester.get_service(
            organization_id=OrganizationID("DummyOrg"), service_id=service.service_id
        )

    # Retreive service list unknown organization ID
    with pytest.raises(SequesterOrganizationNotFoundError):
        await backend.sequester.get_organization_services(
            organization_id=OrganizationID("DummyOrg")
        )

    # Unknown sequestre service
    with pytest.raises(SequesterServiceNotFoundError):
        await backend.sequester.get_service(
            organization_id=coolorg.organization_id, service_id=SequesterServiceID.new()
        )

    # Try retreive service list in a non sequestered organization
    with pytest.raises(SequesterDisabledError):
        await backend.sequester.get_organization_services(organization_id=otherorg.organization_id)

    # Try retreive single service in a non sequestered organization
    with pytest.raises(SequesterDisabledError):
        await backend.sequester.get_service(
            organization_id=otherorg.organization_id, service_id=service.service_id
        )
