# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import Literal, cast

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    HumanHandle,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
    ValidationCode,
)
from parsec.components.account import (
    AccountCreateProceedBadOutcome,
    AccountCreateSendValidationEmailBadOutcome,
    ValidationCodeInfo,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import Q


async def q_take_account_create_write_lock(
    conn: AsyncpgConnection, mode: Literal["write", "read"]
) -> None:
    """
    The tables `account` and `account_create_validation_code` work together to implement
    the account creation:

    - Only a single active account for a given email is allowed.
    - A single validation code is allowed for a given email, provided that
      it doesn't correspond to an already existing account.

    We cannot enforce this purely in PostgreSQL (e.g. with a unique index) since
    two tables are involved, and also removing an account is not done by deleting
    its row but by setting its `deleted_on` column.

    So instead we lock the `account` table for any operation involving account creation
    (i.e. creating a new validation code and creating a new account).

    Note:
    - We do not need to take a lock for solely checking the validation code (since in
      this case the `account` table is not accessed at all).
    - Read and write locks are both compatible with concurrent reads, but the write
      version is needed to prevent deadlocks that would occur if two concurrent operations
      take a read lock on the table, then want to take a write lock on the table
      (see https://www.postgresql.org/docs/current/sql-lock.html)
    """
    match mode:
        case "write":
            # Lock the table so that nobody can update it (but concurrent reads are allowed)
            query = "LOCK TABLE account IN SHARE MODE"
        case "read":
            # Lock the table so that only us can update it (but concurrent reads are allowed)
            query = "LOCK TABLE account IN SHARE ROW EXCLUSIVE MODE"

    await conn.execute(query)


_q_check_account_exists = Q("""
SELECT deleted_on
FROM account
WHERE email = $email
LIMIT 1
FOR UPDATE
""")

_q_insert_validation_code = Q("""
INSERT INTO account_create_validation_code (
    email,
    validation_code,
    created_at
) VALUES (
    $email,
    $validation_code,
    $created_at
)
-- Note we simply overwrite any existing previous validation code
ON CONFLICT (email) DO UPDATE
    SET
        email = excluded.email,
        validation_code = excluded.validation_code,
        created_at = excluded.created_at,
        failed_attempts = 0
""")


async def create_send_validation_email(
    conn: AsyncpgConnection,
    now: DateTime,
    email: EmailAddress,
) -> ValidationCode | AccountCreateSendValidationEmailBadOutcome:
    # Note the `read` lock mode here since we just want to make sure the `account`
    # won't be modified (i.e. a new account with our `email` gets inserted) while
    # we insert into the `account_create_validation_code` table.
    await q_take_account_create_write_lock(conn, mode="read")

    row = await conn.fetchrow(*_q_check_account_exists(email=email.str))
    if row:
        if row["deleted_on"] is not None:
            return AccountCreateSendValidationEmailBadOutcome.ACCOUNT_ALREADY_EXISTS

    validation_code = ValidationCode.generate()
    await conn.execute(
        *_q_insert_validation_code(
            email=email.str, validation_code=validation_code.str, created_at=now
        )
    )

    return validation_code


async def create_check_validation_code(
    pool: AsyncpgPool,
    now: DateTime,
    email: EmailAddress,
    validation_code: ValidationCode,
) -> None | AccountCreateProceedBadOutcome:
    # Acquire a connection from the pool by hand is needed here since we don't
    # want a rollback-on-error behavior (which is what the `@transaction`
    # decorator does).
    async with pool.acquire() as conn, conn.transaction():
        conn = cast(AsyncpgConnection, conn)
        return await _create_check_validation_code(conn, now, email, validation_code)


_q_get_validation_code = Q("""
SELECT
    validation_code AS expected_validation_code,
    created_at,
    failed_attempts
FROM account_create_validation_code
WHERE email = $email LIMIT 1
-- Lock for update since this row should be update on failed attempt
FOR UPDATE
""")

_q_register_failed_attempt = Q("""
UPDATE account_create_validation_code
SET failed_attempts = failed_attempts + 1
WHERE email = $email
""")

_q_insert_account = Q("""
WITH removed_validation_code AS (
    DELETE FROM account_create_validation_code
    WHERE email = $email
    RETURNING validation_code
),

new_account AS (
    INSERT INTO account (
        email,
        human_handle_label
    )
    VALUES (
        $email,
        $human_handle_label
    )
    ON CONFLICT (email) DO
    UPDATE
        SET
            human_handle_label = excluded.human_handle_label,
            deleted_on = NULL
        -- Re-enable the account if it has been previously deleted
        WHERE account.deleted_on IS NOT NULL
    RETURNING _id
),

new_vault AS (
    INSERT INTO vault (
        account
    )
    VALUES (
        -- Note this code will crash if the account already exists and is not deleted
        -- (since in this case the value here will be `NULL`).
        -- This is fine though since this only occurs if the database is in an inconsistent
        -- state (as a non-deleted account should never have a corresponding entry in
        -- `account_create_validation_code` in the first place).
        (SELECT _id FROM new_account)
    )
    RETURNING _id
),

new_vault_authentication_method AS (
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
        (SELECT _id FROM new_vault),
        $now,
        $created_by_ip,
        $created_by_user_agent,
        $auth_method_mac_key,
        $vault_key_access,
        $auth_method_password_algorithm,
        $auth_method_password_algorithm_argon2id_opslimit,
        $auth_method_password_algorithm_argon2id_memlimit_kb,
        $auth_method_password_algorithm_argon2id_parallelism
    )
    ON CONFLICT (auth_method_id) DO NOTHING
    RETURNING _id
)

SELECT
    (SELECT validation_code FROM removed_validation_code) AS expected_validation_code,
    (SELECT _id FROM new_account) AS account_internal_id,
    (SELECT _id FROM new_vault_authentication_method) AS authentication_method_internal_id
""")


async def _create_check_validation_code(
    conn: AsyncpgConnection,
    now: DateTime,
    email: EmailAddress,
    validation_code: ValidationCode,
) -> None | AccountCreateProceedBadOutcome:
    row = await conn.fetchrow(*_q_get_validation_code(email=email.str))
    if not row:
        # No validation code found for this email in the database
        return AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

    match row["expected_validation_code"]:
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
        return AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

    # The validation code in the database is valid...
    if last_mail_info.validation_code != validation_code:
        # ...but doesn't match the one provided by the client, so we register a failed attempt
        await conn.execute(*_q_register_failed_attempt(email=email.str))
        return AccountCreateProceedBadOutcome.INVALID_VALIDATION_CODE

    return None


async def create_proceed(
    pool: AsyncpgPool,
    now: DateTime,
    validation_code: ValidationCode,
    vault_key_access: bytes,
    human_handle: HumanHandle,
    created_by_user_agent: str,
    created_by_ip: str | Literal[""],
    auth_method_id: AccountAuthMethodID,
    auth_method_mac_key: SecretKey,
    auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
) -> None | AccountCreateProceedBadOutcome:
    # Acquire a connection from the pool by hand is needed here since we don't
    # want a rollback-on-error behavior (which is what the `@transaction`
    # decorator does).
    # This is to handle invalid validation code, since in this case an error is
    # returned but we still has to update the database to register the failed attempt.
    async with pool.acquire() as conn:
        conn = cast(AsyncpgConnection, conn)
        transaction = conn.transaction()
        await transaction.start()
        try:
            should_commit, result = await _create_proceed(
                conn,
                now,
                validation_code,
                vault_key_access,
                human_handle,
                created_by_user_agent,
                created_by_ip,
                auth_method_id,
                auth_method_mac_key,
                auth_method_password_algorithm,
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


async def _create_proceed(
    conn: AsyncpgConnection,
    now: DateTime,
    validation_code: ValidationCode,
    vault_key_access: bytes,
    human_handle: HumanHandle,
    created_by_user_agent: str,
    created_by_ip: str | Literal[""],
    auth_method_id: AccountAuthMethodID,
    auth_method_mac_key: SecretKey,
    auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
) -> tuple[bool, None | AccountCreateProceedBadOutcome]:
    # 1) Lock the `account` database so we are the only one allowed to modify it

    await q_take_account_create_write_lock(conn, mode="write")

    # 2) Check the validation code

    outcome = await _create_check_validation_code(conn, now, human_handle.email, validation_code)
    match outcome:
        case None:
            pass
        case AccountCreateProceedBadOutcome() as err:
            # Return an error, but we still want to commit the transaction in order
            # to register the failed attempt in the database.
            return _commit(err)

    # 3) Validation code is valid, proceed with account creation

    match auth_method_password_algorithm:
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

    row = await conn.fetchrow(
        *_q_insert_account(
            now=now,
            vault_key_access=vault_key_access,
            email=human_handle.email.str,
            human_handle_label=human_handle.label,
            created_by_user_agent=created_by_user_agent,
            created_by_ip=created_by_ip,
            auth_method_id=auth_method_id,
            auth_method_mac_key=auth_method_mac_key.secret,
            auth_method_password_algorithm=password_algorithm,
            auth_method_password_algorithm_argon2id_opslimit=password_algorithm_argon2id_opslimit,
            auth_method_password_algorithm_argon2id_memlimit_kb=password_algorithm_argon2id_memlimit_kb,
            auth_method_password_algorithm_argon2id_parallelism=password_algorithm_argon2id_parallelism,
        )
    )
    assert row is not None

    # Sanity check to ensure the transaction was held between validation code check and actual account creation
    assert row["expected_validation_code"] == validation_code.str

    match row["authentication_method_internal_id"]:
        case int():
            pass
        case None:
            # `auth_method_id` already exists in the database, return an error
            # and rollback the transaction.
            return _rollback(AccountCreateProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS)
        case _:
            assert False, row

    # All done! Ask for the transaction to be committed
    return _commit()
