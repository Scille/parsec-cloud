import attr
import trio
import blinker
import logbook
import traceback
from nacl.public import PublicKey
from nacl.signing import VerifyKey
from json import JSONDecodeError
import re

from parsec.utils import ParsecError
from parsec.networking import CookedSocket
from parsec.handshake import HandshakeFormatError, ServerHandshake
from parsec.schema import BaseCmdSchema, fields, validate

from parsec.backend.drivers.memory import (
    MemoryUserComponent,
    MemoryVlobComponent,
    MemoryUserVlobComponent,
    MemoryGroupComponent,
    MemoryMessageComponent,
    MemoryBlockStoreComponent,
)
from parsec.backend.drivers.postgresql import (
    PGHandler,
    PGUserComponent,
    PGVlobComponent,
    PGUserVlobComponent,
    PGGroupComponent,
    PGMessageComponent,
    PGBlockStoreComponent,
)

from parsec.backend.exceptions import NotFoundError

try:
    from parsec.backend.s3_blockstore import S3BlockStoreComponent

    S3_AVAILABLE = True
except ImportError:
    S3_AVAILABLE = False
try:
    from parsec.backend.openstack_blockstore import OpenStackBlockStoreComponent

    OPENSTACK_AVAILABLE = True
except ImportError:
    OPENSTACK_AVAILABLE = False


logger = logbook.Logger("parsec.backend.app")


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class cmd_PING_Schema(BaseCmdSchema):
    ping = fields.String(required=True)


class cmd_EVENT_SUBSCRIBE_Schema(BaseCmdSchema):
    event = fields.String(
        required=True,
        validate=validate.OneOf(
            [
                "vlob_updated",
                "user_vlob_updated",
                "message_arrived",
                "ping",
                "device_try_claim_submitted",
            ]
        ),
    )
    subject = fields.String(missing=None)


class cmd_EVENT_LISTEN_Schema(BaseCmdSchema):
    wait = fields.Boolean(missing=True)


@attr.s
class AnonymousClientContext:
    id = "anonymous"
    anonymous = True


@attr.s
class ClientContext:
    anonymous = False
    id = attr.ib()
    broadcast_key = attr.ib()
    verify_key = attr.ib()
    subscribed_events = attr.ib(default=attr.Factory(dict), init=False)
    events = attr.ib(default=attr.Factory(lambda: trio.Queue(100)), init=False)

    @property
    def user_id(self):
        return self.id.split("@")[0]

    @property
    def device_name(self):
        return self.id.split("@")[1]


