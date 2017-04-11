from asyncio import Queue


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
