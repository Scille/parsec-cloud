# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class BackendConnectionError(Exception):
    pass


class BackendProtocolError(BackendConnectionError):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass


class BackendConnectionRefused(BackendConnectionError):
    pass
