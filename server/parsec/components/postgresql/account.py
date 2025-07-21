# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal, override

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    HumanHandle,
    SecretKey,
    UntrustedPasswordAlgorithm,
    ValidationCode,
)
from parsec.components.account import (
    AccountCreateProceedBadOutcome,
    AccountCreateSendValidationEmailBadOutcome,
    AccountDeleteProceedBadOutcome,
    AccountDeleteSendValidationEmailBadOutcome,
    BaseAccountComponent,
)
from parsec.components.email import SendEmailBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.account_create import (
    create_check_validation_code,
    create_proceed,
    create_send_validation_email,
)
from parsec.components.postgresql.account_delete import delete_proceed, delete_send_validation_email
from parsec.components.postgresql.utils import transaction
from parsec.config import BackendConfig


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config=config)
        self.pool = pool

    @override
    async def create_send_validation_email(
        self,
        now: DateTime,
        email: EmailAddress,
    ) -> ValidationCode | SendEmailBadOutcome | AccountCreateSendValidationEmailBadOutcome:
        outcome = await self._create_send_validation_email_db_operations(now, email)
        match outcome:
            case ValidationCode() as validation_code:
                pass
            case err:
                return err

        # Note we send the email once the PostgreSQL transaction is done, since this
        # operation can be long.
        outcome = await self._send_account_create_validation_email(
            email=email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @transaction
    async def _create_send_validation_email_db_operations(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        email: EmailAddress,
    ) -> ValidationCode | SendEmailBadOutcome | AccountCreateSendValidationEmailBadOutcome:
        return await create_send_validation_email(conn, now, email)

    @override
    # Note this operation doesn't use the regular `@transaction` decorator.
    # This is because this decorator would rollback the transaction on error,
    # while here we want to register the failed attempt in the database.
    async def create_check_validation_code(
        self,
        now: DateTime,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | AccountCreateProceedBadOutcome:
        return await create_check_validation_code(self.pool, now, email, validation_code)

    @override
    # Note this operation doesn't use the regular `@transaction` decorator.
    # This is because this decorator would rollback the transaction on error,
    # while here we want to register the failed attempt in the database.
    async def create_proceed(
        self,
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
        return await create_proceed(
            self.pool,
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

    @override
    async def delete_send_validation_email(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
    ) -> ValidationCode | SendEmailBadOutcome | AccountDeleteSendValidationEmailBadOutcome:
        outcome = await self._delete_send_validation_email_db_operations(now, auth_method_id)
        match outcome:
            case (validation_code, account_email):
                pass
            case err:
                return err

        # Note we send the email once the PostgreSQL transaction is done, since this
        # operation can be long.
        outcome = await self._send_account_delete_validation_email(
            email=account_email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @transaction
    async def _delete_send_validation_email_db_operations(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
    ) -> tuple[ValidationCode, EmailAddress] | AccountDeleteSendValidationEmailBadOutcome:
        return await delete_send_validation_email(conn, now, auth_method_id)

    @override
    # Note this operation doesn't use the regular `@transaction` decorator.
    # This is because this decorator would rollback the transaction on error,
    # while here we want to register the failed attempt in the database.
    async def delete_proceed(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        validation_code: ValidationCode,
    ) -> None | AccountDeleteProceedBadOutcome:
        return await delete_proceed(self.pool, now, auth_method_id, validation_code)
