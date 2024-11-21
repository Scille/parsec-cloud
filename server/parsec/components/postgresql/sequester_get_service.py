# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, OrganizationID, SequesterServiceID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.sequester import (
    BaseSequesterService,
    SequesterGetOrganizationServicesBadOutcome,
    SequesterGetServiceBadOutcome,
    SequesterServiceType,
    StorageSequesterService,
    WebhookSequesterService,
)

_q_get_service = Q("""
WITH my_organization AS (
    SELECT
        _id,
        sequester_authority_verify_key_der IS NOT NULL AS is_sequestered
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_service AS (
    SELECT
        service_id,
        service_label,
        service_certificate,
        created_on,
        revoked_on,
        service_type,
        webhook_url
    FROM sequester_service
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND service_id = $service_id
)

SELECT
    COALESCE(
        (SELECT TRUE FROM my_organization),
        FALSE
    ) AS organization_exists,
    (SELECT is_sequestered FROM my_organization) AS organization_is_sequestered,
    (SELECT service_id FROM my_service),
    (SELECT service_label FROM my_service),
    (SELECT service_certificate FROM my_service),
    (SELECT created_on FROM my_service),
    (SELECT revoked_on FROM my_service),
    (SELECT service_type FROM my_service),
    (SELECT webhook_url FROM my_service)
""")


async def sequester_get_service(
    conn: AsyncpgConnection, organization_id: OrganizationID, service_id: SequesterServiceID
) -> BaseSequesterService | SequesterGetServiceBadOutcome:
    row = await conn.fetchrow(
        *_q_get_service(
            organization_id=organization_id.str,
            service_id=service_id,
        )
    )
    assert row is not None, row

    match row["organization_exists"]:
        case True:
            pass
        case False:
            return SequesterGetServiceBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_sequestered"]:
        case True:
            pass
        case False:
            return SequesterGetServiceBadOutcome.SEQUESTER_DISABLED
        case unknown:
            assert False, repr(unknown)

    match row["service_id"]:
        case str() as raw_service_id:
            service_id = SequesterServiceID.from_hex(raw_service_id)
        case None:
            return SequesterGetServiceBadOutcome.SEQUESTER_SERVICE_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["service_label"]:
        case str() as service_label:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["service_certificate"]:
        case bytes() as service_certificate:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["created_on"]:
        case DateTime() as created_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["revoked_on"]:
        case DateTime() | None as revoked_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match (row["service_type"], row["webhook_url"]):
        case SequesterServiceType.STORAGE.name, None:
            service = StorageSequesterService(
                service_id=service_id,
                service_label=service_label,
                service_certificate=service_certificate,
                created_on=created_on,
                revoked_on=revoked_on,
            )
        case SequesterServiceType.WEBHOOK.name, str() as webhook_url:
            service = WebhookSequesterService(
                service_id=service_id,
                service_label=service_label,
                service_certificate=service_certificate,
                created_on=created_on,
                revoked_on=revoked_on,
                webhook_url=webhook_url,
            )
        case unknown:
            assert False, repr(unknown)

    return service


_q_get_organization_services = Q("""
WITH my_organization AS (
    SELECT
        _id,
        sequester_authority_verify_key_der IS NOT NULL AS is_sequestered
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
)

-- First row is always present and provide info about the organization
SELECT
    -- First row only: organization info columns
    COALESCE(
        (SELECT TRUE FROM my_organization),
        FALSE
    ) AS organization_exists,
    (SELECT is_sequestered FROM my_organization) AS organization_is_sequestered,

    -- Non-first rows only: service info columns
    NULL AS service_id,
    NULL AS service_label,
    NULL AS service_certificate,
    NULL AS created_on,
    NULL AS revoked_on,
    NULL AS service_type,
    NULL AS webhook_url

UNION ALL -- Using UNION ALL is import to avoid sorting !

-- Then following rows are services
SELECT
    -- First row only: organization info columns
    NULL,
    NULL,

    -- Non-first rows only: service info columns
    service_id,
    service_label,
    service_certificate,
    created_on,
    revoked_on,
    service_type,
    webhook_url
FROM sequester_service
WHERE
    organization = (SELECT _id FROM my_organization)
""")


async def sequester_get_organization_services(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> list[BaseSequesterService] | SequesterGetOrganizationServicesBadOutcome:
    rows = await conn.fetch(
        *_q_get_organization_services(
            organization_id=organization_id.str,
        )
    )

    # First row is always present and provide info about the organization

    first_row = rows[0]

    match first_row["organization_exists"]:
        case True:
            pass
        case False:
            return SequesterGetOrganizationServicesBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match first_row["organization_is_sequestered"]:
        case True:
            pass
        case False:
            return SequesterGetOrganizationServicesBadOutcome.SEQUESTER_DISABLED
        case unknown:
            assert False, repr(unknown)

    # Then following rows are services

    services = []
    for row in rows[1:]:
        match row["service_id"]:
            case str() as raw_service_id:
                service_id = SequesterServiceID.from_hex(raw_service_id)
            case unknown:
                assert False, repr(unknown)

        match row["service_label"]:
            case str() as service_label:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["service_certificate"]:
            case bytes() as service_certificate:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["created_on"]:
            case DateTime() as created_on:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["revoked_on"]:
            case DateTime() | None as revoked_on:
                pass
            case unknown:
                assert False, repr(unknown)

        match (row["service_type"], row["webhook_url"]):
            case SequesterServiceType.STORAGE.name, None:
                service = StorageSequesterService(
                    service_id=service_id,
                    service_label=service_label,
                    service_certificate=service_certificate,
                    created_on=created_on,
                    revoked_on=revoked_on,
                )
            case SequesterServiceType.WEBHOOK.name, str() as webhook_url:
                service = WebhookSequesterService(
                    service_id=service_id,
                    service_label=service_label,
                    service_certificate=service_certificate,
                    created_on=created_on,
                    revoked_on=revoked_on,
                    webhook_url=webhook_url,
                )
            case unknown:
                assert False, repr(unknown)

        services.append(service)

    return services
