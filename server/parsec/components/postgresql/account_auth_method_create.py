# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal

from asyncpg import UniqueViolationError

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
)
from parsec.components.account import AccountAuthMethodCreateBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_current_vault_from_auth_method = Q("""
SELECT vault._id AS vault_internal_id
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

_q_disable_password_auth_methods = Q("""
WITH disabled_auth_methods AS (
    UPDATE vault_authentication_method
    SET disabled_on = $now
    WHERE
        vault = $vault_internal_id
        AND password_algorithm IS NOT NULL
        AND disabled_on IS NULL
    RETURNING _id
)

SELECT (SELECT COUNT(*) FROM disabled_auth_methods) AS disabled_count
""")

_q_insert_auth_method = Q("""
INSERT INTO vault_authentication_method (
    auth_method_id,
    vault,
    created_on,
    created_by_ip,
    created_by_user_agent,
    mac_key,
    vault_key_access,
    password_algorithm,
    password_algorithm_argon2id_opslimit,
    password_algorithm_argon2id_memlimit_kb,
    password_algorithm_argon2id_parallelism
)
VALUES (
    $auth_method_id,
    $vault_internal_id,
    $now,
    $created_by_ip,
    $created_by_user_agent,
    $mac_key,
    $vault_key_access,
    $password_algorithm,
    $password_algorithm_argon2id_opslimit,
    $password_algorithm_argon2id_memlimit_kb,
    $password_algorithm_argon2id_parallelism
)
""")


async def auth_method_create(
    conn: AsyncpgConnection,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
    created_by_user_agent: str,
    created_by_ip: str | Literal[""],
    new_auth_method_id: AccountAuthMethodID,
    new_auth_method_mac_key: SecretKey,
    new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
    new_vault_key_access: bytes,
) -> None | AccountAuthMethodCreateBadOutcome:
    # 1) Retrieve account & vault

    row = await conn.fetchrow(*_q_get_current_vault_from_auth_method(auth_method_id=auth_method_id))
    if not row:
        return AccountAuthMethodCreateBadOutcome.ACCOUNT_NOT_FOUND

    match row["vault_internal_id"]:
        case int() as vault_internal_id:
            pass
        case _:
            assert False, row

    # 2) If we're creating a password authentication method, disable any existing
    # password authentication methods in the current vault

    if new_auth_method_password_algorithm is not None:
        disabled_count = await conn.fetchval(
            *_q_disable_password_auth_methods(
                now=now,
                vault_internal_id=vault_internal_id,
            )
        )
        # In theory there should be at most a single active password authentication
        # method per vault.
        assert disabled_count == 0 or disabled_count == 1, disabled_count

    # 3) Actual creation of the authentication method

    match new_auth_method_password_algorithm:
        case None:
            password_algorithm = None
            password_algorithm_argon2id_opslimit = None
            password_algorithm_argon2id_memlimit_kb = None
            password_algorithm_argon2id_parallelism = None
        case UntrustedPasswordAlgorithmArgon2id() as algo:
            password_algorithm = "ARGON2ID"
            password_algorithm_argon2id_opslimit = algo.opslimit
            password_algorithm_argon2id_memlimit_kb = algo.memlimit_kb
            password_algorithm_argon2id_parallelism = algo.parallelism
        case unknown:
            # A new kind of password algorithm has been added, but we forgot to update this code?
            assert False, f"Unexpected auth_method_password_algorithm: {unknown!r}"

    try:
        row = await conn.fetchrow(
            *_q_insert_auth_method(
                now=now,
                vault_internal_id=vault_internal_id,
                created_by_user_agent=created_by_user_agent,
                created_by_ip=created_by_ip,
                auth_method_id=new_auth_method_id,
                mac_key=new_auth_method_mac_key.secret,
                password_algorithm=password_algorithm,
                password_algorithm_argon2id_opslimit=password_algorithm_argon2id_opslimit,
                password_algorithm_argon2id_memlimit_kb=password_algorithm_argon2id_memlimit_kb,
                password_algorithm_argon2id_parallelism=password_algorithm_argon2id_parallelism,
                vault_key_access=new_vault_key_access,
            )
        )

    except UniqueViolationError:
        return AccountAuthMethodCreateBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS
