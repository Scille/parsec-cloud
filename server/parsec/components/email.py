# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import smtplib
import ssl
import sys
import tempfile
from email.message import Message
from enum import auto

import anyio

from parsec._parsec import EmailAddress
from parsec.config import EmailConfig, MockedEmailConfig, SmtpEmailConfig
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()


class SendEmailBadOutcome(BadOutcomeEnum):
    SERVER_UNAVAILABLE = auto()
    RECIPIENT_REFUSED = auto()
    BAD_SMTP_CONFIG = auto()


def _smtp_send_email(
    email_config: SmtpEmailConfig, to_addr: EmailAddress, message: Message
) -> None | SendEmailBadOutcome:
    try:
        context = ssl.create_default_context()
        if email_config.use_ssl:
            server: smtplib.SMTP | smtplib.SMTP_SSL = smtplib.SMTP_SSL(
                email_config.host, email_config.port, context=context
            )
        else:
            server = smtplib.SMTP(email_config.host, email_config.port)

        with server:
            if email_config.use_tls and not email_config.use_ssl:
                if server.starttls(context=context)[0] != 220:
                    logger.warning("Email TLS connection isn't encrypted")
            if email_config.host_user and email_config.host_password:
                server.login(email_config.host_user, email_config.host_password)
            server.sendmail(str(email_config.sender), str(to_addr), message.as_string())

    except smtplib.SMTPConnectError:
        return SendEmailBadOutcome.SERVER_UNAVAILABLE
    except smtplib.SMTPRecipientsRefused:
        return SendEmailBadOutcome.RECIPIENT_REFUSED
    except smtplib.SMTPException as exc:
        logger.warning("SMTP error", exc_info=exc, to_addr=to_addr, subject=message["Subject"])
        return SendEmailBadOutcome.BAD_SMTP_CONFIG
    except Exception:
        # Fail-safe: since email might come after a device/user has been created,
        # we don't want to fail too hard
        logger.exception(
            "Unexpected exception while sending an email", to_addr=to_addr, message=message
        )
        return SendEmailBadOutcome.BAD_SMTP_CONFIG


def _mocked_send_email(
    email_config: MockedEmailConfig, to_addr: EmailAddress, message: Message
) -> None | SendEmailBadOutcome:
    with tempfile.NamedTemporaryFile(
        prefix="tmp-email-",
        suffix=".html",
        dir=email_config.tmpdir,
        mode="w",
        delete_on_close=False,
        delete=False,
    ) as tmpfile:
        tmpfile.write(message.as_string())
    print(
        f"""\
        A request to send an e-mail to {to_addr} has been triggered and mocked.
        The mail file can be found here: {tmpfile.name}\n""",
        tmpfile.name,
        file=sys.stderr,
    )


async def send_email(
    email_config: EmailConfig, to_addr: EmailAddress, message: Message
) -> None | SendEmailBadOutcome:
    if isinstance(email_config, SmtpEmailConfig):
        _send_email = _smtp_send_email
    else:
        _send_email = _mocked_send_email

    return await anyio.to_thread.run_sync(_send_email, email_config, to_addr, message)
