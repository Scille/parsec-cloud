# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto

from pydantic import EmailStr, TypeAdapter

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailValidationToken,
    ParsecAccountEmailValidationAddr,
    SecretKey,
    anonymous_account_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext
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


EmailAdapter = TypeAdapter(EmailStr)


class AccountCreateEmailValidationTokenBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()
    TOO_SOON_AFTER_PREVIOUS_DEMAND = auto()


class AccountCreateAccountWithPasswordBadOutcome(BadOutcomeEnum):
    INVALID_TOKEN = auto()
    AUTH_METHOD_ALREADY_EXISTS = auto()


class AccountGetPasswordSecretKeyBadOutcome(BadOutcomeEnum):
    USER_NOT_FOUND = auto()
    UNABLE_TO_GET_SECRET_KEY = auto()


class BaseAccountComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | AccountCreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
        raise NotImplementedError

    def get_password_mac_key(
        self, user_email: EmailStr
    ) -> SecretKey | AccountGetPasswordSecretKeyBadOutcome:
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

    @api
    async def api_account_create_send_validation_email(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_create_send_validation_email.Req,
    ) -> anonymous_account_cmds.latest.account_create_send_validation_email.Rep:
        try:
            # TODO use specific email type #10211
            email_parsed = EmailAdapter.validate_python(req.email)
        except ValueError:
            return (
                anonymous_account_cmds.latest.account_create_send_validation_email.RepInvalidEmail()
            )

        outcome = await self.create_email_validation_token(email_parsed, DateTime.now())
        match outcome:
            case EmailValidationToken() as token:
                outcome = await self._send_email_validation_token(token, email_parsed)
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
        claimer_email: str,
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

    def test_get_token_by_email(self, email: str) -> EmailValidationToken | None:
        raise NotImplementedError


def generate_email_validation_email(
    from_addr: str,
    to_addr: str,
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
    message["From"] = from_addr
    message["To"] = to_addr

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message
