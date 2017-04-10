# import shutil
import json
from os import path
import unittest

# from asynctest import mock
import gnupg
import pytest

from parsec.server import BaseServer, BaseClientContext
from parsec.session import AuthSession
from parsec.core import CryptoService, GNUPGPubKeysService
from parsec.exceptions import HandshakeError


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


class MockedContext(BaseClientContext):
    def __init__(self, on_recv=None, on_send=None):
        self._on_recv = on_recv
        self._on_send = on_send

    async def recv(self):
        if self._on_recv:
            return self._on_recv()

    async def send(self, body):
        if self._on_send:
            self._on_send(body)


@pytest.fixture
def alice_gpg():
    gpg = gnupg.GPG(homedir=GNUPG_HOME + '/alice')
    gpg.identity = gpg.list_keys(secret=True)[0]['keyid']
    return gpg


@pytest.fixture
def bob_pubkey_srv():
    return GNUPGPubKeysService(homedir=GNUPG_HOME + '/bob')


class TestGNUPGPubKeysHandshake:
    @pytest.mark.asyncio
    async def test_success(self, alice_gpg, bob_pubkey_srv):
        # Alice is the client, Bob the server
        answer = 'not computed yet...'
        events = []

        def on_send(body):
            # 1) Backend send challenge
            msg = json.loads(body.decode())
            assert msg['handshake'] == 'challenge'
            assert msg['challenge']
            events.append(('send', msg))
            nonlocal answer
            answer = alice_gpg.sign(msg['challenge']).data.decode()

        def on_recv():
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
    @unittest.mock.patch('parsec.core.pub_keys_service._generate_challenge', new=lambda: 'MyChallenge')
    async def test_bad_answer(self, alice_gpg, bob_pubkey_srv, bad_answer):
        cooked = bad_answer
        cooked = cooked.replace(b'<identity>', alice_gpg.identity.encode())
        answer = alice_gpg.sign('MyChallenge').data.replace(b'\n', b'\\n')
        cooked = cooked.replace(b'<answer>', answer)
        bad_answer = alice_gpg.sign('DummyStuff').data.replace(b'\n', b'\\n')
        cooked = cooked.replace(b'<bad_answer>', bad_answer)

        def on_recv():
            return cooked

        context = MockedContext(on_recv)
        with pytest.raises(HandshakeError):
            await bob_pubkey_srv.handshake(context)


# class BaseTestPubKeysService:

#     # Helpers

#     # Tests

#     @pytest.mark.asyncio
#     async def test_get_pub_key(self):
#         # Identity not in gpg cache and unknown in Keybase
#         ret = await self.service.dispatch_msg({'cmd': 'get_pub_key', 'identity': 'unknown'})
#         assert ret == {'status': 'not_found', 'label': 'Identity not found in Keybase.'}
#         # Identity not in gpg cache and fetch from Keybase
#         identity = '08B3DE7C93738156219ECA52C360860E645EFFC0'  # touilleman's fingerprint
#         ret = await self.service.dispatch_msg({'cmd': 'get_pub_key', 'identity': identity})
#         assert ret['status'] == 'ok'
#         assert '-----BEGIN PGP PUBLIC KEY BLOCK-----' in ret['pub_key']
#         assert '-----END PGP PUBLIC KEY BLOCK-----' in ret['pub_key']
#         # Identity in gpg cache
#         ret = await self.service.dispatch_msg({'cmd': 'get_pub_key', 'identity': self.fingerprint})
#         assert ret['status'] == 'ok'
#         assert '-----BEGIN PGP PUBLIC KEY BLOCK-----' in ret['pub_key']
#         assert '-----END PGP PUBLIC KEY BLOCK-----' in ret['pub_key']


# class TestPubKeysService(BaseTestPubKeysService):

#     def setup_method(self, gpg):
#         self.fingerprint = '8D74B53B7580166E47E244BB1B3C781556A044F7'
#         # test1@domain.com
#         self.key = """-----BEGIN PGP PRIVATE KEY BLOCK-----
#         Version: GnuPG v1

