# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import smtplib
import ssl
import sys
import tempfile
from collections.abc import Buffer
from dataclasses import dataclass
from email.header import Header
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto
from typing import Callable, TypeAlias

import anyio

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    DeviceID,
    GreeterOrClaimer,
    GreetingAttemptID,
    HashDigest,
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    ParsecInvitationAddr,
    PublicKey,
    UserID,
    authenticated_cmds,
    invited_cmds,
)
from parsec.api import api
from parsec.client_context import AuthenticatedClientContext, InvitedClientContext
from parsec.config import BackendConfig, EmailConfig, MockedEmailConfig, SmtpEmailConfig
from parsec.logging import get_logger
from parsec.templates import get_template
from parsec.types import BadOutcome, BadOutcomeEnum

logger = get_logger()

ShamirRecoveryRecipient: TypeAlias = invited_cmds.latest.invite_info.ShamirRecoveryRecipient
UserOnlineStatus: TypeAlias = invited_cmds.latest.invite_info.UserOnlineStatus
UserGreetingAdministrator: TypeAlias = invited_cmds.latest.invite_info.UserGreetingAdministrator
InviteInfoInvitationCreatedBy: TypeAlias = invited_cmds.latest.invite_info.InvitationCreatedBy
InviteListInvitationCreatedBy: TypeAlias = authenticated_cmds.latest.invite_list.InvitationCreatedBy


@dataclass(slots=True)
class BaseInvitationCreatedBy:
    def for_invite_info(self) -> InviteInfoInvitationCreatedBy:
        match self:
            case InvitationCreatedByUser(user_id, human_handle):
                return invited_cmds.latest.invite_info.InvitationCreatedByUser(
                    user_id, human_handle
                )
            case InvitationCreatedByExternalService(service_label):
                return invited_cmds.latest.invite_info.InvitationCreatedByExternalService(
                    service_label
                )
            case unknown:
                assert False, unknown

    def for_invite_list(self) -> InviteListInvitationCreatedBy:
        match self:
            case InvitationCreatedByUser(user_id, human_handle):
                return authenticated_cmds.latest.invite_list.InvitationCreatedByUser(
                    user_id, human_handle
                )
            case InvitationCreatedByExternalService(service_label):
                return authenticated_cmds.latest.invite_list.InvitationCreatedByExternalService(
                    service_label
                )
            case unknown:
                assert False, unknown


@dataclass(slots=True)
class InvitationCreatedByUser(BaseInvitationCreatedBy):
    user_id: UserID
    human_handle: HumanHandle


@dataclass(slots=True)
class InvitationCreatedByExternalService(BaseInvitationCreatedBy):
    service_label: str


type InvitationCreatedBy = InvitationCreatedByUser | InvitationCreatedByExternalService


@dataclass(slots=True)
class UserInvitation:
    TYPE = InvitationType.USER
    created_by: InvitationCreatedBy
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus

    # User-specific fields
    claimer_email: str
    administrators: list[UserGreetingAdministrator]


@dataclass(slots=True)
class DeviceInvitation:
    TYPE = InvitationType.DEVICE
    created_by: InvitationCreatedBy
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus

    # Device-specific fields
    claimer_user_id: UserID
    claimer_human_handle: HumanHandle


@dataclass(slots=True)
class ShamirRecoveryInvitation:
    TYPE = InvitationType.SHAMIR_RECOVERY
    created_by: InvitationCreatedBy
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus

    # Shamir-specific fields
    claimer_user_id: UserID
    claimer_human_handle: HumanHandle
    threshold: int
    recipients: list[ShamirRecoveryRecipient]
    shamir_recovery_created_on: DateTime
    shamir_recovery_deleted_on: None | DateTime

    @property
    def shamir_recovery_is_deleted(self) -> bool:
        return self.shamir_recovery_deleted_on is not None


Invitation = UserInvitation | DeviceInvitation | ShamirRecoveryInvitation


