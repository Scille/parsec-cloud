# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import re
from functools import wraps
from typing import Any, Awaitable, Callable, Dict, List, Tuple, TypeVar

import triopg
from typing_extensions import Concatenate, ParamSpec

T = TypeVar("T")
P = ParamSpec("P")


class Q:
    """
    Dead simple SQL query composition framework (◠﹏◠)
    """

    def __init__(self, src: str, **kwargs: Any):
        # retrieve variables
        variables: Dict[str, str] = {}
        for candidate in re.findall(r"\$([a-zA-Z0-9_]+)", src):
            if candidate in variables:
                continue
            if candidate.isdigit():
                raise ValueError(f"Invalid variable name `{candidate}`")
            variables[candidate] = f"${len(variables) + 1}"
        self._variables = variables

        # Replace variables with their order-based equivalent
        # Variables are sorted in reverse to differentiate variables from others
        # with same name and a suffix, for example var and var_len
        for variable, order_based in sorted(variables.items(), reverse=True):
            src = src.replace(f"${variable}", order_based)

        self._sql = src
        self._stripped_sql = " ".join([x.strip() for x in src.split()])

    @property
    def sql(self) -> str:
        return self._sql

    def __call__(self, **kwargs: Any) -> List[Any]:
        if kwargs.keys() != self._variables.keys():
            missing = self._variables.keys() - kwargs.keys()
            unknown = kwargs.keys() - self._variables.keys()
            raise ValueError(f"Invalid paramaters, missing: {missing}, unknown: {unknown}")
        args = [self._stripped_sql]
        for variable in self._variables:
            args.append(kwargs[variable])
        return args


def q_organization(
    organization_id: str | None = None,
    _id: str | None = None,
    table: str = "organization",
    select: str = "*",
) -> str:
    assert organization_id is not None or _id is not None
    if _id is not None:
        condition = f"{table}._id = {_id}"
    else:
        condition = f"{table}.organization_id = {organization_id}"
    return f"(SELECT {select} FROM {table} WHERE {condition})"


def q_organization_internal_id(organization_id: str, **kwargs: Any) -> str:
    return q_organization(organization_id=organization_id, select="_id", **kwargs)


def _table_q_factory(
    table: str, public_id_field: str
) -> Tuple[Callable[..., str], Callable[..., str]]:
    def _q(
        organization_id: str | None = None,
        organization: str | None = None,
        _id: str | None = None,
        table_alias: str | None = None,
        select: str = "*",
        suffix: str | None = None,
        **kwargs: Any,
    ) -> str:
        if table_alias:
            from_table = f"{table} as {table_alias}"
            select_table = table_alias
        else:
            from_table = table
            select_table = table
        if _id is not None:
            condition = f"{select_table}._id = {_id}"
        else:
            public_id = kwargs.pop(public_id_field, None)
            assert public_id is not None
            if organization is None:
                assert organization_id is not None
                organization = q_organization_internal_id(organization_id)
            else:
                assert organization is not None
            condition = f"{select_table}.organization = {organization} AND {select_table}.{ public_id_field } = { public_id }"
        assert not kwargs
        suffix = suffix or ""
        return f"(SELECT {select} FROM {from_table} WHERE {condition} {suffix})"

    def _q_internal_id(**kwargs: Any) -> str:
        return _q(select="_id", **kwargs)

    _q.__name__ = f"q_{table}"
    _q_internal_id.__name__ = f"q_{table}_internal_id"

    return _q, _q_internal_id


q_device, q_device_internal_id = _table_q_factory("device", "device_id")
q_user, q_user_internal_id = _table_q_factory("user_", "user_id")
q_realm, q_realm_internal_id = _table_q_factory("realm", "realm_id")
q_block, q_block_internal_id = _table_q_factory("block", "block_id")
q_human, q_human_internal_id = _table_q_factory("human", "email")


def q_vlob_encryption_revision_internal_id(
    encryption_revision: str,
    organization_id: str | None = None,
    organization: str | None = None,
    realm_id: str | None = None,
    realm: str | None = None,
    table: str = "vlob_encryption_revision",
) -> str:
    if realm is None:
        assert realm_id is not None
        assert organization_id is not None or organization is not None
        if organization is None:
            _q_realm = q_realm_internal_id(organization_id=organization_id, realm_id=realm_id)
        else:
            _q_realm = q_realm_internal_id(organization=organization, realm_id=realm_id)
    else:
        _q_realm = realm
    return f"""
(
SELECT _id
FROM {table}
WHERE
    {table}.realm = {_q_realm}
    AND {table}.encryption_revision = {encryption_revision}
)
"""


def q_user_can_read_vlob(
    user: str | None = None,
    user_id: str | None = None,
    realm: str | None = None,
    realm_id: str | None = None,
    organization: str | None = None,
    organization_id: str | None = None,
    table: str = "realm_user_role",
) -> str:
    if user is None:
        assert organization_id is not None and user_id is not None
        _q_user = q_user_internal_id(
            organization=organization, organization_id=organization_id, user_id=user_id
        )
    else:
        _q_user = user

    if realm is None:
        assert organization_id is not None and realm_id is not None
        _q_realm = q_realm_internal_id(
            organization=organization, organization_id=organization_id, realm_id=realm_id
        )
    else:
        _q_realm = realm

    return f"""
COALESCE(
    (
        SELECT { table }.role IS NOT NULL
        FROM { table }
        WHERE
            { table }.realm = { _q_realm }
            AND { table }.user_ = { _q_user }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    False
)
"""


def q_user_can_write_vlob(
    user: str | None = None,
    user_id: str | None = None,
    realm: str | None = None,
    realm_id: str | None = None,
    organization: str | None = None,
    organization_id: str | None = None,
    table: str = "realm_user_role",
) -> str:
    if user is None:
        assert organization_id is not None and user_id is not None
        _q_user = q_user_internal_id(
            organization=organization, organization_id=organization_id, user_id=user_id
        )
    else:
        _q_user = user

    if realm is None:
        assert organization_id is not None and realm_id is not None
        _q_realm = q_realm_internal_id(
            organization=organization, organization_id=organization_id, realm_id=realm_id
        )
    else:
        _q_realm = realm

    return f"""
COALESCE(
    (
        SELECT { table }.role IN ('CONTRIBUTOR', 'MANAGER', 'OWNER')
        FROM { table }
        WHERE
            { table }.realm = { _q_realm }
            AND { table }.user_ = { _q_user }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    False
)
"""


def query(
    in_transaction: bool = False,
) -> Callable[
    [Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]]],
    Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]],
]:
    if in_transaction:

        def decorator(
            fn: Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]]
        ) -> Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]]:
            @wraps(fn)
            async def wrapper(
                conn: triopg._triopg.TrioConnectionProxy, *args: P.args, **kwargs: P.kwargs
            ) -> T:
                async with conn.transaction():
                    return await fn(conn, *args, **kwargs)

            return wrapper

    else:

        def decorator(
            fn: Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]]
        ) -> Callable[Concatenate[triopg._triopg.TrioConnectionProxy, P], Awaitable[T]]:
            return fn

    return decorator
