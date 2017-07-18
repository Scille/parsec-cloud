from asyncio import Queue
from websockets import ConnectionClosed  # noqa: use this exception as our own


class BaseSession:
    def __init__(self, context=None):
        self.context = context
        self.received_events = Queue()

    @property
    def identity(self):
        raise NotImplementedError()


class AnonymousSession(BaseSession):
    @property
    def identity(self):
        raise RuntimeError('AnonymousSession has no identity.')


class AuthSession(BaseSession):
    identity = None  # Shadow the parent property

    def __init__(self, context, identity):
        super().__init__(context)
        self.identity = identity


async def anonymous_handshake(context):
    return AnonymousSession(context)


__all__ = ('ConnectionClosed', 'BaseSession', 'AnonymousSession',
           'AuthSession', 'anonymous_handshake')
