# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal, override

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    HashDigest,
    HumanHandle,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SecretKey,
    UntrustedPasswordAlgorithm,
    ValidationCode,
)
from parsec.components.account import (
    AccountAuthMethodCreateBadOutcome,
    AccountAuthMethodDisableBadOutcome,
    AccountAuthMethodListBadOutcome,
    AccountCreateProceedBadOutcome,
    AccountCreateSendValidationEmailBadOutcome,
    AccountDeleteProceedBadOutcome,
    AccountDeleteSendValidationEmailBadOutcome,
    AccountInfo,
    AccountInfoBadOutcome,
    AccountInviteListBadOutcome,
    AccountOrganizationListBadOutcome,
    AccountOrganizationSelfList,
    AccountRecoverProceedBadOutcome,
    AccountRecoverSendValidationEmailBadOutcome,
    AccountVaultItemListBadOutcome,
    AccountVaultItemRecoveryList,
    AccountVaultItemUploadBadOutcome,
    AccountVaultKeyRotation,
    AuthMethod,
    BaseAccountComponent,
    VaultItemRecoveryList,
    VaultItems,
)
from parsec.components.email import SendEmailBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.account_auth_method_create import auth_method_create
from parsec.components.postgresql.account_auth_method_disable import auth_method_disable
from parsec.components.postgresql.account_auth_method_list import auth_method_list
from parsec.components.postgresql.account_create import (
    create_check_validation_code,
    create_proceed,
    create_send_validation_email,
)
from parsec.components.postgresql.account_delete import delete_proceed, delete_send_validation_email
from parsec.components.postgresql.account_info import account_info
from parsec.components.postgresql.account_invite_self_list import invite_self_list
from parsec.components.postgresql.account_organization_self_list import organization_self_list
from parsec.components.postgresql.account_password_algorithm import (
    get_password_algorithm,
)
from parsec.components.postgresql.account_recover import (
    recover_proceed,
    recover_send_validation_email,
)
from parsec.components.postgresql.account_vault_item_list import vault_item_list
from parsec.components.postgresql.account_vault_item_recovery_list import vault_item_recovery_list
from parsec.components.postgresql.account_vault_item_upload import vault_item_upload
from parsec.components.postgresql.account_vault_key_rotation import vault_key_rotation
from parsec.components.postgresql.utils import no_transaction, transaction
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

    @override
    async def recover_send_validation_email(
        self, now: DateTime, email: EmailAddress
    ) -> ValidationCode | SendEmailBadOutcome | AccountRecoverSendValidationEmailBadOutcome:
        outcome = await self._recover_send_validation_email_db_operations(now, email)
        match outcome:
            case ValidationCode() as validation_code:
                pass
            case err:
                return err

        # Note we send the email once the PostgreSQL transaction is done, since this
        # operation can be long.
        outcome = await self._send_account_recover_validation_email(
            email=email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @transaction
    async def _recover_send_validation_email_db_operations(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        email: EmailAddress,
    ) -> ValidationCode | AccountRecoverSendValidationEmailBadOutcome:
        return await recover_send_validation_email(conn, now, email)

    @override
    # Note this operation doesn't use the regular `@transaction` decorator.
    # This is because this decorator would rollback the transaction on error,
    # while here we want to register the failed attempt in the database.
    async def recover_proceed(
        self,
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
        return await recover_proceed(
            self.pool,
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

    @override
    @no_transaction
    async def get_password_algorithm_or_fake_it(
        self, conn: AsyncpgConnection, email: EmailAddress
    ) -> UntrustedPasswordAlgorithm:
        match await get_password_algorithm(conn, email):
            case None:
                # Account does not exists or it does not have an active password algorithm.
                # In either case, a fake configuration is returned.
                return self._generate_fake_password_algorithm(email)

            case algorithm:
                return algorithm

    @override
    @no_transaction
    async def account_info(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> AccountInfo | AccountInfoBadOutcome:
        return await account_info(conn, auth_method_id)

    @override
    @transaction
    async def vault_item_upload(
        self,
        conn: AsyncpgConnection,
        auth_method_id: AccountAuthMethodID,
        item_fingerprint: HashDigest,
        item: bytes,
    ) -> None | AccountVaultItemUploadBadOutcome:
        return await vault_item_upload(conn, auth_method_id, item_fingerprint, item)

    @override
    @no_transaction
    async def vault_item_list(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> VaultItems | AccountVaultItemListBadOutcome:
        return await vault_item_list(conn, auth_method_id)

    @override
    @transaction
    async def vault_key_rotation(
        self,
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
        return await vault_key_rotation(
            conn,
            now,
            auth_method_id,
            created_by_ip,
            created_by_user_agent,
            new_auth_method_id,
            new_auth_method_mac_key,
            new_auth_method_password_algorithm,
            new_vault_key_access,
            items,
        )

    @override
    @no_transaction
    async def vault_item_recovery_list(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> VaultItemRecoveryList | AccountVaultItemRecoveryList:
        return await vault_item_recovery_list(conn, auth_method_id)

    @override
    @no_transaction
    async def invite_self_list(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> list[tuple[OrganizationID, InvitationToken, InvitationType]] | AccountInviteListBadOutcome:
        return await invite_self_list(conn, auth_method_id)

    @override
    @no_transaction
    async def organization_self_list(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> AccountOrganizationSelfList | AccountOrganizationListBadOutcome:
        return await organization_self_list(conn, auth_method_id)

    @override
    @transaction
    async def auth_method_create(
        self,
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
        return await auth_method_create(
            conn,
            now,
            auth_method_id,
            created_by_user_agent,
            created_by_ip,
            new_auth_method_id,
            new_auth_method_mac_key,
            new_auth_method_password_algorithm,
            new_vault_key_access,
        )

    @override
    @no_transaction
    async def auth_method_list(
        self, conn: AsyncpgConnection, auth_method_id: AccountAuthMethodID
    ) -> list[AuthMethod] | AccountAuthMethodListBadOutcome:
        return await auth_method_list(conn, auth_method_id)

    @override
    @transaction
    async def auth_method_disable(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        to_disable_auth_method_id: AccountAuthMethodID,
    ) -> None | AccountAuthMethodDisableBadOutcome:
        return await auth_method_disable(conn, now, auth_method_id, to_disable_auth_method_id)
