from base64 import encodebytes
import shutil

from asynctest import mock, MagicMock
import gnupg
import pytest

from parsec.core import CryptoService


class AsyncMock(MagicMock):
    async def __call__(self, *args, **kwargs):
        return super(AsyncMock, self).__call__(*args, **kwargs)


class BaseTestCryptoService:

    # Helpers

    # Tests

    @pytest.mark.asyncio
    @mock.patch('parsec.core.crypto_service.urandom', return_value=b'123456789')
    async def test_sym_encrypt(self, urandom_function):
        ret = await self.service.dispatch_msg({'cmd': 'sym_encrypt', 'data': self.test_data})
        assert ret['status'] == 'ok'
        assert '-----BEGIN PGP MESSAGE-----' in ret['data']
        assert '-----END PGP MESSAGE-----' in ret['data']
        assert ret['key'] == encodebytes(b'123456789').decode()

    @pytest.mark.asyncio
    async def test_asym_encrypt(self):
        # Bad recipient
        ret = await self.service.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': 'wrong',
             'data': self.test_data})
        assert ret == {'status': 'error', 'label': 'Encryption failure.'}
        # Working
        ret = await self.service.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': 'D75F53F92D950618D8C868237C0C7DF0F405623E',
             'data': self.test_data})
        assert ret['status'] == 'ok'
        assert '-----BEGIN PGP MESSAGE-----' in ret['data']
        assert '-----END PGP MESSAGE-----' in ret['data']

    @pytest.mark.asyncio
    async def test_sym_decrypt(self):
        original = await self.service.dispatch_msg({'cmd': 'sym_encrypt', 'data': self.test_data})
        # Bad data
        ret = await self.service.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': 'bad',
             'key': original['key']})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Wrong key
        ret = await self.service.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': original['data'],
             'key': 'd3Jvbmc=\n'})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Good key
        ret = await self.service.dispatch_msg(
            {'cmd': 'sym_decrypt',
             'data': original['data'],
             'key': original['key']})
        assert ret == {'status': 'ok', 'data': 'Hello, I am a plaintext. I need to be encrypted.'}

    @pytest.mark.asyncio
    async def test_asym_decrypt(self):
        original = await self.service.dispatch_msg(
            {'cmd': 'asym_encrypt',
             'recipient': 'D75F53F92D950618D8C868237C0C7DF0F405623E',
             'data': self.test_data})
        # Bad data
        ret = await self.service.dispatch_msg(
            {'cmd': 'asym_decrypt',
             'data': 'bad',
             'passphrase': self.asym_passphrase})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Wrong passphrase
        ret = await self.service.dispatch_msg(
            {'cmd': 'asym_decrypt',
             'data': original['data'],
             'passphrase': 'wrong'})
        assert ret == {'status': 'error', 'label': 'Decryption failure.'}
        # Good passphrase
        ret = await self.service.dispatch_msg(
            {'cmd': 'asym_decrypt',
             'data': original['data'],
             'passphrase': self.asym_passphrase})
        assert ret == {'status': 'ok', 'data': 'Hello, I am a plaintext. I need to be encrypted.'}
        # TODO unknown recipient


