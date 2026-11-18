# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)

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
    conn: AsyncpgConnection, organization_internal_id: int
) -> DateTime | None:

    row = await conn.fetchrow(
        *_q_shamir_last_timestamp(
            organization_internal_id=organization_internal_id,
        )
    )

    if not row:
        return None

    match row["last_shamir_recovery_certificate_timestamp"]:
        case DateTime() as last_shamir_recovery_certificate_timestamp:
            return last_shamir_recovery_certificate_timestamp
        case _:
            assert False, row
