# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from pendulum import now as pendulum_now
from functools import lru_cache
from typing import Tuple, List, Optional

from parsec.api.protocol import UserID, OrganizationID, HumanHandle
from parsec.backend.user import HumanFindResultItem
from parsec.backend.postgresql.utils import Q, q_organization_internal_id, query


LIKE_TRANSLATION = {ord("%"): "\\%", ord("_"): "\\_", ord("\\"): "\\\\"}


def _escape_sql_like_arg(arg: str) -> str:
    # 1) Split`arg` by newlines and spaces
    # 2) For each word, escapes special `%`, `_` and `\` characters that are
    #    interpreted by ILIKE operator
    # 3) Combine all words together with `%` (i.e. mach zero or multiple characters)
    # So `foo  bar\tspam` becomes `%foo%bar%spam%` wich is interpreted by SQL LIKE
    # as regex `^.*foo.*bar.*spam.*$`
    return "%" + "%".join(x.translate(LIKE_TRANSLATION) for x in arg.split()) + "%"


_q_retrieve_active_human_by_email = Q(
    f"""
SELECT
    user_.user_id
FROM user_ LEFT JOIN human ON user_.human=human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND human.email = $email
    AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)
LIMIT 1
"""
)


@lru_cache()
def _q_human_factory(with_query: bool, omit_revoked: bool, omit_non_human: bool) -> Q:
    conditions = []
    if omit_revoked:
        conditions.append("AND (user_.revoked_on IS NULL OR user_.revoked_on > $now)")
    # We only query on human
    if omit_non_human or with_query:
        conditions.append("AND user_.human IS NOT NULL")
    if with_query:
        conditions.append("AND ((human.label ILIKE $query) OR (human.email ILIKE $query))")

    # Query with pagination & total result not trivial in SQL:
    # - We do the query with order but without pagination to get all results
    # - From the previous query, we generate a query for total results and
    #   another to get the rows within the pagination range
    # - Finally we merge the two queries together through an UNION
    # However UNION ALL doesn't guarantee the order of the rows is kept,
    # so we have to handle this by hand with the `row_order` field, this
    # guarantee first row is the total count, then remaining rows are the
    # pagination items in the correct order.
    return Q(
        f"""
WITH full_results AS (
    SELECT
        user_.user_id AS user_id,
        human.email AS email,
        human.label AS label,
        user_.revoked_on IS NOT NULL AND user_.revoked_on <= $now AS is_revoked,
        ROW_NUMBER() OVER (ORDER BY LOWER(human.label) NULLS LAST) AS row_order
    FROM user_ LEFT JOIN human ON user_.human=human._id
    WHERE
        user_.organization = { q_organization_internal_id("$organization_id") }
        { " ".join(conditions) }
    ORDER BY LOWER(human.label) NULLS LAST
)
(
    SELECT
        '' AS user_id,
        '' AS email,
        '' AS label,
        false AS is_revoked,
        0 AS row_order,
        count(*) AS total
    FROM full_results
)
UNION ALL
(
    SELECT
        user_id,
        email,
        label,
        is_revoked,
        row_order,
        0 AS total
    FROM full_results
    LIMIT $limit
    OFFSET $offset
)
ORDER BY row_order
"""
    )


@query()
async def query_retrieve_active_human_by_email(
    conn, organization_id: OrganizationID, email: str
) -> Optional[UserID]:
    result = await conn.fetchrow(
        *_q_retrieve_active_human_by_email(
            organization_id=organization_id.str, now=pendulum_now(), email=email
        )
    )
    if result:
        return UserID(result["user_id"])
    return None


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
        with_query=bool(query), omit_revoked=omit_revoked, omit_non_human=omit_non_human
    )
    if query:
        args = q(
            organization_id=organization_id.str,
            now=pendulum_now(),
            query=_escape_sql_like_arg(query),
            offset=offset,
            limit=per_page,
        )
    else:
        args = q(
            organization_id=organization_id.str, now=pendulum_now(), offset=offset, limit=per_page
        )

    raw_results = await conn.fetch(*args)
    total = raw_results[0]["total"]
    results = [
        HumanFindResultItem(
            user_id=UserID(user_id),
            human_handle=HumanHandle(email=email, label=label) if email else None,
            revoked=is_revoked,
        )
        for user_id, email, label, is_revoked, _, _ in raw_results[1:]
    ]

    return results, total
