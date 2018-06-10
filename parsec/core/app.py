import os
import trio
import attr
import blinker
import logbook

from parsec.networking import serve_client
from parsec.core.base import BaseAsyncComponent, NotInitializedError
from parsec.core.sharing import Sharing
from parsec.core.fs import FS
from parsec.core.synchronizer import Synchronizer
from parsec.core.devices_manager import DevicesManager
from parsec.core.backend_connections_multiplexer import BackendConnectionsMultiplexer
from parsec.core.backend_events_manager import BackendEventsManager
from parsec.core.fuse_manager import FuseManager
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_storage import BackendStorage
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.encryption_manager import EncryptionManager


logger = logbook.Logger("parsec.core.app")


class AlreadyLoggedError(Exception):
    pass


class NotLoggedError(Exception):
    pass


class Core(BaseAsyncComponent):
    def __init__(self, config):
        super().__init__()
        self.nursery = None
        self.signal_ns = blinker.Namespace()
        self.devices_manager = DevicesManager(os.path.join(config.base_settings_path, "devices"))

        self.config = config
        self.backend_addr = config.backend_addr

        # Components dependencies tree:
        # app
        # ├─ backend_events_manager
        # ├─ fs
        # │  ├─ manifests_manager
        # │  │  ├─ encryption_manager
        # │  │  │  ├─ backend_connection
        # │  │  │  └─ local_storage
        # │  │  ├─ local_storage
        # │  │  └─ backend_storage
        # │  │     └─ backend_connection
        # │  └─ blocks_manager
        # │     ├─ local_storage
        # │     └─ backend_storage
        # ├─ fuse_manager
        # ├─ synchronizer
        # │  └─ fs
        # └─ sharing
        #    ├─ encryption_manager
        #    └─ backend_connection

        self.components_dep_order = (
            "backend_events_manager",
            "backend_connection",
            "backend_storage",
            "local_storage",
            "encryption_manager",
            "manifests_manager",
            "blocks_manager",
            "fs",
            "fuse_manager",
            "synchronizer",
            "sharing",
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
            self.backend_connection = BackendConnectionsMultiplexer(
                device, self.config.backend_addr
            )
            self.local_storage = LocalStorage(device.local_storage_db_path)
            self.encryption_manager = EncryptionManager(
                device, self.backend_connection, self.local_storage
            )
            self.backend_storage = BackendStorage(self.backend_connection)
            self.manifests_manager = ManifestsManager(
                self.local_storage, self.backend_storage, self.encryption_manager
            )
            self.blocks_manager = BlocksManager(self.local_storage, self.backend_storage)
            self.fs = FS(
                device, self.manifests_manager, self.blocks_manager, self.config.block_size
            )
            self.fuse_manager = FuseManager(self.config.addr, self.signal_ns)
            self.synchronizer = Synchronizer(self.config.auto_sync, self.fs)
            self.sharing = Sharing(
                device,
                self.fs,
                self.backend_connection,
                self.backend_events_manager,
                self.signal_ns,
            )

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

    _signal_ns = attr.ib()
    registered_signals = attr.ib(default=attr.Factory(dict))
    received_signals = attr.ib(default=attr.Factory(lambda: trio.Queue(100)))

    def subscribe_signal(self, signal_name, subject=blinker.ANY):
        key = (signal_name, subject)
        if key in self.registered_signals:
            raise KeyError("%s@%s already subscribed" % key)

        def _handle_event(sender):
            try:
                self.received_signals.put_nowait((signal_name, sender))
            except trio.WouldBlock:
                logger.warning("{!r}: event queue is full", self)

        self.registered_signals[key] = _handle_event
        self._signal_ns.signal(signal_name).connect(_handle_event, sender=subject, weak=True)

    def unsubscribe_signal(self, signal_name, subject=blinker.ANY):
        key = (signal_name, subject)
        # Weakref on _handle_event in signal connection will do the rest
        del self.registered_signals[key]
