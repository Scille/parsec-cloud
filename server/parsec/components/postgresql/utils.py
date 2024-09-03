# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import importlib
import re
import traceback
from functools import wraps
from typing import Any, Callable, Coroutine, Iterable, Protocol, TypeVar, cast

from typing_extensions import Concatenate, ParamSpec

from parsec._parsec import (
    ActiveUsersLimit,
    BlockID,
    DateTime,
    DeviceID,
    GreetingAttemptID,
    UserID,
    VlobID,
)
from parsec.types import BadOutcome

from . import AsyncpgConnection, AsyncpgPool

T = TypeVar("T")
P = ParamSpec("P")
SqlQueryParam = (
    None
    | str
    | bytes
    | int
    | bool
    | float
    | DateTime
    | UserID
    | DeviceID
    | VlobID
    | BlockID
    | GreetingAttemptID
    | ActiveUsersLimit
    | Iterable["SqlQueryParam"]
)


# Yes, we can lint the SQL queries declared with `Q(<sql>)` !!!
#
# This is done when calling `misc/lint_sql.py`.
#
# Basically this script registers a module called `__parsec_lint_sql` that
# contains a hook to call for each sql query to check.
_LINT_Q_SQL: Callable[[str, dict[str, str]], None] | None
try:
    import importlib

    # Deptry (which is run as part of pre-commit) detects that `__parsec_lint_sql`
    # doesn't exist and complain about it !
    # Unfortunately Deptry doesn't support `# ignore` inline disable comment, so
    # we have instead to obfuscate the import so that Deptry no longer detects it...
    mod_name = "__parsec_lint_sql"
    __parsec_lint_sql = importlib.import_module(mod_name)
    _LINT_Q_SQL = __parsec_lint_sql.lint_sql
except ImportError:
    _LINT_Q_SQL = None


