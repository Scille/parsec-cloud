# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterServiceCertificate,
    SequesterVerifyKeyDer,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.utils import Q
from parsec.components.sequester import (
    SequesterCreateServiceStoreBadOutcome,
    SequesterCreateServiceValidateBadOutcome,
    SequesterServiceConfig,
    SequesterServiceType,
    sequester_create_service_validate,
)
from parsec.events import EventSequesterCertificate

_q_lock_sequester_topic_and_get_authority = Q("""
WITH my_organization AS (
    SELECT
        _id,
        sequester_authority_verify_key_der
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
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT sequester_authority_verify_key_der FROM my_organization),
    (SELECT last_timestamp FROM my_locked_sequester_topic) AS last_sequester_certificate_timestamp
""")


_q_create_service = Q("""
WITH new_sequester_service AS (
    INSERT INTO sequester_service (
        organization,
        service_id,
        service_certificate,
        service_label,
        created_on,
        service_type,
        webhook_url
    ) VALUES (
        $organization_internal_id,
        $service_id,
        $service_certificate,
        $service_label,
        $created_on,
        $service_type,
        $webhook_url
    )
    ON CONFLICT (organization, service_id) DO NOTHING
    RETURNING _id
),

updated_sequester_topic AS (
    UPDATE sequester_topic
    SET last_timestamp = $created_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $created_on
    RETURNING TRUE
)

SELECT
    COALESCE(
        (SELECT TRUE FROM new_sequester_service),
        FALSE
    ) AS service_inserted,
    COALESCE(
        (SELECT TRUE FROM updated_sequester_topic),
        FALSE
    ) AS topic_updated
""")


async def sequester_create_service(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    service_certificate: bytes,
    config: SequesterServiceConfig,
) -> (
    SequesterServiceCertificate
    | SequesterCreateServiceValidateBadOutcome
    | SequesterCreateServiceStoreBadOutcome
    | RequireGreaterTimestamp
):
    # 1) Lock sequester topic and get organization & sequester authority

    row = await conn.fetchrow(
        *_q_lock_sequester_topic_and_get_authority(
            organization_id=organization_id.str,
        )
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return SequesterCreateServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["sequester_authority_verify_key_der"]:
        case bytes() as raw_authority_verify_key_der:
            authority_verify_key_der = SequesterVerifyKeyDer(raw_authority_verify_key_der)
        case None:
            return SequesterCreateServiceStoreBadOutcome.SEQUESTER_DISABLED
        case unknown:
            assert False, repr(unknown)

    match row["last_sequester_certificate_timestamp"]:
        case DateTime() as sequester_topic_last_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    # 2) Validate the service certificate

    match sequester_create_service_validate(now, authority_verify_key_der, service_certificate):
        case SequesterServiceCertificate() as certif:
            pass
        case error:
            return error

    if sequester_topic_last_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=sequester_topic_last_timestamp)

    # 3) All checks are good, now we do the actual insertion

    match config:
        case (SequesterServiceType.WEBHOOK, webhook_url):
            service_type = SequesterServiceType.WEBHOOK
        case SequesterServiceType.STORAGE:
            service_type = SequesterServiceType.STORAGE
            webhook_url = None

    row = await conn.fetchrow(
        *_q_create_service(
            organization_internal_id=organization_internal_id,
            service_id=certif.service_id,
            service_certificate=service_certificate,
            service_label=certif.service_label,
            created_on=certif.timestamp,
            service_type=service_type.name,
            webhook_url=webhook_url,
        )
    )
    assert row is not None

    match row["service_inserted"]:
        case True:
            pass
        case False:
            return SequesterCreateServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_EXISTS
        case unknown:
            assert False, repr(unknown)

    match row["topic_updated"]:
        case True:
            pass
        case unknown:
            assert False, repr(unknown)

    await send_signal(
        conn, EventSequesterCertificate(organization_id=organization_id, timestamp=certif.timestamp)
    )

    return certif