class TestCryptoService(BaseTestCryptoService):

    def setup_method(self, gpg):
        self.test_data = 'Hello, I am a plaintext. I need to be encrypted.'
        self.asym_passphrase = 'test1'
        # test1@domain.com, fingerprint is D75F53F92D950618D8C868237C0C7DF0F405623E
        self.key = """-----BEGIN PGP PRIVATE KEY BLOCK-----
        Version: BCPG C# v1.6.1.0

        lQOsBFjkm+kBCACYpCXEqit1E/2WzXz/CT822K57HEaHQ7ZZhNprf66fNxnlj3R3
        b8F4zK4TGyoKDKCkCcFPQoqW5eRgnYsaJW5FCjQyQJgMcWsVRuBySEXqNCcfsxMH
        LNePyvfJvdwUQWY4BgdLzoPzeQXy7EVrTfGKJfRxDEnugBCpH66ugIVCyLfx+icG
        XrRqZcnUKPRHLY9nf5vNqmBj2orPinB8+rrYdYyRXnZ4zKUzicJDrr2oYl2fqLvh
        eH+uja2fcSJffTrxNb5q6j0vgF/I9C6M2mN64w3cRfhQUUlO0Y2yaV16KjdvqNgD
        TrAQcpepj+hOqlOvjdjJRmG4TvfnUUL1DwIhABEBAAH/AwMCWVd9rKvzFGlgFF16
        WSDsJryJsdmDCkqktgf0ddeXashBXs7KT4uHzlN7uJ6CSR+U7Ka/S3/cqwRzCXrS
        jQB19W2ZQOoBoxnGSBu1S+xLOKXymW18YHm7+yJge7cpjugXrnnTHsgWDZelX6Qg
        MhNDBz4W0ZegR5OuxOVC1t32DXWQT/FW4SuQoFDiRwoBXzzNc+8EW1vpU/rHrWK9
        OzT6CYQPqkc50jmSrAgNkMEtcGfrnJnkesMDU/qrJ23YiqQ2uuZ4avO5nnEWytSj
        lm1DuRkncISNwni3x1qhWRmTozS81oBjFIW/V9rIprPrtyPcnDmP/UHqsxrDV7W5
        oHGcc157VVLJz9tfxCTDjwKsC/H4z0JDh8gTanVLw8IdNQ4+oQswz1uJy73i2RGp
        b1mbt0Ns8jikZrX4XSk091HVfdTmLUYAd/XnXgRdwNZknPlV6Iap0KmObf6cY9q4
        7VydFbY0YW9UEowkwI0uNrBZQ44c5OM+bLNY1+KFWewFr2cI8dPkD3/xbsQ5x7hv
        lhtm4peaBUUKBwEGSPK2XjEefqB52oi8xOflCVqqUexeCUwxW0M6FTz0L4xfW68I
        n3RqYm1E4z8GVFRWU0lQh+yZORIfv61snM/DR7OcQfhDH7Wi4HWKGW3V1L8I4TfC
        m/mkMrXXpW4JANntSSTaa82POs3MBkhWwAC1GNtwY3CAZAHgGLueyz9PVSDAfyH1
        6I3jejjyBJ3Tl0n4T/jMpAwqk3xPSjuTEKdMC+ooT6SHhOmLadtkmxkTTOpAbrnf
        O38CTh+EcKWpWtZbCLCmP9xCHYuFdQtCU/8YmG9Z44zgs1gSAZjamtYjP6UTmN4s
        2ECsbQc6l0P6UC0W65dSsar/nLgr36gGwXSgFuesY7QQdGVzdDFAZG9tYWluLmNv
        bYkBHAQQAQIABgUCWOSb6QAKCRB8DH3w9AViPkszB/4kW/zDepuy8LehmEUbh58i
        VNUTgtnj4S8PrkXQ1NAoJXrlgUl7X13ViCvLAOfa7cOl+ic1pLgC+UaRtf8t1mOc
        /61DJScYaghkeQycwNPZwzafNdDpBMnJ5U23bdtZY4wUkQtsWmXdVJIemsZOx1kL
        tKmHaxaxiqctbCl8FnGJAXEQoIAwJo2uEI1/jf/+w/NkfaV5zLX8f2m8243f13Ot
        U7PBO9/t5hJLYqp4jlx17I8eNjZ6REK+GxRpe+cSa28RG7hoFhHaVlRhB8+6NbrX
        rYIuU12ooUsj+XtMpqSFtCTW68afT46+vTaW8ukL2JuBeMO0eW4AK3hfkgxygYBl
        =4v8o
        -----END PGP PRIVATE KEY BLOCK-----
        """

        shutil.rmtree('/tmp/parsec-tests', ignore_errors=True)
        gpg = gnupg.GPG(binary='/usr/bin/gpg', homedir='/tmp/parsec-tests')
        gpg.import_keys(self.key)

        self.service = CryptoService()

        def get_pub_key(recipient):
            if recipient is 'D75F53F92D950618D8C868237C0C7DF0F405623E':
                return recipient
            else:
                raise Exception()

        mock_pub_key = MagicMock()
        mock_pub_key.get_pub_key = AsyncMock(side_effect=get_pub_key)
        mock.patch.object(self.service, 'pub_keys_service', new=mock_pub_key).start()
        self.service.gpg = gpg  # TODO use mock instead?
