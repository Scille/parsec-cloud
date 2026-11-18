# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from enum import auto

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    auth_and_lock_common_read,
)
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.types import BadOutcomeEnum


class ShamirLastTimestampBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


_q_shamir_last_timestamp = Q(
    """
SELECT last_timestamp AS last_shamir_recovery_certificate_timestamp
FROM shamir_recovery_topic
WHERE
    organization = $organization_internal_id
LIMIT 1
"""
)


async def shamir_last_timestamp(
    conn: AsyncpgConnection, organization_id: OrganizationID, author: DeviceID
) -> DateTime | ShamirLastTimestampBadOutcome | None:

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as data:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return ShamirLastTimestampBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return ShamirLastTimestampBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return ShamirLastTimestampBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return ShamirLastTimestampBadOutcome.AUTHOR_REVOKED

    row = await conn.fetchrow(
        *_q_shamir_last_timestamp(
            organization_internal_id=data.organization_internal_id,
        )
    )

    if not row:
        return None

    match row["last_shamir_recovery_certificate_timestamp"]:
        case DateTime() as last_shamir_recovery_certificate_timestamp:
            return last_shamir_recovery_certificate_timestamp
        case _:
            assert False, row
