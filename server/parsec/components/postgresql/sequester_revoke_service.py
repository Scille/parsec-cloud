# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterRevokedServiceCertificate,
    SequesterVerifyKeyDer,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.utils import Q
from parsec.components.sequester import (
    SequesterRevokeServiceStoreBadOutcome,
    SequesterRevokeServiceValidateBadOutcome,
    sequester_revoke_service_validate,
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


_q_revoke_service = Q("""
WITH my_sequester_service AS (
    SELECT
        _id,
        revoked_on IS NOT NULL AS is_revoked
    FROM sequester_service
    WHERE
        organization = $organization_internal_id
        AND service_id = $service_id
    LIMIT 1
),

updated_sequester_service AS (
    UPDATE sequester_service
        SET
            revoked_on = $revoked_on,
            sequester_revoked_service_certificate = $revoked_service_certificate
    WHERE
        _id = (SELECT _id FROM my_sequester_service)
        AND revoked_on IS NULL
    RETURNING TRUE
),

updated_sequester_topic AS (
    UPDATE sequester_topic
    SET last_timestamp = $revoked_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $revoked_on
    RETURNING TRUE
)

SELECT
    COALESCE(
        (SELECT TRUE FROM my_sequester_service),
        FALSE
    ) AS service_exists,
    COALESCE(
        (SELECT FALSE FROM updated_sequester_service),
        TRUE
    ) AS service_already_revoked,
    COALESCE(
        (SELECT TRUE FROM updated_sequester_topic),
        FALSE
    ) AS topic_updated
""")


async def sequester_revoke_service(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    revoked_service_certificate: bytes,
) -> (
    SequesterRevokedServiceCertificate
    | SequesterRevokeServiceValidateBadOutcome
    | SequesterRevokeServiceStoreBadOutcome
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
            return SequesterRevokeServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["sequester_authority_verify_key_der"]:
        case bytes() as raw_authority_verify_key_der:
            authority_verify_key_der = SequesterVerifyKeyDer(raw_authority_verify_key_der)
        case None:
            return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_DISABLED
        case unknown:
            assert False, repr(unknown)

    match row["last_sequester_certificate_timestamp"]:
        case DateTime() as sequester_topic_last_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    # 2) Validate the service certificate

    match sequester_revoke_service_validate(
        now, authority_verify_key_der, revoked_service_certificate
    ):
        case SequesterRevokedServiceCertificate() as certif:
            pass
        case error:
            return error

    if sequester_topic_last_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=sequester_topic_last_timestamp)

    # 4) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_revoke_service(
            organization_internal_id=organization_internal_id,
            service_id=certif.service_id,
            revoked_service_certificate=revoked_service_certificate,
            revoked_on=certif.timestamp,
        )
    )
    assert row is not None, row

    match row["service_exists"]:
        case True:
            pass
        case False:
            return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["service_already_revoked"]:
        case False:
            pass
        case True:
            return SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_REVOKED
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
