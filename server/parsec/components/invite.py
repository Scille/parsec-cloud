# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import smtplib
import ssl
import sys
import tempfile
from collections import defaultdict
from dataclasses import dataclass
from email.header import Header
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import Enum, auto
from typing import assert_never

import anyio
from structlog.stdlib import get_logger

from parsec._parsec import (
    DateTime,
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    ParsecInvitationAddr,
    UserID,
    authenticated_cmds,
    invited_cmds,
)
from parsec.api import api
from parsec.client_context import AuthenticatedClientContext, InvitedClientContext
from parsec.components.events import EventBus
from parsec.config import BackendConfig, EmailConfig, MockedEmailConfig, SmtpEmailConfig
from parsec.events import (
    Event,
    EventInvitation,
)
from parsec.templates import get_template
from parsec.types import BadOutcomeEnum

logger = get_logger()


@dataclass(slots=True)
class UserInvitation:
    TYPE = InvitationType.USER
    claimer_email: str
    greeter_user_id: UserID
    greeter_human_handle: HumanHandle
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus


@dataclass(slots=True)
class DeviceInvitation:
    TYPE = InvitationType.DEVICE
    greeter_user_id: UserID
    greeter_human_handle: HumanHandle
    token: InvitationToken
    created_on: DateTime
    status: InvitationStatus


Invitation = UserInvitation | DeviceInvitation


def generate_invite_email(
    from_addr: str,
    to_addr: str,
    reply_to: str | None,
    greeter_name: str | None,  # None for device invitation
    organization_id: OrganizationID,
    invitation_url: str,
    backend_url: str,
) -> Message:
    # Quick fix to have a similar behavior between Rust and Python
    if backend_url.endswith("/"):
        backend_url = backend_url[:-1]
    html = get_template("invitation_mail.html").render(
        greeter=greeter_name,
        organization_id=organization_id.str,
        invitation_url=invitation_url,
        backend_url=backend_url,
    )
    text = get_template("invitation_mail.txt").render(
        greeter=greeter_name,
        organization_id=organization_id.str,
        invitation_url=invitation_url,
        backend_url=backend_url,
    )

    # mail settings
    message = MIMEMultipart("alternative")
    if greeter_name:
        message["Subject"] = f"[Parsec] { greeter_name } invited you to { organization_id.str }"
    else:
        message["Subject"] = f"[Parsec] New device invitation to { organization_id.str }"
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


class InviteConduitGreeterExchangeBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_DELETED = auto()
    RETRY_NEEDED = auto()


class InviteConduitClaimerExchangeBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_DELETED = auto()
    RETRY_NEEDED = auto()


class InviteConduitExchangeResetReason(Enum):
    NORMAL = auto()
    BAD_SAS_CODE = auto()


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


class InviteCancelBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_ALREADY_DELETED = auto()


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


