# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import itertools
import pendulum
from functools import lru_cache
from typing import Tuple, List
from pypika import PostgreSQLQuery as Query, Parameter
from pypika.functions import Concat

from parsec.api.protocol import UserID, OrganizationID, HumanHandle
from parsec.backend.user import HumanFindResultItem
from parsec.backend.postgresql.utils import IRegex, query
from parsec.backend.postgresql.tables import t_human, t_user, q_organization_internal_id


@lru_cache()
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

    return q.get_sql()


@lru_cache()
def _q_human_factory(query, omit_revoked, omit_non_human):
    _param_count = itertools.count(1)

    def _next_param():
        return Parameter(f"${next(_param_count)}")

    q = (
        Query.from_(t_user)
        .left_join(t_human)
        .on(t_user.human == t_human._id)
        .select(t_user.user_id, t_human.email, t_human.label)
        .where((t_user.organization == q_organization_internal_id(_next_param())))
    )
    if query:
        q = q.where(IRegex(Concat(t_human.label, t_human.email, t_user.user_id), _next_param()))
    if omit_revoked:
        q = q.where(t_user.revoked_on.isnull() | t_user.revoked_on > _next_param())
    if omit_non_human:
        q = q.where(t_user.human.notnull())

    return q.get_sql()


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
            q = _q_factory(query=True, omit_revoked=True)
            args = (organization_id, query, pendulum.now())

        else:
            q = _q_factory(query=True, omit_revoked=False)
            args = (organization_id, query)

    else:

        if omit_revoked:
            q = _q_factory(query=False, omit_revoked=True)
            args = (organization_id, pendulum.now())

        else:
            q = _q_factory(query=False, omit_revoked=False)
            args = (organization_id,)

    all_results = await conn.fetch(q, *args)
    # TODO: should user LIMIT and OFFSET in the SQL query instead
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
    if query:

        if omit_revoked:
            q = _q_human_factory(query=True, omit_revoked=True, omit_non_human=omit_non_human)
            args = (organization_id, query, pendulum.now())

        else:
            q = _q_human_factory(query=True, omit_revoked=False, omit_non_human=omit_non_human)
            args = (organization_id, query)

    else:

        if omit_revoked:
            q = _q_human_factory(query=False, omit_revoked=True, omit_non_human=omit_non_human)
            args = (organization_id, pendulum.now())

        else:
            q = _q_human_factory(query=False, omit_revoked=False, omit_non_human=omit_non_human)
            args = (organization_id,)

    raw_results = await conn.fetch(q, *args)

    humans = [
        HumanFindResultItem(
            user_id=UserID(user_id), human_handle=HumanHandle(email=email, label=label)
        )
        for user_id, email, label in raw_results
        if email is not None
    ]
    non_humans = [
        HumanFindResultItem(user_id=UserID(user_id), human_handle=None)
        for user_id, email, label in raw_results
        if email is None
    ]
    results = [
        *sorted(humans, key=lambda x: x.human_handle.label),
        *sorted(non_humans, key=lambda x: x.user_id),
    ]
    # TODO: should user LIMIT and OFFSET in the SQL query instead
    return results[(page - 1) * per_page : page * per_page], len(results)
