# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto

from pydantic import EmailStr, TypeAdapter

from parsec._parsec import (
    DateTime,
    EmailValidationToken,
    ParsecAccountEmailValidationAddr,
    anonymous_account_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext
from parsec.components.email import SendEmailBadOutcome, send_email
from parsec.config import BackendConfig
from parsec.templates import get_template
from parsec.types import BadOutcomeEnum

EmailAdapter = TypeAdapter(EmailStr)


class AccountCreateEmailValidationTokenBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()
    TOO_SOON_AFTER_PREVIOUS_DEMAND = auto()


class BaseAccountComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | AccountCreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
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
