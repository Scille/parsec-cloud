import trio
from unittest.mock import Mock
from inspect import iscoroutinefunction
from async_generator import asynccontextmanager

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
        spec = kwargs.get("spec")
        if spec:
            for field in dir(spec):
                if iscoroutinefunction(getattr(spec, field)):
                    getattr(self, field).is_async = True

    async def __async_call(self, *args, **kwargs):
        return super().__call__(*args, **kwargs)

    def __call__(self, *args, **kwargs):
        if getattr(self, "is_async", False) is True:
            if iscoroutinefunction(self.side_effect):
                return self.side_effect(*args, **kwargs)

            else:
                return self.__async_call(*args, **kwargs)

        else:
            return super().__call__(*args, **kwargs)


class FreezeTestOnBrokenStreamCookedSocket(CookedSocket):
    """
    When a server crashes during test, it is possible the client coroutine
    receives a `trio.BrokenStreamError` exception. Hence we end up with two
    exceptions: the server crash (i.e. the original exception we are interested
    into) and the client not receiving an answer.
    The solution is simply to freeze the coroutine receiving the broken stream
    error until it will be cancelled by the original exception bubbling up.
    """

    async def send(self, msg):
        try:
            return await super().send(msg)

        except trio.BrokenStreamError as exc:
            # Wait here until this coroutine is cancelled
            await trio.sleep_forever()

    async def recv(self):
        try:
            return await super().recv()

        except trio.BrokenStreamError as exc:
            # Wait here until this coroutine is cancelled
            await trio.sleep_forever()


@asynccontextmanager
async def run_app(app):
    async with trio.open_nursery() as nursery:

        async def connection_factory(*args, **kwargs):
            right, left = trio.testing.memory_stream_pair()
            nursery.start_soon(app.handle_client, left)
            return right

        try:
            yield connection_factory

        finally:
            nursery.cancel_scope.cancel()


@asynccontextmanager
async def backend_factory(**config):
    config = BackendConfig(**config)
    backend = BackendApp(config)
    async with trio.open_nursery() as nursery:
        await backend.init(nursery)
        try:
            yield backend

        finally:
            await backend.shutdown()
            nursery.cancel_scope.cancel()


@asynccontextmanager
async def connect_backend(backend, auth_as=None):
    async with run_app(backend) as connection_factory:
        sockstream = await connection_factory()
        sock = FreezeTestOnBrokenStreamCookedSocket(sockstream)
        if auth_as:
            # Handshake
            if auth_as == "anonymous":
                ch = AnonymousClientHandshake()
            else:
                ch = ClientHandshake(auth_as.id, auth_as.device_signkey)
            challenge_req = await sock.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await sock.send(answer_req)
            result_req = await sock.recv()
            ch.process_result_req(result_req)

        yield sock


@asynccontextmanager
async def connect_core(core):
    async with run_app(core) as connection_factory:
        sockstream = await connection_factory()
        sock = FreezeTestOnBrokenStreamCookedSocket(sockstream)

        yield sock


@asynccontextmanager
async def core_factory(**config):
    config = CoreConfig(**config)
    core = CoreApp(config)
    async with trio.open_nursery() as nursery:
        await core.init(nursery)
        try:
            yield core

        finally:
            await core.shutdown()
            nursery.cancel_scope.cancel()
