# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import itertools
import pendulum
from typing import Tuple, List
from pypika import PostgreSQLQuery as Query, Parameter

from parsec.api.protocol import UserID, OrganizationID
from parsec.backend.postgresql.utils import IRegex, query
from parsec.backend.postgresql.tables import t_user, q_organization_internal_id


def _q_factory(query, omit_revoked):
    _param_count = itertools.count(1)

    def _next_param():
        return Parameter(f"${next(_param_count)}")

    q = (
        Query.from_(t_user)
        .select(t_user.user_id)
        .where((t_user.organization == q_organization_internal_id(_next_param())))
        .orderby(t_user.user_id)
    )
    if query:
        q = q.where(IRegex(t_user.user_id, _next_param()))
    if omit_revoked:
        q = q.where(t_user.revoked_on.isnull() | t_user.revoked_on > _next_param())
    return q


_q_list_users = _q_factory(False, False).get_sql()
_q_list_users_omit_revoked = _q_factory(False, True).get_sql()
_q_filter_users = _q_factory(True, False).get_sql()
_q_filter_users_omit_revoked = _q_factory(True, True).get_sql()


@query()
async def query_find(
    conn, organization_id: OrganizationID, query: str, page: int, per_page: int, omit_revoked: bool
) -> Tuple[List[UserID], int]:
    if query:
        try:
            UserID(query)
        except ValueError:
            # Contains invalid caracters, no need to go further
            return ([], 0)

        if omit_revoked:
            now = pendulum.now()
            args = (_q_filter_users_omit_revoked, organization_id, query, now)

        else:
            args = (_q_filter_users, organization_id, query)

    else:

        if omit_revoked:
            now = pendulum.now()
            args = (_q_list_users_omit_revoked, organization_id, now)

        else:
            args = (_q_list_users, organization_id)

    all_results = await conn.fetch(*args)
    # TODO: should user LIMIT and OFFSET in the SQL query instead
    results = [UserID(x[0]) for x in all_results[(page - 1) * per_page : page * per_page]]
    return results, len(all_results)
