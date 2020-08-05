# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
import trio
from structlog import get_logger
from logging import DEBUG as LOG_LEVEL_DEBUG
from async_generator import asynccontextmanager
import h11

from parsec._version import __version__ as parsec_version
from parsec.backend.backend_events import BackendEvent
from parsec.event_bus import EventBus
from parsec.logging import get_log_level
from parsec.api.transport import TransportError, TransportClosedByPeer, Transport
from parsec.api.protocol import (
    packb,
    unpackb,
    ProtocolError,
    MessageSerializationError,
    InvalidMessageError,
    InvitationStatus,
)
from parsec.backend.utils import CancelledByNewRequest, collect_apis
from parsec.backend.config import BackendConfig
from parsec.backend.client_context import AuthenticatedClientContext, InvitedClientContext
from parsec.backend.handshake import do_handshake
from parsec.backend.memory import components_factory as mocked_components_factory
from parsec.backend.postgresql import components_factory as postgresql_components_factory
from parsec.backend.http import HTTPRequest
from parsec.backend.invite import CloseInviteConnection


logger = get_logger()


def _filter_binary_fields(data):
    return {k: v if not isinstance(v, bytes) else b"[...]" for k, v in data.items()}


@asynccontextmanager
async def backend_app_factory(config: BackendConfig, event_bus: Optional[EventBus] = None):
    event_bus = event_bus or EventBus()

    if config.db_url == "MOCKED":
        components_factory = mocked_components_factory
    else:
        components_factory = postgresql_components_factory

    async with components_factory(config=config, event_bus=event_bus) as components:
        yield BackendApp(
            config=config,
            event_bus=event_bus,
            webhooks=components["webhooks"],
            http=components["http"],
            user=components["user"],
            invite=components["invite"],
            organization=components["organization"],
            message=components["message"],
            realm=components["realm"],
            vlob=components["vlob"],
            ping=components["ping"],
            blockstore=components["blockstore"],
            block=components["block"],
            events=components["events"],
        )


