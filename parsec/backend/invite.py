# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import attr
import trio
import smtplib
import ssl
import sys
import tempfile
from enum import Enum
from collections import defaultdict
from typing import Dict, List, Optional, Union, Set, cast
from parsec._parsec import DateTime
from email.message import Message
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from structlog import get_logger

from parsec._parsec import (
    InvitationType,
    Invite1ClaimerWaitPeerRep,
    Invite1ClaimerWaitPeerRepInvalidState,
    Invite1ClaimerWaitPeerRepNotFound,
    Invite1ClaimerWaitPeerRepOk,
    Invite1ClaimerWaitPeerReq,
    Invite1GreeterWaitPeerRep,
    Invite1GreeterWaitPeerRepAlreadyDeleted,
    Invite1GreeterWaitPeerRepInvalidState,
    Invite1GreeterWaitPeerRepNotFound,
    Invite1GreeterWaitPeerRepOk,
    Invite1GreeterWaitPeerReq,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aClaimerSendHashedNonceRepInvalidState,
    Invite2aClaimerSendHashedNonceRepNotFound,
    Invite2aClaimerSendHashedNonceRepOk,
    Invite2aClaimerSendHashedNonceReq,
    Invite2aGreeterGetHashedNonceRep,
    Invite2aGreeterGetHashedNonceRepAlreadyDeleted,
    Invite2aGreeterGetHashedNonceRepInvalidState,
    Invite2aGreeterGetHashedNonceRepNotFound,
    Invite2aGreeterGetHashedNonceRepOk,
    Invite2aGreeterGetHashedNonceReq,
    Invite2bClaimerSendNonceRepOk,
    Invite2bClaimerSendNonceReq,
    Invite2bGreeterSendNonceRep,
    Invite2bGreeterSendNonceRepAlreadyDeleted,
    Invite2bGreeterSendNonceRepInvalidState,
    Invite2bGreeterSendNonceRepNotFound,
    Invite2bGreeterSendNonceRepOk,
    Invite2bGreeterSendNonceReq,
    Invite3aClaimerSignifyTrustRep,
    Invite3aClaimerSignifyTrustRepInvalidState,
    Invite3aClaimerSignifyTrustRepNotFound,
    Invite3aClaimerSignifyTrustRepOk,
    Invite3aClaimerSignifyTrustReq,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3aGreeterWaitPeerTrustRepAlreadyDeleted,
    Invite3aGreeterWaitPeerTrustRepInvalidState,
    Invite3aGreeterWaitPeerTrustRepNotFound,
    Invite3aGreeterWaitPeerTrustRepOk,
    Invite3aGreeterWaitPeerTrustReq,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustRepInvalidState,
    Invite3bClaimerWaitPeerTrustRepNotFound,
    Invite3bClaimerWaitPeerTrustRepOk,
    Invite3bClaimerWaitPeerTrustReq,
    Invite3bGreeterSignifyTrustRep,
    Invite3bGreeterSignifyTrustRepAlreadyDeleted,
    Invite3bGreeterSignifyTrustRepInvalidState,
    Invite3bGreeterSignifyTrustRepNotFound,
    Invite3bGreeterSignifyTrustRepOk,
    Invite3bGreeterSignifyTrustReq,
    Invite4ClaimerCommunicateRep,
    Invite4ClaimerCommunicateRepInvalidState,
    Invite4ClaimerCommunicateRepNotFound,
    Invite4ClaimerCommunicateRepOk,
    Invite4ClaimerCommunicateReq,
    Invite4GreeterCommunicateRep,
    Invite4GreeterCommunicateRepAlreadyDeleted,
    Invite4GreeterCommunicateRepInvalidState,
    Invite4GreeterCommunicateRepNotFound,
    Invite4GreeterCommunicateRepOk,
    Invite4GreeterCommunicateReq,
    InviteDeleteRep,
    InviteDeleteRepAlreadyDeleted,
    InviteDeleteRepNotFound,
    InviteDeleteRepOk,
    InviteDeleteReq,
    InviteInfoRep,
    InviteInfoRepOk,
    InviteInfoReq,
    InviteListItem,
    InviteListRep,
    InviteListRepOk,
    InviteListReq,
    InviteNewRep,
    InviteNewRepAlreadyMember,
    InviteNewRepNotAllowed,
    InviteNewRepNotAvailable,
    InviteNewRepOk,
    InviteNewReq,
)
from parsec.crypto import PublicKey, HashDigest
from parsec.event_bus import EventBus, EventCallback, EventFilterCallback
from parsec.api.protocol import (
    OrganizationID,
    UserID,
    HumanHandle,
    InvitationToken,
    InvitationDeletedReason,
    InvitationStatus,
    InvitationEmailSentStatus,
    UserProfile,
)
from parsec.api.protocol.base import api_typed_msg_adapter
from parsec.backend.backend_events import BackendEvent
from parsec.backend.templates import get_template
from parsec.backend.utils import catch_protocol_errors, api, ClientType
from parsec.backend.config import (
    BackendConfig,
    EmailConfig,
    SmtpEmailConfig,
    MockedEmailConfig,
)
from parsec._parsec import BackendInvitationAddr


