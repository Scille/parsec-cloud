import pytest
import json
from unittest.mock import patch
import asyncio

from parsec.server import BaseServer, BaseClientContext
from parsec.backend import InMemoryPubKeyService
from parsec.crypto import RSAPrivateKey, RSAPublicKey
from parsec.session import AuthSession
from parsec.exceptions import PubKeyError, PubKeyNotFound

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


ALICE_PRIVATE_RSA = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
/6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
+tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
-----END RSA PRIVATE KEY-----
"""
ALICE_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""
BOB_PRIVATE_RSA = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDCqVQVdVhJqW9rrbObvDZ4ET6FoIyVn6ldWhOJaycMeFYBN3t+
cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S3rAV6JhDKDfDFrkzUVZmcc/g
HhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOKG889JN85nABKR9WkdwIDAQAB
AoGBAJFU3Dr9FgJA5rfMwpiV51CzByu61trqjgbtNkLVZhzwRr23z5Jxmd+yLHik
J6ia6sYvdUuHFLKQegGt/2xOjXn8UBpa725gLojHn2umtJDL7amTlBwiJfNXuZrF
BSKK9+xZnNDWMq1IuCqPeintbve+MNSc62JYuGGtXSz9L5f5AkEA/xBkUksBfEUl
65oEPgxvMKHNjLq48otRmCaG+i3MuQqTYQ+c8Z/l26yQL4OV2b36a8/tTaLhwhAZ
Ibtv05NKfQJBAMNgMbOsUWpY8A1Cec79Oj6RVe79E5ciZ4mW3lx5tjJRyNxwlQag
u+T6SwBIa6xMfLBQeizzxqXqxAyW/riQ6QMCQQCadUu7Re6tWZaAGTGufYsr8R/v
s/dh8ZpEwDgG8otCFzRul6zb6Y+huttJ2q55QIGQnka/N/7srSD6+23Zux1lAkBx
P30PzL6UimD7DqFUnev5AH1zPjbwz/x8AHt71wEJQebQAGIhqWHAZGS9ET14bg2I
ld172QI4glCJi6yyhyzJAkBzfmHZEE8FyLCz4z6b+Z2ghMds2Xz7RwgVqCIXt9Ku
P7Bq0eXXgyaBo+jpr3h4K7QnPh+PaHSlGqSfczZ6GIpx
-----END RSA PRIVATE KEY-----
"""
BOB_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCqVQVdVhJqW9rrbObvDZ4ET6F
oIyVn6ldWhOJaycMeFYBN3t+cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S
3rAV6JhDKDfDFrkzUVZmcc/gHhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOK
G889JN85nABKR9WkdwIDAQAB
-----END PUBLIC KEY-----
"""


class MockedContext(BaseClientContext):

    def __init__(self, expected_send=[], to_recv=[]):
        self.expected_send = list(reversed(expected_send))
        self.to_recv = list(reversed(to_recv))

    async def recv(self):
        assert self.to_recv, 'No more message should be received'
        return self.to_recv.pop()

    async def send(self, body):
        assert self.expected_send, 'Unexpected message %s (no more should be send)' % body
        expected = self.expected_send.pop()
        assert json.loads(body) == json.loads(expected)


async def bootstrap_PostgreSQLPubKeyService(request, event_loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    server = BaseServer()
    server.register_service(module.PostgreSQLService(url))
    msg_svc = module.PostgreSQLPubKeyService()
    server.register_service(msg_svc)
    await server.bootstrap_services()

    def finalize():
        event_loop.run_until_complete(server.teardown_services())

    request.addfinalizer(finalize)
    return msg_svc


@pytest.fixture(params=[InMemoryPubKeyService, bootstrap_PostgreSQLPubKeyService],
                ids=['in_memory', 'postgresql'])
def pubkey_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
@pytest.mark.asyncio
async def alice(pubkey_svc):
    await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
    return {
        'id': 'alice',
        'private_key': RSAPrivateKey(ALICE_PRIVATE_RSA),
        'public_key': RSAPublicKey(ALICE_PUBLIC_RSA)
    }


class TestPubKeyService:

    @pytest.mark.asyncio
    async def test_add_and_get(self, pubkey_svc):
        await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
        key = await pubkey_svc.get_pubkey('alice', raw=True)
        assert key == ALICE_PUBLIC_RSA
        key = await pubkey_svc.get_pubkey('alice')
        assert isinstance(key, RSAPublicKey)

    @pytest.mark.asyncio
    async def test_multiple_add(self, pubkey_svc):
        await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
        with pytest.raises(PubKeyError):
            await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)

    @pytest.mark.asyncio
    async def test_get_missing(self, pubkey_svc):
        with pytest.raises(PubKeyNotFound):
            await pubkey_svc.get_pubkey('alice')

    @pytest.mark.asyncio
    async def test_handshake(self, pubkey_svc, alice):
        with patch('parsec.backend.pubkey_service._generate_challenge', new=lambda: "DUMMY_CHALLENGE"):
            expected_send = [
                '{"handshake": "challenge", "challenge": "DUMMY_CHALLENGE"}',
                '{"status": "ok", "handshake": "done"}',
            ]
            to_recv = [
                '{"handshake": "answer", "identity": "alice", '
                '"answer": "R2MF9UXaBEFPeG/HVPV9k9jputDnJXwOm5JzfAehPZfAzJsR48ailp6mfnTl5kGvTN3nCvyiX0DZMUm1JyWv/puOTbQGJk66S/uQtSuA+zOgeLwnynC9hbuj16OkUuAUg7oyqSATihFvudGpPWLHWR6rlhtUeON1sbcedg3vRzA="}'

            ]
            context = MockedContext(expected_send=expected_send, to_recv=to_recv)
            session = await pubkey_svc.handshake(context)
            assert isinstance(session, AuthSession)
            assert session.identity == 'alice'

    # TODO: test bad handshake as well


class TestPubKeyServiceAPI:
    @pytest.mark.asyncio
    async def test_get(self, pubkey_svc):
        await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)

        cmd = {'cmd': 'pubkey_get', 'id': 'alice'}
        ret = await pubkey_svc.dispatch_msg(cmd)
        assert ret['status'] == 'ok'

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_cmd', [
        {'cmd': 'pubkey_get', 'id': None},
        {'cmd': 'pubkey_get', 'id': 42},
        {'cmd': 'pubkey_get', 'id': 'alice', 'dummy': 'field'},
        {'cmd': 'pubkey_get'},
        {}])
    async def test_bad_get(self, pubkey_svc, bad_cmd):
        await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)

        ret = await pubkey_svc.dispatch_msg(bad_cmd)
        assert ret['status'] == 'bad_msg'
