# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import attr
from typing import Optional
from structlog import get_logger
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.backend_connection import (
    backend_cmds_pool_factory,
    backend_listen_events,
    monitor_backend_connection,
)
from parsec.core.mountpoint import mountpoint_manager_factory
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.messages_monitor import monitor_messages
from parsec.core.sync_monitor import monitor_sync
from parsec.core.fs import UserFS


logger = get_logger()


@attr.s(frozen=True, slots=True)
class LoggedCore:
    config = attr.ib()
    device = attr.ib()
    event_bus = attr.ib()
    remote_devices_manager = attr.ib()
    mountpoint_manager = attr.ib()
    backend_cmds = attr.ib()
    user_fs = attr.ib()


@asynccontextmanager
async def logged_core_factory(
    config: CoreConfig, device: LocalDevice, event_bus: Optional[EventBus] = None
):
    event_bus = event_bus or EventBus()

    # Plenty of nested scope to order components init/teardown
    async with trio.open_nursery() as root_nursery:
        # TODO: Currently backend_listen_events connect to backend and
        # switch to listen events mode, then monitors kick in and send it
        # events about which vlob groups to listen on, obliging to restart the
        # listen connection...

        async with backend_cmds_pool_factory(
            device.organization_addr,
            device.device_id,
            device.signing_key,
            max_pool=config.backend_max_connections,
            keepalive_time=config.backend_connection_keepalive,
        ) as backend_cmds_pool:

            path = config.data_base_dir / device.slug
            remote_devices_manager = RemoteDevicesManager(backend_cmds_pool, device.root_verify_key)
            with UserFS(
                device, path, backend_cmds_pool, remote_devices_manager, event_bus
            ) as user_fs:

                async with trio.open_nursery() as monitor_nursery:
                    # Finally start monitors

                    # Monitor connection must be first given it will watch on
                    # other monitors' events
                    await monitor_nursery.start(monitor_backend_connection, event_bus)
                    await monitor_nursery.start(monitor_messages, user_fs, event_bus)
                    await monitor_nursery.start(monitor_sync, user_fs, event_bus)

                    await root_nursery.start(backend_listen_events, device, event_bus)

                    async with mountpoint_manager_factory(
                        user_fs,
                        event_bus,
                        config.mountpoint_base_dir,
                        enabled=config.mountpoint_enabled,
                    ) as mountpoint_manager:

                        yield LoggedCore(
                            config=config,
                            device=device,
                            event_bus=event_bus,
                            remote_devices_manager=remote_devices_manager,
                            mountpoint_manager=mountpoint_manager,
                            backend_cmds=backend_cmds_pool,
                            user_fs=user_fs,
                        )
                    monitor_nursery.cancel_scope.cancel()
        root_nursery.cancel_scope.cancel()
