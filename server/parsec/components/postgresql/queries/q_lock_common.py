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


@dataclass(slots=True)
class LockCommonData:
    last_common_certificate_timestamp: DateTime


_Q_LOCK_COMMON_TEMPLATE = """
SELECT last_timestamp
FROM common_topic
WHERE organization = $organization_internal_id
LIMIT 1
-- Read or write lock?
{row_lock}
"""

_q_lock_common_write = Q(_Q_LOCK_COMMON_TEMPLATE.format(row_lock="FOR UPDATE"))
_q_lock_common_read = Q(_Q_LOCK_COMMON_TEMPLATE.format(row_lock="FOR SHARE"))


async def lock_common_write(
    conn: AsyncpgConnection,
    organization_internal_id: int,
) -> LockCommonData:
    return await _do_lock_common(
        conn,
        _q_lock_common_write,
        organization_internal_id=organization_internal_id,
    )


async def lock_common_read(
    conn: AsyncpgConnection,
    organization_internal_id: int,
) -> LockCommonData:
    return await _do_lock_common(
        conn,
        _q_lock_common_read,
        organization_internal_id=organization_internal_id,
    )


async def _do_lock_common(
    conn: AsyncpgConnection,
    lock_query: Q,
    organization_internal_id: int,
) -> LockCommonData:
    row = await conn.fetchrow(*lock_query(organization_internal_id=organization_internal_id))
    assert row is not None

    match row["last_timestamp"]:
        case DateTime() as last_timestamp:
            pass
        case _:
            assert False, row

    return LockCommonData(
        last_common_certificate_timestamp=last_timestamp,
    )
