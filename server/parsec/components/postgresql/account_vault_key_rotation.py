# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Literal

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    HashDigest,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
)
from parsec.components.account import AccountVaultKeyRotation
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_account_and_vault_from_auth_method = Q("""
SELECT
    account._id AS account_internal_id,
    vault._id AS vault_internal_id
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

_q_get_vault_item_fingerprints = Q("""
SELECT fingerprint
FROM vault_item
WHERE vault = $vault_internal_id
""")

_q_disable_previous_auth_methods = Q("""
UPDATE vault_authentication_method
SET disabled_on = $now
WHERE
    vault IN (
        SELECT _id
        FROM vault
        WHERE account = $account_internal_id
    ) AND disabled_on IS NULL
RETURNING _id
""")

_q_insert_vault_and_auth_method = Q("""
WITH new_vault AS (
    INSERT INTO vault (
        account
    )
    VALUES (
        $account_internal_id
    )
    RETURNING _id
),

new_auth_method AS (
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
        $new_auth_method_id,
        (SELECT _id FROM new_vault),
        $now,
        $created_by_ip,
        $created_by_user_agent,
        $new_auth_method_mac_key,
        $new_vault_key_access,
        $new_auth_method_password_algorithm,
        $new_auth_method_password_algorithm_argon2id_opslimit,
        $new_auth_method_password_algorithm_argon2id_memlimit_kb,
        $new_auth_method_password_algorithm_argon2id_parallelism
    )
    ON CONFLICT (auth_method_id) DO NOTHING
    RETURNING _id
)

SELECT
    (SELECT _id FROM new_auth_method) AS new_auth_method_internal_id,
    (SELECT _id FROM new_vault) AS new_vault_internal_id
""")

_q_insert_vault_item = Q("""
INSERT INTO vault_item (vault, fingerprint, data)
VALUES ($vault_internal_id, $fingerprint, $data)
""")


async def vault_key_rotation(
    conn: AsyncpgConnection,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
    created_by_ip: str | Literal[""],
    created_by_user_agent: str,
    new_auth_method_id: AccountAuthMethodID,
    new_auth_method_mac_key: SecretKey,
    new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
    new_vault_key_access: bytes,
    items: dict[HashDigest, bytes],
) -> None | AccountVaultKeyRotation:
    # 1) Get account and current vault

    row = await conn.fetchrow(
        *_q_get_account_and_vault_from_auth_method(auth_method_id=auth_method_id)
    )
    if not row:
        return AccountVaultKeyRotation.ACCOUNT_NOT_FOUND

    match row["account_internal_id"]:
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    match row["vault_internal_id"]:
        case int() as current_vault_internal_id:
            pass
        case _:
            assert False, row

    # 2) Ensure that the new items correspond to the ones in the current vault

    rows = await conn.fetch(
        *_q_get_vault_item_fingerprints(vault_internal_id=current_vault_internal_id)
    )
    current_vault_item_fingerprints = {HashDigest(row["fingerprint"]) for row in rows}
    if items.keys() != current_vault_item_fingerprints:
        return AccountVaultKeyRotation.ITEMS_MISMATCH

    # Not that since all operations modifying the account, its vaults or, its authentication
    # methods must first take a lock on the account row, we are guaranteed that we won't
    # get a current vault item upload.

    # 3) Key rotation means the current vault is going to be replaced
    # with a new one, so we need to disable its authentication methods.

    # Must use a separate query to disable previous auth methods.
    # This is because we are going to insert a new auth method, and doing both
    # in the same query is error-prone since it requires precise ordering.
    await conn.execute(
        *_q_disable_previous_auth_methods(account_internal_id=account_internal_id, now=now)
    )

    # 4) Insert the new vault and its initial authentication method

    match new_auth_method_password_algorithm:
        case None:
            algorithm = None
            algorithm_argon2id_opslimit = None
            algorithm_argon2id_memlimit_kb = None
            algorithm_argon2id_parallelism = None
        case UntrustedPasswordAlgorithmArgon2id() as algo:
            algorithm = "ARGON2ID"
            algorithm_argon2id_opslimit = algo.opslimit
            algorithm_argon2id_memlimit_kb = algo.memlimit_kb
            algorithm_argon2id_parallelism = algo.parallelism
        case unknown:
            # A new kind of password algorithm has been added, but we forgot to update this code?
            assert False, f"Unexpected new_auth_method_password_algorithm: {unknown!r}"

    row = await conn.fetchrow(
        *_q_insert_vault_and_auth_method(
            now=now,
            account_internal_id=account_internal_id,
            created_by_user_agent=created_by_user_agent,
            created_by_ip=created_by_ip,
            new_auth_method_id=new_auth_method_id,
            new_auth_method_mac_key=new_auth_method_mac_key.secret,
            new_vault_key_access=new_vault_key_access,
            new_auth_method_password_algorithm=algorithm,
            new_auth_method_password_algorithm_argon2id_opslimit=algorithm_argon2id_opslimit,
            new_auth_method_password_algorithm_argon2id_memlimit_kb=algorithm_argon2id_memlimit_kb,
            new_auth_method_password_algorithm_argon2id_parallelism=algorithm_argon2id_parallelism,
        )
    )
    assert row is not None

    match row["new_auth_method_internal_id"]:
        case int():
            pass
        case None:
            # `new_auth_method_id` already exists in the database, return an error
            # and rollback the transaction.
            return AccountVaultKeyRotation.NEW_AUTH_METHOD_ID_ALREADY_EXISTS
        case _:
            assert False, row

    match row["new_vault_internal_id"]:
        case int() as new_vault_internal_id:
            pass
        case _:
            assert False, row

    # 5) Finally insert all the items into the new vault

    def arg_gen():
        for fingerprint, data in items.items():
            yield _q_insert_vault_item.arg_only(
                vault_internal_id=new_vault_internal_id,
                fingerprint=fingerprint.digest,
                data=data,
            )

    await conn.executemany(_q_insert_vault_item.sql, arg_gen())

    return None
