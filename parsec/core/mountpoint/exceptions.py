class MountpointManagerError(Exception):
    pass


class MountpointManagerNotAvailable(MountpointManagerError):
    pass


class MountpointAlreadyStarted(MountpointManagerError):
    pass


class MountpointNotStarted(MountpointManagerError):
    pass


class MountpointConfigurationError(MountpointManagerError):
    pass
