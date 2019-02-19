# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import attr
from structlog import get_logger

from parsec.event_bus import EventBus
from parsec.api.transport import TransportError, TransportClosedByPeer, Transport
from parsec.api.protocole import (
    packb,
    unpackb,
    ProtocoleError,
    MessageSerializationError,
    InvalidMessageError,
    ServerHandshake,
)
from parsec.backend.events import EventsComponent
from parsec.backend.utils import check_anonymous_api_allowed
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.drivers.memory import (
    MemoryOrganizationComponent,
    MemoryUserComponent,
    MemoryVlobComponent,
    MemoryMessageComponent,
    MemoryBeaconComponent,
    MemoryPingComponent,
)
from parsec.backend.drivers.postgresql import (
    PGHandler,
    PGOrganizationComponent,
    PGUserComponent,
    PGVlobComponent,
    PGMessageComponent,
    PGBeaconComponent,
    PGPingComponent,
)
from parsec.backend.user import UserNotFoundError
from parsec.backend.organization import OrganizationNotFoundError


logger = get_logger()


@attr.s
class LoggedClientContext:
    transport = attr.ib()
    organization_id = attr.ib()
    device_id = attr.ib()
    public_key = attr.ib()
    verify_key = attr.ib()
    event_bus_ctx = attr.ib(default=None)
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    channels = attr.ib(factory=lambda: trio.open_memory_channel(100))

    def __attrs_post_init__(self):
        self.conn_id = self.transport.conn_id
        self.logger = self.transport.logger = self.transport.logger.bind(
            organization_id=str(self.organization_id), client_id=str(self.device_id)
        )

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def send_events_channel(self):
        send_channel, _ = self.channels
        return send_channel

    @property
    def receive_events_channel(self):
        _, receive_channel = self.channels
        return receive_channel


@attr.s
class AnonymousClientContext:
    transport = attr.ib()
    organization_id = attr.ib()
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    device_id = None

    def __attrs_post_init__(self):
        self.conn_id = self.transport.conn_id
        self.logger = self.transport.logger = self.transport.logger.bind(
            organization_id=str(self.organization_id), client_id="<anonymous>"
        )


@attr.s
class AdministratorClientContext:
    transport = attr.ib()
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    device_id = None

    def __attrs_post_init__(self):
        self.conn_id = self.transport.conn_id
        self.logger = self.transport.logger = self.transport.logger.bind(
            client_id="<administrator>"
        )


def _filter_binary_fields(data):
    return {k: v if not isinstance(v, bytes) else b"[...]" for k, v in data.items()}


