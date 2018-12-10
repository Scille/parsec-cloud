from abc import ABC

from parsec.api.protocole import ProtocoleError
from parsec.api.transport import TransportError


class BackendConnectionError(Exception, ABC):
    pass


BackendConnectionError.register(ProtocoleError)


class BackendNotAvailable(BackendConnectionError, ABC):
    pass


BackendNotAvailable.register(TransportError)
