# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
)
from parsec.components.account import AccountAuthMethodDisableBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_account_from_auth_method = Q("""
SELECT account._id AS account_internal_id
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault_authentication_method.auth_method_id = $auth_method_id
    AND vault_authentication_method.disabled_on IS NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
LIMIT 1
-- All queries modifying the account, its vaults or, its authentication methods must
-- first take a lock on the account row.
-- This is to avoid inconsistent state due to concurrent operations (e.g.
-- deleting an account while creating a new authentication method, leading to
-- a deleted account with a non-disabled authentication method!).
FOR UPDATE
""")

_q_check_to_disable_auth_method = Q("""
SELECT
    vault_authentication_method._id AS to_disable_auth_method_internal_id,
    (account._id = $account_internal_id) AS is_same_account,
    vault_authentication_method.disabled_on IS NOT NULL AS already_disabled
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault_authentication_method.auth_method_id = $to_disable_auth_method_id
LIMIT 1
FOR UPDATE
""")

_q_disable_auth_method = Q("""
WITH disabled_auth_method AS (
    UPDATE vault_authentication_method
    SET disabled_on = $now
    WHERE
        _id = $to_disable_auth_method_internal_id
    RETURNING _id
)

SELECT (SELECT COUNT(*) FROM disabled_auth_method) AS disabled_count
""")


async def auth_method_disable(
    conn: AsyncpgConnection,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
    to_disable_auth_method_id: AccountAuthMethodID,
) -> None | AccountAuthMethodDisableBadOutcome:
    if auth_method_id == to_disable_auth_method_id:
        return AccountAuthMethodDisableBadOutcome.SELF_DISABLE_NOT_ALLOWED

    # 1) Retrieve account & vault

    row = await conn.fetchrow(*_q_get_account_from_auth_method(auth_method_id=auth_method_id))
    if not row:
        return AccountAuthMethodDisableBadOutcome.ACCOUNT_NOT_FOUND

    match row["account_internal_id"]:
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    # 2) Check if target

    row = await conn.fetchrow(
        *_q_check_to_disable_auth_method(
            account_internal_id=account_internal_id,
            to_disable_auth_method_id=to_disable_auth_method_id,
        )
    )
    if not row:
        return AccountAuthMethodDisableBadOutcome.AUTH_METHOD_NOT_FOUND

    match row["is_same_account"]:
        case True:
            pass
        case False:
            return AccountAuthMethodDisableBadOutcome.CROSS_ACCOUNT_NOT_ALLOWED
        case _:
            assert False, row

    match row["already_disabled"]:
        case True:
            return AccountAuthMethodDisableBadOutcome.AUTH_METHOD_ALREADY_DISABLED
        case False:
            pass
        case _:
            assert False, row

    match row["to_disable_auth_method_internal_id"]:
        case int() as to_disable_auth_method_internal_id:
            pass
        case _:
            assert False, row

    # 3) Actually disable the auth method

    disabled_count = await conn.fetchval(
        *_q_disable_auth_method(
            now=now,
            to_disable_auth_method_internal_id=to_disable_auth_method_internal_id,
        )
    )
    assert disabled_count == 1, disabled_count
