# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    HashDigest,
    ParsecAccountEmailValidationAddr,
    SecretKey,
    anonymous_account_cmds,
    authenticated_account_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext, AuthenticatedAccountClientContext
from parsec.components.email import SendEmailBadOutcome, send_email
from parsec.config import BackendConfig
from parsec.templates import get_template
from parsec.types import BadOutcomeEnum


@dataclass(slots=True)
class PasswordAlgorithmArgon2ID:
    salt: bytes
    opslimit: int
    memlimit_kb: int
    parallelism: int


# `PasswordAlgorithm` is expected to become a variant once more algorithms are provided
PasswordAlgorithm = PasswordAlgorithmArgon2ID
"""
The algorithm and full configuration to obtain the `auth_method_master_secret` from the user's password.
"""


class AccountCreateEmailValidationTokenBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()
    TOO_SOON_AFTER_PREVIOUS_DEMAND = auto()


class AccountCreateAccountWithPasswordBadOutcome(BadOutcomeEnum):
    INVALID_TOKEN = auto()
    AUTH_METHOD_ALREADY_EXISTS = auto()


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


@dataclass(slots=True)
class VaultItemRecoveryAuthMethodPassword:
    created_on: DateTime
    created_by_ip: str | None
    created_by_user_agent: str
    vault_key_access: bytes
    algorithm: PasswordAlgorithm
    disabled_on: DateTime | None


VaultItemRecoveryAuthMethod = VaultItemRecoveryAuthMethodPassword


@dataclass(slots=True)
class VaultItemRecoveryVault:
    auth_methods: list[VaultItemRecoveryAuthMethod]
    vault_items: dict[HashDigest, bytes]


@dataclass(slots=True)
class VaultItemRecoveryList:
    current_vault: VaultItemRecoveryVault
    previous_vaults: list[VaultItemRecoveryVault]


@dataclass(slots=True)
class VaultItems:
    key_access: bytes
    items: dict[HashDigest, bytes]


class BaseAccountComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def create_email_validation_token(
        self, email: EmailAddress, now: DateTime
    ) -> EmailValidationToken | AccountCreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def create_account_with_password(
        self,
        token: EmailValidationToken,
        now: DateTime,
        mac_key: SecretKey,
        vault_key_access: bytes,
        human_label: str,
        created_by_user_agent: str,
        created_by_ip: str | None,
        password_secret_algorithm: PasswordAlgorithm,
        auth_method_id: AccountAuthMethodID,
    ) -> None | AccountCreateAccountWithPasswordBadOutcome:
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
        created_by_ip: str | None,
        created_by_user_agent: str,
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_password_algorithm: PasswordAlgorithm,
        new_vault_key_access: bytes,
        items: dict[HashDigest, bytes],
    ) -> None | AccountVaultKeyRotation:
        raise NotImplementedError

    async def vault_item_recovery_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> VaultItemRecoveryList | AccountVaultItemRecoveryList:
        raise NotImplementedError

    @api
    async def api_account_create_send_validation_email(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_send_validation_email.Req,
    ) -> anonymous_account_cmds.latest.account_create_send_validation_email.Rep:
        outcome = await self.create_email_validation_token(req.email, DateTime.now())
        match outcome:
            case EmailValidationToken() as token:
                outcome = await self._send_email_validation_token(token, req.email)
                match outcome:
                    case None:
                        return anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()
                    case (
                        SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE
                    ):
                        return anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailServerUnavailable()
                    case SendEmailBadOutcome.RECIPIENT_REFUSED:
                        return anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailRecipientRefused()

            case (
                AccountCreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS
                | AccountCreateEmailValidationTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND
            ):
                # Respond OK without sending token to prevent creating oracle
                return anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    @api
    async def api_account_create_with_password_proceed(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_with_password_proceed.Req,
    ) -> anonymous_account_cmds.latest.account_create_with_password_proceed.Rep:
        now = DateTime.now()
        match req.password_algorithm:
            case (
                anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithmArgon2id() as algo
            ):
                pass
            case _:
                # No other algorithm is supported/implemented for now
                raise NotImplementedError

        outcome = await self.create_account_with_password(
            req.validation_token,
            now,
            req.auth_method_hmac_key,
            req.vault_key_access,
            req.human_label,
            client_ctx.client_user_agent,
            client_ctx.client_ip,
            password_secret_algorithm=PasswordAlgorithm(
                salt=algo.salt,
                opslimit=algo.opslimit,
                memlimit_kb=algo.memlimit_kb,
                parallelism=algo.parallelism,
            ),
            auth_method_id=req.auth_method_id,
        )

        match outcome:
            case None:
                return anonymous_account_cmds.latest.account_create_with_password_proceed.RepOk()
            case AccountCreateAccountWithPasswordBadOutcome.INVALID_TOKEN:
                return anonymous_account_cmds.latest.account_create_with_password_proceed.RepInvalidValidationToken()
            case AccountCreateAccountWithPasswordBadOutcome.AUTH_METHOD_ALREADY_EXISTS:
                return anonymous_account_cmds.latest.account_create_with_password_proceed.RepAuthMethodIdAlreadyExists()

    async def _send_email_validation_token(
        self,
        token: EmailValidationToken,
        claimer_email: EmailAddress,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        validation_url = ParsecAccountEmailValidationAddr.build(
            server_addr=self._config.server_addr,
            token=token,
        ).to_http_redirection_url()

        message = generate_email_validation_email(
            from_addr=self._config.email_config.sender,
            to_addr=claimer_email,
            validation_url=validation_url,
            server_url=self._config.server_addr.to_http_url(),
        )
        return await send_email(
            email_config=self._config.email_config,
            to_addr=claimer_email,
            message=message,
        )

    def should_resend_token(self, now: DateTime, last_email_datetime: DateTime) -> bool:
        return now > last_email_datetime.add(
            seconds=self._config.account_config.account_confirmation_email_resend_delay
        )

    def test_get_token_by_email(self, email: EmailAddress) -> EmailValidationToken | None:
        raise NotImplementedError

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
        match req.new_password_algorithm:
            case (
                authenticated_account_cmds.latest.vault_key_rotation.PasswordAlgorithmArgon2id() as raw
            ):
                new_password_algorithm = PasswordAlgorithmArgon2ID(
                    salt=raw.salt,
                    opslimit=raw.opslimit,
                    memlimit_kb=raw.memlimit_kb,
                    parallelism=raw.parallelism,
                )
            # `PasswordAlgorithm` is an abstract type
            case (
                authenticated_account_cmds.latest.vault_key_rotation.PasswordAlgorithm() as unknown
            ):
                assert False, unknown

        outcome = await self.vault_key_rotation(
            now=DateTime.now(),
            created_by_ip=client_ctx.client_ip_address,
            created_by_user_agent=client_ctx.client_user_agent,
            auth_method_id=client_ctx.auth_method_id,
            new_auth_method_id=req.new_auth_method_id,
            new_auth_method_mac_key=req.new_auth_method_mac_key,
            new_password_algorithm=new_password_algorithm,
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

                def _convert_auth_methods(
                    auth_method: VaultItemRecoveryAuthMethod,
                ) -> authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethod:
                    match auth_method:
                        # Single `case` in this match since so far there is only a single
                        # kind of password algorithm...
                        case VaultItemRecoveryAuthMethodPassword():
                            match auth_method.algorithm:
                                case PasswordAlgorithmArgon2ID() as a:
                                    algorithm = authenticated_account_cmds.latest.vault_item_recovery_list.PasswordAlgorithmArgon2id(
                                        salt=a.salt,
                                        opslimit=a.opslimit,
                                        memlimit_kb=a.memlimit_kb,
                                        parallelism=a.parallelism,
                                    )

                            return authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryAuthMethodPassword(
                                created_on=auth_method.created_on,
                                created_by_ip=auth_method.created_by_ip,
                                created_by_user_agent=auth_method.created_by_user_agent,
                                vault_key_access=auth_method.vault_key_access,
                                algorithm=algorithm,
                                disabled_on=auth_method.disabled_on,
                            )

                def _convert_vault(
                    vault: VaultItemRecoveryVault,
                ) -> authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault:
                    return authenticated_account_cmds.latest.vault_item_recovery_list.VaultItemRecoveryVault(
                        auth_methods=[_convert_auth_methods(m) for m in vault.auth_methods],
                        vault_items=vault.vault_items,
                    )

                return authenticated_account_cmds.latest.vault_item_recovery_list.RepOk(
                    current_vault=_convert_vault(vaults.current_vault),
                    previous_vaults=[_convert_vault(v) for v in vaults.previous_vaults],
                )
            case AccountVaultItemRecoveryList.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()


def generate_email_validation_email(
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    validation_url: str,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    if server_url.endswith("/"):
        server_url = server_url[:-1]

    html = get_template("account_email_validation.html").render(
        validation_url=validation_url,
        server_url=server_url,
    )
    text = get_template("account_email_validation.txt").render(
        validation_url=validation_url,
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
