from typing import List
import trio
import attr
from async_generator import asynccontextmanager

from parsec.types import UserID
from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.backend_connection2 import backend_cmds_create_pool, backend_listen_events
from parsec.core.connection_monitor import monitor_connection
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.mountpoint_manager import MountpointManager
from parsec.core.beacons_monitor import monitor_beacons
from parsec.core.messages_monitor import monitor_messages
from parsec.core.sync_monitor import SyncMonitor
from parsec.core.fs import FS
from parsec.core.local_db import LocalDB


@attr.s(frozen=True, slots=True)
class LoggedCore:
    config = attr.ib()
    device = attr.ib()
    local_db = attr.ib()
    event_bus = attr.ib()
    encryption_manager = attr.ib()
    mountpoint_manager = attr.ib()
    backend_cmds_pool = attr.ib()
    fs = attr.ib()

    async def user_find(
        self, query: str = None, page: int = 1, per_page: int = 100
    ) -> List[UserID]:
        """
        Raises:
            BackendNotAvailable
        """
        async with self.backend_cmds_pool.acquire() as cmds:
            return await cmds.user_find(query, page, per_page)

    async def ping_backend(self) -> None:
        """
        Raises:
            BackendNotAvailable
        """
        async with self.backend_cmds_pool.acquire() as cmds:
            return await cmds.ping()


@asynccontextmanager
async def logged_core_factory(config: CoreConfig, device: LocalDevice, event_bus: EventBus = None):
    event_bus = event_bus or EventBus()

    # Plenty of nested scope to order components init/teardown
    async with trio.open_nursery() as root_nursery:

        # Monitor connection must be first given it will watch on
        # other monitors' events
        await root_nursery.start(monitor_connection)

        async with trio.open_nursery() as backend_conn_nursery:
            await backend_conn_nursery.start(
                backend_listen_events, device, config.backend_addr, event_bus
            )

            async with backend_cmds_create_pool(
                config.backend_addr, device.device_id, device.signing_key
            ) as backend_cmds_pool:

                encryption_manager = EncryptionManager(backend_cmds_pool, event_bus)
                mountpoint_manager = MountpointManager(event_bus)

                async with trio.open_nursery() as monitor_nursery:
                    # TODO: rework mountpoint manager to avoid init/teardown
                    await mountpoint_manager.init(monitor_nursery)

                    local_db = LocalDB(config.local_db_folder, device)
                    fs = FS(device, local_db, backend_cmds_pool, encryption_manager, event_bus)

                    # Finally start monitoring coroutines
                    await monitor_nursery.start(monitor_beacons, device, fs, event_bus)
                    await monitor_nursery.start(monitor_messages, fs, event_bus)
                    # TODO: replace SyncMonitor by a function
                    sync_monitor = SyncMonitor(fs, event_bus)
                    await monitor_nursery.start(sync_monitor.run)

                    try:
                        yield LoggedCore(
                            config=config,
                            device=device,
                            local_db=local_db,
                            event_bus=event_bus,
                            encryption_manager=encryption_manager,
                            mountpoint_manager=mountpoint_manager,
                            backend_cmds_pool=backend_cmds_pool,
                            fs=fs,
                        )
                        root_nursery.cancel_scope.cancel()

                    finally:
                        await mountpoint_manager.teardown()
