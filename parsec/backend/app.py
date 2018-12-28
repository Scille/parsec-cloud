import trio
import attr
from uuid import uuid4
from structlog import get_logger

from parsec.event_bus import EventBus
from parsec.api.transport import TransportError, ServerTransportFactory
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


logger = get_logger()


@attr.s
class LoggedClientContext:
    anonymous = False
    device_id = attr.ib()
    public_key = attr.ib()
    verify_key = attr.ib()
    conn_id = attr.ib(factory=lambda: uuid4().hex)
    subscribed_events = attr.ib(factory=dict, init=False)
    events = attr.ib(factory=lambda: trio.Queue(100), init=False)

    def __attrs_post_init__(self):
        self.logger = logger.bind(client_id=str(self.device_id), conn_id=self.conn_id)

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def device_name(self):
        return self.device_id.device_name


@attr.s
class AnonymousClientContext:
    device_id = None
    anonymous = True
    logger = logger


def _filter_binary_fields(data):
    return {k: v if not isinstance(v, bytes) else b"[...]" for k, v in data.items()}


class BackendApp:
    def __init__(self, config, event_bus=None):
        self.event_bus = event_bus or EventBus()
        self.config = config
        self.nursery = None
        self.dbh = None
        self.events = EventsComponent(self.event_bus)

        self.transport_factory = ServerTransportFactory(
            config.transport_scheme, certfile=config.certfile, keyfile=config.keyfile
        )

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
            "organization_create": self.organization.api_organization_create,
            "organization_bootstrap": self.organization.api_organization_bootstrap,
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
                context = AnonymousClientContext()
                result_req = hs.build_result_req()

            else:
                try:
                    user = await self.user.get_user(hs.identity.user_id)
                    device = user.devices[hs.identity.device_name]

                except (UserNotFoundError, KeyError):
                    result_req = hs.build_bad_identity_result_req()

                else:
                    if device.revocated_on:
                        result_req = hs.build_revoked_device_result_req()

                    else:
                        context = LoggedClientContext(
                            hs.identity, user.public_key, device.verify_key
                        )
                        result_req = hs.build_result_req(device.verify_key)

        except ProtocoleError:
            result_req = hs.build_bad_format_result_req()

        await transport.send(result_req)
        return context

    async def handle_client(self, stream, swallow_crash=False):
        transport = await self.transport_factory.wrap_with_transport(stream)
        client_ctx = None
        try:
            logger.debug("start handshake")
            client_ctx = await self._do_handshake(transport)
            if not client_ctx:
                # Invalid handshake
                logger.debug("bad handshake")
                return

            client_ctx.logger.debug("handshake done")

            await self._handle_client_loop(transport, client_ctx)

        except (TransportError, MessageSerializationError) as exc:
            # Client has closed connection or sent an invalid trame
            rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
            try:
                await transport.send(packb(rep))
            except TransportError:
                pass
            await transport.aclose()

        except Exception as exc:
            # If we are here, something unexpected happened...
            if client_ctx:
                client_ctx.logger.error("Unexpected crash", exc_info=exc)
            else:
                logger.error("Unexpected crash", exc_info=exc)
            await transport.aclose()
            if not swallow_crash:
                raise

    async def _handle_client_loop(self, transport, client_ctx):
        while True:
            raw_req = await transport.recv()
            if not raw_req:  # Client disconnected
                client_ctx.logger.debug("CLIENT DISCONNECTED")
                break

            req = unpackb(raw_req)
            client_ctx.logger.debug("req", req=_filter_binary_fields(req))
            try:
                cmd = req.get("cmd", "<missing>")
                if not isinstance(cmd, str):
                    raise KeyError()
                if client_ctx.anonymous:
                    cmd_func = self.anonymous_cmds[cmd]
                else:
                    cmd_func = self.logged_cmds[cmd]

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

                except ProtocoleError as exc:
                    rep = {"status": "bad_message", "reason": str(exc)}

            client_ctx.logger.debug("rep", rep=_filter_binary_fields(rep))
            raw_rep = packb(rep)
            await transport.send(raw_rep)
