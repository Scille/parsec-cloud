# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class MountpointManagerError(Exception):
    pass


class MountpointManagerNotAvailable(MountpointManagerError):
    pass


class MountpointConfigurationError(MountpointManagerError):
    pass
