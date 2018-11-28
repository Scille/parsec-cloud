import trio
from structlog import get_logger
import json

from parsec.schema import UnknownCheckedSchema, OneOfSchema, fields
from parsec.core.base import BaseAsyncComponent
from parsec.core.devices_manager import Device
from parsec.core.backend_connection import (
    backend_connection_factory,
    HandshakeError,
    BackendNotAvailable,
)


MAX_COOLDOWN = 30
logger = get_logger()


class BackendEventPingRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class BackendEventDeviceTryClaimSubmittedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("device.try_claim_submitted", required=True)
    device_name = fields.String(required=True)
    config_try_id = fields.String(required=True)


class BackendEventBeaconUpdatedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("beacon.updated", required=True)
    beacon_id = fields.UUID(required=True)
    index = fields.Integer(required=True)
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class BackendEventMessageReceivedRepSchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("message.received", required=True)
    index = fields.Integer(required=True)


class BackendEventListenRepSchema(OneOfSchema):
    type_field = "event"
    type_field_remove = False
    type_schemas = {
        "ping": BackendEventPingRepSchema(),
        "device.try_claim_submitted": BackendEventDeviceTryClaimSubmittedRepSchema(),
        "beacon.updated": BackendEventBeaconUpdatedRepSchema(),
        "message.received": BackendEventMessageReceivedRepSchema(),
    }

    def get_obj_type(self, obj):
        return obj["event"]


backend_event_listen_rep_schema = BackendEventListenRepSchema()


class SubscribeBackendEventError(Exception):
    pass


class ListenBackendEventError(Exception):
    pass


