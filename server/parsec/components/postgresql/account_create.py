# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    EmailAddress,
    ValidationCode,
)
from parsec.components.account import (
    AccountCreateProceedBadOutcome,
    AccountCreateSendValidationEmailBadOutcome,
    ValidationCodeInfo,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import Q


async def q_take_account_create_write_lock(conn: AsyncpgConnection) -> None:
    # TODO: documentation
    # TODO: use SHARE ROW EXCLUSIVE instead ? (cf. https://www.postgresql.org/docs/current/sql-lock.html)
    """
    Only a single active account for a given email is allowed.

    However we cannot enforce this purely in PostgreSQL (e.g. with a unique index)
    since removing an account is not done by deleting its row but by setting
    its `deleted_on` column.

    So the easy way to solve this is to get rid of the concurrency altogether
    (considering invitation creation is far from being performance intensive !)
    by requesting a per-organization PostgreSQL Advisory Lock to be held before
    the invitation creation procedure starts any checks involving the invitations.
    """
    await conn.execute("LOCK TABLE account IN SHARE MODE")


_q_check_account_exists = Q("""
SELECT deleted_on FROM account WHERE email = $email LIMIT 1 FOR UPDATE
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
    email = EXCLUDED.email,
    validation_code = EXCLUDED.validation_code,
    created_at = EXCLUDED.created_at,
    failed_attempts = 0
""")

_q_get_validation_code = Q("""
SELECT
    validation_code as expected_validation_code,
    created_at,
    failed_attempts
FROM account_create_validation_code WHERE email = $email LIMIT 1
FOR UPDATE
""")

_q_register_failed_attempt = Q("""
UPDATE account_create_validation_code SET failed_attempts = failed_attempts + 1 WHERE email = $email
""")


async def create_send_validation_email(
    conn: AsyncpgConnection,
    now: DateTime,
    email: EmailAddress,
) -> ValidationCode | AccountCreateSendValidationEmailBadOutcome:
    await q_take_account_create_write_lock(conn)

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
    async with pool.acquire() as conn:
        row = await conn.fetchrow(*_q_get_validation_code(email=email.str))
        if not row:
            # No validation code found for this email in the database
            return AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

        match row["expected_validation_code"]:
            case str() as raw_expected_validation_code:
                expected_validation_code = ValidationCode(raw_expected_validation_code)
            case unknown:
                assert False, unknown

        match row["created_at"]:
            case DateTime() as created_at:
                pass
            case unknown:
                assert False, unknown

        match row["failed_attempts"]:
            case int() as failed_attempts:
                assert failed_attempts >= 0, failed_attempts
            case unknown:
                assert False, unknown

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