class BaseInviteComponent:
    def __init__(self, event_bus: EventBus, config: BackendConfig):
        self._event_bus = event_bus
        self._config = config
        # We use the `invite.status_changed` event to keep a list of all the
        # invitation claimers connected across all Parsec server instances.
        #
        # This is useful to display the invitations ready to be greeted.
        # Note we rely on a per-server list in memory instead of storing this
        # information in database so that we default to no claimer present
        # (which is the most likely when a server is restarted) .
        #
        # However there are multiple ways this list can go out of sync:
        # - a claimer can be connected to a server, then another server starts
        # - the server the claimer is connected to crashes without being able
        #   to notify the other servers
        # - a claimer open multiple connections at the same time, then is
        #   considered disconnected as soon as he closes one of his connections
        #
        # This is considered "fine enough" given all the claimer has to do
        # to fix this is to retry a connection, which precisely the kind of
        # "I.T., have you tried to turn it off and on again ?" a human is
        # expected to do ;-)
        self._claimers_ready: dict[OrganizationID, set[InvitationToken]] = defaultdict(set)
        self._event_bus.connect(self._on_event)
        # Note we don't have a `__del__` to disconnect from the event bus: the lifetime
        # of this component is basically equivalent of the one of the event bus anyway

    def _on_event(self, event: Event) -> None:
        if isinstance(event, EventInvitation):
            if event.status == InvitationStatus.READY:
                self._claimers_ready[event.organization_id].add(event.token)
            else:  # Invitation deleted or back to idle
                self._claimers_ready[event.organization_id].discard(event.token)

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
            from_addr=self._config.email_config.sender,
            to_addr=claimer_email,
            greeter_name=greeter_human_handle.label,
            reply_to=greeter_human_handle.email,
            organization_id=organization_id,
            invitation_url=invitation_url,
            backend_url=self._config.server_addr.to_http_domain_url(),
        )

        await send_email(
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
            from_addr=self._config.email_config.sender,
            to_addr=email,
            greeter_name=None,
            reply_to=None,
            organization_id=organization_id,
            invitation_url=invitation_url,
            backend_url=self._config.server_addr.to_http_domain_url(),
        )

        await send_email(
            email_config=self._config.email_config,
            to_addr=email,
            message=message,
        )

    #
    # Public methods
    #

    async def conduit_greeter_exchange(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        step: int,
        greeter_payload: bytes,
        last: bool = False,
        reset_reason: InviteConduitExchangeResetReason = InviteConduitExchangeResetReason.NORMAL,
    ) -> bytes | InviteConduitExchangeResetReason | InviteConduitGreeterExchangeBadOutcome:
        raise NotImplementedError

    async def conduit_claimer_exchange(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
        step: int,
        claimer_payload: bytes,
        reset_reason: InviteConduitExchangeResetReason = InviteConduitExchangeResetReason.NORMAL,
    ) -> (
        tuple[bytes, bool]
        | InviteConduitExchangeResetReason
        | InviteConduitClaimerExchangeBadOutcome
    ):
        raise NotImplementedError

    async def _claimer_joined(
        self, organization_id: OrganizationID, token: InvitationToken, greeter: UserID
    ) -> None:
        raise NotImplementedError

    async def _claimer_left(
        self, organization_id: OrganizationID, token: InvitationToken, greeter: UserID
    ) -> None:
        raise NotImplementedError

    async def new_for_user(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: UserID,
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
        author: UserID,
        send_email: bool,
        # Only needed for testbed template
        force_token: InvitationToken | None = None,
    ) -> tuple[InvitationToken, None | SendEmailBadOutcome] | InviteNewForDeviceBadOutcome:
        raise NotImplementedError

    async def cancel(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: UserID,
        token: InvitationToken,
    ) -> None | InviteCancelBadOutcome:
        raise NotImplementedError

    async def list_as_user(
        self, organization_id: OrganizationID, author: UserID
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
            author=client_ctx.device_id.user_id,
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
                SendEmailBadOutcome.BAD_SMTP_CONFIG
                | SendEmailBadOutcome.SERVER_UNAVAILABLE,
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
            case unknown:
                assert_never(unknown)

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
            author=client_ctx.device_id.user_id,
            send_email=req.send_email,
        )
        match outcome:
            case (InvitationToken() as token, None):
                email_sent = (
                    authenticated_cmds.latest.invite_new_device.InvitationEmailSentStatus.SUCCESS
                )
            case (
                InvitationToken() as token,
                SendEmailBadOutcome.BAD_SMTP_CONFIG
                | SendEmailBadOutcome.SERVER_UNAVAILABLE,
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
            case unknown:
                assert_never(unknown)

        return authenticated_cmds.latest.invite_new_device.RepOk(
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
            author=client_ctx.device_id.user_id,
            token=req.token,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.invite_cancel.RepOk()
            case InviteCancelBadOutcome.INVITATION_NOT_FOUND:
                return authenticated_cmds.latest.invite_cancel.RepInvitationNotFound()
            case InviteCancelBadOutcome.INVITATION_ALREADY_DELETED:
                return authenticated_cmds.latest.invite_cancel.RepInvitationAlreadyDeleted()
            case InviteCancelBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteCancelBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteCancelBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteCancelBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case unknown:
                assert_never(unknown)

    @api
    async def api_invite_list(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_list.Req,
    ) -> authenticated_cmds.latest.invite_list.Rep:
        outcome = await self.list_as_user(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id.user_id,
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
            case unknown:
                assert_never(unknown)

        cooked_invitations = []
        for invitation in invitations:
            match invitation:
                case UserInvitation():
                    cooked = authenticated_cmds.latest.invite_list.InviteListItemUser(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        claimer_email=invitation.claimer_email,
                        status=invitation.status,
                    )
                case DeviceInvitation():
                    cooked = authenticated_cmds.latest.invite_list.InviteListItemDevice(
                        token=invitation.token,
                        created_on=invitation.created_on,
                        status=invitation.status,
                    )
                case unknown:
                    assert_never(unknown)
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
                    invited_cmds.latest.invite_info.UserOrDeviceUser(
                        claimer_email=invitation.claimer_email,
                        greeter_user_id=invitation.greeter_user_id,
                        greeter_human_handle=invitation.greeter_human_handle,
                    )
                )
            case DeviceInvitation() as invitation:
                return invited_cmds.latest.invite_info.RepOk(
                    invited_cmds.latest.invite_info.UserOrDeviceDevice(
                        greeter_user_id=invitation.greeter_user_id,
                        greeter_human_handle=invitation.greeter_human_handle,
                    )
                )
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteAsInvitedInfoBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteAsInvitedInfoBadOutcome.INVITATION_NOT_FOUND:
                return client_ctx.invitation_invalid_abort()
            case InviteAsInvitedInfoBadOutcome.INVITATION_DELETED:
                return client_ctx.invitation_invalid_abort()
            case unknown:
                assert_never(unknown)

    @api
    async def api_invite_greeter_exchange(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.invite_exchange.Req,
    ) -> authenticated_cmds.latest.invite_exchange.Rep:
        reset_reason = InviteConduitExchangeResetReason.NORMAL
        match req.reset_reason:
            case authenticated_cmds.latest.invite_exchange.InviteExchangeResetReason.NORMAL:
                reset_reason = InviteConduitExchangeResetReason.NORMAL
            case authenticated_cmds.latest.invite_exchange.InviteExchangeResetReason.BAD_SAS_CODE:
                reset_reason = InviteConduitExchangeResetReason.BAD_SAS_CODE
            case None:
                if req.step == 0:
                    return authenticated_cmds.latest.invite_exchange.RepStep0RequiresResetReason()
        outcome = await self.conduit_greeter_exchange(
            organization_id=client_ctx.organization_id,
            token=req.token,
            step=req.step,
            greeter_payload=req.greeter_payload,
            last=req.last,
            reset_reason=reset_reason,
        )
        match outcome:
            case bytes() | bytearray() | memoryview() as claimer_payload:
                return authenticated_cmds.latest.invite_exchange.RepOk(
                    claimer_payload=claimer_payload,
                )
            case InviteConduitGreeterExchangeBadOutcome.RETRY_NEEDED:
                return authenticated_cmds.latest.invite_exchange.RepRetryNeeded()
            case InviteConduitExchangeResetReason.NORMAL:
                return authenticated_cmds.latest.invite_exchange.RepEnrollmentWrongStep(
                    reason=authenticated_cmds.latest.invite_exchange.InviteExchangeResetReason.NORMAL
                )
            case InviteConduitExchangeResetReason.BAD_SAS_CODE:
                return authenticated_cmds.latest.invite_exchange.RepEnrollmentWrongStep(
                    reason=authenticated_cmds.latest.invite_exchange.InviteExchangeResetReason.BAD_SAS_CODE
                )
            case InviteConduitGreeterExchangeBadOutcome.INVITATION_NOT_FOUND:
                return authenticated_cmds.latest.invite_exchange.RepInvitationNotFound()
            case InviteConduitGreeterExchangeBadOutcome.INVITATION_DELETED:
                return authenticated_cmds.latest.invite_exchange.RepInvitationDeleted()
            case InviteConduitGreeterExchangeBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteConduitGreeterExchangeBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteConduitGreeterExchangeBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case InviteConduitGreeterExchangeBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case unknown:
                assert_never(unknown)

    @api
    async def api_invite_claimer_exchange(
        self,
        client_ctx: InvitedClientContext,
        req: invited_cmds.latest.invite_exchange.Req,
    ) -> invited_cmds.latest.invite_exchange.Rep:
        reset_reason = InviteConduitExchangeResetReason.NORMAL
        match req.reset_reason:
            case invited_cmds.latest.invite_exchange.InviteExchangeResetReason.NORMAL:
                reset_reason = InviteConduitExchangeResetReason.NORMAL
            case invited_cmds.latest.invite_exchange.InviteExchangeResetReason.BAD_SAS_CODE:
                reset_reason = InviteConduitExchangeResetReason.BAD_SAS_CODE
            case None:
                if req.step == 0:
                    return invited_cmds.latest.invite_exchange.RepStep0RequiresResetReason()
        outcome = await self.conduit_claimer_exchange(
            organization_id=client_ctx.organization_id,
            token=client_ctx.token,
            step=req.step,
            claimer_payload=req.claimer_payload,
            reset_reason=reset_reason,
        )
        match outcome:
            case (greeter_payload, last):
                return invited_cmds.latest.invite_exchange.RepOk(
                    greeter_payload=greeter_payload, last=last
                )
            case InviteConduitClaimerExchangeBadOutcome.RETRY_NEEDED:
                return invited_cmds.latest.invite_exchange.RepRetryNeeded()
            case InviteConduitExchangeResetReason.NORMAL:
                return invited_cmds.latest.invite_exchange.RepEnrollmentWrongStep(
                    reason=invited_cmds.latest.invite_exchange.InviteExchangeResetReason.NORMAL
                )
            case InviteConduitExchangeResetReason.BAD_SAS_CODE:
                return invited_cmds.latest.invite_exchange.RepEnrollmentWrongStep(
                    reason=invited_cmds.latest.invite_exchange.InviteExchangeResetReason.BAD_SAS_CODE
                )
            case InviteConduitClaimerExchangeBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case InviteConduitClaimerExchangeBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case InviteConduitClaimerExchangeBadOutcome.INVITATION_NOT_FOUND:
                client_ctx.invitation_invalid_abort()
            case InviteConduitClaimerExchangeBadOutcome.INVITATION_DELETED:
                client_ctx.invitation_invalid_abort()
            case unknown:
                assert_never(unknown)
