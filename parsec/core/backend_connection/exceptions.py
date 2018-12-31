class BackendConnectionError(Exception):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass


class BackendHandshakeError(BackendConnectionError):
    pass


class BackendDeviceRevokedError(BackendHandshakeError):
    pass


class BackendCmdsInvalidRequest(BackendConnectionError):
    pass


class BackendCmdsInvalidResponse(BackendConnectionError):
    pass


class BackendCmdsBadResponse(BackendConnectionError):
    @property
    def status(self):
        return self.args[0]["status"]
