import trio
import attr
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.backend_connection2 import backend_cmds_create_pool, backend_listen_events
from parsec.core.connection_monitor import monitor_connection
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.mountpoint import mountpoint_manager_factory
from parsec.core.beacons_monitor import monitor_beacons
from parsec.core.messages_monitor import monitor_messages
from parsec.core.sync_monitor import SyncMonitor
from parsec.core.fs import FS
from parsec.core.local_db import LocalDB
from parsec.core.backend_connection2 import BackendNotAvailable


def _expose_cmds_with_retrier(name, wrapper_name=None):
    async def wrapper(self, *args, **kwargs):
        try:
            async with self.backend_cmds_pool.acquire() as cmds:
                return await getattr(cmds, name)(*args, **kwargs)

        except BackendNotAvailable as exc:
            print(exc)
            import pdb

            pdb.set_trace()
            async with self.backend_cmds_pool.acquire(force_fresh=True) as cmds:
                return await getattr(cmds, name)(*args, **kwargs)

    wrapper.__name__ = wrapper_name or name

    return wrapper


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

    backend_ping = _expose_cmds_with_retrier("ping", wrapper_name="backend_ping")

    user_get = _expose_cmds_with_retrier("user_get")
    user_find = _expose_cmds_with_retrier("user_find")
    user_invite = _expose_cmds_with_retrier("user_invite")
    user_cancel_invitation = _expose_cmds_with_retrier("user_cancel_invitation")
    user_create = _expose_cmds_with_retrier("user_create")

    device_invite = _expose_cmds_with_retrier("device_invite")
    device_cancel_invitation = _expose_cmds_with_retrier("device_cancel_invitation")
    device_create = _expose_cmds_with_retrier("device_create")
    device_revoke = _expose_cmds_with_retrier("device_revoke")


@asynccontextmanager
async def logged_core_factory(config: CoreConfig, device: LocalDevice, event_bus: EventBus = None):
    event_bus = event_bus or EventBus()

    # Plenty of nested scope to order components init/teardown
    async with trio.open_nursery() as root_nursery:

        # Monitor connection must be first given it will watch on
        # other monitors' events
        await root_nursery.start(monitor_connection, event_bus)

        async with trio.open_nursery() as backend_conn_nursery:
            await backend_conn_nursery.start(backend_listen_events, device, event_bus)

            async with backend_cmds_create_pool(
                device.backend_addr,
                device.device_id,
                device.signing_key,
                config.backend_max_connections,
            ) as backend_cmds_pool:

                # TODO
                # encryption_manager = EncryptionManager(backend_cmds_pool, event_bus)
                from unittest.mock import Mock

                encryption_manager = Mock()

                async with trio.open_nursery() as monitor_nursery:

                    local_db = LocalDB(config.data_dir, device)
                    # TODO
                    from tests.common import AsyncMock

                    global FS
                    fs = AsyncMock(spec=FS)
                    # fs = FS(device, local_db, backend_cmds_pool, encryption_manager, event_bus)

                    # Finally start monitoring coroutines
                    await monitor_nursery.start(monitor_beacons, device, fs, event_bus)
                    await monitor_nursery.start(monitor_messages, fs, event_bus)
                    # TODO: replace SyncMonitor by a function
                    sync_monitor = SyncMonitor(fs, event_bus)
                    await monitor_nursery.start(sync_monitor.run)

                    mountpoint_manager = mountpoint_manager_factory(fs, event_bus)
                    # TODO: rework mountpoint manager to avoid init/teardown
                    await mountpoint_manager.init(monitor_nursery)

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
