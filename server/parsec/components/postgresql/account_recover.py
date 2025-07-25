# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Literal, cast

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
    ValidationCode,
)
from parsec.components.account import (
    AccountRecoverProceedBadOutcome,
    AccountRecoverSendValidationEmailBadOutcome,
    ValidationCodeInfo,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import Q

_q_check_account_exists_and_not_deleted = Q("""
SELECT _id
FROM account
WHERE
    email = $email
    AND deleted_on IS NULL
LIMIT 1
-- All queries modifying the account, its vaults or, its authentication methods must
-- first take a lock on the account row.
-- This is to avoid inconsistent state due to concurrent operations (e.g.
-- deleting an account while creating a new authentication method, leading to
-- a deleted account with a non-disabled authentication method!).
FOR UPDATE
""")


_q_insert_validation_code = Q("""
INSERT INTO account_recover_validation_code (
    account,
    validation_code,
    created_at
) VALUES (
    $account_internal_id,
    $validation_code,
    $created_at
)
-- Note we simply overwrite any existing previous validation code
ON CONFLICT (account) DO UPDATE
    SET
        account = excluded.account,
        validation_code = excluded.validation_code,
        created_at = excluded.created_at,
        failed_attempts = 0
""")


async def recover_send_validation_email(
    conn: AsyncpgConnection,
    now: DateTime,
    email: EmailAddress,
) -> ValidationCode | AccountRecoverSendValidationEmailBadOutcome:
    account_internal_id = await conn.fetchval(
        *_q_check_account_exists_and_not_deleted(email=email.str)
    )
    match account_internal_id:
        case None:
            return AccountRecoverSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND
        case int():
            pass
        case _:
            assert False, account_internal_id

    validation_code = ValidationCode.generate()
    await conn.execute(
        *_q_insert_validation_code(
            account_internal_id=account_internal_id,
            validation_code=validation_code.str,
            created_at=now,
        )
    )

    return validation_code


_q_get_account_and_validation_code = Q("""
WITH my_account AS (
    SELECT _id
    FROM account
    WHERE
        email = $email
        -- Extra safety check since a deleted account should not have a validation code
        AND deleted_on IS NULL
    LIMIT 1
    -- All queries modifying the account, its vaults or, its authentication methods must
    -- first take a lock on the account row.
    -- This is to avoid inconsistent state due to concurrent operations (e.g.
    -- deleting an account while creating a new authentication method, leading to
    -- a deleted account with a non-disabled authentication method!).
    FOR UPDATE
),

my_validation_code AS (
    SELECT
        validation_code,
        created_at,
        failed_attempts
    FROM account_recover_validation_code
    WHERE account = (SELECT my_account._id FROM my_account)
    LIMIT 1
    -- Lock for update since this row should be updated on failed attempt
    FOR UPDATE
)

SELECT
    (SELECT * FROM my_account) AS account_internal_id,
    (SELECT validation_code FROM my_validation_code) AS expected_validation_code,
    (SELECT created_at FROM my_validation_code) AS validation_code_created_at,
    (SELECT failed_attempts FROM my_validation_code) AS validation_code_failed_attempts
""")


_q_register_failed_attempt = Q("""
UPDATE account_recover_validation_code
SET failed_attempts = failed_attempts + 1
WHERE account = $account_internal_id
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


_q_recover_account = Q("""
WITH removed_validation_code AS (
    DELETE FROM account_recover_validation_code
    WHERE account = $account_internal_id
    RETURNING validation_code
),

new_vault AS (
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
    (SELECT validation_code FROM removed_validation_code) AS expected_validation_code,
    (SELECT _id FROM new_auth_method) AS new_auth_method_internal_id
""")


async def recover_proceed(
    pool: AsyncpgPool,
    now: DateTime,
    validation_code: ValidationCode,
    email: EmailAddress,
    created_by_user_agent: str,
    created_by_ip: str | Literal[""],
    new_vault_key_access: bytes,
    new_auth_method_id: AccountAuthMethodID,
    new_auth_method_mac_key: SecretKey,
    new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
) -> None | AccountRecoverProceedBadOutcome:
    # Acquire a connection from the pool by hand is needed here since we don't
    # want a rollback-on-error behavior (which is what the `@transaction`
    # decorator does).
    # This is to handle invalid validation code, since in this case an error is
    # returned but we still have to update the database to register the failed attempt.
    async with pool.acquire() as conn:
        conn = cast(AsyncpgConnection, conn)
        transaction = conn.transaction()
        await transaction.start()
        try:
            should_commit, result = await _recover_proceed(
                conn,
                now,
                validation_code,
                email,
                created_by_user_agent,
                created_by_ip,
                new_vault_key_access,
                new_auth_method_id,
                new_auth_method_mac_key,
                new_auth_method_password_algorithm,
            )
        except:
            await transaction.rollback()
            raise
        if should_commit:
            await transaction.commit()
        else:
            await transaction.rollback()
        return result


def _commit[R](x: R = None) -> tuple[Literal[True], R]:
    return (True, x)


def _rollback[R](x: R = None) -> tuple[Literal[False], R]:
    return (False, x)


async def _recover_proceed(
    conn: AsyncpgConnection,
    now: DateTime,
    validation_code: ValidationCode,
    email: EmailAddress,
    created_by_user_agent: str,
    created_by_ip: str | Literal[""],
    new_vault_key_access: bytes,
    new_auth_method_id: AccountAuthMethodID,
    new_auth_method_mac_key: SecretKey,
    new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
) -> tuple[bool, None | AccountRecoverProceedBadOutcome]:
    # 1) Check the validation code and retrieve the account

    row = await conn.fetchrow(*_q_get_account_and_validation_code(email=email.str))
    assert row is not None

    match row["expected_validation_code"]:
        case None:
            # No validation code found for this email in the database
            return _rollback(AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED)
        case str() as raw_expected_validation_code:
            expected_validation_code = ValidationCode(raw_expected_validation_code)
        case _:
            assert False, row

    # Since the validation code exist, we are guaranteed to have an account
    match row["account_internal_id"]:
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    match row["validation_code_created_at"]:
        case DateTime() as created_at:
            pass
        case _:
            assert False, row

    match row["validation_code_failed_attempts"]:
        case int() as failed_attempts:
            assert failed_attempts >= 0, row
        case _:
            assert False, row

    last_mail_info = ValidationCodeInfo(
        validation_code=expected_validation_code,
        created_at=created_at,
        failed_attempts=failed_attempts,
    )

    if not last_mail_info.can_be_used(now):
        # The validation code in the database is too old, consider it doesn't exist
        return _rollback(AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED)

    # The validation code in the database is valid...
    if last_mail_info.validation_code != validation_code:
        # ...but doesn't match the one provided by the client, so we register a failed attempt
        await conn.execute(*_q_register_failed_attempt(account_internal_id=account_internal_id))
        return _commit(AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE)

    # 3) Validation code is valid, proceed with account recovery

    # Must use a separate query to disable previous auth methods.
    # This is because we are going to insert a new auth method, and doing both
    # in the same query is error-prone since it requires precise ordering.
    await conn.execute(
        *_q_disable_previous_auth_methods(account_internal_id=account_internal_id, now=now)
    )

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
        *_q_recover_account(
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

    # Sanity check to ensure the transaction was held between validation code check and actual account recovery
    assert row["expected_validation_code"] == validation_code.str, row

    match row["new_auth_method_internal_id"]:
        case int():
            pass
        case None:
            # `new_auth_method_id` already exists in the database, return an error
            # and rollback the transaction.
            return _rollback(AccountRecoverProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS)
        case _:
            assert False, row

    # All done! Ask for the transaction to be committed
    return _commit()
