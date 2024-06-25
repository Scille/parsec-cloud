# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import re
from functools import wraps
from typing import Any, Awaitable, Callable, Protocol, TypeVar, cast

from typing_extensions import Concatenate, ParamSpec

from parsec.types import BadOutcome

from . import AsyncpgConnection, AsyncpgPool

T = TypeVar("T")
P = ParamSpec("P")


class Q:
    """
    Dead simple SQL query composition framework (◠_◠)

    A Q object is constructed from an SQL query string containing variables
    (such as `$user_id`). These variables are replaced by [positional parameters](https://www.postgresql.org/docs/current/sql-expressions.html#SQL-EXPRESSIONS-PARAMETERS-POSITIONAL)
    (such as `$1`). When the Q object is called, the values are supplied
    externally to the SQL query.

    Example:
    >>> _q_get_user = Q("SELECT name FROM user WHERE user_id = $user_id")
    >>> print(_q_get_user(user_id=42))
    ['SELECT name FROM user WHERE user_id = $1', 42]
    """

    def __init__(self, src: str, **kwargs: Any):
        # Retrieve variables in src query string
        variables: dict[str, str] = {}
        for candidate in re.findall(r"\$([a-zA-Z0-9_]+)", src):
            if candidate in variables:
                continue
            if candidate.isdigit():
                raise ValueError(f"Invalid variable name `{candidate}`")
            variables[candidate] = f"${len(variables) + 1}"
        self._variables = variables

        # Replace variables with their order-based positional parameters
        # Variables are sorted in reverse order in case a variable name
        # is a suffix of another variable name (i.e. replace "var_len"
        # before "var", otherwise we'll get something like "$1_len")
        for variable, positional_parameter in sorted(variables.items(), reverse=True):
            src = src.replace(f"${variable}", positional_parameter)

        self._sql = src
        self._stripped_sql = " ".join([x.strip() for x in src.split()])

    @property
    def sql(self) -> str:
        return self._sql

    def __call__(self, **kwargs: Any) -> list[Any]:
        return [self._stripped_sql, *self.arg_only(**kwargs)]

    def arg_only(self, **kwargs: Any) -> list[Any]:
        if kwargs.keys() != self._variables.keys():
            missing = self._variables.keys() - kwargs.keys()
            unknown = kwargs.keys() - self._variables.keys()
            raise ValueError(f"Invalid parameters, missing: {missing}, unknown: {unknown}")

        return [kwargs[variable] for variable in self._variables]


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
) -> tuple[Callable[..., str], Callable[..., str]]:
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
q_invitation, q_invitation_internal_id = _table_q_factory("invitation", "token")


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
    [Callable[Concatenate[AsyncpgConnection, P], Awaitable[T]]],
    Callable[Concatenate[AsyncpgConnection, P], Awaitable[T]],
]:
    if not in_transaction:
        return lambda fn: fn

    def decorator(
        fn: Callable[Concatenate[AsyncpgConnection, P], Awaitable[T]],
    ) -> Callable[Concatenate[AsyncpgConnection, P], Awaitable[T]]:
        @wraps(fn)
        async def wrapper(conn: AsyncpgConnection, *args: P.args, **kwargs: P.kwargs) -> T:
            async with conn.transaction():
                return await fn(conn, *args, **kwargs)

        return wrapper

    return decorator


class WithPool(Protocol):
    pool: AsyncpgPool


def transaction[**P, T, S: WithPool](
    func: Callable[Concatenate[S, AsyncpgConnection, P], Awaitable[T]],
):
    """
    This is used to decorate an API method that needs to be executed in a transaction.

    It makes sure that the transaction is rolled back if the function returns a BadOutcome.
    """

    async def wrapper(self: S, *args: P.args, **kwargs: P.kwargs) -> T:
        async with self.pool.acquire() as conn:
            conn = cast(AsyncpgConnection, conn)
            transaction = conn.transaction()
            await transaction.start()
            try:
                result = await func(self, conn, *args, **kwargs)
            except:
                await transaction.rollback()
                raise
            match result:
                case BadOutcome():
                    await transaction.rollback()
                case _:
                    await transaction.commit()
            return result

    return wrapper
