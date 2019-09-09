# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class RemoteDevicesManagerError(Exception):
    pass


class RemoteDevicesManagerBackendOfflineError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerValidationError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerPackingError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerNotFoundError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerInvalidTrustchainError(RemoteDevicesManagerError):
    pass
