# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import Optional
from structlog import get_logger
from functools import partial
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.backend_connection import BackendAuthenticatedConn
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
    backend_conn = attr.ib()
    user_fs = attr.ib()

    @property
    def backend_cmds(self):
        return self.backend_conn.cmds

    def are_monitors_idle(self):
        return self.backend_conn.are_monitors_idle()

    async def wait_idle_monitors(self):
        await self.backend_conn.wait_idle_monitors()


@asynccontextmanager
async def logged_core_factory(
    config: CoreConfig, device: LocalDevice, event_bus: Optional[EventBus] = None
):
    event_bus = event_bus or EventBus()

    backend_conn = BackendAuthenticatedConn(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        event_bus=event_bus,
        max_cooldown=config.backend_max_cooldown,
        max_pool=config.backend_max_connections,
        keepalive=config.backend_connection_keepalive,
    )

    path = config.data_base_dir / device.slug
    remote_devices_manager = RemoteDevicesManager(backend_conn.cmds, device.root_verify_key)
    async with UserFS.run(
        device, path, backend_conn.cmds, remote_devices_manager, event_bus
    ) as user_fs:

        backend_conn.register_monitor(partial(monitor_messages, user_fs, event_bus))
        backend_conn.register_monitor(partial(monitor_sync, user_fs, event_bus))

        async with backend_conn.run():
            async with mountpoint_manager_factory(
                user_fs,
                event_bus,
                config.mountpoint_base_dir,
                mount_all=config.mountpoint_enabled,
                mount_on_workspace_created=config.mountpoint_enabled,
                mount_on_workspace_shared=config.mountpoint_enabled,
                unmount_on_workspace_revoked=config.mountpoint_enabled,
                exclude_from_mount_all=config.disabled_workspaces,
            ) as mountpoint_manager:

                yield LoggedCore(
                    config=config,
                    device=device,
                    event_bus=event_bus,
                    remote_devices_manager=remote_devices_manager,
                    mountpoint_manager=mountpoint_manager,
                    backend_conn=backend_conn,
                    user_fs=user_fs,
                )