class BackendApp:
    def __init__(self, config, event_bus=None):
        self.event_bus = event_bus or EventBus()
        self.config = config
        self.nursery = None
        self.dbh = None
        self.events = EventsComponent(self.event_bus)

        if self.config.db_url == "MOCKED":
            self.user = MemoryUserComponent(self.event_bus)
            self.organization = MemoryOrganizationComponent(self.user)
            self.message = MemoryMessageComponent(self.event_bus)
            self.beacon = MemoryBeaconComponent(self.event_bus)
            self.vlob = MemoryVlobComponent(self.event_bus, self.beacon)
            self.ping = MemoryPingComponent(self.event_bus)
            self.blockstore = blockstore_factory(self.config.blockstore_config)

        else:
            self.dbh = PGHandler(self.config.db_url, self.event_bus)
            self.user = PGUserComponent(self.dbh, self.event_bus)
            self.organization = PGOrganizationComponent(self.dbh, self.user)
            self.message = PGMessageComponent(self.dbh)
            self.beacon = PGBeaconComponent(self.dbh)
            self.vlob = PGVlobComponent(self.dbh, self.beacon)
            self.ping = PGPingComponent(self.dbh)
            self.blockstore = blockstore_factory(
                self.config.blockstore_config, postgresql_dbh=self.dbh
            )

        self.logged_cmds = {
            "events_subscribe": self.events.api_events_subscribe,
            "events_listen": self.events.api_events_listen,
            "ping": self.ping.api_ping,
            "beacon_read": self.beacon.api_beacon_read,
            # Message
            "message_get": self.message.api_message_get,
            "message_send": self.message.api_message_send,
            # User&Device
            "user_get": self.user.api_user_get,
            "user_find": self.user.api_user_find,
            "user_invite": self.user.api_user_invite,
            "user_cancel_invitation": self.user.api_user_cancel_invitation,
            "user_create": self.user.api_user_create,
            "device_invite": self.user.api_device_invite,
            "device_cancel_invitation": self.user.api_device_cancel_invitation,
            "device_create": self.user.api_device_create,
            "device_revoke": self.user.api_device_revoke,
            # Blockstore
            "blockstore_create": self.blockstore.api_blockstore_create,
            "blockstore_read": self.blockstore.api_blockstore_read,
            # Vlob
            "vlob_group_check": self.vlob.api_vlob_group_check,
            "vlob_create": self.vlob.api_vlob_create,
            "vlob_read": self.vlob.api_vlob_read,
            "vlob_update": self.vlob.api_vlob_update,
        }
        self.anonymous_cmds = {
            "user_claim": self.user.api_user_claim,
            "user_get_invitation_creator": self.user.api_user_get_invitation_creator,
            "device_claim": self.user.api_device_claim,
            "device_get_invitation_creator": self.user.api_device_get_invitation_creator,
            "organization_bootstrap": self.organization.api_organization_bootstrap,
            "ping": self.ping.api_ping,
        }
        self.administration_cmds = {
            "organization_create": self.organization.api_organization_create,
            "ping": self.ping.api_ping,
        }
        for fn in self.anonymous_cmds.values():
            check_anonymous_api_allowed(fn)

    async def init(self, nursery):
        self.nursery = nursery
        if self.dbh:
            await self.dbh.init(nursery)

    async def teardown(self):
        if self.dbh:
            await self.dbh.teardown()

    async def _do_handshake(self, transport):
        context = None
        try:
            hs = ServerHandshake(self.config.handshake_challenge_size)
            challenge_req = hs.build_challenge_req()
            await transport.send(challenge_req)
            answer_req = await transport.recv()

            hs.process_answer_req(answer_req)

            if hs.is_anonymous():

                if hs.organization_id == self.config.administrator_token:
                    context = AdministratorClientContext(transport)
                    result_req = hs.build_result_req()

                else:
                    try:
                        organization = await self.organization.get(hs.organization_id)

                    except OrganizationNotFoundError:
                        result_req = hs.build_bad_identity_result_req()

                    else:
                        if (
                            hs.root_verify_key
                            and organization.root_verify_key != hs.root_verify_key
                        ):
                            result_req = hs.build_rvk_mismatch_result_req()

                        else:
                            context = AnonymousClientContext(transport, hs.organization_id)
                            result_req = hs.build_result_req()

            else:
                try:
                    organization = await self.organization.get(hs.organization_id)
                    user = await self.user.get_user(hs.organization_id, hs.device_id.user_id)
                    device = user.devices[hs.device_id.device_name]

                except (OrganizationNotFoundError, UserNotFoundError, KeyError):
                    result_req = hs.build_bad_identity_result_req()

                else:
                    if organization.root_verify_key != hs.root_verify_key:
                        result_req = hs.build_rvk_mismatch_result_req()

                    elif device.revocated_on:
                        result_req = hs.build_revoked_device_result_req()

                    else:
                        context = LoggedClientContext(
                            transport,
                            hs.organization_id,
                            hs.device_id,
                            user.public_key,
                            device.verify_key,
                        )
                        result_req = hs.build_result_req(device.verify_key)

        except ProtocoleError:
            result_req = hs.build_bad_format_result_req()

        await transport.send(result_req)
        return context

    async def handle_client(self, stream):
        try:
            transport = await Transport.init_for_server(stream)

        except TransportClosedByPeer:
            return

        except TransportError:
            # A crash during transport setup could mean the client tried to
            # access us from a web browser (hence sending http request).

            content_body = b"This service requires use of the WebSocket protocol"
            content = (
                b"HTTP/1.1 426 OK\r\n"
                b"Upgrade: WebSocket\r\n"
                b"Content-Length: %d\r\n"
                b"Connection: Upgrade\r\n"
                b"Content-Type: text/html; charset=UTF-8\r\n"
                b"\r\n"
            ) % len(content_body)

            try:
                await stream.send_all(content + content_body)
                await stream.aclose()

            except TransportError:
                # Stream is really dead, nothing else to do...
                pass

            return

        transport.logger.info("Client joined")

        try:
            transport.logger.debug("start handshake")
            client_ctx = await self._do_handshake(transport)
            if not client_ctx:
                # Invalid handshake
                logger.debug("bad handshake")
                return

            client_ctx.logger.debug("handshake done")

            with self.event_bus.connection_context() as client_ctx.event_bus_ctx:

                await self._handle_client_loop(transport, client_ctx)

        except TransportClosedByPeer:
            transport.logger.info("Client has left")
            return

        except (TransportError, MessageSerializationError):
            transport.logger.info("Close client connection due to invalid data")
            rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
            try:
                await transport.send(packb(rep))
            except TransportError:
                pass
            await transport.aclose()

    async def _handle_client_loop(self, transport, client_ctx):
        transport.logger.info("Client handshake done")
        while True:
            raw_req = await transport.recv()
            req = unpackb(raw_req)
            client_ctx.logger.debug("req", req=_filter_binary_fields(req))
            try:
                cmd = req.get("cmd", "<missing>")
                if not isinstance(cmd, str):
                    raise KeyError()

                if isinstance(client_ctx, AdministratorClientContext):
                    cmd_func = self.administration_cmds[cmd]

                elif isinstance(client_ctx, LoggedClientContext):
                    cmd_func = self.logged_cmds[cmd]

                else:
                    cmd_func = self.anonymous_cmds[cmd]

            except KeyError:
                client_ctx.logger.info("Invalid request", bad_cmd=cmd)
                rep = {"status": "unknown_command", "reason": "Unknown command"}

            else:
                client_ctx.logger.info("Request", cmd=cmd)
                try:
                    rep = await cmd_func(client_ctx, req)

                except InvalidMessageError as exc:
                    rep = {
                        "status": "bad_message",
                        "errors": exc.errors,
                        "reason": "Invalid message.",
                    }

                except ProtocoleError as exc:
                    rep = {"status": "bad_message", "reason": str(exc)}

            client_ctx.logger.debug("rep", rep=_filter_binary_fields(rep))
            raw_rep = packb(rep)
            await transport.send(raw_rep)
