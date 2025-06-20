# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto
from typing import Literal

from parsec._parsec import (
    AccountAuthMethodID,
    AccountDeletionToken,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    HashDigest,
    ParsecAccountDeletionAddr,
    ParsecAccountEmailValidationAddr,
    SecretKey,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
    anonymous_account_cmds,
    authenticated_account_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext, AuthenticatedAccountClientContext
from parsec.components.email import SendEmailBadOutcome, send_email
from parsec.config import BackendConfig
from parsec.templates import get_template
from parsec.types import BadOutcomeEnum


class AccountCreateEmailValidationTokenBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()
    TOO_SOON_AFTER_PREVIOUS_DEMAND = auto()


class AccountCreateAccountBadOutcome(BadOutcomeEnum):
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


class AccountCreateAccountDeletionTokenBadOutcome(BadOutcomeEnum):
    TOO_SOON_AFTER_PREVIOUS_DEMAND = auto()


@dataclass(slots=True)
class VaultItemRecoveryAuthMethod:
    created_on: DateTime
    # IP address or empty string if not available
    created_by_ip: str | Literal[""]
    created_by_user_agent: str
    vault_key_access: bytes
    password_algorithm: UntrustedPasswordAlgorithm | None
    disabled_on: DateTime | None


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

    def _generate_fake_password_algorithm(self, email: EmailAddress) -> UntrustedPasswordAlgorithm:
        return UntrustedPasswordAlgorithm.generate_fake_from_seed(
            email.str, self._config.fake_account_password_algorithm_seed
        )

    async def create_email_validation_token(
        self, email: EmailAddress, now: DateTime
    ) -> EmailValidationToken | AccountCreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def create_account(
        self,
        token: EmailValidationToken,
        now: DateTime,
        mac_key: SecretKey,
        vault_key_access: bytes,
        human_label: str,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        auth_method_id: AccountAuthMethodID,
        auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
    ) -> None | AccountCreateAccountBadOutcome:
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
    async def api_account_create_proceed(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_proceed.Req,
    ) -> anonymous_account_cmds.latest.account_create_proceed.Rep:
        now = DateTime.now()
        match req.auth_method_password_algorithm:
            case UntrustedPasswordAlgorithmArgon2id() as algo:
                pass
            case _:
                # No other algorithm is supported/implemented for now
                raise NotImplementedError

        outcome = await self.create_account(
            token=req.validation_token,
            now=now,
            mac_key=req.auth_method_hmac_key,
            vault_key_access=req.vault_key_access,
            human_label=req.human_label,
            created_by_user_agent=client_ctx.client_user_agent,
            created_by_ip=client_ctx.client_ip_address,
            auth_method_id=req.auth_method_id,
            auth_method_password_algorithm=UntrustedPasswordAlgorithmArgon2id(
                opslimit=algo.opslimit,
                memlimit_kb=algo.memlimit_kb,
                parallelism=algo.parallelism,
            ),
        )

        match outcome:
            case None:
                return anonymous_account_cmds.latest.account_create_proceed.RepOk()
            case AccountCreateAccountBadOutcome.INVALID_TOKEN:
                return (
                    anonymous_account_cmds.latest.account_create_proceed.RepInvalidValidationToken()
                )
            case AccountCreateAccountBadOutcome.AUTH_METHOD_ALREADY_EXISTS:
                return anonymous_account_cmds.latest.account_create_proceed.RepAuthMethodIdAlreadyExists()

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
            seconds=self._config.account_confirmation_email_resend_delay
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

                def _convert_auth_methods(
                    auth_method: VaultItemRecoveryAuthMethod,
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
                        auth_methods=[_convert_auth_methods(m) for m in vault.auth_methods],
                        vault_items=vault.vault_items,
                    )

                return authenticated_account_cmds.latest.vault_item_recovery_list.RepOk(
                    current_vault=_convert_vault(vaults.current_vault),
                    previous_vaults=[_convert_vault(v) for v in vaults.previous_vaults],
                )
            case AccountVaultItemRecoveryList.ACCOUNT_NOT_FOUND:
                client_ctx.account_not_found_abort()

    async def _send_email_deletion_token(
        self, token: AccountDeletionToken, email: EmailAddress
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        deletion_url = ParsecAccountDeletionAddr.build(
            server_addr=self._config.server_addr,
            token=token,
        ).to_http_redirection_url()

        message = generate_email_deletion_email(
            from_addr=self._config.email_config.sender,
            to_addr=email,
            deletion_url=deletion_url,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config, to_addr=email, message=message
        )

    async def create_email_deletion_token(
        self,
        email: EmailAddress,
        now: DateTime,
    ) -> AccountDeletionToken | AccountCreateAccountDeletionTokenBadOutcome:
        raise NotImplementedError

    @api
    async def api_account_delete_send_validation_email(
        self,
        client_ctx: AuthenticatedAccountClientContext,
        req: authenticated_account_cmds.latest.account_delete_send_validation_token.Req,
    ) -> authenticated_account_cmds.latest.account_delete_send_validation_token.Rep:
        outcome = await self.create_email_deletion_token(client_ctx.account_email, DateTime.now())
        match outcome:
            case AccountDeletionToken() as token:
                outcome = await self._send_email_deletion_token(token, client_ctx.account_email)
                match outcome:
                    case None:
                        return authenticated_account_cmds.latest.account_delete_send_validation_token.RepOk()
                    case (
                        SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE
                    ):
                        return authenticated_account_cmds.latest.account_delete_send_validation_token.RepEmailServerUnavailable()
                    case SendEmailBadOutcome.RECIPIENT_REFUSED:
                        return authenticated_account_cmds.latest.account_delete_send_validation_token.RepEmailRecipientRefused()
            case AccountCreateAccountDeletionTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND:
                return (
                    authenticated_account_cmds.latest.account_delete_send_validation_token.RepOk()
                )


def generate_email_validation_email(
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    validation_url: str,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    server_url = server_url.removesuffix("/")

    html = get_template("email/account_validation.html.j2").render(
        validation_url=validation_url,
        server_url=server_url,
    )
    text = get_template("email/account_validation.txt.j2").render(
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


def generate_email_deletion_email(
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    deletion_url: str,
    server_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    server_url = server_url.removesuffix("/")

    html = get_template("email/account_deletion.html.j2").render(
        deletion_url=deletion_url,
        server_url=server_url,
    )
    text = get_template("email/account_deletion.txt.j2").render(
        deletion_url=deletion_url,
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
