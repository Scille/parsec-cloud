import pytest
from base64 import encodebytes

from parsec.server.base import BaseClientContext


def b64(raw):
    return encodebytes(raw).decode()


class MockedContext(BaseClientContext):
    def __init__(self, on_recv=None, on_send=None):
        super().__init__()
        self._on_recv = on_recv
        self._on_send = on_send

    async def recv(self):
        if self._on_recv:
            return await self._on_recv()

    async def send(self, body):
        if self._on_send:
            await self._on_send(body)


def can_side_effect_or_skip():
    if pytest.config.getoption('tx'):
        pytest.skip('Cannot run test with side effects with xdist concurrency')
