from os import path

import gnupg
import pytest

from parsec.server import BaseServer
from parsec.core import CryptoService, GNUPGPubKeysService


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def alice_gpg():
    gpg = gnupg.GPG(homedir=GNUPG_HOME + '/alice')
    gpg.identity = gpg.list_keys(secret=True)[0]['keyid']
    return gpg


@pytest.fixture
def crypto_svc(alice_gpg):
    service = CryptoService()
    service.gnupg = alice_gpg
    service.gnupg_agentless = service.gnupg
    server = BaseServer()
    server.register_service(service)
    server.register_service(GNUPGPubKeysService())
    server.bootstrap_services()
    return service


class TestCryptoService:

    @pytest.mark.asyncio
    async def test_sym_encrypt(self, crypto_svc):
        ret = await crypto_svc.dispatch_msg({'cmd': 'sym_encrypt', 'data': 'foo'})
        assert ret['status'] == 'ok'
        assert '-----BEGIN PGP MESSAGE-----' in ret['data']
        assert '-----END PGP MESSAGE-----' in ret['data']
        assert ret['key'] is not None

    @pytest.mark.asyncio
    async def test_asym_encrypt(self, crypto_svc):
        # Bad recipient
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': 'wrong',
             'data': 'foo'})
        assert ret == {'status': 'error', 'label': 'Encryption failure.'}
        # Working
        fingerprint = crypto_svc.gnupg.list_keys()[0]['fingerprint']
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': fingerprint,
             'data': 'foo'})
        assert ret['status'] == 'ok'
        assert '-----BEGIN PGP MESSAGE-----' in ret['data']
        assert '-----END PGP MESSAGE-----' in ret['data']

    @pytest.mark.asyncio
    async def test_sym_decrypt(self, crypto_svc):
        original = await crypto_svc.dispatch_msg({'cmd': 'sym_encrypt', 'data': 'foo'})
        # Bad data
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': 'bad',
             'key': original['key']})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Wrong key
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': original['data'],
             'key': 'd3Jvbmc=\n'})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Good key
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': original['data'],
             'key': original['key']})
        assert ret == {'status': 'ok', 'data': 'foo'}

    @pytest.mark.asyncio
    async def test_asym_decrypt(self, crypto_svc):
        fingerprint = crypto_svc.gnupg.list_keys()[0]['fingerprint']
        original = await crypto_svc.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': fingerprint,
             'data': 'foo'})
        # Bad data
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'asym_decrypt',
             'data': 'bad'})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Working
        ret = await crypto_svc.dispatch_msg(
            {'cmd': 'asym_decrypt',
             'data': original['data']})
        assert ret == {'status': 'ok', 'data': 'foo'}
