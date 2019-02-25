# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.mountpoint.base import BaseMountpointManager
from parsec.core.mountpoint.exceptions import MountpointManagerNotAvailable


class NotAvailableMountpointManager(BaseMountpointManager):
    def __init__(self, fs, event_bus, mode="thread", debug: bool = True, nothreads: bool = False):
        pass

    @property
    def mountpoint(self):
        return None

    def get_abs_mountpoint(self):
        raise MountpointManagerNotAvailable("Mountpoint manager not available")

    async def init(self, nursery):
        pass

    def is_started(self):
        raise MountpointManagerNotAvailable("Mountpoint manager not available")

    async def start(self, mountpoint):
        raise MountpointManagerNotAvailable("Mountpoint manager not available")

    async def stop(self):
        raise MountpointManagerNotAvailable("Mountpoint manager not available")

    async def teardown(self):
        pass
