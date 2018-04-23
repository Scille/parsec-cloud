import trio
import blinker
import logbook
from interface import implements

from parsec.networking import serve_client
from parsec.core.base import IAsyncComponent
from parsec.core.sharing import Sharing
from parsec.core.fs import FS
from parsec.core.synchronizer import Synchronizer
from parsec.core.devices_manager import DevicesManager
from parsec.core.backend_connection import BackendConnection
from parsec.core.fuse_manager import FuseManager
from parsec.core.local_storage import LocalStorage
from parsec.core.backend_storage import BackendStorage
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager


logger = logbook.Logger("parsec.core.app")


class AlreadyLoggedError(Exception):
    pass


class NotLoggedError(Exception):
    pass


class CoreApp(implements(IAsyncComponent)):

    def __init__(self, config):
        self.nursery = None
        self.signal_ns = blinker.Namespace()
        self.devices_manager = DevicesManager(config.base_settings_path)

        self.config = config
        self.backend_addr = config.backend_addr

        # Components dependencies tree:
        # app
        # ├─ fs
        # │  ├─ manifests_manager
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

        self.components_dep_order = (
            "backend_connection",
            "backend_storage",
            "local_storage",
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

        # TODO: replace this by signal passing
        self._config_try_pendings = {}

    async def init(self, nursery):
        self.nursery = nursery

    async def teardown(self):
        try:
            await self.logout()
        except NotLoggedError:
            pass

    async def login(self, device):
        async with self.auth_lock:
            if self.auth_device:
                raise AlreadyLoggedError("Already logged as `%s`" % self.auth_device)

            # First create components
            self.backend_connection = BackendConnection(
                device, self.config.backend_addr, self.signal_ns
            )
            self.local_storage = LocalStorage(device.local_storage_db_path)
            self.backend_storage = BackendStorage(self.backend_connection)
            self.manifests_manager = ManifestsManager(
                device, self.local_storage, self.backend_storage
            )
            self.blocks_manager = BlocksManager(self.local_storage, self.backend_storage)
            self.fs = FS(self.manifests_manager, self.blocks_manager)
            self.fuse_manager = FuseManager(self.config.addr, self.signal_ns)
            self.synchronizer = Synchronizer(self.config.auto_sync, self.fs)
            self.sharing = Sharing(device, self.signal_ns, self.fs, self.backend_connection)

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
                    await component_.teardown()
                # Don't unset components and auth_* stuff after teardown to
                # easier post-mortem debugging
                raise

    async def logout(self):
        async with self.auth_lock:
            if not self.auth_device:
                raise NotLoggedError('No user logged')

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

        await serve_client(lambda req, ctx: dispatch_request(req, ctx, self), sockstream)


Core = CoreApp
