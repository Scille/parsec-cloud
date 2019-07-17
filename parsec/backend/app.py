# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import trio
from structlog import get_logger

from parsec.api.protocole import (
    InvalidMessageError,
    MessageSerializationError,
    ProtocoleError,
    ServerHandshake,
    packb,
    unpackb,
)
from parsec.api.transport import Transport, TransportClosedByPeer, TransportError
from parsec.backend.blockstore import blockstore_factory
from parsec.backend.drivers.memory import (
    MemoryBlockComponent,
    MemoryMessageComponent,
    MemoryOrganizationComponent,
    MemoryPingComponent,
    MemoryRealmComponent,
    MemoryUserComponent,
    MemoryVlobComponent,
)
from parsec.backend.drivers.postgresql import (
    PGBlockComponent,
    PGHandler,
    PGMessageComponent,
    PGOrganizationComponent,
    PGPingComponent,
    PGRealmComponent,
    PGUserComponent,
    PGVlobComponent,
)
from parsec.backend.events import EventsComponent
from parsec.backend.organization import OrganizationNotFoundError
from parsec.backend.user import UserNotFoundError
from parsec.backend.utils import check_anonymous_api_allowed
from parsec.event_bus import EventBus

logger = get_logger()


@attr.s(slots=True, repr=False)
class LoggedClientContext:
    transport = attr.ib()
    organization_id = attr.ib()
    is_admin = attr.ib()
    device_id = attr.ib()
    public_key = attr.ib()
    verify_key = attr.ib()
    event_bus_ctx = attr.ib(default=None)
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    channels = attr.ib(factory=lambda: trio.open_memory_channel(100))
    realms = attr.ib(factory=set)

    def __repr__(self):
        return (
            f"LoggedClientContext(conn={self.conn_id}, "
            f"org={self.organization_id}, "
            f"device={self.device_id})"
        )

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


@attr.s(slots=True, repr=False)
class AnonymousClientContext:
    transport = attr.ib()
    organization_id = attr.ib()
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    device_id = None

    def __repr__(self):
        return f"AnonymousClientContext(conn={self.conn_id}, org={self.organization_id})"

    def __attrs_post_init__(self):
        self.conn_id = self.transport.conn_id
        self.logger = self.transport.logger = self.transport.logger.bind(
            organization_id=str(self.organization_id), client_id="<anonymous>"
        )


@attr.s
class AdministrationClientContext:
    transport = attr.ib()
    conn_id = attr.ib(init=False)
    logger = attr.ib(init=False)
    device_id = None

    def __attrs_post_init__(self):
        self.conn_id = self.transport.conn_id
        self.logger = self.transport.logger = self.transport.logger.bind(
            client_id="<administration>"
        )


def _filter_binary_fields(data):
    return {k: v if not isinstance(v, bytes) else b"[...]" for k, v in data.items()}