#         lQIGBFjnXZ8BBADROLKNyhkDuNYz4ybUanhmt8t3r46nNovXFI9ylo+drOFc62hd
#         mlieXq2no+lffVWGi2MjY2lkbA3GsVON7XYn2lzK6+Bd2dQaO34uCaNmLidrJ9jz
#         aSF9TYpBgp5J9gU6kH1xUcElU/C4BOSC9SCUrTwu7iYI0DDKAiD4owgOswARAQAB
#         /gcDAo3HiOpbxevrYE/ZjV7RpVTVzQRuf1zoU7VHCrqLvLjWWOPNM66RjhcqlB2O
#         0gnFBR8Bl/3cwJ/S0qhrztVWxMa5FGGJAPmDlzNSMVx3+EYrCZrbweKMqpcTGoqy
#         wW9z3wj3m70n97ptPPEJR7z431DKKX/Kkpq9c9HNz2HBxkmWCGT0opox/EGo6lpR
#         7jyB2krmF1T+SicEXEzExkLeAfVRf/8Y9ZH7mu1zy25caOf8czq7+602N/W1J1/Q
#         7KP2vTOYDoF5tDDgoPnBqPhMpgHTBK1I0ue7MJsdG8lW2FsgiohBw6nZ3tX6XxxD
#         DOgN9WZPcsY5iHO9nVV4/6AhbsY0PBhCs1suYwBmcDHWmOY4Yvt1ay/PJZ0Ly89j
#         XhL4xjATZwh49m98AofjQg1M12MzPW8Q0pcoIJnPvj3O+murNYSFVQsATU8Xt5hy
#         dVOm+irIcYIUKNkqXrVfG7DF7I/Smh4je2+lCD4h7xpZmfkgXcQF8Ne0JFRlc3Qx
#         IChQcm90ZWN0ZWQpIDx0ZXN0MUBkb21haW4uY29tPoi4BBMBAgAiBQJY512fAhsD
#         BgsJCAcDAgYVCAIJCgsEFgIDAQIeAQIXgAAKCRAbPHgVVqBE9+WvA/953hjG4nxB
#         wtNir0auKe1TFmd3tqV26CC1nJ6JT6xD7lpBmOzEZ8nEjKfbzXKSMCfuv+zM9Alk
#         Xa14F25cVSNN00MrJ0PcFsHn9hBb9MD+79Z0vn7wb+fU2tWj3Hy4STgB3Ndpa/X8
#         jMhUd/QrXo//YO15Sk9rziucV4+CLx4+QJ0CBgRY512fAQQA2X1M0+a+OfC1l9xC
#         TwxNAc48KiAYVw/WH0x530tSO8ztNDs1STTsN6l2RJuyQkREXKPM25didW4MLXEC
#         mOblkaC3Xt9/bHZbB1rrtVxIHRW0oWbO0v6a5JXfTtl773ngKWbMEFc0fxCZ0Em0
#         6ZUpjjahKIswdIKBS3tSF5O4CrMAEQEAAf4HAwKNx4jqW8Xr62A9KNQjb1i6nhev
#         I//F4vzkoPh3bO+L094daylbrVhhQm+Hkg5+JLqL2z85FVKkX3jGh99PL0wHhkdP
#         PDgVreVJVlzqVbXNiNi4ZON5U5zVb2WhhdFiXAvE9iGg+dAk5vajDOfh+Oh/ETYs
#         JOsdlViCFqLPk09jLDojxEyeuSH0e96Q/gzRVsPFpp/uyb4kXiQirJtyMY5TWrXA
#         xjWLGxxh1p35yefZvE7B1qY++toqd7hDcp89aIN/ftIH5V9Mstwp50UNLBk59zr9
#         y1xsr+wSTlklFhSkuoVvojt3oJSpoVcaUS1kF1nYqXWz7oF4KwC3n4KePxN2RLVO
#         xT6s74fjq1Jj//IWR0Dn5gbpU0CmuYXjhmOqxoHMCQcl9jd2t8rJDHTwmWN/bZPI
#         SsJRk5yu2psWgoz9xX+XD7InyHn/dZBZfermjfaqBGtduPR0MTeT1AUkwrXSCaZd
#         rbw7Xnj6kaDPnY2M7rqtToEMiJ8EGAECAAkFAljnXZ8CGwwACgkQGzx4FVagRPdn
#         bwP8DfhoZAHHtees8MqjuGlwmBTZNgbYAB2nhW7tnnmoUZZig9jBoKwg+avx8HXu
#         8KH627r4IS41TIuNeqBZRsO+A8Cb8vJQCstwxQS3L1YaWoK1BBmDxDSCgWGFoHq4
#         J2vfh35fe1nx1NWP4E0wZAZqKutPMYu+5oD2csINY2PwA78=
#         =4VRv
#         -----END PGP PRIVATE KEY BLOCK-----
#         """

#         shutil.rmtree('/tmp/parsec-tests', ignore_errors=True)
#         gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='/tmp/parsec-tests')
#         gpg.import_keys(self.key)
#         mock.patch.object(gnupg, 'GPG', return_value=gpg).start()

#         self.service = PubKeysService()
#         server = BaseServer()
#         server.register_service(self.service)
#         server.register_service(CryptoService())
#         server.bootstrap_services()
