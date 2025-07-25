# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Literal, cast

from parsec._parsec import AccountAuthMethodID, DateTime, EmailAddress, ValidationCode
from parsec.components.account import (
    AccountDeleteProceedBadOutcome,
    AccountDeleteSendValidationEmailBadOutcome,
    ValidationCodeInfo,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import Q

_q_get_account_from_auth_method = Q("""
SELECT
    account._id AS account_internal_id,
    account.email
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


_q_insert_validation_code = Q("""
INSERT INTO account_delete_validation_code (
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
RETURNING TRUE
""")


async def delete_send_validation_email(
    conn: AsyncpgConnection,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
) -> tuple[ValidationCode, EmailAddress] | AccountDeleteSendValidationEmailBadOutcome:
    row = await conn.fetchrow(*_q_get_account_from_auth_method(auth_method_id=auth_method_id))
    if row is None:
        return AccountDeleteSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND

    match row["account_internal_id"]:
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    match row["email"]:
        case str() as raw_email:
            email = EmailAddress(raw_email)
        case _:
            assert False, row

    validation_code = ValidationCode.generate()
    ok = await conn.fetchval(
        *_q_insert_validation_code(
            account_internal_id=account_internal_id,
            validation_code=validation_code.str,
            created_at=now,
        )
    )
    assert ok is True

    return validation_code, email


_q_get_account_and_validation_code_from_auth_method = Q("""
WITH my_account AS (
    SELECT account._id
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
),

my_validation_code AS (
    SELECT
        validation_code,
        created_at,
        failed_attempts
    FROM account_delete_validation_code
    WHERE account = (SELECT my_account._id FROM my_account)
    LIMIT 1
    -- Lock for update since this row should be updated on failed attempt
    FOR UPDATE
)

SELECT
    (SELECT _id FROM my_account) AS account_internal_id,
    (SELECT validation_code FROM my_validation_code) AS expected_validation_code,
    (SELECT created_at FROM my_validation_code) AS created_at,
    (SELECT failed_attempts FROM my_validation_code) AS failed_attempts
""")

_q_register_failed_attempt = Q("""
UPDATE account_delete_validation_code
SET failed_attempts = failed_attempts + 1
WHERE account = $account_internal_id
""")

_q_delete_account = Q("""
WITH removed_validation_code AS (
    DELETE FROM account_delete_validation_code
    WHERE account = $account_internal_id
    RETURNING validation_code
),

updated_account AS (
    UPDATE account SET deleted_on = $now
    WHERE _id = $account_internal_id
    RETURNING TRUE
),

disabled_auth_methods AS (
    UPDATE vault_authentication_method
    SET disabled_on = $now
    WHERE
        vault IN (
            SELECT _id FROM vault
            WHERE account = $account_internal_id
        )
        AND disabled_on IS NULL
    RETURNING _id
)

SELECT
    (SELECT validation_code FROM removed_validation_code) AS expected_validation_code,
    (SELECT COUNT(*) FROM updated_account) AS account_deleted_count,
    (SELECT COUNT(*) FROM disabled_auth_methods) AS disabled_auth_methods_count
""")


async def delete_proceed(
    pool: AsyncpgPool,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
    validation_code: ValidationCode,
) -> None | AccountDeleteProceedBadOutcome:
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
            should_commit, result = await _delete_proceed(
                conn,
                now,
                auth_method_id,
                validation_code,
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


async def _delete_proceed(
    conn: AsyncpgConnection,
    now: DateTime,
    auth_method_id: AccountAuthMethodID,
    validation_code: ValidationCode,
) -> tuple[bool, None | AccountDeleteProceedBadOutcome]:
    # 1) Find the account associated with this auth method

    row = await conn.fetchrow(
        *_q_get_account_and_validation_code_from_auth_method(auth_method_id=auth_method_id)
    )
    assert row is not None

    match row["account_internal_id"]:
        case None:
            return _rollback(AccountDeleteProceedBadOutcome.ACCOUNT_NOT_FOUND)
        case int() as account_internal_id:
            pass
        case _:
            assert False, row

    # 2) Check the validation code

    match row["expected_validation_code"]:
        case None:
            # No validation code found for this email in the database
            return _rollback(AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED)
        case str() as raw_expected_validation_code:
            expected_validation_code = ValidationCode(raw_expected_validation_code)
        case _:
            assert False, row

    match row["created_at"]:
        case DateTime() as created_at:
            pass
        case _:
            assert False, row

    match row["failed_attempts"]:
        case int() as failed_attempts:
            assert failed_attempts >= 0, failed_attempts
        case _:
            assert False, row

    last_mail_info = ValidationCodeInfo(
        validation_code=expected_validation_code,
        created_at=created_at,
        failed_attempts=failed_attempts,
    )

    if not last_mail_info.can_be_used(now):
        # The validation code in the database is too old, consider it doesn't exist
        return _rollback(AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED)

    # The validation code in the database is valid...
    if last_mail_info.validation_code != validation_code:
        # ...but doesn't match the one provided by the client, so we register a failed attempt
        await conn.execute(*_q_register_failed_attempt(account_internal_id=account_internal_id))
        return _commit(AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE)

    # 3) Validation code is valid, proceed with account deletion

    row = await conn.fetchrow(
        *_q_delete_account(
            now=now,
            account_internal_id=account_internal_id,
        )
    )
    assert row is not None

    # Sanity check to ensure the transaction was held between validation code check and actual account deletion
    assert row["expected_validation_code"] == validation_code.str
    assert row["account_deleted_count"] == 1, row
    # At least, the auth method used to do this operation should have been disabled!
    assert row["disabled_auth_methods_count"] >= 1, row

    # All done! Ask for the transaction to be committed
    return _commit()
