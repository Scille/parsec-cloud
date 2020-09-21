# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pendulum import now as pendulum_now
from functools import lru_cache
from typing import Tuple, List, Optional

from parsec.api.protocol import UserID, OrganizationID, HumanHandle
from parsec.backend.user import HumanFindResultItem
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, query


@lru_cache()
def _q_factory(query, omit_revoked, offset, limit):
    conditions = []
    if query:
        conditions.append("AND user_id ~* $query")
    if omit_revoked:
        conditions.append("AND (revoked_on IS NULL OR revoked_on > $now)")
    return Q(
        f"""
SELECT
    user_id,
    count(*) OVER() AS full_count
FROM user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    { " ".join(conditions) }
ORDER BY user_id
LIMIT { limit }
OFFSET {offset}
    """
    )


@lru_cache()
def _q_human_factory(query, omit_revoked, omit_non_human, offset, limit):
    conditions = []
    if omit_revoked:
        conditions.append("AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)")
    # We only query on human
    if omit_non_human or query:
        conditions.append("AND user_.human IS NOT NULL")
    if query:
        conditions.append("AND CONCAT(human.label,human.email) ~* $query")
    return Q(
        f"""SELECT
    user_.user_id,
    human.email,
    human.label,
    user_.revoked_on IS NOT NULL AND user_.revoked_on <= $now,
    count(*) OVER() AS full_count
    FROM user_ LEFT JOIN human ON user_.human=human._id
    WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    { " ".join(conditions) }
    ORDER BY human.label
    LIMIT { limit }
    OFFSET {offset}
    """
    )


@query()
async def query_find(
    conn,
    organization_id: OrganizationID,
    page: int = 1,
    per_page: int = 100,
    omit_revoked: bool = False,
    query: Optional[str] = None,
) -> Tuple[List[UserID], int]:
    if page >= 1:
        offset = (page - 1) * per_page
    else:
        return ([], 0)
    if query:
        try:
            UserID(query)
        except ValueError:
            # Contains invalid caracters, no need to go further
            return ([], 0)

    q = _q_factory(
        query=True if query else False,
        omit_revoked=True if omit_revoked else False,
        offset=offset,
        limit=per_page,
    )
    if query:
        if omit_revoked:
            args = q(organization_id=organization_id, now=pendulum_now(), query=query)
        else:
            args = q(organization_id=organization_id, query=query)
    else:
        if omit_revoked:
            args = q(organization_id=organization_id, now=pendulum_now())
        else:
            args = q(organization_id=organization_id)

    raw_results = await conn.fetch(*args)
    if raw_results:
        total = raw_results[0]["full_count"]
    else:
        total = 0
    results = [UserID(x[0]) for x in raw_results]

    return results, total


@query()
async def query_find_humans(
    conn,
    organization_id: OrganizationID,
    omit_revoked: bool = False,
    omit_non_human: bool = False,
    page: int = 1,
    per_page: int = 100,
    query: Optional[str] = None,
) -> Tuple[List[HumanFindResultItem], int]:
    if page >= 1:
        offset = (page - 1) * per_page
    else:
        return ([], 0)

    q = _q_human_factory(
        query=True if query else False,
        omit_revoked=True if omit_revoked else False,
        omit_non_human=True if omit_non_human else False,
        offset=offset,
        limit=per_page,
    )
    if query:
        args = q(organization_id=organization_id, now=pendulum_now(), query=query)
    else:
        args = q(organization_id=organization_id, now=pendulum_now())

    raw_results = await conn.fetch(*args)
    if raw_results:
        total = raw_results[0]["full_count"]
    else:
        total = 0
    results = [
        *[res for res in raw_results if res.human_handle],
        *[res for res in raw_results if not res.human_handle],
    ]
    results = [
        HumanFindResultItem(
            user_id=UserID(user_id),
            human_handle=HumanHandle(email=email, label=label) if email is not None else None,
            revoked=revoked,
        )
        for user_id, email, label, revoked, _ in raw_results
    ]

    return results, total
