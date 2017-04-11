import shutil

import gnupg
import pytest

from parsec.server import BaseServer
from parsec.core import CryptoService, IdentityService, PubKeysService


class BaseTestIdentityService:

    @pytest.mark.asyncio
    async def test_load_identity(self):
        # Default identity not found
        ret = await self.service.dispatch_msg({'cmd': 'load_identity'})
        assert ret == {'status': 'not_found', 'label': 'Default identity not found.'}
        # Default identity found
        self.gpg.import_keys(self.unprotected_key)
        ret = await self.service.dispatch_msg({'cmd': 'load_identity'})
        assert ret == {'status': 'ok'}
        self.gpg.import_keys(self.protected_key)
        # Multiple identity found
        ret = await self.service.dispatch_msg({'cmd': 'load_identity'})
        assert ret == {'status': 'error', 'label': 'Multiple identities found.'}
        # Identity not found
        ret = await self.service.dispatch_msg({'cmd': 'load_identity', 'identity': 'unknown'})
        assert ret == {'status': 'not_found', 'label': 'Identity not found.'}
        # Wrong passphrase for protected key
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_protected_key,
                                               'passphrase': 'wrong'})
        assert ret == {'status': 'error', 'label': 'Bad passphrase.'}
        # Passphrase with an unprotected key (should be ignored)
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_unprotected_key,
                                               'passphrase': self.passphrase})
        assert ret == {'status': 'ok'}
        # Working with passphrase and protected key
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_protected_key,
                                               'passphrase': self.passphrase})
        assert ret == {'status': 'ok'}
        # Working without passphrase and unprotected key
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_unprotected_key})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_get_identity(self):
        self.gpg.import_keys(self.unprotected_key)
        self.gpg.import_keys(self.protected_key)
        # Identity loaded
        ret = await self.service.dispatch_msg({'cmd': 'get_identity'})
        assert ret == {'status': 'ok', 'identity': None}
        # Identity loaded
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_protected_key,
                                               'passphrase': self.passphrase})
        assert ret == {'status': 'ok'}
        ret = await self.service.dispatch_msg({'cmd': 'get_identity'})
        assert ret == {'status': 'ok', 'identity': self.fingerprint_protected_key}

    @pytest.mark.asyncio
    async def test_encrypt(self):
        self.gpg.import_keys(self.unprotected_key)
        self.gpg.import_keys(self.protected_key)
        # Identity not loaded
        ret = await self.service.dispatch_msg({'cmd': 'encrypt', 'data': self.test_data})
        assert ret == {'status': 'not_found', 'label': 'No identity loaded.'}
        # Identity loaded
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_protected_key,
                                               'passphrase': self.passphrase})
        assert ret == {'status': 'ok'}
        ret = await self.service.dispatch_msg({'cmd': 'encrypt', 'data': self.test_data})
        assert ret['status'] == 'ok'
        assert '-----BEGIN PGP MESSAGE-----' in ret['data']
        assert '-----END PGP MESSAGE-----' in ret['data']

    @pytest.mark.asyncio
    async def test_decrypt(self):
        self.gpg.import_keys(self.unprotected_key)
        self.gpg.import_keys(self.protected_key)
        # Identity not loaded
        ret = await self.service.dispatch_msg({'cmd': 'decrypt', 'data': self.test_data})
        assert ret == {'status': 'not_found', 'label': 'No identity loaded.'}
        # Identity loaded
        ret = await self.service.dispatch_msg({'cmd': 'load_identity',
                                               'identity': self.fingerprint_protected_key,
                                               'passphrase': self.passphrase})
        assert ret == {'status': 'ok'}
        original = await self.service.dispatch_msg({'cmd': 'encrypt', 'data': self.test_data})
        ret = await self.service.dispatch_msg({'cmd': 'decrypt', 'data': original['data']})
        assert ret == {'status': 'ok', 'data': self.test_data}


class TestIdentityService(BaseTestIdentityService):

    def setup_method(self, gpg):
        self.test_data = 'Hello, I am a plaintext. I need to be encrypted.'
        self.passphrase = 'test1'
        self.fingerprint_protected_key = '8D74B53B7580166E47E244BB1B3C781556A044F7'
        # test1@domain.com
        self.protected_key = """-----BEGIN PGP PRIVATE KEY BLOCK-----
        Version: GnuPG v1

        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
        =4VRv
        -----END PGP PRIVATE KEY BLOCK-----
        """

        self.fingerprint_unprotected_key = '5AB61191B023C1D93758FC41ECA2B0A1F5DB7465'
        # test2@domain.com
        self.unprotected_key = """-----BEGIN PGP PRIVATE KEY BLOCK-----
        Version: GnuPG v1

        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
        =QSf4
        -----END PGP PRIVATE KEY BLOCK-----
        """

        shutil.rmtree('/tmp/parsec-tests', ignore_errors=True)
        self.gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='/tmp/parsec-tests')

        self.service = IdentityService('foo', 123)
        server = BaseServer()
        server.register_service(self.service)
        crypto_service = CryptoService()
        server.register_service(crypto_service)
        server.register_service(PubKeysService())
        server.bootstrap_services()

        crypto_service.gpg = self.gpg  # TODO user mock
        # mock.patch.object(gnupg, 'GPG', return_value=gpg).start()
