# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto
from typing import Literal

from jinja2 import Environment

from parsec._parsec import (
    AccountAuthMethodID,
    ActiveUsersLimit,
    DateTime,
    EmailAddress,
    HashDigest,
    HumanHandle,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
    UserID,
    UserProfile,
    ValidationCode,
    anonymous_account_cmds,
    authenticated_account_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext, AuthenticatedAccountClientContext
from parsec.components.email import SendEmailBadOutcome, send_email
from parsec.config import AccountVaultStrategy, AllowedClientAgent, BackendConfig
from parsec.types import BadOutcomeEnum


class AccountInfoBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountCreateSendValidationEmailBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()


class AccountCreateProceedBadOutcome(BadOutcomeEnum):
    INVALID_VALIDATION_CODE = auto()
    SEND_VALIDATION_EMAIL_REQUIRED = auto()
    AUTH_METHOD_ID_ALREADY_EXISTS = auto()


class AccountDeleteSendValidationEmailBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountDeleteProceedBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    INVALID_VALIDATION_CODE = auto()
    SEND_VALIDATION_EMAIL_REQUIRED = auto()


class AccountRecoverSendValidationEmailBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountRecoverProceedBadOutcome(BadOutcomeEnum):
    # Note there is no `ACCOUNT_NOT_FOUND` here (since only a valid account
    # can have a validation code in the first place!).
    INVALID_VALIDATION_CODE = auto()
    SEND_VALIDATION_EMAIL_REQUIRED = auto()
    AUTH_METHOD_ID_ALREADY_EXISTS = auto()


class AccountGetPasswordSecretKeyBadOutcome(BadOutcomeEnum):
    USER_NOT_FOUND = auto()
    UNABLE_TO_GET_SECRET_KEY = auto()


class AccountVaultItemUploadBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    FINGERPRINT_ALREADY_EXISTS = auto()


class AccountVaultItemListBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountVaultKeyRotation(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    ITEMS_MISMATCH = auto()
    NEW_AUTH_METHOD_ID_ALREADY_EXISTS = auto()


class AccountVaultItemRecoveryList(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountInviteListBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountOrganizationListBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountAuthMethodCreateBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    AUTH_METHOD_ID_ALREADY_EXISTS = auto()


class AccountAuthMethodListBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()


class AccountAuthMethodDisableBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    AUTH_METHOD_NOT_FOUND = auto()
    AUTH_METHOD_ALREADY_DISABLED = auto()
    SELF_DISABLE_NOT_ALLOWED = auto()
    CROSS_ACCOUNT_NOT_ALLOWED = auto()


@dataclass(slots=True)
class AccountInfo:
    human_handle: HumanHandle


@dataclass(slots=True)
class AuthMethod:
    auth_method_id: AccountAuthMethodID
    created_on: DateTime
    # IP address or empty string if not available
    created_by_ip: str | Literal[""]
    created_by_user_agent: str
    vault_key_access: bytes
    password_algorithm: UntrustedPasswordAlgorithm | None
    disabled_on: DateTime | None


@dataclass(slots=True)
class VaultItemRecoveryVault:
    auth_methods: list[AuthMethod]
    vault_items: dict[HashDigest, bytes]


@dataclass(slots=True)
class VaultItemRecoveryList:
    current_vault: VaultItemRecoveryVault
    previous_vaults: list[VaultItemRecoveryVault]


@dataclass(slots=True)
class VaultItems:
    key_access: bytes
    items: dict[HashDigest, bytes]


@dataclass(slots=True)
class AccountOrganizationSelfListActiveUser:
    user_id: UserID
    created_on: DateTime
    current_profile: UserProfile
    is_frozen: bool
    organization_id: OrganizationID
    organization_is_expired: bool
    organization_user_profile_outsider_allowed: bool
    organization_active_users_limit: ActiveUsersLimit
    organization_allowed_client_agent: AllowedClientAgent
    organization_account_vault_strategy: AccountVaultStrategy


@dataclass(slots=True)
class AccountOrganizationSelfListRevokedUser:
    user_id: UserID
    created_on: DateTime
    revoked_on: DateTime
    current_profile: UserProfile
    organization_id: OrganizationID


@dataclass(slots=True)
class AccountOrganizationSelfList:
    active: list[AccountOrganizationSelfListActiveUser]
    revoked: list[AccountOrganizationSelfListRevokedUser]


VALIDATION_CODE_VALIDITY_DURATION_SECONDS: int = 3600
"""
How much seconds a validation code remain usable (valid)
"""

VALIDATION_CODE_MAX_FAILED_ATTEMPTS: int = 3
"""How much an user can "guess" the code"""


@dataclass(slots=True)
class ValidationCodeInfo:
    validation_code: ValidationCode
    created_at: DateTime
    failed_attempts: int = 0

    def _is_expired(self, now: DateTime) -> bool:
        """Check that the validation code is not expired"""
        return (now - self.created_at) > VALIDATION_CODE_VALIDITY_DURATION_SECONDS

    def _has_remaining_attempts(self) -> bool:
        """Check if we are below the max attempts for the code"""
        return self.failed_attempts < VALIDATION_CODE_MAX_FAILED_ATTEMPTS

    def can_be_used(self, now: DateTime) -> bool:
        return not self._is_expired(now) and self._has_remaining_attempts()


class BaseAccountComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    # Used by `create_proceed` implementations
    async def _send_account_create_validation_email(
        self,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        message = _generate_account_create_validation_email(
            jinja_env=self._config.jinja_env,
            from_addr=self._config.email_config.sender,
            to_addr=email,
            validation_code=validation_code,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config,
            to_addr=email,
            message=message,
        )

    # Used by `delete_proceed` implementations
    async def _send_account_delete_validation_email(
        self,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        message = _generate_account_delete_validation_email(
            jinja_env=self._config.jinja_env,
            from_addr=self._config.email_config.sender,
            to_addr=email,
            validation_code=validation_code,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config, to_addr=email, message=message
        )

    # Used by `recover_proceed` implementations
    async def _send_account_recover_validation_email(
        self,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        message = _generate_account_recover_validation_email(
            jinja_env=self._config.jinja_env,
            from_addr=self._config.email_config.sender,
            to_addr=email,
            validation_code=validation_code,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config, to_addr=email, message=message
        )

    async def get_password_algorithm_or_fake_it(
        self, email: EmailAddress
    ) -> UntrustedPasswordAlgorithm:
        """
        Return the password algorithm configuration corresponding to the account
        of the given email address.

        If the email address doesn't correspond to a valid account, a fake (stable)
        configuration is returned in order to prevent an attacker guessing whether
        the email address is present in the database or not.
        """
        raise NotImplementedError

    # Used by `get_password_algorithm_or_fake_it` implementations
    def _generate_fake_password_algorithm(self, email: EmailAddress) -> UntrustedPasswordAlgorithm:
        return UntrustedPasswordAlgorithm.generate_fake_from_seed(
            email.str, self._config.fake_account_password_algorithm_seed
        )

    async def account_info(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> AccountInfo | AccountInfoBadOutcome:
        raise NotImplementedError

    async def create_send_validation_email(
        self, now: DateTime, email: EmailAddress
    ) -> ValidationCode | SendEmailBadOutcome | AccountCreateSendValidationEmailBadOutcome:
        raise NotImplementedError

    async def create_check_validation_code(
        self,
        now: DateTime,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | AccountCreateProceedBadOutcome:
        raise NotImplementedError

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
        raise NotImplementedError

    async def delete_send_validation_email(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
    ) -> ValidationCode | SendEmailBadOutcome | AccountDeleteSendValidationEmailBadOutcome:
        raise NotImplementedError

    async def delete_proceed(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        validation_code: ValidationCode,
    ) -> None | AccountDeleteProceedBadOutcome:
        raise NotImplementedError

    async def recover_send_validation_email(
        self, now: DateTime, email: EmailAddress
    ) -> ValidationCode | SendEmailBadOutcome | AccountRecoverSendValidationEmailBadOutcome:
        raise NotImplementedError

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
        raise NotImplementedError

    async def vault_item_upload(
        self, auth_method_id: AccountAuthMethodID, item_fingerprint: HashDigest, item: bytes
    ) -> None | AccountVaultItemUploadBadOutcome:
        raise NotImplementedError

    async def vault_item_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> VaultItems | AccountVaultItemListBadOutcome:
        raise NotImplementedError

    async def vault_key_rotation(
        self,
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
        raise NotImplementedError

    async def vault_item_recovery_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> VaultItemRecoveryList | AccountVaultItemRecoveryList:
        raise NotImplementedError

    async def invite_self_list(
        self, auth_method_id: AccountAuthMethodID
    ) -> list[tuple[OrganizationID, InvitationToken, InvitationType]] | AccountInviteListBadOutcome:
        raise NotImplementedError

    async def organization_self_list(
        self, auth_method_id: AccountAuthMethodID
    ) -> AccountOrganizationSelfList | AccountOrganizationListBadOutcome:
        raise NotImplementedError

    async def auth_method_create(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
        new_vault_key_access: bytes,
    ) -> None | AccountAuthMethodCreateBadOutcome:
        raise NotImplementedError

    async def auth_method_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> list[AuthMethod] | AccountAuthMethodListBadOutcome:
        raise NotImplementedError

    async def auth_method_disable(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        to_disable_auth_method_id: AccountAuthMethodID,
    ) -> None | AccountAuthMethodDisableBadOutcome:
        raise NotImplementedError

    @api
    async def api_auth_method_password_get_algorithm(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.auth_method_password_get_algorithm.Req,
    ) -> anonymous_account_cmds.latest.auth_method_password_get_algorithm.Rep:
        password_algorithm = await self.get_password_algorithm_or_fake_it(req.email)
        return anonymous_account_cmds.latest.auth_method_password_get_algorithm.RepOk(
            password_algorithm=password_algorithm
        )

    @api
    async def api_account_create_send_validation_email(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_send_validation_email.Req,
    ) -> anonymous_account_cmds.latest.account_create_send_validation_email.Rep:
        now = DateTime.now()

        match self._config.email_rate_limit.register_send_intent(
            now=now,
            client_ip_address=client_ctx.client_ip_address,
            recipient=req.email,
        ):
            case None:
                pass
            case DateTime() as wait_until:
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailSendingRateLimited(
                    wait_until=wait_until,
                )

        outcome = await self.create_send_validation_email(
            now=now,
            email=req.email,
        )
        match outcome:
            case ValidationCode():
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

            case AccountCreateSendValidationEmailBadOutcome.ACCOUNT_ALREADY_EXISTS:
                # Respond OK without sending token to prevent creating oracle
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

            case SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE:
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailServerUnavailable()

            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailRecipientRefused()

    @api
    async def api_account_create_proceed(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_proceed.Req,
    ) -> anonymous_account_cmds.latest.account_create_proceed.Rep:
        match req.account_create_step:
            case (
                anonymous_account_cmds.latest.account_create_proceed.AccountCreateStepNumber0CheckCode() as step
            ):
                outcome = await self.create_check_validation_code(
                    now=DateTime.now(),
                    email=step.email,
                    validation_code=step.validation_code,
                )

            case (
                anonymous_account_cmds.latest.account_create_proceed.AccountCreateStepNumber1Create() as step
            ):
                outcome = await self.create_proceed(
                    now=DateTime.now(),
                    validation_code=step.validation_code,
                    vault_key_access=step.vault_key_access,
                    human_handle=step.human_handle,
                    created_by_user_agent=client_ctx.client_user_agent,
                    created_by_ip=client_ctx.client_ip_address,
                    auth_method_id=step.auth_method_id,
                    auth_method_mac_key=step.auth_method_mac_key,
                    auth_method_password_algorithm=step.auth_method_password_algorithm,
                )

            case unknown:
                assert False, f"Unknown step {unknown}"

        match outcome:
            case None:
                return anonymous_account_cmds.latest.account_create_proceed.RepOk()

            case AccountCreateProceedBadOutcome.INVALID_VALIDATION_CODE:
                return (
                    anonymous_account_cmds.latest.account_create_proceed.RepInvalidValidationCode()
                )

            case AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                return anonymous_account_cmds.latest.account_create_proceed.RepSendValidationEmailRequired()

            case AccountCreateProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS:
                return anonymous_account_cmds.latest.account_create_proceed.RepAuthMethodIdAlreadyExists()

    @api
    async def api_account_delete_send_validation_email(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.account_delete_send_validation_email.Req,
    ) -> authenticated_account_cmds.latest.account_delete_send_validation_email.Rep:
        now = DateTime.now()

        match self._config.email_rate_limit.register_send_intent(
            now=now,
            client_ip_address=client_ctx.client_ip_address,
            recipient=client_ctx.account_email,
        ):
            case None:
                pass
            case DateTime() as wait_until:
                return authenticated_account_cmds.latest.account_delete_send_validation_email.RepEmailSendingRateLimited(
                    wait_until=wait_until,
                )

        outcome = await self.delete_send_validation_email(now, client_ctx.auth_method_id)
        match outcome:
            case ValidationCode():
                return (
                    authenticated_account_cmds.latest.account_delete_send_validation_email.RepOk()
                )

            case SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE:
                return authenticated_account_cmds.latest.account_delete_send_validation_email.RepEmailServerUnavailable()

            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                return authenticated_account_cmds.latest.account_delete_send_validation_email.RepEmailRecipientRefused()

            case AccountDeleteSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_delete_proceed(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.account_delete_proceed.Req,
    ) -> authenticated_account_cmds.latest.account_delete_proceed.Rep:
        outcome = await self.delete_proceed(
            DateTime.now(), client_ctx.auth_method_id, req.validation_code
        )
        match outcome:
            case None:
                return authenticated_account_cmds.latest.account_delete_proceed.RepOk()
            case AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE:
                return authenticated_account_cmds.latest.account_delete_proceed.RepInvalidValidationCode()
            case AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                return authenticated_account_cmds.latest.account_delete_proceed.RepSendValidationEmailRequired()
            case AccountDeleteProceedBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_recover_send_validation_email(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_recover_send_validation_email.Req,
    ) -> anonymous_account_cmds.latest.account_recover_send_validation_email.Rep:
        now = DateTime.now()

        match self._config.email_rate_limit.register_send_intent(
            now=now,
            client_ip_address=client_ctx.client_ip_address,
            recipient=req.email,
        ):
            case None:
                pass
            case DateTime() as wait_until:
                return anonymous_account_cmds.latest.account_recover_send_validation_email.RepEmailSendingRateLimited(
                    wait_until=wait_until,
                )

        outcome = await self.recover_send_validation_email(now, req.email)
        match outcome:
            case ValidationCode():
                return anonymous_account_cmds.latest.account_recover_send_validation_email.RepOk()

            case AccountRecoverSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND:
                # Respond OK without sending token to prevent creating oracle
                return anonymous_account_cmds.latest.account_recover_send_validation_email.RepOk()

            case SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE:
                return anonymous_account_cmds.latest.account_recover_send_validation_email.RepEmailServerUnavailable()

            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                return anonymous_account_cmds.latest.account_recover_send_validation_email.RepEmailRecipientRefused()

    @api
    async def api_account_recover_proceed(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_recover_proceed.Req,
    ) -> anonymous_account_cmds.latest.account_recover_proceed.Rep:
        outcome = await self.recover_proceed(
            now=DateTime.now(),
            validation_code=req.validation_code,
            email=req.email,
            created_by_user_agent=client_ctx.client_user_agent,
            created_by_ip=client_ctx.client_ip_address,
            new_vault_key_access=req.new_vault_key_access,
            new_auth_method_id=req.new_auth_method_id,
            new_auth_method_mac_key=req.new_auth_method_mac_key,
            new_auth_method_password_algorithm=req.new_auth_method_password_algorithm,
        )
        match outcome:
            case None:
                return anonymous_account_cmds.latest.account_recover_proceed.RepOk()

            case AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE:
                return (
                    anonymous_account_cmds.latest.account_recover_proceed.RepInvalidValidationCode()
                )

            case AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                return anonymous_account_cmds.latest.account_recover_proceed.RepSendValidationEmailRequired()

            case AccountRecoverProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS:
                return anonymous_account_cmds.latest.account_recover_proceed.RepAuthMethodIdAlreadyExists()

    @api
    async def api_account_vault_item_upload(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.vault_item_upload.Req,
    ) -> authenticated_account_cmds.latest.vault_item_upload.Rep:
        outcome = await self.vault_item_upload(
            auth_method_id=client_ctx.auth_method_id,
            item_fingerprint=req.item_fingerprint,
            item=req.item,
        )
        match outcome:
            case None:
                return authenticated_account_cmds.latest.vault_item_upload.RepOk()
            case AccountVaultItemUploadBadOutcome.FINGERPRINT_ALREADY_EXISTS:
                return authenticated_account_cmds.latest.vault_item_upload.RepFingerprintAlreadyExists()
            case AccountVaultItemUploadBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_vault_item_list(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.vault_item_list.Req,
    ) -> authenticated_account_cmds.latest.vault_item_list.Rep:
        outcome = await self.vault_item_list(auth_method_id=client_ctx.auth_method_id)
        match outcome:
            case VaultItems():
                return authenticated_account_cmds.latest.vault_item_list.RepOk(
                    key_access=outcome.key_access, items=outcome.items
                )
            case AccountVaultItemListBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_vault_key_rotation(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.vault_key_rotation.Req,
    ) -> authenticated_account_cmds.latest.vault_key_rotation.Rep:
        match req.new_auth_method_password_algorithm:
            case None as new_auth_method_password_algorithm:
                pass
            case UntrustedPasswordAlgorithmArgon2id() as raw:
                new_auth_method_password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                    opslimit=raw.opslimit,
                    memlimit_kb=raw.memlimit_kb,
                    parallelism=raw.parallelism,
                )
            # `UntrustedPasswordAlgorithm` is an abstract type
            case UntrustedPasswordAlgorithm() as unknown:
                assert False, unknown

        outcome = await self.vault_key_rotation(
            now=DateTime.now(),
            created_by_ip=client_ctx.client_ip_address,
            created_by_user_agent=client_ctx.client_user_agent,
            auth_method_id=client_ctx.auth_method_id,
            new_auth_method_id=req.new_auth_method_id,
            new_auth_method_mac_key=req.new_auth_method_mac_key,
            new_auth_method_password_algorithm=new_auth_method_password_algorithm,
            new_vault_key_access=req.new_vault_key_access,
            items=req.items,
        )
        match outcome:
            case None:
                return authenticated_account_cmds.latest.vault_key_rotation.RepOk()
            case AccountVaultKeyRotation.ITEMS_MISMATCH:
                return authenticated_account_cmds.latest.vault_key_rotation.RepItemsMismatch()
            case AccountVaultKeyRotation.NEW_AUTH_METHOD_ID_ALREADY_EXISTS:
                return authenticated_account_cmds.latest.vault_key_rotation.RepNewAuthMethodIdAlreadyExists()
            case AccountVaultKeyRotation.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_vault_item_recovery_list(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.vault_item_recovery_list.Req,
    ) -> authenticated_account_cmds.latest.vault_item_recovery_list.Rep:
        outcome = await self.vault_item_recovery_list(
            auth_method_id=client_ctx.auth_method_id,
        )
        match outcome:
            case VaultItemRecoveryList() as vaults:

                def _convert_auth_method(
                    auth_method: AuthMethod,
                ) -> authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod:
                    match auth_method.password_algorithm:
                        case None:
                            password_algorithm = None
                        case UntrustedPasswordAlgorithmArgon2id() as a:
                            password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                                opslimit=a.opslimit,
                                memlimit_kb=a.memlimit_kb,
                                parallelism=a.parallelism,
                            )
                        # `UntrustedPasswordAlgorithm` is an abstract type
                        case UntrustedPasswordAlgorithm() as unknown:
                            assert False, unknown

                    return authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod(
                        auth_method_id=auth_method.auth_method_id,
                        created_on=auth_method.created_on,
                        created_by_ip=auth_method.created_by_ip,
                        created_by_user_agent=auth_method.created_by_user_agent,
                        vault_key_access=auth_method.vault_key_access,
                        password_algorithm=password_algorithm,
                        disabled_on=auth_method.disabled_on,
                    )

                def _convert_vault(
                    vault: VaultItemRecoveryVault,
                ) -> authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault:
                    return authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault(
                        auth_methods=[_convert_auth_method(m) for m in vault.auth_methods],
                        vault_items=vault.vault_items,
                    )

                return authenticated_account_cmds.latest.vault_item_recovery_list.RepOk(
                    current_vault=_convert_vault(vaults.current_vault),
                    previous_vaults=[_convert_vault(v) for v in vaults.previous_vaults],
                )
            case AccountVaultItemRecoveryList.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_account_account_info(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.account_info.Req,
    ) -> authenticated_account_cmds.latest.account_info.Rep:
        outcome = await self.account_info(auth_method_id=client_ctx.auth_method_id)
        match outcome:
            case AccountInfo() as info:
                return authenticated_account_cmds.latest.account_info.RepOk(
                    human_handle=info.human_handle
                )
            case AccountInfoBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_invite_self_list(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.invite_self_list.Req,
    ) -> authenticated_account_cmds.latest.invite_self_list.Rep:
        outcome = await self.invite_self_list(
            client_ctx.auth_method_id,
        )
        match outcome:
            case list() as invitations:
                pass
            case AccountInviteListBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

        return authenticated_account_cmds.latest.invite_self_list.RepOk(invitations=invitations)

    @api
    async def api_organization_self_list(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.organization_self_list.Req,
    ) -> authenticated_account_cmds.latest.organization_self_list.Rep:
        organization_self_list = authenticated_account_cmds.latest.organization_self_list

        outcome = await self.organization_self_list(
            client_ctx.auth_method_id,
        )
        match outcome:
            case AccountOrganizationSelfList() as result:
                pass

            case AccountOrganizationListBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

        cooked_active = []
        for active in result.active:
            match active.organization_allowed_client_agent:
                case AllowedClientAgent.NATIVE_ONLY:
                    cooked_allowed_client_agent = (
                        organization_self_list.AllowedClientAgent.NATIVE_ONLY
                    )
                case AllowedClientAgent.NATIVE_OR_WEB:
                    cooked_allowed_client_agent = (
                        organization_self_list.AllowedClientAgent.NATIVE_OR_WEB
                    )

            match active.organization_account_vault_strategy:
                case AccountVaultStrategy.ALLOWED:
                    cooked_account_vault_strategy = (
                        organization_self_list.AccountVaultStrategy.ALLOWED
                    )
                case AccountVaultStrategy.FORBIDDEN:
                    cooked_account_vault_strategy = (
                        organization_self_list.AccountVaultStrategy.FORBIDDEN
                    )

            cooked_active.append(
                organization_self_list.ActiveUser(
                    user_id=active.user_id,
                    created_on=active.created_on,
                    is_frozen=active.is_frozen,
                    current_profile=active.current_profile,
                    organization_id=active.organization_id,
                    organization_config=organization_self_list.OrganizationConfig(
                        is_expired=active.organization_is_expired,
                        user_profile_outsider_allowed=active.organization_user_profile_outsider_allowed,
                        active_users_limit=active.organization_active_users_limit,
                        allowed_client_agent=cooked_allowed_client_agent,
                        account_vault_strategy=cooked_account_vault_strategy,
                    ),
                )
            )

        cooked_revoked = [
            organization_self_list.RevokedUser(
                user_id=revoked.user_id,
                created_on=revoked.created_on,
                revoked_on=revoked.revoked_on,
                current_profile=revoked.current_profile,
                organization_id=revoked.organization_id,
            )
            for revoked in result.revoked
        ]

        return organization_self_list.RepOk(
            active=cooked_active,
            revoked=cooked_revoked,
        )

    @api
    async def api_auth_method_create(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.auth_method_create.Req,
    ) -> authenticated_account_cmds.latest.auth_method_create.Rep:
        outcome = await self.auth_method_create(
            now=DateTime.now(),
            auth_method_id=client_ctx.auth_method_id,
            created_by_user_agent=client_ctx.client_user_agent,
            created_by_ip=client_ctx.client_ip_address,
            new_auth_method_id=req.auth_method_id,
            new_auth_method_mac_key=req.auth_method_mac_key,
            new_auth_method_password_algorithm=req.auth_method_password_algorithm,
            new_vault_key_access=req.vault_key_access,
        )
        match outcome:
            case None:
                return authenticated_account_cmds.latest.auth_method_create.RepOk()
            case AccountAuthMethodCreateBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS:
                return authenticated_account_cmds.latest.auth_method_create.RepAuthMethodIdAlreadyExists()
            case AccountAuthMethodCreateBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_auth_method_disable(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.auth_method_disable.Req,
    ) -> authenticated_account_cmds.latest.auth_method_disable.Rep:
        outcome = await self.auth_method_disable(
            now=DateTime.now(),
            auth_method_id=client_ctx.auth_method_id,
            to_disable_auth_method_id=req.auth_method_id,
        )
        match outcome:
            case None:
                return authenticated_account_cmds.latest.auth_method_disable.RepOk()
            case AccountAuthMethodDisableBadOutcome.AUTH_METHOD_NOT_FOUND:
                return authenticated_account_cmds.latest.auth_method_disable.RepAuthMethodNotFound()
            case AccountAuthMethodDisableBadOutcome.AUTH_METHOD_ALREADY_DISABLED:
                return authenticated_account_cmds.latest.auth_method_disable.RepAuthMethodAlreadyDisabled()
            case AccountAuthMethodDisableBadOutcome.SELF_DISABLE_NOT_ALLOWED:
                return (
                    authenticated_account_cmds.latest.auth_method_disable.RepSelfDisableNotAllowed()
                )
            case AccountAuthMethodDisableBadOutcome.CROSS_ACCOUNT_NOT_ALLOWED:
                # From the client's perspective, it can't access an auth method from another account
                return authenticated_account_cmds.latest.auth_method_disable.RepAuthMethodNotFound()
            case AccountAuthMethodDisableBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    @api
    async def api_auth_method_list(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.auth_method_list.Req,
    ) -> authenticated_account_cmds.latest.auth_method_list.Rep:
        outcome = await self.auth_method_list(
            auth_method_id=client_ctx.auth_method_id,
        )
        match outcome:
            case list() as items:

                def _convert_auth_method(
                    auth_method: AuthMethod,
                ) -> authenticated_account_cmds.latest.auth_method_list.AuthMethod:
                    match auth_method.password_algorithm:
                        case None:
                            password_algorithm = None
                        case UntrustedPasswordAlgorithmArgon2id() as a:
                            password_algorithm = UntrustedPasswordAlgorithmArgon2id(
                                opslimit=a.opslimit,
                                memlimit_kb=a.memlimit_kb,
                                parallelism=a.parallelism,
                            )
                        # `UntrustedPasswordAlgorithm` is an abstract type
                        case UntrustedPasswordAlgorithm() as unknown:
                            assert False, unknown

                    return authenticated_account_cmds.latest.auth_method_list.AuthMethod(
                        auth_method_id=auth_method.auth_method_id,
                        created_on=auth_method.created_on,
                        created_by_ip=auth_method.created_by_ip,
                        created_by_user_agent=auth_method.created_by_user_agent,
                        vault_key_access=auth_method.vault_key_access,
                        password_algorithm=password_algorithm,
                    )

                converted_items = [_convert_auth_method(item) for item in items]
                return authenticated_account_cmds.latest.auth_method_list.RepOk(
                    items=converted_items
                )
            case AccountAuthMethodListBadOutcome.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()


def _generate_account_create_validation_email(
    jinja_env: Environment,
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    validation_code: ValidationCode,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    server_url = server_url.removesuffix("/")

    html = jinja_env.get_template("email/account_create.html.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )
    text = jinja_env.get_template("email/account_create.txt.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )

    # mail settings
    message = MIMEMultipart("alternative")

    message["Subject"] = "Parsec Account: Confirm your email address"
    message["From"] = str(from_addr)
    message["To"] = str(to_addr)

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message


def _generate_account_delete_validation_email(
    jinja_env: Environment,
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    validation_code: ValidationCode,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    server_url = server_url.removesuffix("/")

    html = jinja_env.get_template("email/account_delete.html.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )
    text = jinja_env.get_template("email/account_delete.txt.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )

    # mail settings
    message = MIMEMultipart("alternative")

    message["Subject"] = "Parsec Account: Confirm account deletion"
    message["From"] = str(from_addr)
    message["To"] = str(to_addr)

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message


def _generate_account_recover_validation_email(
    jinja_env: Environment,
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    validation_code: ValidationCode,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    server_url = server_url.removesuffix("/")

    html = jinja_env.get_template("email/account_recover.html.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )
    text = jinja_env.get_template("email/account_recover.txt.j2").render(
        validation_code=validation_code.str,
        server_url=server_url,
    )

    # mail settings
    message = MIMEMultipart("alternative")

    message["Subject"] = "Parsec Account: Confirm account recovery"
    message["From"] = str(from_addr)
    message["To"] = str(to_addr)

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message
