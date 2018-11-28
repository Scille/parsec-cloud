import os
import trio
import attr
from structlog import get_logger

from parsec.event_bus import EventBus
from parsec.networking import serve_client
from parsec.core.base import BaseAsyncComponent, taskify
from parsec.core.fs import FS
from parsec.core.sync_monitor import SyncMonitor
from parsec.core.beacons_monitor import monitor_beacons
from parsec.core.messages_monitor import monitor_messages
from parsec.core.devices_manager import LocalDevicesManager
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.backend_cmds_sender import BackendCmdsSender
from parsec.core.backend_events_manager import BackendEventsManager
from parsec.core.connection_monitor import monitor_connection
from parsec.core.mountpoint import mountpoint_manager_factory


logger = get_logger()


class AlreadyLoggedError(Exception):
    pass


class NotLoggedError(Exception):
    pass


class Core(BaseAsyncComponent):
    def __init__(self, config, event_bus=None):
        super().__init__()
        self.config = config
        self.event_bus = event_bus or EventBus()
        self.backend_addr = config.backend_addr
        self.local_devices_manager = LocalDevicesManager(
            os.path.join(config.base_settings_path, "devices")
        )
        self._nursery = None
        self._logged_client_manager = None
        self._auth_lock = trio.Lock()

    @property
    def fs(self):
        if not self._logged_client_manager:
            raise NotLoggedError("No user logged")
        return self._logged_client_manager.fs

    @property
    def mountpoint_manager(self):
        if not self._logged_client_manager:
            raise NotLoggedError("No user logged")
        return self._logged_client_manager.mountpoint_manager

    @property
    def backend_cmds_sender(self):
        # TODO: should be exposing higher level methods instead of this one
        if not self._logged_client_manager:
            raise NotLoggedError("No user logged")
        return self._logged_client_manager.backend_cmds_sender

    @property
    def auth_device(self):
        if not self._logged_client_manager:
            return None
        return self._logged_client_manager.device

    async def _init(self, nursery):
        self._nursery = nursery

    async def _teardown(self):
        try:
            await self.logout()
        except NotLoggedError:
            pass

    async def login(self, device):
        async with self._auth_lock:
            if self._logged_client_manager:
                raise AlreadyLoggedError(f"Already logged as `{self.auth_device.id}`")
            logged_client_manager = LoggedClientManager(
                device, self.config, self.event_bus, self._nursery
            )

            await logged_client_manager.start()
            self._logged_client_manager = logged_client_manager

            self.event_bus.send("user_login", device=device)

    async def logout(self):
        async with self._auth_lock:
            if not self._logged_client_manager:
                raise NotLoggedError("No user logged")
            await self._logged_client_manager.stop()
            self._logged_client_manager = None

            self.event_bus.send("user_logout")

    async def user_find(self, query: str = None, page: int = 1, per_page: int = 100):
        """
        Raises:
            BackendNotAvailable
        """
        rep = await self.backend_cmds_sender.send(
            {"cmd": "user_find", "query": query, "page": page, "per_page": per_page}
        )
        # TODO: better answer deserialization
        assert rep["status"] == "ok"
        return rep["results"], rep["total"]

    async def ping_backend(self):
        if self._logged_client_manager:
            return await self._logged_client_manager.backend_cmds_sender.ping()
        else:
            return False

    async def handle_client(self, sockstream):
        from parsec.core.api import dispatch_request

        ctx = ClientContext(self.event_bus)
        await serve_client(lambda req: dispatch_request(req, ctx, self), sockstream)


