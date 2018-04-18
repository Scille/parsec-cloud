import trio
import blinker
import logbook

from parsec.networking import serve_client
from parsec.core.sharing import Sharing
from parsec.core.fs import fs_factory
from parsec.core.synchronizer import Synchronizer
from parsec.core.devices_manager import DevicesManager
from parsec.core.backend_connection import BackendConnection
from parsec.core.fuse_manager import FuseManager


logger = logbook.Logger("parsec.core.app")


class CoreApp:

    def __init__(self, config):
        self.signal_ns = blinker.Namespace()
        self.config = config
        self.backend_addr = config.backend_addr

        self._config_try_pendings = {}

        self.nursery = None
        # TODO: create a context object to store/manipulate auth_* data
        self.auth_device = None
        self.auth_privkey = None
        self.auth_subscribed_events = None
        self.auth_events = None
        self.fs = None
        self.synchronizer = None
        self.sharing = None
        self.backend_connection = None
        self.devices_manager = DevicesManager(config.base_settings_path)
        self.fuse_manager = None
        self.mountpoint = None

    async def init(self, nursery):
        self.nursery = nursery

    async def shutdown(self):
        if self.auth_device:
            await self.logout()

    async def login(self, device):
        # TODO: create a login/logout lock to avoid concurrency crash
        # during logout
        self.auth_subscribed_events = {}
        self.auth_events = trio.Queue(100)
        self.fuse_manager = FuseManager(self.config.addr, self.signal_ns)
        self.backend_connection = BackendConnection(
            device, self.config.backend_addr, self.signal_ns
        )
        await self.backend_connection.init(self.nursery)
        try:
            self.fs = await fs_factory(device, self.config, self.backend_connection)
            if self.config.auto_sync:
                self.synchronizer = Synchronizer()
                await self.synchronizer.init(self.nursery, self.fs)
            try:
                # local_storage = LocalStorage()
                # backend_storage = BackendStorage()
                # manifests_manager = ManifestsManager(self.auth_device,
                #                                      local_storage,
                #                                      backend_storage)
                # blocks_manager = BlocksManager(self.auth_device, local_storage, backend_storage)
                # # await manifests_manager.init()
                # # await blocks_manager.init()
                # self.fs = FS(manifests_manager, blocks_manager)
                await self.fs.init()
                try:
                    self.sharing = Sharing(
                        device, self.signal_ns, self.fs, self.backend_connection
                    )
                    await self.sharing.init(self.nursery)
                except BaseException:
                    await self.fs.teardown()
                    raise

            except BaseException:
                if self.synchronizer:
                    await self.synchronizer.teardown()
                raise

        except BaseException:
            await self.backend_connection.teardown()
            raise

        # Keep this last to guarantee login was ok if it is set
        self.auth_device = device

    async def logout(self):
        self._handle_new_message = None
        await self.fuse_manager.teardown()
        await self.sharing.teardown()
        await self.fs.teardown()
        if self.synchronizer:
            await self.synchronizer.teardown()
        await self.backend_connection.teardown()
        self.backend_connection = None
        # await self.fs.manifests_manager.teardown()
        # await self.fs.blocks_manager.teardown()
        self.auth_device = None
        self.auth_subscribed_events = None
        self.auth_events = None
        self.synchronizer = None
        self.fs = None
        self.sharing = None
        self.fuse_manager = None

    async def handle_client(self, sockstream):
        from parsec.core.api import dispatch_request

        await serve_client(
            lambda req, ctx: dispatch_request(req, ctx, self), sockstream
        )


Core = CoreApp
