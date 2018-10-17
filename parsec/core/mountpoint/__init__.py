from parsec.core.mountpoint.manager import FuseMountpointManager, FUSE_AVAILABLE
from parsec.core.mountpoint.not_available import NotAvailableMountpointManager
from parsec.core.mountpoint.exceptions import (
    MountpointManagerError,
    MountpointManagerNotAvailable,
    MountpointAlreadyStarted,
    MountpointNotStarted,
    MountpointConfigurationError,
)


def mountpoint_manager_factory(fs, event_bus, **kwargs):
    if FUSE_AVAILABLE:
        return FuseMountpointManager(fs, event_bus, **kwargs)
    else:
        return NotAvailableMountpointManager(fs, event_bus, **kwargs)


__all__ = (
    "mountpoint_manager_factory",
    "MountpointManagerError",
    "MountpointManagerNotAvailable",
    "MountpointAlreadyStarted",
    "MountpointNotStarted",
    "MountpointConfigurationError",
)
