class MountpointManagerError(Exception):
    pass


class MountpointManagerNotAvailable(MountpointManagerError):
    pass


class MountpointConfigurationError(MountpointManagerError):
    pass
