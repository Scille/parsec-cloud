# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import OrganizationID, SequesterServiceID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.sequester import (
    SequesterServiceConfig,
    SequesterServiceType,
    SequesterUpdateConfigForServiceStoreBadOutcome,
)

_q_lock_sequester_topic_and_update_service = Q("""
WITH my_organization AS (
    SELECT
        _id,
        sequester_authority_certificate IS NOT NULL AS organization_is_sequestered
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

-- Sequester topic write lock
my_locked_sequester_topic AS (
    SELECT last_timestamp
    FROM sequester_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR UPDATE
),

updated_sequester_service AS (
    UPDATE sequester_service
        SET
            webhook_url = $webhook_url,
            service_type = $service_type
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND service_id = $service_id
    RETURNING TRUE
)

SELECT
    COALESCE(
        (SELECT TRUE FROM my_organization),
        FALSE
    ) AS organization_exists,
    COALESCE(
        (SELECT organization_is_sequestered FROM my_organization),
        FALSE
    ) AS organization_is_sequestered,
    COALESCE(
        (SELECT TRUE FROM updated_sequester_service),
        FALSE
    ) AS service_exists
""")


async def sequester_update_config_for_service(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    service_id: SequesterServiceID,
    config: SequesterServiceConfig,
) -> None | SequesterUpdateConfigForServiceStoreBadOutcome:
    match config:
        case SequesterServiceType.STORAGE as service_type:
            webhook_url = None
        case (SequesterServiceType.WEBHOOK as service_type, webhook_url):
            pass

    row = await conn.fetchrow(
        *_q_lock_sequester_topic_and_update_service(
            organization_id=organization_id.str,
            service_id=service_id,
            service_type=service_type.name,
            webhook_url=webhook_url,
        )
    )
    assert row is not None, row

    match row["organization_exists"]:
        case True:
            pass
        case False:
            return SequesterUpdateConfigForServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_sequestered"]:
        case True:
            pass
        case False:
            return SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_DISABLED
        case unknown:
            assert False, repr(unknown)

    match row["service_exists"]:
        case True:
            pass
        case False:
            return SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND
        case unknown:
            assert False, repr(unknown)