logger = get_logger()


class CloseInviteConnection(Exception):
    pass


class InvitationError(Exception):
    pass


class InvitationNotFoundError(InvitationError):
    pass


class InvitationAlreadyDeletedError(InvitationError):
    pass


class InvitationInvalidStateError(InvitationError):
    pass


class InvitationAlreadyMemberError(InvitationError):
    pass


class InvitationEmailConfigError(InvitationError):
    pass


class InvitationEmailRecipientError(InvitationError):
    pass


class ConduitState(Enum):
    STATE_1_WAIT_PEERS = "1_WAIT_PEERS"
    STATE_2_1_CLAIMER_HASHED_NONCE = "2_1_CLAIMER_HASHED_NONCE"
    STATE_2_2_GREETER_NONCE = "2_2_GREETER_NONCE"
    STATE_2_3_CLAIMER_NONCE = "2_3_CLAIMER_NONCE"
    STATE_3_1_CLAIMER_TRUST = "3_1_CLAIMER_TRUST"
    STATE_3_2_GREETER_TRUST = "3_2_GREETER_TRUST"
    STATE_4_COMMUNICATE = "4_COMMUNICATE"


NEXT_CONDUIT_STATE = {
    ConduitState.STATE_1_WAIT_PEERS: ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
    ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE: ConduitState.STATE_2_2_GREETER_NONCE,
    ConduitState.STATE_2_2_GREETER_NONCE: ConduitState.STATE_2_3_CLAIMER_NONCE,
    ConduitState.STATE_2_3_CLAIMER_NONCE: ConduitState.STATE_3_1_CLAIMER_TRUST,
    ConduitState.STATE_3_1_CLAIMER_TRUST: ConduitState.STATE_3_2_GREETER_TRUST,
    ConduitState.STATE_3_2_GREETER_TRUST: ConduitState.STATE_4_COMMUNICATE,
    ConduitState.STATE_4_COMMUNICATE: ConduitState.STATE_4_COMMUNICATE,
}


@attr.s(slots=True, frozen=True, auto_attribs=True)
class ConduitListenCtx:
    organization_id: OrganizationID
    greeter: Optional[UserID]
    token: InvitationToken
    state: ConduitState
    payload: bytes
    peer_payload: Optional[bytes]

    @property
    def is_greeter(self):
        return self.greeter is not None


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserInvitation:
    TYPE = InvitationType.USER
    greeter_user_id: UserID
    greeter_human_handle: Optional[HumanHandle]
    claimer_email: str
    token: InvitationToken = attr.ib(factory=InvitationToken.new)
    created_on: DateTime = attr.ib(factory=DateTime.now)
    status: InvitationStatus = InvitationStatus.IDLE

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceInvitation:
    TYPE = InvitationType.DEVICE
    greeter_user_id: UserID
    greeter_human_handle: Optional[HumanHandle]
    token: InvitationToken = attr.ib(factory=InvitationToken.new)
    created_on: DateTime = attr.ib(factory=DateTime.now)
    status: InvitationStatus = InvitationStatus.IDLE

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


Invitation = Union[UserInvitation, DeviceInvitation]