class BackendApp:
    def __init__(self, config, event_bus=None):
        self.event_bus = event_bus or EventBus()
        self.config = config
        self.nursery = None
        self.dbh = None

        if self.config.db_url == "MOCKED":
            self.user = MemoryUserComponent(self.event_bus)
            self.organization = MemoryOrganizationComponent(self.user)
            self.message = MemoryMessageComponent(self.event_bus)
            self.realm = MemoryRealmComponent(self.event_bus, self.user, self.message)
            self.vlob = MemoryVlobComponent(self.event_bus, self.realm)
            self.realm._register_maintenance_reencryption_hooks(
                self.vlob._maintenance_reencryption_start_hook,
                self.vlob._maintenance_reencryption_is_finished_hook,
            )
            self.ping = MemoryPingComponent(self.event_bus)
            self.blockstore = blockstore_factory(self.config.blockstore_config)
            self.block = MemoryBlockComponent(self.blockstore, self.realm)

        else:
            self.dbh = PGHandler(self.config.db_url, self.event_bus)
            self.user = PGUserComponent(self.dbh, self.event_bus)
            self.organization = PGOrganizationComponent(self.dbh, self.user)
            self.message = PGMessageComponent(self.dbh)
            self.realm = PGRealmComponent(self.dbh)
            self.vlob = PGVlobComponent(self.dbh)
            self.ping = PGPingComponent(self.dbh)
            self.blockstore = blockstore_factory(
                self.config.blockstore_config, postgresql_dbh=self.dbh
            )
            self.block = PGBlockComponent(self.dbh, self.blockstore, self.vlob)

        self.events = EventsComponent(self.event_bus, self.realm)

        self.logged_cmds = {
            "events_subscribe": self.events.api_events_subscribe,
            "events_listen": self.events.api_events_listen,
            "ping": self.ping.api_ping,
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
            # Block
            "block_create": self.block.api_block_create,
            "block_read": self.block.api_block_read,
            # Vlob
            "vlob_group_check": self.vlob.api_vlob_group_check,
            "vlob_poll_changes": self.vlob.api_vlob_poll_changes,
            "vlob_create": self.vlob.api_vlob_create,
            "vlob_read": self.vlob.api_vlob_read,
            "vlob_update": self.vlob.api_vlob_update,
            "vlob_maintenance_get_reencryption_batch": self.vlob.api_vlob_maintenance_get_reencryption_batch,
            "vlob_maintenance_save_reencryption_batch": self.vlob.api_vlob_maintenance_save_reencryption_batch,
            # Realm
            "realm_create": self.realm.api_realm_create,
            "realm_status": self.realm.api_realm_status,
            "realm_get_roles": self.realm.api_realm_get_roles,
            "realm_get_role_certificates": self.realm.api_realm_get_role_certificates,
            "realm_update_roles": self.realm.api_realm_update_roles,
            "realm_start_reencryption_maintenance": self.realm.api_realm_start_reencryption_maintenance,
            "realm_finish_reencryption_maintenance": self.realm.api_realm_finish_reencryption_maintenance,
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

            if hs.answer_type == "authenticated":
                organization_id = hs.answer_data["organization_id"]
                device_id = hs.answer_data["device_id"]
                expected_rvk = hs.answer_data["rvk"]
                try:
                    organization = await self.organization.get(organization_id)
                    user, device = await self.user.get_user_with_device(organization_id, device_id)

                except (OrganizationNotFoundError, UserNotFoundError, KeyError):
                    result_req = hs.build_bad_identity_result_req()

                else:
                    if organization.root_verify_key != expected_rvk:
                        result_req = hs.build_rvk_mismatch_result_req()

                    elif device.revoked_on:
                        result_req = hs.build_revoked_device_result_req()

                    else:
                        context = LoggedClientContext(
                            transport,
                            organization_id,
                            user.is_admin,
                            device_id,
                            user.public_key,
                            device.verify_key,
                        )
                        result_req = hs.build_result_req(device.verify_key)

            elif hs.answer_type == "anonymous":
                organization_id = hs.answer_data["organization_id"]
                expected_rvk = hs.answer_data["rvk"]
                try:
                    organization = await self.organization.get(organization_id)

                except OrganizationNotFoundError:
                    result_req = hs.build_bad_identity_result_req()

                else:
                    if expected_rvk and organization.root_verify_key != expected_rvk:
                        result_req = hs.build_rvk_mismatch_result_req()

                    else:
                        context = AnonymousClientContext(transport, organization_id)
                        result_req = hs.build_result_req()

            else:  # admin
                context = AdministrationClientContext(transport)
                if hs.answer_data["token"] == self.config.administration_token:
                    result_req = hs.build_result_req()
                else:
                    result_req = hs.build_bad_administration_token_result_req()

        except ProtocoleError as exc:
            result_req = hs.build_bad_format_result_req(str(exc))

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

            if hasattr(client_ctx, "event_bus_ctx"):
                with self.event_bus.connection_context() as client_ctx.event_bus_ctx:

                    await self._handle_client_loop(transport, client_ctx)

            else:
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

                if isinstance(client_ctx, AdministrationClientContext):
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