def generate_invite_email(
    invitation_type: InvitationType,
    from_addr: str,
    to_addr: str,
    organization_id: OrganizationID,
    invitation_url: str,
    server_url: str,
    reply_to: str | None = None,
    greeter_name: str | None = None,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    if server_url.endswith("/"):
        server_url = server_url[:-1]

    is_user_invitation = invitation_type == invitation_type.USER
    is_device_invitation = invitation_type == invitation_type.DEVICE
    is_shamir_recovery_invitation = invitation_type == invitation_type.SHAMIR_RECOVERY

    html = get_template("invitation_mail.html").render(
        is_user_invitation=is_user_invitation,
        is_device_invitation=is_device_invitation,
        is_shamir_recovery_invitation=is_shamir_recovery_invitation,
        greeter=greeter_name,
        organization_id=organization_id.str,
        invitation_url=invitation_url,
        server_url=server_url,
    )
    text = get_template("invitation_mail.txt").render(
        is_user_invitation=is_user_invitation,
        is_device_invitation=is_device_invitation,
        is_shamir_recovery_invitation=is_shamir_recovery_invitation,
        greeter=greeter_name,
        organization_id=organization_id.str,
        invitation_url=invitation_url,
        server_url=server_url,
    )

    # mail settings
    message = MIMEMultipart("alternative")
    if greeter_name:
        message["Subject"] = f"[Parsec] {greeter_name} invited you to {organization_id.str}"
    else:
        message["Subject"] = f"[Parsec] New device invitation to {organization_id.str}"
    message["From"] = from_addr
    message["To"] = to_addr
    if reply_to is not None and greeter_name is not None:
        # Contrary to the other address fields, the greeter name can include non-ascii characters
        # Example: "Jean-José" becomes "=?utf-8?q?Jean-Jos=C3=A9?="
        encoded_greeter_name = Header(greeter_name.encode("utf-8"), "utf-8").encode()
        message["Reply-To"] = f"{encoded_greeter_name} <{reply_to}>"

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message


class SendEmailBadOutcome(BadOutcomeEnum):
    SERVER_UNAVAILABLE = auto()
    RECIPIENT_REFUSED = auto()
    BAD_SMTP_CONFIG = auto()


def _smtp_send_email(
    email_config: SmtpEmailConfig, to_addr: str, message: Message
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
            server.sendmail(email_config.sender, to_addr, message.as_string())

    except smtplib.SMTPConnectError:
        return SendEmailBadOutcome.SERVER_UNAVAILABLE
    except smtplib.SMTPRecipientsRefused:
        return SendEmailBadOutcome.RECIPIENT_REFUSED
    except smtplib.SMTPException as exc:
        logger.warning("SMTP error", exc_info=exc, to_addr=to_addr, subject=message["Subject"])
        return SendEmailBadOutcome.BAD_SMTP_CONFIG
    except Exception:
        # Fail-safe: since the device/user has been created, we don't want to fail too hard
        logger.exception(
            "Unexpected exception while sending an email", to_addr=to_addr, message=message
        )
        return SendEmailBadOutcome.BAD_SMTP_CONFIG


def _mocked_send_email(
    email_config: MockedEmailConfig, to_addr: str, message: Message
) -> None | SendEmailBadOutcome:
    tmpfile_fd, tmpfile_path = tempfile.mkstemp(
        prefix="tmp-email-", suffix=".html", dir=email_config.tmpdir
    )
    del tmpfile_fd  # Unused
    tmpfile = open(tmpfile_path, "w")
    tmpfile.write(message.as_string())
    print(
        f"""\
A request to send an e-mail to {to_addr} has been triggered and mocked.
The mail file can be found here: {tmpfile.name}\n""",
        tmpfile.name,
        file=sys.stderr,
    )


async def send_email(
    email_config: EmailConfig, to_addr: str, message: Message
) -> None | SendEmailBadOutcome:
    if isinstance(email_config, SmtpEmailConfig):
        _send_email = _smtp_send_email
    else:
        _send_email = _mocked_send_email

    return await anyio.to_thread.run_sync(_send_email, email_config, to_addr, message)


class InviteNewForUserBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    CLAIMER_EMAIL_ALREADY_ENROLLED = auto()


class InviteNewForDeviceBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


class InviteNewForShamirRecoveryBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    USER_NOT_FOUND = auto()


class InviteListBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


class InviteAsInvitedInfoBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_DELETED = auto()


class InviteShamirRecoveryRevealBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_DELETED = auto()
    BAD_INVITATION_TYPE = auto()
    BAD_REVEAL_TOKEN = auto()


@dataclass
class GreetingAttemptCancelledBadOutcome(BadOutcome):
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: DateTime


@dataclass
class NotReady:
    pass


class InviteGreeterStartGreetingAttemptBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    # Specific outcomes
    INVITATION_NOT_FOUND = auto()
    INVITATION_COMPLETED = auto()
    INVITATION_CANCELLED = auto()
    AUTHOR_NOT_ALLOWED = auto()


class InviteClaimerStartGreetingAttemptBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_CANCELLED = auto()
    INVITATION_COMPLETED = auto()
    # Specific outcomes
    GREETER_NOT_FOUND = auto()
    GREETER_REVOKED = auto()
    GREETER_NOT_ALLOWED = auto()


class InviteGreeterCancelGreetingAttemptBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    # Specific outcomes
    INVITATION_COMPLETED = auto()
    INVITATION_CANCELLED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    GREETING_ATTEMPT_NOT_FOUND = auto()
    GREETING_ATTEMPT_NOT_JOINED = auto()


class InviteClaimerCancelGreetingAttemptBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_CANCELLED = auto()
    INVITATION_COMPLETED = auto()
    # Specific outcomes
    GREETER_REVOKED = auto()
    GREETER_NOT_ALLOWED = auto()
    GREETING_ATTEMPT_NOT_FOUND = auto()
    GREETING_ATTEMPT_NOT_JOINED = auto()


class InviteGreeterStepBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    # Specific outcomes
    INVITATION_COMPLETED = auto()
    INVITATION_CANCELLED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    GREETING_ATTEMPT_NOT_FOUND = auto()
    GREETING_ATTEMPT_NOT_JOINED = auto()
    STEP_TOO_ADVANCED = auto()
    STEP_MISMATCH = auto()


class InviteClaimerStepBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_CANCELLED = auto()
    INVITATION_COMPLETED = auto()
    # Specific outcomes
    GREETER_REVOKED = auto()
    GREETER_NOT_ALLOWED = auto()
    GREETING_ATTEMPT_NOT_FOUND = auto()
    GREETING_ATTEMPT_NOT_JOINED = auto()
    STEP_TOO_ADVANCED = auto()
    STEP_MISMATCH = auto()


class InviteCancelBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    # Specific outcomes
    AUTHOR_NOT_ALLOWED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_ALREADY_CANCELLED = auto()
    INVITATION_COMPLETED = auto()


class InviteCompleteBadOutcome(BadOutcomeEnum):
    # Common outcomes
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    # Specific outcomes
    AUTHOR_NOT_ALLOWED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_CANCELLED = auto()
    INVITATION_ALREADY_COMPLETED = auto()


def process_greeter_step(
    step: authenticated_cmds.latest.invite_greeter_step.GreeterStep,
) -> tuple[
    int, bytes, Callable[[bytes], authenticated_cmds.latest.invite_greeter_step.ClaimerStep]
]:
    match step:
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer():
            return (
                0,
                step.public_key.encode(),
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber0WaitPeer(
                    PublicKey(data)
                ),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber1GetHashedNonce():
            return (
                1,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber1SendHashedNonce(
                    HashDigest(data)
                ),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber2SendNonce():
            return (
                2,
                step.greeter_nonce,
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber2GetNonce(),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce():
            return (
                3,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber3SendNonce(
                    data
                ),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber4WaitPeerTrust():
            return (
                4,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber4SignifyTrust(),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber5SignifyTrust():
            return (
                5,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber5WaitPeerTrust(),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber6GetPayload():
            return (
                6,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber6SendPayload(
                    data
                ),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber7SendPayload():
            return (
                7,
                step.greeter_payload,
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber7GetPayload(),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber8WaitPeerAcknowledgment():
            return (
                8,
                b"",
                lambda data: authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber8Acknowledge(),
            )
        case authenticated_cmds.latest.invite_greeter_step.GreeterStep():
            pass
    assert False, f"Unknown greeter step: {step}"


def process_claimer_step(
    step: invited_cmds.latest.invite_claimer_step.ClaimerStep,
) -> tuple[int, bytes, Callable[[bytes], invited_cmds.latest.invite_claimer_step.GreeterStep]]:
    match step:
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer():
            return (
                0,
                step.public_key.encode(),
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber0WaitPeer(
                    PublicKey(data)
                ),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber1SendHashedNonce():
            return (
                1,
                step.hashed_nonce.digest,
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber1GetHashedNonce(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce():
            return (
                2,
                b"",
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber2SendNonce(
                    data
                ),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber3SendNonce():
            return (
                3,
                step.claimer_nonce,
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber3GetNonce(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber4SignifyTrust():
            return (
                4,
                b"",
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber4WaitPeerTrust(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber5WaitPeerTrust():
            return (
                5,
                b"",
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber5SignifyTrust(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber6SendPayload():
            return (
                6,
                step.claimer_payload,
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber6GetPayload(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber7GetPayload():
            return (
                7,
                b"",
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber7SendPayload(
                    data
                ),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStepNumber8Acknowledge():
            return (
                8,
                b"",
                lambda data: invited_cmds.latest.invite_claimer_step.GreeterStepNumber8WaitPeerAcknowledgment(),
            )
        case invited_cmds.latest.invite_claimer_step.ClaimerStep():
            pass
    assert False, f"Unknown claimer step: {step}"


class BaseInviteComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    # Used by `new_for_user` implementations
    async def _send_user_invitation_email(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        claimer_email: str,
        greeter_human_handle: HumanHandle,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        invitation_url = ParsecInvitationAddr.build(
            server_addr=self._config.server_addr,
            organization_id=organization_id,
            invitation_type=InvitationType.USER,
            token=token,
        ).to_http_redirection_url()

        message = generate_invite_email(
            invitation_type=InvitationType.USER,
            from_addr=self._config.email_config.sender,
            to_addr=claimer_email,
            greeter_name=greeter_human_handle.label,
            reply_to=greeter_human_handle.email,
            organization_id=organization_id,
            invitation_url=invitation_url,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config,
            to_addr=claimer_email,
            message=message,
        )

    # Used by `new_for_device` implementations
    async def _send_device_invitation_email(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        email: str,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        invitation_url = ParsecInvitationAddr.build(
            server_addr=self._config.server_addr,
            organization_id=organization_id,
            invitation_type=InvitationType.DEVICE,
            token=token,
        ).to_http_redirection_url()

        message = generate_invite_email(
            invitation_type=InvitationType.DEVICE,
            from_addr=self._config.email_config.sender,
            to_addr=email,
            organization_id=organization_id,
            invitation_url=invitation_url,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config,
            to_addr=email,
            message=message,
        )

    # Used by `new_for_shamir_recovery` implementations
    async def _send_shamir_recovery_invitation_email(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        email: str,
        greeter_human_handle: HumanHandle,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        invitation_url = ParsecInvitationAddr.build(
            server_addr=self._config.server_addr,
            organization_id=organization_id,
            invitation_type=InvitationType.SHAMIR_RECOVERY,
            token=token,
        ).to_http_redirection_url()

        message = generate_invite_email(
            invitation_type=InvitationType.SHAMIR_RECOVERY,
            from_addr=self._config.email_config.sender,
            to_addr=email,
            greeter_name=greeter_human_handle.label,
            reply_to=greeter_human_handle.email,
            organization_id=organization_id,
            invitation_url=invitation_url,
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config,
            to_addr=email,
            message=message,
        )

    #
    # Public methods
    #

    async def new_for_user(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        claimer_email: str,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForUserBadOutcome:
        raise NotImplementedError

    async def new_for_device(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForDeviceBadOutcome:
        raise NotImplementedError

    async def new_for_shamir_recovery(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        send_email: bool,
        claimer_user_id: UserID,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForShamirRecoveryBadOutcome:
        raise NotImplementedError

    async def cancel(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCancelBadOutcome:
        raise NotImplementedError

    async def list(
        self, organization_id: OrganizationID, author: DeviceID
    ) -> list[Invitation] | InviteListBadOutcome:
        raise NotImplementedError

    async def info_as_invited(
        self, organization_id: OrganizationID, token: InvitationToken
    ) -> Invitation | InviteAsInvitedInfoBadOutcome:
        raise NotImplementedError

    async def test_dump_all_invitations(
        self, organization_id: OrganizationID
    ) -> dict[UserID, list[Invitation]]:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_invite_new_user(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_new_user.Req,
    ) -> authenticated_cmds.latest.invite_new_user.Rep:
        outcome = await self.new_for_user(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            claimer_email=req.claimer_email,
            send_email=req.send_email,
        )
        match outcome:
            case (InvitationToken() as token, None):
                email_sent = (
                    authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.SUCCESS
                )
            case (
                InvitationToken() as token,
                SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE,
            ):
                email_sent = authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case (InvitationToken() as token, SendEmailBadOutcome.RECIPIENT_REFUSED):
                email_sent = authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case InviteNewForUserBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_new_user.RepAuthorNotAllowed()
            case InviteNewForUserBadOutcome.CLAIMER_EMAIL_ALREADY_ENROLLED:
                return authenticated_cmds.latest.invite_new_user.RepClaimerEmailAlreadyEnrolled()
            case InviteNewForUserBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteNewForUserBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteNewForUserBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteNewForUserBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

        return authenticated_cmds.latest.invite_new_user.RepOk(
            token=token,
            email_sent=email_sent,
        )

    @api
    async def api_invite_new_device(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_new_device.Req,
    ) -> authenticated_cmds.latest.invite_new_device.Rep:
        outcome = await self.new_for_device(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            send_email=req.send_email,
        )
        match outcome:
            case (InvitationToken() as token, None):
                email_sent = (
                    authenticated_cmds.latest.invite_new_device.InvitationEmailSentStatus.SUCCESS
                )
            case (
                InvitationToken() as token,
                SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE,
            ):
                email_sent = authenticated_cmds.latest.invite_new_device.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case (InvitationToken() as token, SendEmailBadOutcome.RECIPIENT_REFUSED):
                email_sent = authenticated_cmds.latest.invite_new_device.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case InviteNewForDeviceBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteNewForDeviceBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteNewForDeviceBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteNewForDeviceBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

        return authenticated_cmds.latest.invite_new_device.RepOk(
            token=token,
            email_sent=email_sent,
        )

    @api
    async def api_invite_new_shamir_recovery(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_new_shamir_recovery.Req,
    ) -> authenticated_cmds.latest.invite_new_shamir_recovery.Rep:
        outcome = await self.new_for_shamir_recovery(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            send_email=req.send_email,
            claimer_user_id=req.claimer_user_id,
        )
        match outcome:
            case (InvitationToken() as token, None):
                email_sent = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.SUCCESS
            case (
                InvitationToken() as token,
                SendEmailBadOutcome.BAD_SMTP_CONFIG | SendEmailBadOutcome.SERVER_UNAVAILABLE,
            ):
                email_sent = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case (InvitationToken() as token, SendEmailBadOutcome.RECIPIENT_REFUSED):
                email_sent = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_new_shamir_recovery.RepAuthorNotAllowed()
            case InviteNewForShamirRecoveryBadOutcome.USER_NOT_FOUND:
                return authenticated_cmds.latest.invite_new_shamir_recovery.RepUserNotFound()
            case InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteNewForShamirRecoveryBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteNewForShamirRecoveryBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteNewForShamirRecoveryBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

        return authenticated_cmds.latest.invite_new_shamir_recovery.RepOk(
            token=token,
            email_sent=email_sent,
        )

    @api
    async def api_invite_cancel(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_cancel.Req,
    ) -> authenticated_cmds.latest.invite_cancel.Rep:
        outcome = await self.cancel(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            token=req.token,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.invite_cancel.RepOk()
            case InviteCancelBadOutcome.INVITATION_NOT_FOUND:
                return authenticated_cmds.latest.invite_cancel.RepInvitationNotFound()
            case InviteCancelBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_cancel.RepAuthorNotAllowed()
            case InviteCancelBadOutcome.INVITATION_COMPLETED:
                return authenticated_cmds.latest.invite_cancel.RepInvitationCompleted()
            case InviteCancelBadOutcome.INVITATION_ALREADY_CANCELLED:
                return authenticated_cmds.latest.invite_cancel.RepInvitationAlreadyCancelled()
            case InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteCancelBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteCancelBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteCancelBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_invite_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_list.Req,
    ) -> authenticated_cmds.latest.invite_list.Rep:
        outcome = await self.list(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )
        match outcome:
            case list() as invitations:
                pass
            case InviteListBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteListBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteListBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteListBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

        cooked_invitations = []
        for invitation in invitations:
            match invitation:
                case UserInvitation():
                    cooked = authenticated_cmds.latest.invite_list.InviteListItemUser(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by=invitation.created_by.for_invite_list(),
                        claimer_email=invitation.claimer_email,
                        status=invitation.status,
                    )
                case DeviceInvitation():
                    cooked = authenticated_cmds.latest.invite_list.InviteListItemDevice(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by=invitation.created_by.for_invite_list(),
                        status=invitation.status,
                    )
                case ShamirRecoveryInvitation():
                    cooked = authenticated_cmds.latest.invite_list.InviteListItemShamirRecovery(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        created_by=invitation.created_by.for_invite_list(),
                        status=invitation.status,
                        claimer_user_id=invitation.claimer_user_id,
                        shamir_recovery_created_on=invitation.shamir_recovery_created_on,
                    )
            cooked_invitations.append(cooked)

        return authenticated_cmds.latest.invite_list.RepOk(invitations=cooked_invitations)

    @api
    async def api_invite_info(
        self, client_ctx: InvitedClientContext, req: invited_cmds.latest.invite_info.Req
    ) -> invited_cmds.latest.invite_info.Rep:
        outcome = await self.info_as_invited(
            organization_id=client_ctx.organization_id, token=client_ctx.token
        )
        match outcome:
            case UserInvitation() as invitation:
                return invited_cmds.latest.invite_info.RepOk(
                    invited_cmds.latest.invite_info.InvitationTypeUser(
                        claimer_email=invitation.claimer_email,
                        created_by=invitation.created_by.for_invite_info(),
                        administrators=invitation.administrators,
                    )
                )
            case DeviceInvitation() as invitation:
                return invited_cmds.latest.invite_info.RepOk(
                    invited_cmds.latest.invite_info.InvitationTypeDevice(
                        claimer_user_id=invitation.claimer_user_id,
                        claimer_human_handle=invitation.claimer_human_handle,
                        created_by=invitation.created_by.for_invite_info(),
                    )
                )
            case ShamirRecoveryInvitation() as invitation:
                return invited_cmds.latest.invite_info.RepOk(
                    invited_cmds.latest.invite_info.InvitationTypeShamirRecovery(
                        claimer_user_id=invitation.claimer_user_id,
                        claimer_human_handle=invitation.claimer_human_handle,
                        created_by=invitation.created_by.for_invite_info(),
                        shamir_recovery_created_on=invitation.shamir_recovery_created_on,
                        threshold=invitation.threshold,
                        recipients=invitation.recipients,
                    )
                )
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND:
                return client_ctx.invitation_not_found_abort()
            case InviteAsInvitedInfoBadOutcome.INVITATION_DELETED:
                return client_ctx.invitation_already_used_or_deleted_abort()

    # New invite transport API

    @api
    async def api_invite_greeter_start_greeting_attempt(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_greeter_start_greeting_attempt.Req,
    ) -> authenticated_cmds.latest.invite_greeter_start_greeting_attempt.Rep:
        outcome = await self.greeter_start_greeting_attempt(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            greeter=client_ctx.user_id,
            token=req.token,
        )
        match outcome:
            # OK case
            case GreetingAttemptID() as greeting_attempt_id:
                return authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk(
                    greeting_attempt_id
                )
            # Standard auth errors
            case InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteGreeterStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            # Specific errors
            case InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND:
                return authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationNotFound()
            case InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_CANCELLED:
                return authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationCancelled()
            case InviteGreeterStartGreetingAttemptBadOutcome.INVITATION_COMPLETED:
                return authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationCompleted()
            case InviteGreeterStartGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()

    async def greeter_start_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        token: InvitationToken,
    ) -> GreetingAttemptID | InviteGreeterStartGreetingAttemptBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_claimer_start_greeting_attempt(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.invite_claimer_start_greeting_attempt.Req,
    ) -> invited_cmds.latest.invite_claimer_start_greeting_attempt.Rep:
        outcome = await self.claimer_start_greeting_attempt(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            greeter=req.greeter,
            token=client_ctx.token,
        )
        match outcome:
            # OK case
            case GreetingAttemptID() as greeting_attempt_id:
                return invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk(
                    greeting_attempt_id
                )
            # Standard auth errors
            case InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteClaimerStartGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_NOT_FOUND:
                client_ctx.invitation_not_found_abort()
            case InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_CANCELLED:
                client_ctx.invitation_already_used_or_deleted_abort()
            case InviteClaimerStartGreetingAttemptBadOutcome.INVITATION_COMPLETED:
                client_ctx.invitation_already_used_or_deleted_abort()
            # Specific errors
            case InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_FOUND:
                return (
                    invited_cmds.latest.invite_claimer_start_greeting_attempt.RepGreeterNotFound()
                )
            case InviteClaimerStartGreetingAttemptBadOutcome.GREETER_REVOKED:
                return invited_cmds.latest.invite_claimer_start_greeting_attempt.RepGreeterRevoked()
            case InviteClaimerStartGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED:
                return (
                    invited_cmds.latest.invite_claimer_start_greeting_attempt.RepGreeterNotAllowed()
                )

    async def claimer_start_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeter: UserID,
    ) -> GreetingAttemptID | InviteClaimerStartGreetingAttemptBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_greeter_cancel_greeting_attempt(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.Req,
    ) -> authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.Rep:
        outcome = await self.greeter_cancel_greeting_attempt(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            greeter=client_ctx.user_id,
            greeting_attempt=req.greeting_attempt,
            reason=req.reason,
        )
        match outcome:
            # OK case
            case None:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepOk()
            # Standard auth errors
            case InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteGreeterCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            # Specific errors
            case InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepInvitationCancelled()
            case InviteGreeterCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepInvitationCompleted()
            case InviteGreeterCancelGreetingAttemptBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepAuthorNotAllowed()
            case InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotFound()
            case InviteGreeterCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED:
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
            case GreetingAttemptCancelledBadOutcome(origin, reason, timestamp):
                return authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
                    origin=origin, reason=reason, timestamp=timestamp
                )

    async def greeter_cancel_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteGreeterCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_claimer_cancel_greeting_attempt(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.invite_claimer_cancel_greeting_attempt.Req,
    ) -> invited_cmds.latest.invite_claimer_cancel_greeting_attempt.Rep:
        outcome = await self.claimer_cancel_greeting_attempt(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            token=client_ctx.token,
            greeting_attempt=req.greeting_attempt,
            reason=req.reason,
        )
        match outcome:
            # OK case
            case None:
                return invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepOk()
            # Standard auth errors
            case InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteClaimerCancelGreetingAttemptBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_NOT_FOUND:
                client_ctx.invitation_not_found_abort()
            case InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_CANCELLED:
                client_ctx.invitation_already_used_or_deleted_abort()
            case InviteClaimerCancelGreetingAttemptBadOutcome.INVITATION_COMPLETED:
                client_ctx.invitation_already_used_or_deleted_abort()
            # Specific errors
            case InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_REVOKED:
                return (
                    invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreeterRevoked()
                )
            case InviteClaimerCancelGreetingAttemptBadOutcome.GREETER_NOT_ALLOWED:
                return invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreeterNotAllowed()
            case InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_FOUND:
                return invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotFound()
            case InviteClaimerCancelGreetingAttemptBadOutcome.GREETING_ATTEMPT_NOT_JOINED:
                return invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
            case GreetingAttemptCancelledBadOutcome(origin, reason, timestamp):
                return invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
                    origin=origin, reason=reason, timestamp=timestamp
                )

    async def claimer_cancel_greeting_attempt(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        reason: CancelledGreetingAttemptReason,
    ) -> None | InviteClaimerCancelGreetingAttemptBadOutcome | GreetingAttemptCancelledBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_greeter_step(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_greeter_step.Req,
    ) -> authenticated_cmds.latest.invite_greeter_step.Rep:
        step_index, greeter_data, load_claimer_data = process_greeter_step(req.greeter_step)
        outcome = await self.greeter_step(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            greeter=client_ctx.user_id,
            greeting_attempt=req.greeting_attempt,
            step_index=step_index,
            greeter_data=greeter_data,
        )
        match outcome:
            # OK case
            case Buffer() as claimer_data:
                claimer_step = load_claimer_data(claimer_data)
                return authenticated_cmds.latest.invite_greeter_step.RepOk(
                    claimer_step=claimer_step
                )
            # NotReady case
            case NotReady():
                return authenticated_cmds.latest.invite_greeter_step.RepNotReady()
            # Standard auth errors
            case InviteGreeterStepBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteGreeterStepBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteGreeterStepBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteGreeterStepBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            # Specific errors
            case InviteGreeterStepBadOutcome.INVITATION_CANCELLED:
                return authenticated_cmds.latest.invite_greeter_step.RepInvitationCancelled()
            case InviteGreeterStepBadOutcome.INVITATION_COMPLETED:
                return authenticated_cmds.latest.invite_greeter_step.RepInvitationCompleted()
            case InviteGreeterStepBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_greeter_step.RepAuthorNotAllowed()
            case InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND:
                return authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptNotFound()
            case InviteGreeterStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED:
                return authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptNotJoined()
            case InviteGreeterStepBadOutcome.STEP_TOO_ADVANCED:
                return authenticated_cmds.latest.invite_greeter_step.RepStepTooAdvanced()
            case InviteGreeterStepBadOutcome.STEP_MISMATCH:
                return authenticated_cmds.latest.invite_greeter_step.RepStepMismatch()
            case GreetingAttemptCancelledBadOutcome(origin, reason, timestamp):
                return authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptCancelled(
                    origin=origin, reason=reason, timestamp=timestamp
                )

    async def greeter_step(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        greeter: UserID,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        greeter_data: bytes,
    ) -> bytes | NotReady | InviteGreeterStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_claimer_step(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.invite_claimer_step.Req,
    ) -> invited_cmds.latest.invite_claimer_step.Rep:
        step_index, claimer_data, load_greeter_data = process_claimer_step(req.claimer_step)
        outcome = await self.claimer_step(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            token=client_ctx.token,
            greeting_attempt=req.greeting_attempt,
            step_index=step_index,
            claimer_data=claimer_data,
        )
        match outcome:
            # OK case
            case Buffer() as greeter_data:
                greeter_step = load_greeter_data(greeter_data)
                return invited_cmds.latest.invite_claimer_step.RepOk(greeter_step=greeter_step)
            # NotReady case
            case NotReady():
                return invited_cmds.latest.invite_claimer_step.RepNotReady()
            # Standard auth errors
            case InviteClaimerStepBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteClaimerStepBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteClaimerStepBadOutcome.INVITATION_NOT_FOUND:
                client_ctx.invitation_not_found_abort()
            case InviteClaimerStepBadOutcome.INVITATION_CANCELLED:
                client_ctx.invitation_already_used_or_deleted_abort()
            case InviteClaimerStepBadOutcome.INVITATION_COMPLETED:
                client_ctx.invitation_already_used_or_deleted_abort()
            # Specific errors
            case InviteClaimerStepBadOutcome.GREETER_NOT_ALLOWED:
                return invited_cmds.latest.invite_claimer_step.RepGreeterNotAllowed()
            case InviteClaimerStepBadOutcome.GREETER_REVOKED:
                return invited_cmds.latest.invite_claimer_step.RepGreeterRevoked()
            case InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_FOUND:
                return invited_cmds.latest.invite_claimer_step.RepGreetingAttemptNotFound()
            case InviteClaimerStepBadOutcome.GREETING_ATTEMPT_NOT_JOINED:
                return invited_cmds.latest.invite_claimer_step.RepGreetingAttemptNotJoined()
            case InviteClaimerStepBadOutcome.STEP_TOO_ADVANCED:
                return invited_cmds.latest.invite_claimer_step.RepStepTooAdvanced()
            case InviteClaimerStepBadOutcome.STEP_MISMATCH:
                return invited_cmds.latest.invite_claimer_step.RepStepMismatch()
            case GreetingAttemptCancelledBadOutcome(origin, reason, timestamp):
                return invited_cmds.latest.invite_claimer_step.RepGreetingAttemptCancelled(
                    origin=origin, reason=reason, timestamp=timestamp
                )

    async def claimer_step(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
        step_index: int,
        claimer_data: bytes,
    ) -> bytes | NotReady | InviteClaimerStepBadOutcome | GreetingAttemptCancelledBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_complete(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_complete.Req,
    ) -> authenticated_cmds.latest.invite_complete.Rep:
        outcome = await self.complete(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            token=req.token,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.invite_complete.RepOk()
            case InviteCompleteBadOutcome.INVITATION_NOT_FOUND:
                return authenticated_cmds.latest.invite_complete.RepInvitationNotFound()
            case InviteCompleteBadOutcome.INVITATION_CANCELLED:
                return authenticated_cmds.latest.invite_complete.RepInvitationCancelled()
            case InviteCompleteBadOutcome.INVITATION_ALREADY_COMPLETED:
                return authenticated_cmds.latest.invite_complete.RepInvitationAlreadyCompleted()
            case InviteCompleteBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteCompleteBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteCompleteBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteCompleteBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case InviteCompleteBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.invite_complete.RepAuthorNotAllowed()

    async def complete(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        token: InvitationToken,
    ) -> None | InviteCompleteBadOutcome:
        raise NotImplementedError

    @api
    async def api_invite_shamir_recovery_reveal(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.invite_shamir_recovery_reveal.Req,
    ) -> invited_cmds.latest.invite_shamir_recovery_reveal.Rep:
        outcome = await self.shamir_recovery_reveal(
            organization_id=client_ctx.organization_id,
            token=client_ctx.token,
            reveal_token=req.reveal_token,
        )
        match outcome:
            case Buffer() as ciphered_data:
                return invited_cmds.latest.invite_shamir_recovery_reveal.RepOk(
                    ciphered_data=bytes(ciphered_data)
                )
            case InviteShamirRecoveryRevealBadOutcome.BAD_INVITATION_TYPE:
                return invited_cmds.latest.invite_shamir_recovery_reveal.RepBadInvitationType()
            case InviteShamirRecoveryRevealBadOutcome.BAD_REVEAL_TOKEN:
                return invited_cmds.latest.invite_shamir_recovery_reveal.RepBadRevealToken()
            case InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteShamirRecoveryRevealBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteShamirRecoveryRevealBadOutcome.INVITATION_NOT_FOUND:
                return client_ctx.invitation_not_found_abort()
            case InviteShamirRecoveryRevealBadOutcome.INVITATION_DELETED:
                return client_ctx.invitation_already_used_or_deleted_abort()

    async def shamir_recovery_reveal(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        reveal_token: InvitationToken,
    ) -> bytes | InviteShamirRecoveryRevealBadOutcome:
        raise NotImplementedError