class Q:
    """
    Dead simple SQL query composition framework (◠_◠)

    A Q object is constructed from an SQL query string containing variables
    (such as `$user_id`). These variables are replaced by [positional parameters](https://www.postgresql.org/docs/current/sql-expressions.html#SQL-EXPRESSIONS-PARAMETERS-POSITIONAL)
    (such as `$1`).

    When the Q object is called, the values are supplied after the query.

    Example:
    >>> _q_get_user = Q("SELECT name FROM user WHERE user_id = $user_id")
    >>> print(_q_get_user.sql)
    SELECT name FROM user WHERE user_id = $1
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

        # Remove comment (e.g. `-- foo`)
        lines = [line.split("--")[0] for line in src.splitlines()]
        # Remove unecessary indentation and newlines
        self._stripped_sql = " ".join([x.strip() for line in lines for x in line.split()])

        if _LINT_Q_SQL:
            _LINT_Q_SQL(self._sql, variables)

    @property
    def sql(self) -> str:
        return self._sql

    async def test_explain_and_exit(
        self, conn: AsyncpgConnection, **kwargs: SqlQueryParam
    ) -> tuple[Any, ...]:
        """
        Quit'n dirty way to debug a query, example:

        ```python
            _q_get_foo = Q("SELECT * FROM foo WHERE id = $id")
            row = await conn.fetch(*_q_get_foo(id=42))  # <-- Modify this...
            row = await conn.fetch(*await _q_get_foo.test_explain_and_exit(id=42)) # <-- ...into this !
        ```

        This will print the explain query on stdout and exit the program.
        """
        args = self.arg_only(**kwargs)
        sql = "EXPLAIN " + self._stripped_sql
        rows = await conn.fetch(sql, *args)

        caller_frame = traceback.extract_stack()[-2]

        BOLD_RED = "\x1b[1;31m"
        GREY = "\x1b[37m"
        NO_COLOR = "\x1b[0;0m"

        msg = "\n".join(
            (
                f"{BOLD_RED}In {caller_frame.filename}:{caller_frame.lineno}{NO_COLOR}",
                f"----------------------- EXPLAIN Q in {caller_frame.name} --------------------",
                GREY,
                *(row[0] for row in rows),
                NO_COLOR,
                f"--------------------- END EXPLAIN Q in {caller_frame.name} -------------------",
            )
        )

        # Existing the program here has two purposes:
        # - It makes easy to display the query (e.g. no need to pass `-s` when running pytest)
        # - PostgreSQL's EXPLAIN actually execute the query, hence if the query
        #   is an INSERT/UPDATE our program is broken anyway if we try to continue.
        raise SystemExit(msg)

    def __call__(self, **kwargs: SqlQueryParam) -> tuple[Any, ...]:
        return (self._stripped_sql, *self.arg_only(**kwargs))

    def arg_only(self, **kwargs: SqlQueryParam) -> tuple[Any, ...]:
        if kwargs.keys() != self._variables.keys():
            missing = self._variables.keys() - kwargs.keys()
            unknown = kwargs.keys() - self._variables.keys()
            raise ValueError(f"Invalid parameters, missing: {missing}, unknown: {unknown}")

        return tuple(kwargs[variable] for variable in self._variables)


def q_organization(
    organization_id: str | None = None,
    _id: str | None = None,
    select: str = "*",
) -> str:
    assert organization_id is not None or _id is not None
    if _id is not None:
        condition = f"organization._id = {_id}"
    else:
        condition = f"organization.organization_id = {organization_id}"
    return f"(SELECT {select} FROM organization WHERE {condition})"


def q_organization_internal_id(organization_id: str, **kwargs: Any) -> str:
    """
    Query the organization's internal ID for the given organization public ID

    ```sql
    SELECT _id FROM organization WHERE organization.organization_id = {organization_id}
    ```
    """
    return q_organization(organization_id=organization_id, select="_id", **kwargs)


def _table_q_factory(
    table: str,
    public_id_column: str,
) -> tuple[Callable[..., str], Callable[..., str]]:
    """
    Returns a tuple of helper functions to query the specified table.

    The first one can be used to query any column from the table. It is useful
    as a subquery where internal ID (`_id`) should equal the column of an external
    query (such as `author` in the example below).

    The second one is used to query internal ID (`_id`) for the given public ID
    (`public_id_column`) of the table.

    >>> q_device, q_device_internal_id = _table_q_factory("device", "device_id")

    >>> q_device(select="device_id", _id="author")
    SELECT device_id FROM device WHERE device._id = author

    >>> q_device_internal_id(organization_id="$organization_id", device_id="$author")
    SELECT _id
    FROM device
    WHERE device.device_id = $author
      AND device.organization = (SELECT _id
                                 FROM organization
                                 WHERE organization.organization_id = $organization_id)
    """

    def _q(
        organization_id: str | None = None,
        organization: str | None = None,
        _id: str | None = None,
        table_alias: str | None = None,
        select: str = "*",
        suffix: str | None = None,
        **kwargs: Any,
    ) -> str:
        # Use table alias if specified
        from_table = f"{table} AS {table_alias}" if table_alias else table
        select_table = table_alias if table_alias else table

        # Use either internal ID (_id) or public ID for the WHERE condition
        if _id is not None:
            condition = f"{select_table}._id = {_id}"
        else:
            public_id = kwargs.pop(public_id_column, None)
            assert public_id is not None

            # When using public ID, the organization should also be specified.
            # If the organization's internal ID (`organization`) is not specified,
            # the organization's public ID (`organization_id`) is used to retrieve it.
            if organization is None:
                assert organization_id is not None
                organization = q_organization_internal_id(organization_id)

            filter1 = f"{select_table}.{public_id_column} = {public_id}"
            filter2 = f"{select_table}.organization = {organization}"
            condition = f"{filter1} AND {filter2}"

        assert not kwargs
        suffix = suffix or ""
        return f"(SELECT {select} FROM {from_table} WHERE {condition} {suffix})"

    def _q_internal_id(**kwargs: Any) -> str:
        return _q(select="_id", **kwargs)

    # Useful for function representation & debugging
    _q.__name__ = f"q_{table}"
    _q_internal_id.__name__ = f"q_{table}_internal_id"

    return _q, _q_internal_id


# Helper query functions for the main tables (see _table_q_factory docstring).
q_device, q_device_internal_id = _table_q_factory("device", "device_id")
q_user, q_user_internal_id = _table_q_factory("user_", "user_id")
q_realm, q_realm_internal_id = _table_q_factory("realm", "realm_id")
q_block, q_block_internal_id = _table_q_factory("block", "block_id")
q_human, q_human_internal_id = _table_q_factory("human", "email")
q_invitation, q_invitation_internal_id = _table_q_factory("invitation", "token")


class WithPool(Protocol):
    pool: AsyncpgPool


def transaction[**P, T, S: WithPool](
    func: Callable[Concatenate[S, AsyncpgConnection, P], Coroutine[Any, Any, T]],
) -> Callable[Concatenate[S, P], Coroutine[Any, Any, T]]:
    """
    This is used to decorate an API method that needs to be executed in a transaction.

    It makes sure that the transaction is rolled back if the function returns a BadOutcome.
    """

    @wraps(func)
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


def no_transaction[**P, T, S: WithPool](
    func: Callable[Concatenate[S, AsyncpgConnection, P], Coroutine[Any, Any, T]],
) -> Callable[Concatenate[S, P], Coroutine[Any, Any, T]]:
    """
    This is used to decorate an API method that needs to request the database
    without transaction (i.e. for query that doesn't do write operations).
    """

    @wraps(func)
    async def wrapper(self: S, *args: P.args, **kwargs: P.kwargs) -> T:
        async with self.pool.acquire() as conn:
            conn = cast(AsyncpgConnection, conn)
            return await func(self, conn, *args, **kwargs)

    return wrapper


class RetryNeeded(BaseException):
    pass


def retryable[S, **P, T](
    func: Callable[Concatenate[S, P], Coroutine[Any, Any, T]],
) -> Callable[Concatenate[S, P], Coroutine[Any, Any, T]]:
    @wraps(func)
    async def wrapper(self: S, *args: P.args, **kwargs: P.kwargs) -> T:
        while True:
            try:
                return await func(self, *args, **kwargs)
            except RetryNeeded:
                pass

    return wrapper
