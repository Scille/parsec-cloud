# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import smtplib
import ssl
import sys
from base64 import b64decode
from email.message import Message
from enum import auto

import anyio

from parsec._parsec import DateTime, EmailAddress
from parsec.config import EmailConfig, MockedEmailConfig, MockedSentEmail, SmtpEmailConfig
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
        logger.debug("SSL Context", context=context)
        if email_config.use_ssl:
            logger.debug("Configure SMTP client with SSL")
            server: smtplib.SMTP | smtplib.SMTP_SSL = smtplib.SMTP_SSL(
                email_config.host, email_config.port, context=context
            )
        else:
            logger.debug("Configure SMTP client without SSL")
            server = smtplib.SMTP(email_config.host, email_config.port)

        with server:
            if email_config.use_tls and not email_config.use_ssl:
                logger.debug("Start SMTP session in TLS mode")
                if server.starttls(context=context)[0] != 220:
                    logger.warning("Email TLS connection isn't encrypted")
            if email_config.host_user and email_config.host_password:
                server.login(email_config.host_user, email_config.host_password)
            server.sendmail(email_config.sender.str, to_addr.str, message.as_string())

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
    message_body = message.as_string()
    timestamp = DateTime.now()
    email_config.sent_emails.append(
        MockedSentEmail(
            sender=email_config.sender, recipient=to_addr, timestamp=timestamp, body=message_body
        )
    )

    # Try to only display the plain text part for brevity...
    for part in message.walk():
        payload = part.get_payload()
        if part.get_content_type() != "text/plain":
            continue
        if not isinstance(payload, str):
            continue
        match part.get_content_charset():
            case "utf-8":
                if part.get("Content-Transfer-Encoding") != "base64":
                    continue
                display_message_body = b64decode(payload).decode("utf8")
            case "us-ascii":
                display_message_body = payload
            case _:
                continue
        break
    else:
        # ...or fallback to showing the full body :/
        display_message_body = message_body

    print(
        f"""A request to send an e-mail has been triggered and mocked:
FROM: {email_config.sender.str}
TO: {to_addr.str}
AT: {timestamp.to_rfc3339()}
================== EMAIL BODY ===================
{display_message_body}
=============== END OF EMAIL BODY ===============
""",
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
