class BackendConnectionError(Exception):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass
