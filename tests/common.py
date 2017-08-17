import pytest
from base64 import encodebytes
from unittest.mock import Mock


class AsyncMock(Mock):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.is_asyncret = False

    def set_asyncret(self, *args):
        self.is_asyncret = True
        if args:
            self.return_value = args[0]

    def __call__(self, *args, **kwargs):
        ret = super().__call__(*args, **kwargs)
        if self.is_asyncret:
            async def coro():
                return ret
            return coro()
        else:
            return ret

    async def __aexit__(self, exc_type, exc, tb):
        pass

    async def __aenter__(self, *args, **kwargse):
        return self.aenter


def b64(raw):
    return encodebytes(raw).decode()


def can_side_effect_or_skip():
    if pytest.config.getoption('tx'):
        pytest.skip('Cannot run test with side effects with xdist concurrency')