class BackendApp:

    def __init__(self, config):
        self.signal_ns = blinker.Namespace()
        self.config = config
        self.nursery = None
        self.blockstore_postgresql = config.blockstore_postgresql
        self.blockstore_openstack = config.blockstore_openstack
        self.blockstore_s3 = config.blockstore_s3
        self.dbh = None

        if self.config.dburl in [None, "mocked://"]:
            # TODO: validate BLOCKSTORE value
            if self.blockstore_postgresql:
                self.blockstore = MemoryBlockStoreComponent(self.signal_ns)
            elif S3_AVAILABLE and self.blockstore_s3:
                self.blockstore = S3BlockStoreComponent(
                    self.signal_ns, *self.blockstore_s3.split(":")
                )
            elif OPENSTACK_AVAILABLE and self.blockstore_openstack:
                container, user, tenant_password, url = self.blockstore_openstack.split(":", 3)
                tenant, password = tenant_password.split("@")
                self.blockstore = OpenStackBlockStoreComponent(
                    self.signal_ns, url, container, user, tenant, password
                )
            else:
                self.blockstore = None

            self.user = MemoryUserComponent(self.signal_ns)
            self.vlob = MemoryVlobComponent(self.signal_ns)
            self.user_vlob = MemoryUserVlobComponent(self.signal_ns)
            self.message = MemoryMessageComponent(self.signal_ns)
            self.group = MemoryGroupComponent(self.signal_ns)

        else:
            self.dbh = PGHandler(self.config.dburl, self.signal_ns)

            if self.blockstore_postgresql:
                self.blockstore = MemoryBlockStoreComponent(self.signal_ns)
            elif S3_AVAILABLE and self.blockstore_s3:
                self.blockstore = S3BlockStoreComponent(
                    self.signal_ns, *self.blockstore_s3.split(":")
                )
            elif OPENSTACK_AVAILABLE and self.blockstore_openstack:
                container, user, tenant_password, url = self.blockstore_openstack.split(":", 3)
                tenant, password = tenant_password.split("@")
                self.blockstore = OpenStackBlockStoreComponent(
                    self.signal_ns, url, container, user, tenant, password
                )
            else:
                self.blockstore = None

            self.user = PGUserComponent(self.dbh, self.signal_ns)
            self.vlob = PGVlobComponent(self.dbh, self.signal_ns)
            self.user_vlob = PGUserVlobComponent(self.dbh, self.signal_ns)
            self.message = PGMessageComponent(self.dbh, self.signal_ns)
            self.group = PGGroupComponent(self.dbh, self.signal_ns)

        self.anonymous_cmds = {
            "user_claim": self.user.api_user_claim,
            "device_configure": self.user.api_device_configure,
            "ping": self._api_ping,
        }

        self.cmds = {
            "event_subscribe": self._api_event_subscribe,
            "event_unsubscribe": self._api_event_unsubscribe,
            "event_listen": self._api_event_listen,
            "event_list_subscribed": self._api_event_list_subscribed,
            "user_get": self.user.api_user_get,
            "user_invite": self.user.api_user_invite,
            "device_declare": self.user.api_device_declare,
            "device_get_configuration_try": self.user.api_device_get_configuration_try,
            "device_accept_configuration_try": self.user.api_device_accept_configuration_try,
            "device_refuse_configuration_try": self.user.api_device_refuse_configuration_try,
            "blockstore_post": self._api_blockstore_post,
            "blockstore_get": self._api_blockstore_get,
            "blockstore_get_url": self._api_blockstore_get_url,
            "vlob_create": self.vlob.api_vlob_create,
            "vlob_read": self.vlob.api_vlob_read,
            "vlob_update": self.vlob.api_vlob_update,
            "user_vlob_read": self.user_vlob.api_user_vlob_read,
            "user_vlob_update": self.user_vlob.api_user_vlob_update,
            "group_read": self.group.api_group_read,
            "group_create": self.group.api_group_create,
            "group_add_identities": self.group.api_group_add_identities,
            "group_remove_identities": self.group.api_group_remove_identities,
            "message_get": self.message.api_message_get,
            "message_new": self.message.api_message_new,
            "ping": self._api_ping,
        }

    async def init(self, nursery):
        if self.dbh:
            await self.dbh.init(nursery)

    async def teardown(self):
        if self.dbh:
            await self.dbh.teardown()

    async def _api_ping(self, client_ctx, msg):
        msg = cmd_PING_Schema().load_or_abort(msg)
        self.signal_ns.signal("ping").send(msg["ping"])
        return {"status": "ok", "pong": msg["ping"]}

    async def _api_blockstore_post(self, client_ctx, msg):
        if not self.blockstore:
            return {"status": "not_available", "reason": "Blockstore not available"}

        return await self.blockstore.api_blockstore_post(client_ctx, msg)

    async def _api_blockstore_get(self, client_ctx, msg):
        if not self.blockstore:
            return {"status": "not_available", "reason": "Blockstore not available"}

        return await self.blockstore.api_blockstore_get(client_ctx, msg)

    async def _api_blockstore_get_url(self, client_ctx, msg):
        return {"status": "ok", "url": self.blockstore_url}

    async def _api_event_subscribe(self, client_ctx, msg):
        msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(msg)
        event = msg["event"]
        subject = msg["subject"]

        if (
            event in ("user_vlob_updated", "message_arrived", "device_try_claim")
            and subject not in (None, client_ctx.user_id)
        ):
            # TODO: is the `subject == None` valid here ?
            return {"status": "private_event", "reason": "This type of event is private."}

        def _handle_event(sender):
            try:
                client_ctx.events.put_nowait((event, sender))
            except trio.WouldBlock:
                logger.warning("event queue is full for %s" % client_ctx.id)

        client_ctx.subscribed_events[event, subject] = _handle_event
        if subject:
            self.signal_ns.signal(event).connect(_handle_event, sender=subject, weak=True)
        else:
            self.signal_ns.signal(event).connect(_handle_event, weak=True)
        return {"status": "ok"}

    async def _api_event_unsubscribe(self, client_ctx, msg):
        msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(msg)
        try:
            del client_ctx.subscribed_events[msg["event"], msg["subject"]]
        except KeyError:
            return {
                "status": "not_subscribed", "reason": "Not subscribed to this event/subject couple"
            }

        return {"status": "ok"}

    async def _api_event_listen(self, client_ctx, msg):
        msg = cmd_EVENT_LISTEN_Schema().load_or_abort(msg)
        if msg["wait"]:
            event, subject = await client_ctx.events.get()
        else:
            try:
                event, subject = client_ctx.events.get_nowait()
            except trio.WouldBlock:
                return {"status": "ok"}

        return {"status": "ok", "event": event, "subject": subject}

    async def _api_event_list_subscribed(self, client_ctx, msg):
        BaseCmdSchema().load_or_abort(msg)  # empty msg expected
        return {"status": "ok", "subscribed": list(client_ctx.subscribed_events.keys())}

    async def _do_handshake(self, sock):
        context = None
        try:
            hs = ServerHandshake(self.config.handshake_challenge_size)
            challenge_req = hs.build_challenge_req()
            await sock.send(challenge_req)
            answer_req = await sock.recv()

            hs.process_answer_req(answer_req)
            if hs.identity == "anonymous":
                context = AnonymousClientContext()
                result_req = hs.build_result_req()
            else:
                try:
                    userid, deviceid = hs.identity.split("@")
                except ValueError:
                    raise HandshakeFormatError()

                try:
                    user = await self.user.get(userid)
                    device = user["devices"][deviceid]
                except (NotFoundError, KeyError):
                    result_req = hs.build_bad_identity_result_req()
                else:
                    broadcast_key = PublicKey(user["broadcast_key"])
                    verify_key = VerifyKey(device["verify_key"])
                    context = ClientContext(hs.identity, broadcast_key, verify_key)
                    result_req = hs.build_result_req(verify_key)

        except HandshakeFormatError:
            result_req = hs.build_bad_format_result_req()
        await sock.send(result_req)
        return context

    async def handle_client(self, sockstream):
        sock = CookedSocket(sockstream)
        try:
            logger.debug("START HANDSHAKE")
            client_ctx = await self._do_handshake(sock)
            if not client_ctx:
                # Invalid handshake
                logger.debug("BAD HANDSHAKE")
                return

            logger.debug("HANDSHAKE DONE, CLIENT IS `%s`" % client_ctx.id)

            await self._handle_client_loop(sock, client_ctx)

        except trio.BrokenStreamError:
            # Client has closed connection
            pass
        except Exception:
            # If we are here, something unexpected happened...
            logger.error(traceback.format_exc())
            await sock.aclose()
            raise

    async def _handle_client_loop(self, sock, client_ctx):
        while True:
            try:
                req = await sock.recv()
            except JSONDecodeError:
                rep = {"status": "invalid_msg_format", "reason": "Invalid message format"}
                await sock.send(rep)
                continue

            if not req:  # Client disconnected
                logger.debug("CLIENT DISCONNECTED")
                break

            logger.debug("REQ %s" % req)
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
            logger.debug("REP %s" % rep)
            await sock.send(rep)
