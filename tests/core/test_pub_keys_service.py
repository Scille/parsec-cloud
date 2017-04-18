# import shutil
import json
from os import path
import unittest

# from asynctest import mock
import gnupg
import pytest

from parsec.session import AuthSession
from parsec.core import GNUPGPubKeysService
from parsec.exceptions import HandshakeError

from tests.common import MockedContext


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def alice_gpg():
    gpg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    gpg.identity = gpg.list_keys(secret=True)[0]['keyid']
    return gpg


@pytest.fixture
def bob_pubkey_srv():
    return GNUPGPubKeysService(homedir=GNUPG_HOME + '/pub_env')


class TestGNUPGPubKeysHandshake:
    @pytest.mark.asyncio
    async def test_success(self, alice_gpg, bob_pubkey_srv):
        # Alice is the client, Bob the server
        answer = 'not computed yet...'
        events = []

        async def on_send(body):
            # 1) Backend send challenge
            msg = json.loads(body.decode())
            assert msg['handshake'] == 'challenge'
            assert msg['challenge']
            events.append(('send', msg))
            nonlocal answer
            answer = alice_gpg.sign(msg['challenge']).data.decode()

        async def on_recv():
            # 2) Client respond to the challenge
            msg = {'handshake': 'answer', 'answer': answer, 'identity': alice_gpg.identity}
            events.append(('recv', msg))
            return json.dumps(msg).encode()

        context = MockedContext(on_recv, on_send)
        session = await bob_pubkey_srv.handshake(context)
        assert len(events) == 2
        assert isinstance(session, AuthSession)
        assert session.identity == alice_gpg.identity

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_answer', [
        b'', b'not a json',
        b'{"handshake": "answer", "identity": "<identity>", "answer": "<answer>", "foo": 42}',
        b'{"handshake": "answer", "identity": "<identity>", "answer": "<answer>"}',
        b'{"handshake": "answer", "identity": "645EFFC0", "answer": "<answer>"}',
        b'{"handshake": "answer", "identity": "<identity>", "answer": "<bad_answer>"}',
        b'{"handshake": "answer", "identity": "<identity>"}',
        b'{"handshake": "answer", "answer": "<answer>"}',
        b'{"identity": "<identity>", "answer": "<answer>"}',
    ])
    @unittest.mock.patch('parsec.core.pub_keys_service._generate_challenge',
                         new=lambda: 'MyChallenge')
    async def test_bad_answer(self, alice_gpg, bob_pubkey_srv, bad_answer):
        cooked = bad_answer
        cooked = cooked.replace(b'<identity>', alice_gpg.identity.encode())
        answer = alice_gpg.sign('MyChallenge').data.replace(b'\n', b'\\n')
        cooked = cooked.replace(b'<answer>', answer)
        bad_answer = alice_gpg.sign('DummyStuff').data.replace(b'\n', b'\\n')
        cooked = cooked.replace(b'<bad_answer>', bad_answer)

        async def on_recv():
            return cooked

        context = MockedContext(on_recv)
        with pytest.raises(HandshakeError):
            await bob_pubkey_srv.handshake(context)
