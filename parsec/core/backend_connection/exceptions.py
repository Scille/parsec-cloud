class BackendConnectionError(Exception):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass


class BackendHandshakeError(BackendConnectionError):
    pass


class BackendDeviceRevokedError(BackendHandshakeError):
    pass
