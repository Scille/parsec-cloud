import trio
from trio._util import acontextmanager
from unittest.mock import Mock
from inspect import iscoroutinefunction

try:
    from libfaketime import freeze_time
except ImportError:
    from freezegun import freeze_time

from parsec.core import CoreApp, CoreConfig

from parsec.handshake import ClientHandshake, AnonymousClientHandshake
from parsec.utils import CookedSocket

from parsec.backend import BackendApp, BackendConfig


class AsyncMock(Mock):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        spec = kwargs.get('spec')
        if spec:
            for field in dir(spec):
                if iscoroutinefunction(getattr(spec, field)):
                    getattr(self, field).is_async = True

    async def __async_call(self, *args, **kwargs):
        return super().__call__(*args, **kwargs)

    def __call__(self, *args, **kwargs):
        if getattr(self, 'is_async', False) is True:
            if iscoroutinefunction(self.side_effect):
                return self.side_effect(*args, **kwargs)
            else:
                return self.__async_call(*args, **kwargs)
        else:
            return super().__call__(*args, **kwargs)


class QuitTestDueToBrokenStream(Exception):
    pass


class QuitTestOnBrokenStreamCookedSocket(CookedSocket):
    """
    When a server crashes during test, it is possible the client coroutine
    receives a `trio.BrokenStreamError` exception. Hence we end up with two
    exceptions: the server crash (i.e. the original exception we are interested
    into) and the client not receiving an answer.
    """

    async def send(self, msg):
        try:
            return await super().send(msg)
        except trio.BrokenStreamError as exc:
            raise QuitTestDueToBrokenStream() from exc

    async def recv(self):
        try:
            return await super().recv()
        except trio.BrokenStreamError as exc:
            raise QuitTestDueToBrokenStream() from exc


@acontextmanager
async def run_app(app):
    async with trio.open_nursery() as nursery:

        async def connection_factory():
            right, left = trio.testing.memory_stream_pair()
            nursery.start_soon(app.handle_client, left)
            return right

        yield connection_factory

        nursery.cancel_scope.cancel()


@acontextmanager
async def backend_factory(**config):
    config = BackendConfig(**config)
    backend = BackendApp(config)
    async with trio.open_nursery() as nursery:
        await backend.init(nursery)

        yield backend

        await backend.shutdown()


@acontextmanager
async def connect_backend(backend, auth_as=None):
    async with run_app(backend) as connection_factory:
        sockstream = await connection_factory()
        sock = QuitTestOnBrokenStreamCookedSocket(sockstream)
        try:

            if auth_as:
                # Handshake
                if auth_as == 'anonymous':
                    ch = AnonymousClientHandshake()
                else:
                    ch = ClientHandshake(auth_as.id, auth_as.device_signkey)
                challenge_req = await sock.recv()
                answer_req = ch.process_challenge_req(challenge_req)
                await sock.send(answer_req)
                result_req = await sock.recv()
                ch.process_result_req(result_req)

            yield sock
        except QuitTestDueToBrokenStream:
            # Exception should be raised from the handle_client coroutine in nursery
            pass


@acontextmanager
async def connect_core(core, swallow_broken_stream=True):
    async with run_app(core) as connection_factory:
        sockstream = await connection_factory()
        sock = QuitTestOnBrokenStreamCookedSocket(sockstream)
        try:
            yield sock
        except QuitTestDueToBrokenStream:
            if not swallow_broken_stream:
                raise


@acontextmanager
async def core_factory(**config):
    config = CoreConfig(**config)
    core = CoreApp(config)
    async with trio.open_nursery() as nursery:
        await core.init(nursery)

        yield core

        await core.shutdown()
