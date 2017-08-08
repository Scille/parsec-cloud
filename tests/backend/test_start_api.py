import pytest
from aiohttp import web
from effect2 import ComposedDispatcher

from parsec.backend.start_api import register_start_api
from parsec.backend.pubkey import MockedPubKeyComponent
from parsec.backend.privkey import MockedPrivKeyComponent


@pytest.fixture
def pubkey():
    return MockedPubKeyComponent()


@pytest.fixture
def privkey():
    return MockedPrivKeyComponent()


@pytest.fixture
def app(privkey, pubkey):
    dispatcher = ComposedDispatcher([pubkey.get_dispatcher(), privkey.get_dispatcher()])
    app = web.Application()
    register_start_api(app, dispatcher)
    return app


@pytest.fixture
def client(loop, test_client, app):
    return loop.run_until_complete(test_client(app))


class TestPubKeyAPI:
    async def test_new_pubkey(self, client, pubkey):
        ret = await client.post('/start/pubkey/alice', data=b'<alice-key>')
        assert ret.status == 200
        assert pubkey._keys == {'alice': b'<alice-key>'}


    async def test_new_pubkey_already_exists(self, client, pubkey):
        pubkey._keys = {'alice': b'<alice-key>'}
        ret = await client.post('/start/pubkey/alice', data=b'<bob-key>')
        assert ret.status == 409
        assert pubkey._keys == {'alice': b'<alice-key>'}


class TestCipherKeyAPI:
    async def test_new_cipherkey(self, client, privkey):
        ret = await client.post('/start/cipherkey/1337ABC', data=b'<alice-cipher-key>')
        assert ret.status == 200
        assert privkey._keys == {'1337ABC': b'<alice-cipher-key>'}

    async def test_new_cipherkey_collision(self, client, privkey):
        privkey._keys = {'424242': b'<alice-cipher-key>'}
        ret = await client.post('/start/cipherkey/424242', data=b'<bob-cipher-key>')
        assert ret.status == 409
        assert privkey._keys == {'424242': b'<alice-cipher-key>'}

    async def test_get_cipherkey(self, client, privkey):
        privkey._keys = {'424242': b'<alice-cipher-key>'}
        ret = await client.get('/start/cipherkey/424242')
        assert ret.status == 200
        cipherkey = await ret.content.read()
        assert cipherkey == b'<alice-cipher-key>'

    async def test_get_unknown_cipherkey(self, client, privkey):
        ret = await client.get('/start/cipherkey/0000')
        assert ret.status == 404
