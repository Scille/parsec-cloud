import os
import trio
import attr
import logbook

from parsec.signals import Namespace as SignalNamespace
from parsec.networking import serve_client
from parsec.core.base import BaseAsyncComponent, NotInitializedError
from parsec.core.fs import FSManager
from parsec.core.devices_manager import LocalDevicesManager
from parsec.core.fuse_manager import FuseManager
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.backend_cmds_sender import BackendCmdsSender
from parsec.core.backend_events_manager import BackendEventsManager


logger = logbook.Logger("parsec.core.app")


class AlreadyLoggedError(Exception):
    pass


class NotLoggedError(Exception):
    pass


class Core(BaseAsyncComponent):
    def __init__(self, config, signal_ns=None):
        super().__init__()
        self.nursery = None
        self.signal_ns = signal_ns or SignalNamespace()
        self.config = config
        self.backend_addr = config.backend_addr
        self.local_devices_manager = LocalDevicesManager(
            os.path.join(config.base_settings_path, "devices")
        )

        # Components dependencies tree:
        # app
        # ├─ backend_events_manager
        # ├─ backend_cmds_sender
        # ├─ encryption_manager
        # │  └─ backend_cmds_sender
        # ├─ fs
        # │  ├─ encryption_manager
        # │  └─ backend_cmds_sender
        # └─ fuse_manager

        self.components_dep_order = (
            "backend_cmds_sender",
            "encryption_manager",
            "fs",
            "fuse_manager",
            # Keep event manager last, so it will know what events the other
            # modules need before connecting to the backend
            "backend_events_manager",
        )
        for cname in self.components_dep_order:
            setattr(self, cname, None)

        # TODO: create a context object to store/manipulate auth_* data
        self.auth_lock = trio.Lock()
        self.auth_device = None
        self.auth_privkey = None
        self.auth_subscribed_events = None
        self.auth_events = None

    async def _init(self, nursery):
        self.nursery = nursery

    async def _teardown(self):
        try:
            await self.logout()
        except NotLoggedError:
            pass

    async def login(self, device):
        async with self.auth_lock:
            if self.auth_device:
                raise AlreadyLoggedError("Already logged as `%s`" % self.auth_device)

            # First create components
            self.backend_events_manager = BackendEventsManager(
                device, self.config.backend_addr, self.signal_ns
            )
            self.backend_cmds_sender = BackendCmdsSender(device, self.config.backend_addr)
            self.encryption_manager = EncryptionManager(device, self.backend_cmds_sender)
            self.fs = FSManager(
                device,
                self.backend_cmds_sender,
                self.encryption_manager,
                self.signal_ns,
                auto_sync=self.config.auto_sync,
            )
            self.fuse_manager = FuseManager(self.config.addr, self.signal_ns)

            # Then initialize them, order must respect dependencies here !
            try:
                for cname in self.components_dep_order:
                    component = getattr(self, cname)
                    await component.init(self.nursery)

                # Keep this last to guarantee login was ok if it is set
                self.auth_subscribed_events = {}
                self.auth_events = trio.Queue(100)
                self.auth_device = device

            except Exception:
                # Make sure to teardown all the already initialized components
                # if something goes wrong
                for cname_ in reversed(self.components_dep_order):
                    component_ = getattr(self, cname_)
                    try:
                        await component_.teardown()
                    except NotInitializedError:
                        pass
                # Don't unset components and auth_* stuff after teardown to
                # easier post-mortem debugging
                raise

    async def logout(self):
        async with self.auth_lock:
            await self._logout_no_lock()

    async def _logout_no_lock(self):
        if not self.auth_device:
            raise NotLoggedError("No user logged")

        # Teardown in init reverse order
        for cname in reversed(self.components_dep_order):
            component = getattr(self, cname)
            await component.teardown()
            setattr(self, cname, None)

        # Keep this last to guarantee logout was ok if it is unset
        self.auth_subscribed_events = None
        self.auth_events = None
        self.auth_device = None

    async def handle_client(self, sockstream):
        from parsec.core.api import dispatch_request

        ctx = ClientContext(self.signal_ns)
        await serve_client(lambda req: dispatch_request(req, ctx, self), sockstream)


@attr.s
class ClientContext:
    @property
    def ctxid(self):
        return id(self)

    signal_ns = attr.ib()
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
                logger.warning(f"Event queue is full for {self.id}")

        self.registered_signals[key] = _handle_event
        self.signal_ns.signal(event_name).connect(_handle_event, weak=True)

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
