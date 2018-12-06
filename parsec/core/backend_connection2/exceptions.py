from abc import ABC

from parsec.api.protocole import ProtocoleError
from parsec.api.transport import TransportError


class BackendConnectionError(Exception, ABC):
    pass


BackendConnectionError.register(ProtocoleError)
BackendConnectionError.register(TransportError)


class BackendNotAvailable(BackendConnectionError):
    pass
