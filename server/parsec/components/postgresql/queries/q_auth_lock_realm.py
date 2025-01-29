# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    DateTime,
    RealmRole,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.types import BadOutcomeEnum


class LockRealmWriteRealmBadOutcome(BadOutcomeEnum):
    REALM_NOT_FOUND = auto()
    USER_NOT_IN_REALM = auto()


@dataclass(slots=True)
class LockRealmWriteRealmData:
    last_realm_certificate_timestamp: DateTime
    realm_internal_id: int
    realm_key_index: int
    realm_user_current_role: RealmRole


# Two important notes about preventing race conditions during realm-related operations:
# - 1. Selecting the `realm_topic` table with a `FOR SHARE` or `FOR UPDATE` clause
#   allows us to effectively lock the access to the realm, for read and write operations
#   respectively. More specifically, concurrent read-locking queries on the same realm
#   will block until the ongoing write transaction is done. Similarly, concurrent write-locking
#   queries on the same realm will block until all ongoing transactions are done (read or write).
# - 2. It is tempting to use a `WITH` clause to have a single query that both
#   locks the realm topic and fetches information about the realm. However, this
#   would expose use to race conditions, as the `WITH` clause does not provide
#   any guarantee on the order of execution of its subqueries. More generally,
#   the order of execution of subqueries in an SQL query is not guaranteed, whether
#   it is using a `WITH` clause, the `(SELECT ...)` syntax, or the `FROM ... JOIN ...`.
#   This is why we have to split the `lock_realm_*` functions in two queries, one to
#   lock the realm topic and one to fetch the information about the realm.


_Q_LOCK_REALM_TEMPLATE = """
SELECT realm._id AS realm_internal_id
FROM realm
INNER JOIN realm_topic ON realm._id = realm_topic.realm
WHERE
    realm.organization = $organization_internal_id
    AND realm_id = $realm_id
{row_lock} OF realm_topic
"""

_q_lock_realm_write = Q(_Q_LOCK_REALM_TEMPLATE.format(row_lock="FOR UPDATE"))
_q_lock_realm_read = Q(_Q_LOCK_REALM_TEMPLATE.format(row_lock="FOR SHARE"))


_q_realm_info_after_lock = Q(
    """
WITH my_realm_user_role AS (
    SELECT role
    FROM realm_user_role
    WHERE
        user_ = $user_internal_id
        AND realm = $realm_internal_id
    ORDER BY certified_on DESC
    LIMIT 1
)

SELECT
    realm_topic.last_timestamp AS last_realm_certificate_timestamp,
    key_index AS realm_key_index,
    (SELECT role FROM my_realm_user_role) AS realm_user_current_role
FROM realm
INNER JOIN realm_topic ON realm._id = realm_topic.realm
WHERE
    realm.organization = $organization_internal_id
    AND realm._id = $realm_internal_id
"""
)


async def lock_realm_write(
    conn: AsyncpgConnection,
    organization_internal_id: int,
    user_internal_id: int,
    realm_id: VlobID,
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    return await _do_lock_realm(
        conn,
        _q_lock_realm_write,
        organization_internal_id=organization_internal_id,
        user_internal_id=user_internal_id,
        realm_id=realm_id,
    )


async def lock_realm_read(
    conn: AsyncpgConnection,
    organization_internal_id: int,
    user_internal_id: int,
    realm_id: VlobID,
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    return await _do_lock_realm(
        conn,
        _q_lock_realm_read,
        organization_internal_id=organization_internal_id,
        user_internal_id=user_internal_id,
        realm_id=realm_id,
    )


async def _do_lock_realm(
    conn: AsyncpgConnection,
    lock_query: Q,
    organization_internal_id: int,
    user_internal_id: int,
    realm_id: VlobID,
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    # 0) Lock realm

    realm_internal_id = await conn.fetchval(
        *lock_query(organization_internal_id=organization_internal_id, realm_id=realm_id)
    )

    match realm_internal_id:
        case int() as realm_internal_id:
            pass
        case None:
            return LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    row = await conn.fetchrow(
        *_q_realm_info_after_lock(
            organization_internal_id=organization_internal_id,
            user_internal_id=user_internal_id,
            realm_internal_id=realm_internal_id,
        )
    )
    assert row is not None

    # 1) Check realm

    match row["realm_key_index"]:
        case int() as realm_key_index:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["realm_user_current_role"]:
        case str() as raw_realm_user_current_role:
            realm_user_current_role = RealmRole.from_str(raw_realm_user_current_role)
        case None:
            return LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM
        case unknown:
            assert False, repr(unknown)

    # 2) Check topics

    match row["last_realm_certificate_timestamp"]:
        case DateTime() as last_realm_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    return LockRealmWriteRealmData(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp,
        realm_internal_id=realm_internal_id,
        realm_key_index=realm_key_index,
        realm_user_current_role=realm_user_current_role,
    )
