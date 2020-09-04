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
ORDER BY user_id
LIMIT {limit} OFFSET {offset}
    """
    )


@lru_cache()
def _q_count_total_human(omit_revoked, omit_non_human):
    conditions = []
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
def _q_human_factory(query, omit_revoked, omit_non_human, limit, offset):
    conditions = []
    if query:
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
LIMIT {limit} OFFSET {offset}
    """
    )


@query()
async def query_find(
    conn, organization_id: OrganizationID, query: str, page: int, per_page: int, omit_revoked: bool
) -> Tuple[List[UserID], int]:
    if page > 1:
        offset = ((page - 1) * per_page) - 1
    else:
        offset = 0
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

    all_results = await conn.fetch(*args)
    results = [UserID(x[0]) for x in all_results[(page - 1) * per_page : page * per_page]]
    return results, len(all_results)


@query()
async def query_find_humans(
    conn,
    organization_id: OrganizationID,
    query: str,
    page: int,
    per_page: int,
    omit_revoked: bool,
    omit_non_human: bool,
) -> Tuple[List[HumanFindResultItem], int]:
    if page > 1:
        offset = ((page - 1) * per_page) - 1
    else:
        offset = 0
    if query:

        if omit_revoked:
            q = _q_human_factory(
                query=True,
                omit_revoked=True,
                omit_non_human=omit_non_human,
                limit=per_page,
                offset=offset,
            )
            args = q(organization_id=organization_id, now=pendulum_now(), query=query)

        else:
            q = _q_human_factory(
                query=True,
                omit_revoked=False,
                omit_non_human=omit_non_human,
                offset=offset,
                limit=per_page,
            )
            args = q(organization_id=organization_id, now=pendulum_now(), query=query)

    else:

        if omit_revoked:
            q = _q_human_factory(
                query=False,
                omit_revoked=True,
                omit_non_human=omit_non_human,
                limit=per_page,
                offset=offset,
            )
            args = q(organization_id=organization_id, now=pendulum_now())

        else:
            q = _q_human_factory(
                query=False,
                omit_revoked=False,
                omit_non_human=omit_non_human,
                offset=offset,
                limit=per_page,
            )
            args = q(organization_id=organization_id, now=pendulum_now())

    raw_results = await conn.fetch(*args)

    humans = [
        HumanFindResultItem(
            user_id=UserID(user_id),
            human_handle=HumanHandle(email=email, label=label),
            revoked=revoked,
        )
        for user_id, email, label, revoked in raw_results
        if email is not None
    ]
    non_humans = [
        HumanFindResultItem(user_id=UserID(user_id), human_handle=None, revoked=revoked)
        for user_id, email, label, revoked in raw_results
        if email is None
    ]
    results = [
        *sorted(humans, key=lambda x: (x.human_handle.label.lower(), x.user_id.lower())),
        *sorted(non_humans, key=lambda x: x.user_id.lower()),
    ]

    # count total human
    q = _q_count_total_human(omit_revoked=omit_revoked, omit_non_human=omit_non_human)
    if omit_revoked:
        total = await conn.fetchrow(*q(organization_id=organization_id, now=pendulum_now()))
    else:
        total = await conn.fetchrow(*q(organization_id=organization_id))
    result_page = results[(page - 1) * per_page : page * per_page]
    return (result_page, total[0])