class BackendEventsManager(BaseAsyncComponent):
    # TODO: modify this definition now we have an "event bus"
    """
    Difference between signal and event:
    - signals are in-process notifications
    - events are message sent downstream from the backend to the client core
    """

    def __init__(self, device: Device, backend_addr: str, event_bus, cert_path, ca_path):
        super().__init__()
        self.device = device
        self.backend_addr = backend_addr
        self.event_bus = event_bus
        self._nursery = None
        self._backend_online_event = trio.Event()
        self._subscribed_events = {
            ("message.received",),
            ("device.try_claim_submitted",),
            ("pinged", None),
        }
        self._subscribed_events_changed = trio.Event()
        self._task_info = None
        self.event_bus.connect("backend.beacon.listen", self._on_beacon_listen, weak=True)
        self.event_bus.connect("backend.beacon.unlisten", self._on_beacon_unlisten, weak=True)
        self.cert_path = cert_path
        self.ca_path = ca_path

    def _on_beacon_listen(self, sender, beacon_id):
        key = ("beacon.updated", beacon_id)
        if key in self._subscribed_events:
            return
        self._subscribed_events.add(key)
        self._subscribed_events_changed.set()

    def _on_beacon_unlisten(self, sender, beacon_id):
        key = ("beacon.updated", beacon_id)
        try:
            self._subscribed_events.remove(key)
        except KeyError:
            return
        self._subscribed_events_changed.set()

    async def _init(self, nursery):
        self._nursery = nursery
        self._task_info = await self._nursery.start(self._task)

    async def _teardown(self):
        cancel_scope, closed_event = self._task_info
        cancel_scope.cancel()
        await closed_event.wait()
        self._task_info = None

    async def wait_backend_online(self):
        await self._backend_online_event.wait()

    def _event_pump_lost(self):
        self._backend_online_event.clear()
        self.event_bus.send("backend.offline")

    def _event_pump_ready(self):
        if not self._backend_online_event.is_set():
            self._backend_online_event.set()
            self.event_bus.send("backend.online")
        self.event_bus.send("backend.listener.restarted", events=self._subscribed_events.copy())

    async def _task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        closed_event = trio.Event()
        try:
            async with trio.open_nursery() as nursery:
                # If backend is online, we want to wait before calling
                # `task_status.started` until we are connected to the backend
                # with events listener ready.
                waiter = self.event_bus.waiter_on_first("backend.online", "backend.offline")

                async def _wait_first_backend_connection_outcome():
                    event, _ = await waiter.wait()
                    task_status.started((nursery.cancel_scope, closed_event))

                nursery.start_soon(_wait_first_backend_connection_outcome)
                await self._event_listener_manager()

        finally:
            closed_event.set()

    async def _event_listener_manager(self):
        backend_connection_failures = 0
        while True:
            try:

                async with trio.open_nursery() as nursery:
                    self._subscribed_events_changed.clear()
                    event_pump_cancel_scope = await nursery.start(self._event_pump)
                    backend_connection_failures = 0
                    self._event_pump_ready()
                    while True:
                        await self._subscribed_events_changed.wait()
                        self._subscribed_events_changed.clear()
                        new_cancel_scope = await nursery.start(self._event_pump)
                        self._event_pump_ready()
                        event_pump_cancel_scope.cancel()
                        event_pump_cancel_scope = new_cancel_scope

            except (BackendNotAvailable, trio.BrokenStreamError, trio.ClosedStreamError) as exc:
                # In case of connection failure, wait a bit and restart
                self._event_pump_lost()
                cooldown_time = 2 ** backend_connection_failures
                backend_connection_failures += 1
                if cooldown_time > MAX_COOLDOWN:
                    cooldown_time = MAX_COOLDOWN
                logger.debug(
                    "Connection lost with backend, retrying after cooldown",
                    reason=exc,
                    cooldown_time=cooldown_time,
                )
                await trio.sleep(cooldown_time)

            except (SubscribeBackendEventError, ListenBackendEventError, json.JSONDecodeError):
                self._event_pump_lost()
                logger.exception("Invalid response sent by backend, restarting connection in 1s...")
                # TODO: remove the event that provoked the error ?
                await trio.sleep(1)

            except HandshakeError as exc:
                # Handshake error means there is no need retrying the connection
                # Only thing we can do is sending a signal to notify the
                # trouble...
                # TODO: think about this kind of signal format
                self._event_pump_lost()
                logger.exception("Handshake error with backend, retrying in 1s...")
                self.event_bus.send("panic", exc=exc)
                await trio.sleep(1)

            except Exception as exc:
                print(exc)
                raise exc

    async def _event_pump(self, *, task_status=trio.TASK_STATUS_IGNORED):
        with trio.open_cancel_scope() as cancel_scope:
            sock = await backend_connection_factory(
                self.backend_addr,
                self.device.id,
                self.device.device_signkey,
                self.cert_path,
                self.ca_path,
            )

            # Copy `self._subscribed_events` to avoid concurrent modifications
            subscribed_events = self._subscribed_events.copy()

            # TODO: allow to subscribe to multiple events in a single query...
            for args in subscribed_events:
                payload = {"cmd": "event_subscribe", "event": args[0]}
                if payload["event"] == "beacon.updated":
                    payload["beacon_id"] = args[1]
                await sock.send(payload)
                rep = await sock.recv()
                if rep.get("status") != "ok":
                    raise SubscribeBackendEventError(f"Cannot subscribe to event {args}: {rep}")

            # Given the backend won't notify us for messages that arrived while
            # we were offline, we must actively check this ourself.
            self.event_bus.send("backend.message.polling_needed")

            task_status.started(cancel_scope)
            while True:
                await sock.send({"cmd": "event_listen"})
                rep = await sock.recv()
                rep, errors = backend_event_listen_rep_schema.load(rep)
                if errors:
                    raise ListenBackendEventError(
                        "Bad reponse %r while listening for event: %r" % (rep, errors)
                    )

                if rep["event"] == "message.received":
                    self.event_bus.send("backend.message.received", index=rep["index"])
                elif rep["event"] == "pinged":
                    self.event_bus.send("backend.pinged")
                elif rep["event"] == "beacon.updated":
                    self.event_bus.send(
                        "backend.beacon.updated",
                        beacon_id=rep["beacon_id"],
                        index=rep["index"],
                        src_id=rep["src_id"],
                        src_version=rep["src_version"],
                    )
                elif rep["event"] == "device.try_claim_submitted":
                    self.event_bus.send(
                        "backend.device.try_claim_submitted",
                        device_name=rep["device_name"],
                        config_try_id=rep["config_try_id"],
                    )
                else:
                    logger.warning("Backend sent unknown event", event_msg=rep)
