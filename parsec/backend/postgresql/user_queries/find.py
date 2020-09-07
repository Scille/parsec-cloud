# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS
from pendulum import now as pendulum_now
from functools import lru_cache
from typing import Tuple, List

from parsec.api.protocol import UserID, OrganizationID, HumanHandle
from parsec.backend.user import HumanFindResultItem
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, query


@lru_cache()
def _q_factory(query, omit_revoked, limit, offset):
    conditions = []
    if query:
        conditions.append("AND user_id ~* $query")
    if omit_revoked:
        conditions.append("AND (revoked_on IS NULL OR revoked_on > $now)")
    return Q(
        f"""
SELECT user_id
FROM user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    { " ".join(conditions) }
ORDER BY user_.user_id
LIMIT {limit} OFFSET {offset}
    """
    )


@lru_cache()
def _q_count_total_human(query, omit_revoked, omit_non_human, in_find=False, no_filter_by_id=False):
    conditions = []
    if query:
        if in_find:
            conditions.append(f"AND user_id ~* '{query}'")
        elif no_filter_by_id:
            conditions.append(f"AND CONCAT(human.label,human.email) ~* '{query}'")
        else:
            conditions.append(f"AND CONCAT(human.label,human.email,user_.user_id) ~* '{query}'")
    if omit_revoked:
        conditions.append("AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)")
    if omit_non_human:
        conditions.append("AND user_.human IS NOT NULL")
    return Q(
        f"""
SELECT COUNT(*)
FROM user_ LEFT JOIN human ON user_.human=human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    { " ".join(conditions) }
    """
    )


@lru_cache()
def _q_human_factory(query, omit_revoked, omit_non_human, limit, offset, no_filter_by_id=False):
    conditions = []
    if query:
        if no_filter_by_id:
            conditions.append("AND CONCAT(human.label,human.email) ~* $query")
        else:
            conditions.append("AND CONCAT(human.label,human.email,user_.user_id) ~* $query")
    if omit_revoked:
        conditions.append("AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)")
    if omit_non_human:
        conditions.append("AND user_.human IS NOT NULL")
    return Q(
        f"""
SELECT
    user_.user_id,
    human.email,
    human.label,
    user_.revoked_on IS NOT NULL AND user_.revoked_on <= $now
FROM user_ LEFT JOIN human ON user_.human=human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    { " ".join(conditions) }
ORDER BY user_.user_id
LIMIT {limit} OFFSET {offset}
    """
    )


@query()
async def query_find(
    conn, organization_id: OrganizationID, query: str, page: int, per_page: int, omit_revoked: bool
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

        if omit_revoked:
            q = _q_factory(query=True, omit_revoked=True, limit=per_page, offset=offset)
            args = q(organization_id=organization_id, query=query, now=pendulum_now())

        else:
            q = _q_factory(query=True, omit_revoked=False, limit=per_page, offset=offset)
            args = q(organization_id=organization_id, query=query)

    else:

        if omit_revoked:
            q = _q_factory(query=False, omit_revoked=True, limit=per_page, offset=offset)
            args = q(organization_id=organization_id, now=pendulum_now())

        else:
            q = _q_factory(query=False, omit_revoked=False, limit=per_page, offset=offset)
            args = q(organization_id=organization_id)

    all_results = [user["user_id"] for user in await conn.fetch(*args)]
    q = _q_count_total_human(query, omit_revoked=omit_revoked, omit_non_human=True, in_find=True)
    if omit_revoked:
        total = await conn.fetchrow(*q(organization_id=organization_id, now=pendulum_now()))
    else:
        total = await conn.fetchrow(*q(organization_id=organization_id))
    return all_results, total[0]


@query()
async def query_find_humans(
    conn,
    organization_id: OrganizationID,
    query: str,
    page: int,
    per_page: int,
    no_filter_by_id: bool,
    omit_revoked: bool,
    omit_non_human: bool,
) -> Tuple[List[HumanFindResultItem], int]:
    if page >= 1:
        offset = (page - 1) * per_page
    else:
        return ([], 0)
    if query:

        q = _q_human_factory(
            query=True,
            omit_revoked=omit_revoked,
            omit_non_human=omit_non_human,
            limit=per_page,
            offset=offset,
            no_filter_by_id=no_filter_by_id,
        )
        args = q(organization_id=organization_id, now=pendulum_now(), query=query)

    else:

        q = _q_human_factory(
            query=False,
            omit_revoked=omit_revoked,
            omit_non_human=omit_non_human,
            limit=per_page,
            offset=offset,
        )
        args = q(organization_id=organization_id, now=pendulum_now())

    raw_results = await conn.fetch(*args)

    humans = [
        HumanFindResultItem(
            user_id=UserID(user_id),
            human_handle=HumanHandle(email=email, label=label) if email is not None else None,
            revoked=revoked,
        )
        for user_id, email, label, revoked in raw_results
    ]
    q = _q_count_total_human(query, omit_revoked=omit_revoked, omit_non_human=omit_non_human)
    if omit_revoked:
        total = await conn.fetchrow(*q(organization_id=organization_id, now=pendulum_now()))
    else:
        total = await conn.fetchrow(*q(organization_id=organization_id))
    return (humans, total[0])
