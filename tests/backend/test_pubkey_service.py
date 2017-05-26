import pytest
import json
from unittest.mock import patch
import asyncio

from parsec.server import BaseServer, BaseClientContext
from parsec.backend import MockedPubKeyService
from parsec.crypto import RSAPrivateKey, RSAPublicKey
from parsec.session import AuthSession
from parsec.exceptions import PubKeyError, PubKeyNotFound

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


ALICE_PRIVATE_RSA = b"""
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAsSle/x4jtr+kaxiv
9BYlL+gffH/VLC+Q/WTTB+1FIU1fdmgdZVGaIlAWJHqr9qEZBfwXYzlQv8pIMn+W
5pqvHQICAQECQDv5FjKAvWW1a3Twc1ia67eQUmDugu8VFTDsV2BRUS0jlxJ0yCL+
TEBpOwH95TFgvfRBYee97APHjhvLLlzmEyECIQDZdjMg/j9N7sw602DER0ciERPI
Ps9rU8RqRXaWPYtWOQIhANCO1h/z7iFjlpENbKDOCinfsXd9ulVsoNYWhKm58gAF
AiEAzMT3XdKFUlljq+/hl/Nt0GPA8lkHDGjG5ZAaAAYnj/ECIQCB125lkuHy61LH
4INhH6azeFaUGnn7aHwJxE6beL6BgQIhALbajJWsBf5LmeO190adM2jAVN94YqVD
aOrHGFFqrjJ3
-----END PRIVATE KEY-----
"""
ALICE_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBALEpXv8eI7a/pGsYr/QWJS/oH3x/1Swv
kP1k0wftRSFNX3ZoHWVRmiJQFiR6q/ahGQX8F2M5UL/KSDJ/luaarx0CAgEB
-----END PUBLIC KEY-----
"""

BOB_PRIVATE_RSA = b"""
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAyKLdYjpibsnTCBcw
eEPZF0n+VemdLrmxh2j0lDBnHaKECR3y03uLUHL81HFEsvaEK68fwL8pOnMAXYq+
137/SwICAQECQQCKLmykRhf2ougAl3ESFNRE1VOF4KUIRh2g/pKH7YnBumgyh0cp
I57woQBc6rviSTRdK5oFNpNLDiUJpez0rLbZAiEA+C85pI1TGy+JoXcaYqKqFfuj
qr96DC8IlEmXdd5V7xUCIQDO9FKBtRdt3wEIZZzP0GoYCDiffsM/8lL95K0ID2PM
3wIhAL81UUQBTPoMt7sm93DY9pZqNl+wZ/1u8bD/6znwBnB5AiACanKFAB8nInqI
kKA2OTgGQdbTC3DY5vAI8L1Jzl/LmwIhALEV7Xb8MqhT/Li7PgkBzEmjxgZWBX8w
jrL/lrN8BM6W
-----END PRIVATE KEY-----
"""
BOB_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBAMii3WI6Ym7J0wgXMHhD2RdJ/lXpnS65
sYdo9JQwZx2ihAkd8tN7i1By/NRxRLL2hCuvH8C/KTpzAF2Kvtd+/0sCAgEB
-----END PUBLIC KEY-----
"""


class MockedContext(BaseClientContext):


    def __init__(self, expected_send=[], to_recv=[]):
        self.expected_send = expected_send
        self.to_recv = to_recv

    async def recv(self):
        assert self.to_recv, 'No more message should be received'
        return self.to_recv.pop()

    async def send(self, body):
        assert self.expected_send, 'Unexpected message %s (no more should be send)' % body
        expected = self.expected_send.pop()
        assert json.loads(body.decode()) == json.loads(expected.decode())


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


@pytest.fixture(params=[MockedPubKeyService, bootstrap_PostgreSQLPubKeyService],
                ids=['in_memory', 'postgresql'])
def pubkey_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
@pytest.mark.asyncio
async def alice(pubkey_svc):
    pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
    return {
        'id': 'alice',
        'private_key': RSAPrivateKey(ALICE_PRIVATE_RSA),
        'public_key': RSAPublicKey(ALICE_PUBLIC_RSA)
    }


class TestPubKeyService:

    @pytest.mark.asyncio
    async def test_add_and_get(self, pubkey_svc):
        await pubkey_svc.add_pubkey('alice', ALICE_PUBLIC_RSA)
        key = await pubkey_svc.get_pubkey('alice')
        assert key == ALICE_PUBLIC_RSA

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
                b'{"handshake": "challenge", "challenge": "DUMMY_CHALLENGE"}',
            ]
            challenge_signature = alice['private_key'].sign(b"DUMMY_CHALLENGE")
            to_recv = [
                b'{"handshake": "answer", "identity": "alice", "answer": "%s"}' % challenge_signature
            ]
            context = MockedContext(expected_send=expected_send, to_recv=to_recv)
            session = await pubkey_svc.handshake(context)
            assert isinstance(session, AuthSession)
            assert session.identity == 'alice'


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
