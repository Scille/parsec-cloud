# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass

from parsec._parsec import (
    DateTime,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)

_q_lock_shamir_topic_write = Q(
    """
WITH my_locked_shamir_recovery_topic AS (
    SELECT last_timestamp
    FROM shamir_recovery_topic
    WHERE organization = $organization_internal_id
    LIMIT 1
    FOR SHARE
),

my_last_shamir_recovery AS (
    SELECT
        shamir_recovery_setup._id,
        shamir_recovery_setup.created_on,
        shamir_recovery_setup.deleted_on
    FROM shamir_recovery_setup
    WHERE user_ = $user_internal_id
    AND organization = $organization_internal_id
    ORDER BY shamir_recovery_setup.created_on DESC
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_internal_id,
    (SELECT created_on FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_created_on,
    (SELECT deleted_on FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_deleted_on,
    (SELECT last_timestamp FROM my_locked_shamir_recovery_topic) AS last_shamir_recovery_certificate_timestamp
"""
)


@dataclass(slots=True)
class LockShamirData:
    last_shamir_recovery_certificate_timestamp: DateTime | None
    last_shamir_recovery_setup_internal_id: int | None
    last_shamir_recovery_setup_created_on: DateTime | None
    last_shamir_recovery_setup_deleted_on: DateTime | None


async def lock_shamir_write(
    conn: AsyncpgConnection,
    organization_internal_id: int,
    user_internal_id: int,
) -> LockShamirData:
    row = await conn.fetchrow(
        *_q_lock_shamir_topic_write(
            organization_internal_id=organization_internal_id,
            user_internal_id=user_internal_id,
        )
    )
    assert row is not None

    match row["last_shamir_recovery_certificate_timestamp"]:
        case DateTime() | None as last_shamir_recovery_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_setup_internal_id"]:
        case int() | None as last_shamir_recovery_setup_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_setup_created_on"]:
        case DateTime() | None as last_shamir_recovery_setup_created_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_setup_deleted_on"]:
        case DateTime() | None as last_shamir_recovery_setup_deleted_on:
            pass
        case unknown:
            assert False, repr(unknown)

    return LockShamirData(
        last_shamir_recovery_certificate_timestamp=last_shamir_recovery_certificate_timestamp,
        last_shamir_recovery_setup_internal_id=last_shamir_recovery_setup_internal_id,
        last_shamir_recovery_setup_created_on=last_shamir_recovery_setup_created_on,
        last_shamir_recovery_setup_deleted_on=last_shamir_recovery_setup_deleted_on,
    )
