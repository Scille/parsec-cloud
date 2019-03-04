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
    backend_cmds_factory,
    backend_listen_events,
    monitor_backend_connection,
)
from parsec.core.mountpoint import mountpoint_manager_factory
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.vlob_groups_monitor import monitor_vlob_groups
from parsec.core.messages_monitor import monitor_messages
from parsec.core.sync_monitor import monitor_sync
from parsec.core.fs import FS
from parsec.core.local_db import LocalDB


logger = get_logger()


@attr.s(frozen=True, slots=True)
class LoggedCore:
    config = attr.ib()
    device = attr.ib()
    local_db = attr.ib()
    event_bus = attr.ib()
    encryption_manager = attr.ib()
    mountpoint_manager = attr.ib()
    backend_cmds = attr.ib()
    fs = attr.ib()


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
        backend_online = await root_nursery.start(backend_listen_events, device, event_bus)

        async with backend_cmds_factory(
            device.organization_addr,
            device.device_id,
            device.signing_key,
            config.backend_max_connections,
        ) as backend_cmds_pool:

            local_db = LocalDB(config.data_base_dir / device.device_id)

            encryption_manager = EncryptionManager(device, local_db, backend_cmds_pool)
            fs = FS(device, local_db, backend_cmds_pool, encryption_manager, event_bus)

            async with trio.open_nursery() as monitor_nursery:
                # Finally start monitors

                # Monitor connection must be first given it will watch on
                # other monitors' events
                await monitor_nursery.start(monitor_backend_connection, backend_online, event_bus)
                await monitor_nursery.start(monitor_vlob_groups, device, fs, event_bus)
                await monitor_nursery.start(monitor_messages, backend_online, fs, event_bus)
                await monitor_nursery.start(monitor_sync, backend_online, fs, event_bus)

                async with mountpoint_manager_factory(
                    fs, event_bus, config.mountpoint_base_dir, enabled=config.mountpoint_enabled
                ) as mountpoint_manager:

                    yield LoggedCore(
                        config=config,
                        device=device,
                        local_db=local_db,
                        event_bus=event_bus,
                        encryption_manager=encryption_manager,
                        mountpoint_manager=mountpoint_manager,
                        backend_cmds=backend_cmds_pool,
                        fs=fs,
                    )
                    root_nursery.cancel_scope.cancel()