def generate_invite_email(
    from_addr: str,
    to_addr: str,
    reply_to: Optional[str],
    greeter_name: Optional[str],  # Noe for device invitation
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
    if reply_to:
        message["Reply-To"] = reply_to

    # Turn parts into MIMEText objects
    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    # Add HTML/plain-text parts to MIMEMultipart message
    # The email client will try to render the last part first
    message.attach(part1)
    message.attach(part2)

    return message


async def _smtp_send_mail(email_config: SmtpEmailConfig, to_addr: str, message: Message) -> None:
    def _do():
        try:
            context = ssl.create_default_context()
            if email_config.use_ssl:
                server = smtplib.SMTP_SSL(email_config.host, email_config.port, context=context)
            else:
                server = smtplib.SMTP(email_config.host, email_config.port)

            with server:
                if email_config.use_tls and not email_config.use_ssl:
                    if server.starttls(context=context)[0] != 220:
                        logger.warning("Email TLS connexion isn't encrypted")
                if email_config.host_user and email_config.host_password:
                    server.login(email_config.host_user, email_config.host_password)
                server.sendmail(email_config.sender, to_addr, message.as_string())

        except smtplib.SMTPRecipientsRefused as e:
            raise InvitationEmailRecipientError from e
        except smtplib.SMTPException as e:
            logger.warning("SMTP error", exc_info=e, to_addr=to_addr, subject=message["Subject"])
            raise InvitationEmailConfigError from e

    await trio.to_thread.run_sync(_do)


async def _mocked_send_mail(
    email_config: MockedEmailConfig, to_addr: str, message: Message
) -> None:
    def _do():
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

    await trio.to_thread.run_sync(_do)


async def send_email(email_config: EmailConfig, to_addr: str, message: Message) -> None:
    if isinstance(email_config, SmtpEmailConfig):
        await _smtp_send_mail(email_config, to_addr, message)
    else:
        await _mocked_send_mail(email_config, to_addr, message)


class BaseInviteComponent:
    def __init__(self, event_bus: EventBus, config: BackendConfig):
        self._event_bus = event_bus
        self._config = config
        # We use the `invite.status_changed` event to keep a list of all the
        # invitation claimers connected accross all backends.
        #
        # This is useful to display the invitations ready to be greeted.
        # Note we rely on a per-backend list in memory instead of storing this
        # information in database so that we default to no claimer present
        # (which is the most likely when a backend is restarted) .
        #
        # However there are multiple ways this list can go out of sync:
        # - a claimer can be connected to a backend, then another backend starts
        # - the backend the claimer is connected to crashes witout being able
        #   to notify the other backends
        # - a claimer open multiple connections at the same time, then is
        #   considered disconnected as soon as he closes one of his connections
        #
        # This is considered "fine enough" given all the claimer has to do
        # to fix this is to retry a connection, which precisely the kind of
        # "I.T., have you tried to turn it off and on again ?" a human is
        # expected to do ;-)
        self._claimers_ready: Dict[OrganizationID, Set[InvitationToken]] = defaultdict(set)

        def _on_status_changed(
            event: BackendEvent,
            organization_id: OrganizationID,
            greeter: UserID,
            token: InvitationToken,
            status: InvitationStatus,
        ) -> None:
            if status == InvitationStatus.READY:
                self._claimers_ready[organization_id].add(token)
            else:  # Invitation deleted or back to idle
                self._claimers_ready[organization_id].discard(token)

        self._event_bus.connect(
            BackendEvent.INVITE_STATUS_CHANGED, cast(EventCallback, _on_status_changed)
        )

    @api("invite_new")
    @catch_protocol_errors
    @api_typed_msg_adapter(InviteNewReq, InviteNewRep)
    async def api_invite_new(self, client_ctx, req):
        # Define helper
        def _to_http_redirection_url(
            client_ctx, invitation: Union[UserInvitation, DeviceInvitation]
        ) -> str:
            assert self._config.backend_addr
            return BackendInvitationAddr.build(
                backend_addr=self._config.backend_addr,
                organization_id=client_ctx.organization_id,
                invitation_type=invitation.TYPE,
                token=invitation.token,
            ).to_http_redirection_url()

        # Create new user / new device
        if req.type == InvitationType.USER:
            if client_ctx.profile != UserProfile.ADMIN:
                return InviteNewRepNotAllowed()
            try:
                invitation = await self.new_for_user(
                    organization_id=client_ctx.organization_id,
                    greeter_user_id=client_ctx.user_id,
                    claimer_email=req.claimer_email,
                )
            except InvitationAlreadyMemberError:
                return InviteNewRepAlreadyMember()
        else:  # Device
            if req.send_email and not client_ctx.human_handle:
                return InviteNewRepNotAvailable()

            invitation = await self.new_for_device(
                organization_id=client_ctx.organization_id,
                greeter_user_id=client_ctx.user_id,
            )

        # No need to send email, we're done
        if not req.send_email:
            # Note: before parsec v2.13.0, we used to reply with a missing `email_sent` field in this case.
            # However, we'd rather limit the use of missing fields to compatibility use cases (e.g when a
            # field has been added in a new version but does not exist in older versions). In this case, we
            # can replace the missing field with `SUCCESS` without breaking compatibility with older clients
            # since they also choose `SUCCESS` as value when getting an `AttributeError` on the reply.
            return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.SUCCESS)

        # Backend address not configured, we won't be able to send the email
        if not self._config.backend_addr:
            return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.NOT_AVAILABLE)

        # Generate email message
        if req.type == InvitationType.USER:
            to_addr = invitation.claimer_email
            if client_ctx.human_handle:
                greeter_name = client_ctx.human_handle.label
                reply_to = f"{client_ctx.human_handle.label} <{client_ctx.human_handle.email}>"
            else:
                greeter_name = client_ctx.user_id.str
                reply_to = None
            message = generate_invite_email(
                from_addr=self._config.email_config.sender,
                to_addr=invitation.claimer_email,
                greeter_name=greeter_name,
                reply_to=reply_to,
                organization_id=client_ctx.organization_id,
                invitation_url=_to_http_redirection_url(client_ctx, invitation),
                backend_url=self._config.backend_addr.to_http_domain_url(),
            )
        else:  # Device
            to_addr = client_ctx.human_handle.email
            message = generate_invite_email(
                from_addr=self._config.email_config.sender,
                to_addr=client_ctx.human_handle.email,
                greeter_name=None,
                reply_to=None,
                organization_id=client_ctx.organization_id,
                invitation_url=_to_http_redirection_url(client_ctx, invitation),
                backend_url=self._config.backend_addr.to_http_domain_url(),
            )

        # Send the email
        try:
            await send_email(
                email_config=self._config.email_config,
                to_addr=to_addr,
                message=message,
            )
        except InvitationEmailRecipientError:
            return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.BAD_RECIPIENT)
        except InvitationEmailConfigError:
            return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.NOT_AVAILABLE)
        except Exception:
            # Fail-safe: since the device/user has been created, we don't want to fail too hard
            logger.exception("Unexpected exception while sending an email")
            return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.NOT_AVAILABLE)

        # The email has been successfully sent
        return InviteNewRepOk(invitation.token, InvitationEmailSentStatus.SUCCESS)

    @api("invite_delete")
    @catch_protocol_errors
    @api_typed_msg_adapter(InviteDeleteReq, InviteDeleteRep)
    async def api_invite_delete(self, client_ctx, msg):
        try:
            await self.delete(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                on=DateTime.now(),
                reason=msg.reason,
            )

        except InvitationNotFoundError:
            return InviteDeleteRepNotFound()

        except InvitationAlreadyDeletedError:
            return InviteDeleteRepAlreadyDeleted()

        return InviteDeleteRepOk()

    @api("invite_list")
    @catch_protocol_errors
    @api_typed_msg_adapter(InviteListReq, InviteListRep)
    async def api_invite_list(self, client_ctx, _):
        invitations = await self.list(
            organization_id=client_ctx.organization_id, greeter=client_ctx.user_id
        )

        return InviteListRepOk(
            [
                InviteListItem.User(item.token, item.created_on, item.claimer_email, item.status)
                if isinstance(item, UserInvitation)
                else InviteListItem.Device(item.token, item.created_on, item.status)
                for item in invitations
            ]
        )

    @api("invite_info", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(InviteInfoReq, InviteInfoRep)
    async def api_invite_info(self, client_ctx, _):
        # Invitation has already been fetched during handshake, this
        # means we don't have to access the database at all here.
        # Not accessing the database also means we cannot detect if invitation
        # has been deleted but it's no big deal given we don't modify anything !
        # (and the connection will eventually be closed by backend event anyway)
        invitation = client_ctx.invitation
        if isinstance(invitation, UserInvitation):
            return InviteInfoRepOk(
                InvitationType.USER,
                invitation.claimer_email,
                invitation.greeter_user_id,
                invitation.greeter_human_handle,
            )
        else:  # DeviceInvitation
            return InviteInfoRepOk(
                InvitationType.DEVICE,
                claimer_email=None,
                greeter_user_id=invitation.greeter_user_id,
                greeter_human_handle=invitation.greeter_human_handle,
            )

    @api(
        "invite_1_claimer_wait_peer",
        cancel_on_client_sending_new_cmd=True,
        client_types=[ClientType.INVITED],
    )
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite1ClaimerWaitPeerReq, Invite1ClaimerWaitPeerRep)
    async def api_invite_1_claimer_wait_peer(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            greeter_public_key = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_1_WAIT_PEERS,
                payload=msg.claimer_public_key.encode(),
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite1ClaimerWaitPeerRepNotFound()

        except InvitationInvalidStateError:
            return Invite1ClaimerWaitPeerRepInvalidState()

        return Invite1ClaimerWaitPeerRepOk(PublicKey(greeter_public_key))

    @api("invite_1_greeter_wait_peer")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite1GreeterWaitPeerReq, Invite1GreeterWaitPeerRep)
    async def api_invite_1_greeter_wait_peer(self, client_ctx, msg):
        try:
            claimer_public_key_raw = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_1_WAIT_PEERS,
                payload=msg.greeter_public_key.encode(),
            )

        except InvitationNotFoundError:
            return Invite1GreeterWaitPeerRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite1GreeterWaitPeerRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite1GreeterWaitPeerRepInvalidState()

        return Invite1GreeterWaitPeerRepOk(PublicKey(claimer_public_key_raw))

    @api(
        "invite_2a_claimer_send_hashed_nonce",
        client_types=[ClientType.INVITED],
    )
    @catch_protocol_errors
    @api_typed_msg_adapter(
        Invite2aClaimerSendHashedNonceReq,
        Invite2aClaimerSendHashedNonceRep,
    )
    async def api_invite_2a_claimer_send_hash_nonce(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
                payload=msg.claimer_hashed_nonce.digest,
            )

            greeter_nonce = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_2_2_GREETER_NONCE,
                payload=b"",
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite2aClaimerSendHashedNonceRepNotFound()

        except InvitationInvalidStateError:
            return Invite2aClaimerSendHashedNonceRepInvalidState()

        return Invite2aClaimerSendHashedNonceRepOk(greeter_nonce)

    @api("invite_2a_greeter_get_hashed_nonce")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite2aGreeterGetHashedNonceReq, Invite2aGreeterGetHashedNonceRep)
    async def api_invite_2a_greeter_get_hashed_nonce(self, client_ctx, msg):
        try:
            claimer_hashed_nonce = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_2_1_CLAIMER_HASHED_NONCE,
                payload=b"",
            )
            # Should not fail given data is check on DB insertion
            claimer_hashed_nonce = HashDigest(claimer_hashed_nonce)

        except InvitationNotFoundError:
            return Invite2aGreeterGetHashedNonceRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite2aGreeterGetHashedNonceRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite2aGreeterGetHashedNonceRepInvalidState()

        return Invite2aGreeterGetHashedNonceRepOk(claimer_hashed_nonce)

    @api("invite_2b_greeter_send_nonce")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite2bGreeterSendNonceReq, Invite2bGreeterSendNonceRep)
    async def api_invite_2b_greeter_send_nonce(self, client_ctx, msg):
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_2_2_GREETER_NONCE,
                payload=msg.greeter_nonce,
            )

            claimer_nonce = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_2_3_CLAIMER_NONCE,
                payload=b"",
            )

        except InvitationNotFoundError:
            return Invite2bGreeterSendNonceRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite2bGreeterSendNonceRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite2bGreeterSendNonceRepInvalidState()

        return Invite2bGreeterSendNonceRepOk(claimer_nonce)

    @api("invite_2b_claimer_send_nonce", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite2bClaimerSendNonceReq, Invite2bGreeterSendNonceRep)
    async def api_invite_2b_claimer_send_nonce(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_2_3_CLAIMER_NONCE,
                payload=msg.claimer_nonce,
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite2bGreeterSendNonceRepNotFound()

        except InvitationInvalidStateError:
            return Invite2bGreeterSendNonceRepInvalidState()

        return Invite2bClaimerSendNonceRepOk()

    @api("invite_3a_greeter_wait_peer_trust")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite3aGreeterWaitPeerTrustReq, Invite3aGreeterWaitPeerTrustRep)
    async def api_invite_3a_greeter_wait_peer_trust(self, client_ctx, msg):
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_3_1_CLAIMER_TRUST,
                payload=b"",
            )

        except InvitationNotFoundError:
            return Invite3aGreeterWaitPeerTrustRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite3aGreeterWaitPeerTrustRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite3aGreeterWaitPeerTrustRepInvalidState()

        return Invite3aGreeterWaitPeerTrustRepOk()

    @api("invite_3b_claimer_wait_peer_trust", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite3bClaimerWaitPeerTrustReq, Invite3bClaimerWaitPeerTrustRep)
    async def api_invite_3b_claimer_wait_peer_trust(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_3_2_GREETER_TRUST,
                payload=b"",
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite3bClaimerWaitPeerTrustRepNotFound()

        except InvitationInvalidStateError:
            return Invite3bClaimerWaitPeerTrustRepInvalidState()

        return Invite3bClaimerWaitPeerTrustRepOk()

    @api("invite_3b_greeter_signify_trust")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite3bGreeterSignifyTrustReq, Invite3bGreeterSignifyTrustRep)
    async def api_invite_3b_greeter_signify_trust(self, client_ctx, msg):
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_3_2_GREETER_TRUST,
                payload=b"",
            )

        except InvitationNotFoundError:
            return Invite3bGreeterSignifyTrustRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite3bGreeterSignifyTrustRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite3bGreeterSignifyTrustRepInvalidState()

        return Invite3bGreeterSignifyTrustRepOk()

    @api("invite_3a_claimer_signify_trust", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite3aClaimerSignifyTrustReq, Invite3aClaimerSignifyTrustRep)
    async def api_invite_3a_claimer_signify_trust(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_3_1_CLAIMER_TRUST,
                payload=b"",
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite3aClaimerSignifyTrustRepNotFound()

        except InvitationInvalidStateError:
            return Invite3aClaimerSignifyTrustRepInvalidState()

        return Invite3aClaimerSignifyTrustRepOk()

    @api("invite_4_greeter_communicate")
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite4GreeterCommunicateReq, Invite4GreeterCommunicateRep)
    async def api_invite_4_greeter_communicate(self, client_ctx, msg):
        try:
            answer_payload = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=client_ctx.user_id,
                token=msg.token,
                state=ConduitState.STATE_4_COMMUNICATE,
                payload=msg.payload,
            )

        except InvitationNotFoundError:
            return Invite4GreeterCommunicateRepNotFound()

        except InvitationAlreadyDeletedError:
            return Invite4GreeterCommunicateRepAlreadyDeleted()

        except InvitationInvalidStateError:
            return Invite4GreeterCommunicateRepInvalidState()

        return Invite4GreeterCommunicateRepOk(answer_payload)

    @api("invite_4_claimer_communicate", client_types=[ClientType.INVITED])
    @catch_protocol_errors
    @api_typed_msg_adapter(Invite4ClaimerCommunicateReq, Invite4ClaimerCommunicateRep)
    async def api_invite_4_claimer_communicate(self, client_ctx, msg):
        """
        Raises:
            CloseInviteConnection
        """
        try:
            answer_payload = await self.conduit_exchange(
                organization_id=client_ctx.organization_id,
                greeter=None,
                token=client_ctx.invitation.token,
                state=ConduitState.STATE_4_COMMUNICATE,
                payload=msg.payload,
            )

        except InvitationAlreadyDeletedError as exc:
            # Notify parent that the connection shall be close because the invitation token is no longer valid.
            raise CloseInviteConnection from exc

        except InvitationNotFoundError:
            return Invite4ClaimerCommunicateRepNotFound()

        except InvitationInvalidStateError:
            return Invite4ClaimerCommunicateRepInvalidState()

        return Invite4ClaimerCommunicateRepOk(answer_payload)

    async def conduit_exchange(
        self,
        organization_id: OrganizationID,
        greeter: Optional[UserID],
        token: InvitationToken,
        state: ConduitState,
        payload: bytes,
    ) -> bytes:
        # Conduit exchange is done in two steps:
        # First we "talk" by providing our payload and retrieve the peer's
        # payload if he has talked prior to us.
        # Then we "listen" by waiting for the peer to provide his payload if we
        # have talked first, or to confirm us it has received our payload if we
        # have talked after him.
        filter_organization_id = organization_id
        filter_token = token

        def _event_filter(
            event: Enum,
            organization_id: OrganizationID,
            token: InvitationToken,
            **kwargs,
        ):
            return organization_id == filter_organization_id and token == filter_token

        with self._event_bus.waiter_on_first(
            BackendEvent.INVITE_CONDUIT_UPDATED,
            BackendEvent.INVITE_STATUS_CHANGED,
            filter=cast(EventFilterCallback, _event_filter),
        ) as waiter:

            listen_ctx = await self._conduit_talk(organization_id, greeter, token, state, payload)

            # Unlike what it name may imply, `_conduit_listen` doesn't wait for the peer
            # to answer (it returns `None` instead), so we wait for some events to occure
            # before calling:
            # - INVITE_CONDUIT_UPDATED: Triggered when the peer has completed it own talk
            #   step, `_conduit_listen` will most likely return the peer payload now
            # - INVITE_STATUS_CHANGED: Triggered if the peer reset the invitation or if the
            #   invitation has been deleted, in any case `_conduit_listen` will detect the
            #   listen is not longer possible and raise an exception accordingly
            while True:
                await waiter.wait()
                waiter.clear()
                peer_payload = await self._conduit_listen(listen_ctx)
                if peer_payload is not None:
                    return peer_payload

    async def _conduit_talk(
        self,
        organization_id: OrganizationID,
        greeter: Optional[UserID],  # None for claimer
        token: InvitationToken,
        state: ConduitState,
        payload: bytes,
    ) -> ConduitListenCtx:
        """
        Raises:
            InvitationNotFoundError
            InvitationAlreadyDeletedError
            InvitationInvalidStateError
        """
        raise NotImplementedError()

    async def _conduit_listen(self, ctx: ConduitListenCtx) -> Optional[bytes]:
        """
        Returns ``None`` is listen is still needed
        Raises:
            InvitationNotFoundError
            InvitationAlreadyDeletedError
            InvitationInvalidStateError
        """
        raise NotImplementedError()

    async def new_for_user(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        claimer_email: str,
        created_on: Optional[DateTime] = None,
    ) -> UserInvitation:
        """
        Raise: Nothing
        """
        raise NotImplementedError()

    async def new_for_device(
        self,
        organization_id: OrganizationID,
        greeter_user_id: UserID,
        created_on: Optional[DateTime] = None,
    ) -> DeviceInvitation:
        """
        Raise: Nothing
        """
        raise NotImplementedError()

    async def delete(
        self,
        organization_id: OrganizationID,
        greeter: UserID,
        token: InvitationToken,
        on: DateTime,
        reason: InvitationDeletedReason,
    ) -> None:
        """
        Raises:
            InvitationNotFoundError
            InvitationAlreadyDeletedError
        """
        raise NotImplementedError()

    async def list(self, organization_id: OrganizationID, greeter: UserID) -> List[Invitation]:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def info(self, organization_id: OrganizationID, token: InvitationToken) -> Invitation:
        """
        Raises:
            InvitationNotFoundError
            InvitationAlreadyDeletedError
        """
        raise NotImplementedError()

    async def claimer_joined(
        self, organization_id: OrganizationID, greeter: UserID, token: InvitationToken
    ) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def claimer_left(
        self, organization_id: OrganizationID, greeter: UserID, token: InvitationToken
    ) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()