class LoggedClientManager:
    def __init__(self, device, config, event_bus, nursery):
        self.device = device
        self.config = config
        self.event_bus = event_bus
        self._nursery = nursery

        # Components dependencies tree:
        # logged client
        # ├─ backend_events_manager
        # ├─ backend_cmds_sender
        # ├─ encryption_manager
        # │  └─ backend_cmds_sender
        # ├─ fs <-- Note fs doesn't need to be initialized
        # │  ├─ backend_cmds_sender
        # │  └─ encryption_manager
        # └─ mountpoint_manager

        self.backend_events_manager = BackendEventsManager(
            device,
            self.config.backend_addr,
            self.event_bus,
            self.config.cert_path,
            self.config.ca_path,
        )
        self.backend_cmds_sender = BackendCmdsSender(
            device, self.config.backend_addr, self.config.cert_path, self.config.ca_path
        )
        self.encryption_manager = EncryptionManager(device, self.backend_cmds_sender)

        self.fs = FS(device, self.backend_cmds_sender, self.encryption_manager, self.event_bus)

        self.mountpoint_manager = mountpoint_manager_factory(self.fs, self.event_bus)
        self.sync_monitor = SyncMonitor(self.fs, self.event_bus)

    async def start(self):
        # Monitor connection must be first given it will watch on
        # other monitors' events
        self._stop_monitor_connection = await self._nursery.start(
            taskify(monitor_connection, self.event_bus)
        )

        # Components initialization must respect dependencies
        await self.backend_cmds_sender.init(self._nursery)
        await self.encryption_manager.init(self._nursery)
        await self.mountpoint_manager.init(self._nursery)
        # Keep event manager last, so it will know what events the other
        # modules need before connecting to the backend
        await self.backend_events_manager.init(self._nursery)

        # Finally start monitoring coroutines
        self._stop_monitor_beacons = await self._nursery.start(
            taskify(monitor_beacons, self.device, self.fs, self.event_bus)
        )
        self._stop_monitor_messages = await self._nursery.start(
            taskify(monitor_messages, self.fs, self.event_bus)
        )
        self._stop_monitor_sync = await self._nursery.start(taskify(self.sync_monitor.run))

    async def stop(self):
        # First make sure fuse is not started
        await self.mountpoint_manager.teardown()

        # Then stop monitoring coroutine
        await self._stop_monitor_beacons()
        await self._stop_monitor_messages()
        await self._stop_monitor_sync()

        # Then teardown components, again while respecting dependencies
        await self.encryption_manager.teardown()
        await self.backend_cmds_sender.teardown()
        await self.backend_events_manager.teardown()

        await self._stop_monitor_connection()


@attr.s
class ClientContext:
    @property
    def ctxid(self):
        return id(self)

    event_bus = attr.ib()
    registered_signals = attr.ib(default=attr.Factory(dict))
    received_signals = attr.ib(default=attr.Factory(lambda: trio.Queue(100)))

    # TODO: rework this
    def subscribe_signal(self, signal_name, arg=None):

        # TODO: remove the deprecated naming
        if signal_name in ("device_try_claim_submitted", "backend.device.try_claim_submitted"):
            event_name = "backend.device.try_claim_submitted"

            def _build_event_msg(device_name, config_try_id):
                return {
                    "event": signal_name,
                    "device_name": device_name,
                    "config_try_id": config_try_id,
                }

            key = (event_name,)

        elif signal_name == "pinged":
            event_name = "pinged"
            expected_ping = arg
            key = (event_name, expected_ping)

            def _build_event_msg(ping):
                if ping != expected_ping:
                    return None
                return {"event": signal_name, "ping": ping}

        elif signal_name == "fuse_mountpoint_need_stop":
            event_name = "fuse.mountpoint.need_stop"
            expected_mountpoint = arg
            key = (event_name, expected_mountpoint)

            def _build_event_msg(mountpoint):
                if mountpoint != expected_mountpoint:
                    return None
                return {"event": signal_name, "mountpoint": mountpoint}

        else:
            raise NotImplementedError()

        if key in self.registered_signals:
            raise KeyError(f"{key} already subscribed")

        def _handle_event(sender, **kwargs):
            try:
                msg = _build_event_msg(**kwargs)
                if msg:
                    self.received_signals.put_nowait(msg)
            except trio.WouldBlock:
                logger.warning("Event queue is full", client=self.id)

        self.registered_signals[key] = _handle_event
        self.event_bus.connect(event_name, _handle_event, weak=True)

    def unsubscribe_signal(self, signal_name, arg=None):
        if signal_name in ("device_try_claim_submitted", "backend.device.try_claim_submitted"):
            event_name = "backend.device.try_claim_submitted"
            key = (event_name,)

        elif signal_name == "pinged":
            event_name = "pinged"
            expected_ping = arg
            key = (event_name, expected_ping)

        elif signal_name == "fuse_mountpoint_need_stop":
            event_name = "fuse.mountpoint.need_stop"
            expected_mountpoint = arg
            key = (event_name, expected_mountpoint)

        else:
            raise NotImplementedError()

        # Weakref on _handle_event in signal connection will do the rest
        del self.registered_signals[key]
