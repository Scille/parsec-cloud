# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class MountpointError(Exception):
    pass


class MountpointDriverCrash(MountpointError):
    pass


class MountpointAlreadyMounted(MountpointError):
    pass


class MountpointNotMounted(MountpointError):
    pass


class MountpointNotAvailable(MountpointError):
    pass


class MountpointConfigurationError(MountpointError):
    pass


class MountpointConfigurationWorkspaceFSTimestampedError(MountpointConfigurationError):
    pass


class MountpointWinfspNotAvailable(MountpointError):
    pass


class MountpointFuseNotAvailable(MountpointError):
    pass


class MountpointNoDriveAvailable(MountpointError):
    pass
