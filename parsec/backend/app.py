import attr
import trio
from uuid import uuid4
from structlog import get_logger

from parsec.types import DeviceID
from parsec.utils import ParsecError
from parsec.event_bus import EventBus
from parsec.schema import BaseCmdSchema, fields
from parsec.api.transport import PatateTCPTransport, TransportError
from parsec.api.protocole import HandshakeFormatError, ServerHandshake
from parsec.backend.exceptions import BackendAuthError
from parsec.backend.events import EventsComponent
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.drivers.memory import (
    MemoryUserComponent,
    MemoryVlobComponent,
    MemoryMessageComponent,
    MemoryBeaconComponent,
)
from parsec.backend.drivers.postgresql import (
    PGHandler,
    PGUserComponent,
    PGVlobComponent,
    PGMessageComponent,
    PGBeaconComponent,
)

from parsec.backend.exceptions import NotFoundError
from parsec.backend.utils import check_anonymous_api_allowed, anonymous_api


logger = get_logger()


class _cmd_PING_Schema(BaseCmdSchema):
    ping = fields.String(required=True)


cmd_PING_Schema = _cmd_PING_Schema()


@attr.s
class AnonymousClientContext:
    id = "anonymous"
    anonymous = True
    logger = logger


@attr.s
class ClientContext:
    anonymous = False
    # TODO: rename to device_id
    id = attr.ib()
    # TODO: rename to public_key
    broadcast_key = attr.ib()
    verify_key = attr.ib()
    conn_id = attr.ib(factory=lambda: uuid4().hex)
    subscribed_events = attr.ib(factory=dict, init=False)
    events = attr.ib(factory=lambda: trio.Queue(100), init=False)

    def __attrs_post_init__(self):
        self.logger = logger.bind(client_id=self.id, conn_id=self.conn_id)

    @property
    def user_id(self):
        return self.id.user_id

    @property
    def device_id(self):
        return self.id

    @property
    def device_name(self):
        return self.id.device_name


class BackendApp:
    def __init__(self, config, event_bus=None):
        self.event_bus = event_bus or EventBus()
        self.config = config
        self.nursery = None
        self.dbh = None
        self.events = EventsComponent(self.event_bus)

        if self.config.db_url == "MOCKED":
            self.user = MemoryUserComponent(config.root_verify_key, self.event_bus)
            self.message = MemoryMessageComponent(self.event_bus)
            self.beacon = MemoryBeaconComponent(self.event_bus)
            self.vlob = MemoryVlobComponent(self.event_bus, self.beacon)

            self.blockstore = blockstore_factory(self.config.blockstore_config)

        else:
            self.dbh = PGHandler(self.config.db_url, self.event_bus)
            self.user = PGUserComponent(self.dbh, self.event_bus)
            self.message = PGMessageComponent(self.dbh, self.event_bus)
            self.beacon = PGBeaconComponent(self.dbh, self.event_bus)
            self.vlob = PGVlobComponent(self.dbh, self.event_bus, self.beacon)

            self.blockstore = blockstore_factory(
                self.config.blockstore_config, postgresql_dbh=self.dbh
            )

        self.anonymous_cmds = {
            "user_claim": self.user.api_user_claim,
            "user_get_invitation_creator": self.user.api_user_get_invitation_creator,
            "device_claim": self.user.api_device_claim,
            "device_get_invitation_creator": self.user.api_device_get_invitation_creator,
            "ping": self._api_ping,
        }
        for fn in self.anonymous_cmds.values():
            check_anonymous_api_allowed(fn)

        self.cmds = {
            "events_subscribe": self.events.api_events_subscribe,
            "events_listen": self.events.api_events_listen,
            "ping": self._api_ping,
            "beacon_read": self.beacon.api_beacon_read,
            # Message
            "message_get": self.message.api_message_get,
            "message_new": self.message.api_message_new,
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

    async def init(self, nursery):
        self.nursery = nursery
        if self.dbh:
            await self.dbh.init(nursery)

    async def teardown(self):
        if self.dbh:
            await self.dbh.teardown()

    @anonymous_api
    async def _api_ping(self, client_ctx, msg):
        msg = cmd_PING_Schema.load_or_abort(msg)
        if self.dbh:
            await self.dbh.ping(author=client_ctx.id, ping=msg["ping"])
        else:
            self.event_bus.send("pinged", author=client_ctx.id, ping=msg["ping"])
        return {"status": "ok", "pong": msg["ping"]}

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
                    device_id = DeviceID(hs.identity)
                except ValueError:
                    raise HandshakeFormatError()

                try:
                    user = await self.user.get_user(device_id.user_id)
                    device = user.devices[device_id.device_name]
                except (NotFoundError, KeyError):
                    result_req = hs.build_bad_identity_result_req()
                else:
                    if device.revocated_on:
                        raise BackendAuthError("Device revoked.")
                    context = ClientContext(hs.identity, user.public_key, device.verify_key)
                    result_req = hs.build_result_req(device.verify_key)

        except BackendAuthError:
            result_req = hs.build_revoked_device_result_req()
        except HandshakeFormatError:
            result_req = hs.build_bad_format_result_req()
        await transport.send(result_req)
        return context

    async def handle_client(self, sockstream, swallow_crash=False):
        transport = PatateTCPTransport(sockstream)
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

        except TransportError as exc:
            # Client has closed connection or sent an invalid trame
            rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
            try:
                await transport.send(rep)
            except TransportError:
                pass
            await transport.aclose()

        except ParsecError as exc:
            logger.debug("BAD HANDSHAKE")
            rep = exc.to_dict()
            await transport.send(rep)
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

        # TODO: Should find a way to avoid using this filter if we're not in log debug...
        def _filter_big_fields(data):
            # As hacky as arbitrary... but works well so far !
            filtered_data = data.copy()
            try:
                if len(data["block"]) > 200:
                    filtered_data["block"] = f"{data['block'][:100]}[...]{data['block'][-100:]}"
            except (KeyError, ValueError, TypeError):
                pass
            try:
                if len(data["blob"]) > 200:
                    filtered_data["blob"] = f"{data['blob'][:100]}[...]{data['blob'][-100:]}"
            except (KeyError, ValueError, TypeError):
                pass
            return filtered_data

        while True:
            req = await transport.recv()
            if not req:  # Client disconnected
                client_ctx.logger.debug("CLIENT DISCONNECTED")
                break

            client_ctx.logger.debug("req", req=_filter_big_fields(req))
            # TODO: handle bad msg
            try:
                cmd = req.get("cmd", "<missing>")
                if client_ctx.anonymous:
                    cmd_func = self.anonymous_cmds[cmd]
                else:
                    cmd_func = self.cmds[cmd]
            except KeyError:
                rep = {"status": "unknown_command", "reason": "Unknown command"}
            else:
                try:
                    rep = await cmd_func(client_ctx, req)
                except ParsecError as err:
                    rep = err.to_dict()
            client_ctx.logger.debug("rep", rep=_filter_big_fields(rep))
            await transport.send(rep)
