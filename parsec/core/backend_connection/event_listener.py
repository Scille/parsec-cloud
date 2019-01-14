import trio
from structlog import get_logger

from parsec.event_bus import EventBus
from parsec.api.transport import TransportError
from parsec.core.types import LocalDevice
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendHandshakeError,
    BackendDeviceRevokedError,
    BackendCmdsInvalidResponse,
    BackendCmdsBadResponse,
)
from parsec.core.backend_connection.porcelain import backend_cmds_factory


MAX_COOLDOWN = 30
logger = get_logger()


class BackendEventsManager:
    def __init__(self, device: LocalDevice, event_bus: EventBus):
        self.device = device
        self.event_bus = event_bus
        self._backend_online = None
        self._subscribed_beacons = set()
        self._subscribed_beacons_changed = trio.Event()
        self._task_info = None
        self.event_bus.connect("backend.beacon.listen", self._on_beacon_listen, weak=True)
        self.event_bus.connect("backend.beacon.unlisten", self._on_beacon_unlisten, weak=True)

    def _on_beacon_listen(self, sender, beacon_id):
        if beacon_id in self._subscribed_beacons:
            return
        self._subscribed_beacons.add(beacon_id)
        self._subscribed_beacons_changed.set()

    def _on_beacon_unlisten(self, sender, beacon_id):
        try:
            self._subscribed_beacons.remove(beacon_id)
        except KeyError:
            return
        self._subscribed_beacons_changed.set()

    async def _teardown(self):
        cancel_scope, closed_event = self._task_info
        cancel_scope.cancel()
        await closed_event.wait()
        self._task_info = None

    def _event_pump_lost(self):
        if self._backend_online is not False:
            self._backend_online = False
            self.event_bus.send("backend.offline")

    def _event_pump_ready(self):
        if self._backend_online is not True:
            self._backend_online = True
            self.event_bus.send("backend.online")
        self.event_bus.send("backend.listener.restarted")

    async def run(self, *, task_status=trio.TASK_STATUS_IGNORED):
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
                    logger.info("Try to connect to backend...")
                    self._subscribed_beacons_changed.clear()
                    event_pump_cancel_scope = await nursery.start(self._event_pump)
                    backend_connection_failures = 0
                    logger.info("Backend online")
                    self._event_pump_ready()
                    while True:
                        await self._subscribed_beacons_changed.wait()
                        self._subscribed_beacons_changed.clear()
                        new_cancel_scope = await nursery.start(self._event_pump)
                        self._event_pump_ready()
                        event_pump_cancel_scope.cancel()
                        event_pump_cancel_scope = new_cancel_scope
                        logger.info("Event listener restarted")

            except (TransportError, BackendNotAvailable) as exc:
                # In case of connection failure, wait a bit and restart
                self._event_pump_lost()
                cooldown_time = 2 ** backend_connection_failures
                backend_connection_failures += 1
                if cooldown_time > MAX_COOLDOWN:
                    cooldown_time = MAX_COOLDOWN
                logger.info("Backend offline", reason=exc, cooldown_time=cooldown_time)
                await trio.sleep(cooldown_time)

            except (BackendCmdsInvalidResponse, BackendCmdsBadResponse):
                # Backend is drunk, what can we do about it ?
                self._event_pump_lost()
                logger.exception("Invalid response sent by backend, restarting connection in 1s...")
                await trio.sleep(1)

            except (BackendHandshakeError, BackendDeviceRevokedError):
                # No need to retry given backend don't want to talk to us...
                self._event_pump_lost()
                logger.exception("Cannot connect to the backend")
                return

    async def _event_pump(self, *, task_status=trio.TASK_STATUS_IGNORED):
        with trio.open_cancel_scope() as cancel_scope:
            async with backend_cmds_factory(
                self.device.backend_addr, self.device.device_id, self.device.signing_key, max_pool=1
            ) as cmds:
                # Copy `self._subscribed_beacons` to avoid concurrent modifications
                await cmds.events_subscribe(
                    message_received=True, beacon_updated=self._subscribed_beacons.copy()
                )

                # Given the backend won't notify us for messages that arrived while
                # we were offline, we must actively check this ourself.
                self.event_bus.send("backend.message.polling_needed")

                task_status.started(cancel_scope)
                await self._event_pump_do(cmds)

    async def _event_pump_do(self, cmds):
        while True:
            rep = await cmds.events_listen()

            if rep["event"] == "message.received":
                self.event_bus.send("backend.message.received", index=rep["index"])

            elif rep["event"] == "pinged":
                self.event_bus.send("backend.pinged", ping=rep["ping"])

            elif rep["event"] == "beacon.updated":
                self.event_bus.send(
                    "backend.beacon.updated",
                    beacon_id=rep["beacon_id"],
                    index=rep["index"],
                    src_id=rep["src_id"],
                    src_version=rep["src_version"],
                )

            else:
                logger.warning("Backend sent unknown event", event_msg=rep)


async def backend_listen_events(device, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    backend_events_manager = BackendEventsManager(device, event_bus)
    await backend_events_manager.run(task_status=task_status)


# async def backend_listen_events2(device, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
#         self.device = device
#         self.event_bus = event_bus
#         self._backend_online = False
#         self._subscribed_beacons = set()
#         self._subscribed_beacons_changed = trio.Event()
#         self._task_info = None
#         self.event_bus.connect("backend.beacon.listen", self._on_beacon_listen, weak=True)
#         self.event_bus.connect("backend.beacon.unlisten", self._on_beacon_unlisten, weak=True)


#     backend_events_manager = BackendEventsManager(device, event_bus)


#     await backend_events_manager.run(task_status=task_status)