class BackendApp:
    def __init__(
        self,
        config,
        event_bus,
        webhooks,
        http,
        user,
        invite,
        organization,
        message,
        realm,
        vlob,
        ping,
        blockstore,
        block,
        events,
    ):
        self.config = config
        self.event_bus = event_bus

        self.webhooks = webhooks
        self.http = http
        self.user = user
        self.invite = invite
        self.organization = organization
        self.message = message
        self.realm = realm
        self.vlob = vlob
        self.ping = ping
        self.blockstore = blockstore
        self.block = block
        self.events = events

        self.apis = collect_apis(
            user, invite, organization, message, realm, vlob, ping, blockstore, block, events
        )

    async def handle_client_websocket(self, stream, event, first_request_data=None):
        selected_logger = logger

        try:
            transport = await Transport.init_for_server(
                stream, first_request_data=first_request_data
            )

        except TransportClosedByPeer as exc:
            selected_logger.info("Connection dropped: client has left", reason=str(exc))
            return

        except TransportError as exc:
            selected_logger.info("Connection dropped: websocket error", reason=str(exc))
            return

        selected_logger = transport.logger

        try:
            client_ctx, error_infos = await do_handshake(self, transport)
            if not client_ctx:
                # Invalid handshake
                # TODO Fragile test based on reason, make it more robust
                if error_infos and error_infos.get("reason", "") == "Expired organization":
                    organization_id = error_infos["organization_id"]
                    await self.events.send(
                        BackendEvent.ORGANIZATION_EXPIRED, organization_id=organization_id
                    )
                selected_logger.info("Connection dropped: bad handshake", **error_infos)
                return

            selected_logger = client_ctx.logger
            selected_logger.info("Connection established")

            if isinstance(client_ctx, AuthenticatedClientContext):
                with trio.CancelScope() as cancel_scope:
                    with self.event_bus.connection_context() as client_ctx.event_bus_ctx:

                        def _on_revoked(event, organization_id, user_id):
                            if (
                                organization_id == client_ctx.organization_id
                                and user_id == client_ctx.user_id
                            ):
                                cancel_scope.cancel()

                        def _on_expired(event, organization_id):
                            if organization_id == client_ctx.organization_id:
                                cancel_scope.cancel()

                        client_ctx.event_bus_ctx.connect(BackendEvent.USER_REVOKED, _on_revoked)
                        client_ctx.event_bus_ctx.connect(
                            BackendEvent.ORGANIZATION_EXPIRED, _on_expired
                        )
                        await self._handle_client_loop(transport, client_ctx)

            elif isinstance(client_ctx, InvitedClientContext):
                await self.invite.claimer_joined(
                    organization_id=client_ctx.organization_id,
                    greeter=client_ctx.invitation.greeter_user_id,
                    token=client_ctx.invitation.token,
                )
                try:
                    with trio.CancelScope() as cancel_scope:
                        with self.event_bus.connection_context() as event_bus_ctx:

                            def _on_invite_status_changed(
                                event, organization_id, greeter, token, status
                            ):
                                if (
                                    status == InvitationStatus.DELETED
                                    and organization_id == client_ctx.organization_id
                                    and token == client_ctx.invitation.token
                                ):
                                    cancel_scope.cancel()

                            event_bus_ctx.connect(
                                BackendEvent.INVITE_STATUS_CHANGED, _on_invite_status_changed
                            )
                            await self._handle_client_loop(transport, client_ctx)

                except CloseInviteConnection:
                    # If the invitation has been deleted after the invited handshake,
                    # invitation commands can raise an InvitationAlreadyDeletedError.
                    # This error is converted to a CloseInviteConnection
                    # The connection shall be closed due to a BackendEvent.INVITE_STATUS_CHANGED
                    # but nothing garantie that the event will be handled before cmd_func
                    # errors returns. If this happen, let's also close the connection
                    pass

                finally:
                    with trio.CancelScope(shield=True):
                        await self.invite.claimer_left(
                            organization_id=client_ctx.organization_id,
                            greeter=client_ctx.invitation.greeter_user_id,
                            token=client_ctx.invitation.token,
                        )

            else:
                await self._handle_client_loop(transport, client_ctx)

            await transport.aclose()

        except TransportClosedByPeer as exc:
            selected_logger.info("Connection dropped: client has left", reason=str(exc))

        except (TransportError, MessageSerializationError) as exc:
            rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
            try:
                await transport.send(packb(rep))
            except TransportError:
                pass
            await transport.aclose()
            selected_logger.info("Connection dropped: invalid data", reason=str(exc))

    async def handle_client(self, stream):
        MAX_RECV = 5
        try:
            conn = h11.Connection(h11.SERVER)
            first_request_data = b""

            while True:
                if conn.they_are_waiting_for_100_continue:
                    self.info("Sending 100 Continue")
                    go_ahead = h11.InformationalResponse(
                        status_code=100, headers=self.basic_headers()
                    )
                    await self.send(go_ahead)
                try:
                    data = await stream.receive_some(MAX_RECV)
                    first_request_data += data

                except ConnectionError:
                    # They've stopped listening. Not much we can do about it here.
                    data = b""
                conn.receive_data(data)

                event = conn.next_event()
                if event is not h11.NEED_DATA:
                    break

            if not isinstance(event, h11.Request):
                await stream.aclose()
                return

            # Websocket upgrade, else HTTP
            if (b"connection", b"Upgrade") in event.headers:
                await self.handle_client_websocket(
                    stream, event, first_request_data=first_request_data
                )
            else:
                await self.handle_client_http(stream, event, conn)

        except h11.RemoteProtocolError:
            # Peer is drunk...
            pass

        finally:
            try:
                await stream.aclose()
            except trio.BrokenResourceError:
                # They're already gone, nothing to do
                pass

    async def handle_client_http(self, stream, event, conn):
        # TODO: right now we handle a single request then close the connection
        # hence HTTP 1.1 keep-alive is not supported
        req = HTTPRequest.from_h11_req(event)
        rep = await self.http.handle_request(req)

        if self.config.debug:
            server_header = f"parsec/{parsec_version} {h11.PRODUCT_ID}"
        else:
            server_header = "parsec"
        rep.headers.append(("server", server_header))

        # TODO: Specify reason ? (currently we have `HTTP/1.1 200 \r\n` instead of `HTTP/1.1 200 OK\r\n`)
        try:
            await stream.send_all(
                conn.send(
                    h11.Response(
                        status_code=rep.status_code, headers=rep.headers, reason=rep.reason
                    )
                )
            )
            if rep.data:
                await stream.send_all(conn.send(h11.Data(data=rep.data)))
            await stream.send_all(conn.send(h11.EndOfMessage()))
        except trio.BrokenResourceError:
            # Peer is already gone, nothing to do
            pass

    async def _handle_client_loop(self, transport, client_ctx):
        # Retrieve the allowed commands according to api version and auth type
        api_cmds = self.apis[client_ctx.handshake_type]

        raw_req = None
        while True:
            # raw_req can be already defined if we received a new request
            # while processing a command
            raw_req = raw_req or await transport.recv()
            req = unpackb(raw_req)
            if get_log_level() <= LOG_LEVEL_DEBUG:
                client_ctx.logger.debug("Request", req=_filter_binary_fields(req))
            try:
                cmd = req.get("cmd", "<missing>")
                if not isinstance(cmd, str):
                    raise KeyError()

                cmd_func = api_cmds[cmd]

            except KeyError:
                rep = {"status": "unknown_command", "reason": "Unknown command"}

            else:
                try:
                    rep = await cmd_func(client_ctx, req)

                except InvalidMessageError as exc:
                    rep = {
                        "status": "bad_message",
                        "errors": exc.errors,
                        "reason": "Invalid message.",
                    }

                except ProtocolError as exc:
                    rep = {"status": "bad_message", "reason": str(exc)}

                except CancelledByNewRequest as exc:
                    # Long command handling such as message_get can be cancelled
                    # when the peer send a new request
                    raw_req = exc.new_raw_req
                    continue

            if get_log_level() <= LOG_LEVEL_DEBUG:
                client_ctx.logger.debug("Response", rep=_filter_binary_fields(rep))
            else:
                client_ctx.logger.info("Request", cmd=cmd, status=rep["status"])
            raw_rep = packb(rep)
            await transport.send(raw_rep)
            raw_req = None
