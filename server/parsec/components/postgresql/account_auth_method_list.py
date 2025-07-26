# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    UntrustedPasswordAlgorithmArgon2id,
)
from parsec.components.account import AccountAuthMethodListBadOutcome, AuthMethod
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_active_auth_methods = Q("""
WITH my_vault AS (
    SELECT vault._id
    FROM vault_authentication_method
    INNER JOIN vault ON vault_authentication_method.vault = vault._id
    INNER JOIN account ON vault.account = account._id
    WHERE
        vault_authentication_method.auth_method_id = $auth_method_id
        AND vault_authentication_method.disabled_on IS NULL
        -- Extra safety check since a deleted account should not have any active authentication method
        AND account.deleted_on IS NULL
)

SELECT
    vault_authentication_method.auth_method_id,
    vault_authentication_method.created_on,
    vault_authentication_method.created_by_ip,
    vault_authentication_method.created_by_user_agent,
    vault_authentication_method.vault_key_access,
    vault_authentication_method.password_algorithm,
    vault_authentication_method.password_algorithm_argon2id_opslimit,
    vault_authentication_method.password_algorithm_argon2id_memlimit_kb,
    vault_authentication_method.password_algorithm_argon2id_parallelism
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    vault._id = (SELECT my_vault._id FROM my_vault)
    -- Note that since we have already checked `disabled_on` in `my_vault` CTE,
    -- we are guaranteed to get at least one row here (i.e. our own auth method !).
    AND vault_authentication_method.disabled_on IS NULL
ORDER BY vault_authentication_method.created_on ASC
""")


async def auth_method_list(
    conn: AsyncpgConnection,
    auth_method_id: AccountAuthMethodID,
) -> list[AuthMethod] | AccountAuthMethodListBadOutcome:
    rows = await conn.fetch(*_q_get_active_auth_methods(auth_method_id=auth_method_id))

    if not rows:
        return AccountAuthMethodListBadOutcome.ACCOUNT_NOT_FOUND

    auth_methods = []
    for row in rows:
        match row["auth_method_id"]:
            case str() as raw_auth_method_id:
                auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id)
            case _:
                assert False, row

        match row["created_on"]:
            case DateTime() as created_on:
                pass
            case _:
                assert False, row

        match row["created_by_ip"]:
            case str() as created_by_ip:
                pass
            case _:
                assert False, row

        match row["created_by_user_agent"]:
            case str() as created_by_user_agent:
                pass
            case _:
                assert False, row

        match row["vault_key_access"]:
            case bytes() as vault_key_access:
                pass
            case _:
                assert False, row

        # Parse password algorithm
        match row["password_algorithm"]:
            case None:
                password_algorithm = None
            case "ARGON2ID":
                password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                    opslimit=row["password_algorithm_argon2id_opslimit"],
                    memlimit_kb=row["password_algorithm_argon2id_memlimit_kb"],
                    parallelism=row["password_algorithm_argon2id_parallelism"],
                )
            case _:
                assert False, row

        auth_methods.append(
            AuthMethod(
                auth_method_id=auth_method_id,
                created_on=created_on,
                created_by_ip=created_by_ip,
                created_by_user_agent=created_by_user_agent,
                vault_key_access=vault_key_access,
                password_algorithm=password_algorithm,
                disabled_on=None,
            )
        )

    return auth_methods
