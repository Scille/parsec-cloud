from hashlib import md5
from base64 import encodebytes, decodebytes
from string import hexdigits
from random import sample

from parsec.crypto.abstract import BaseSymCipher, BaseAsymCipher
from parsec.crypto.base import AsymCryptoError, SymCryptoError


class MockSymCipher(BaseSymCipher):

    """
    WARNING : NEVER USE THIS IMPLEMENTATION IN PRODUCTION. THIS CLASS IS ONLY
    USEFUL FOR TESTING PURPOSE.
    """

    def __init__(self, override):
        assert override == "I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE"

    def encrypt(self, raw: bytes):
        key = ''.join(sample(hexdigits, 10)).encode()
        ciphertext = encodebytes(raw)
        return (key, b'iv:%s:%s:tag' % (key, ciphertext))

    def decrypt(self, key: bytes, enc: bytes):
        cleartext = None
        try:
            iv, cipherkey, ciphertext, tag = enc.split(b':')
            cleartext = decodebytes(ciphertext)
            if iv != b'iv' or tag != b'tag' or key != cipherkey:
                raise SymCryptoError("GMAC verification failed")
        except ValueError:
            raise SymCryptoError("Cannot decrypt data")
        return cleartext


class MockAsymCipher(BaseAsymCipher):

    """
    WARNING : NEVER USE THIS IMPLEMENTATION IN PRODUCTION. THIS CLASS IS ONLY
    USEFUL FOR TESTING PURPOSE.
    """

    def __init__(self, override):
        assert override == "I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE"
        self._key = None
        self._key_counter = 0

    def ready(self):
        return self._key is not None

    def generate_key(self, key_size: int=2):
        if key_size % 2:
            raise AsymCryptoError("Invalid key size.")
        self._key = ''.join(sample(hexdigits, key_size)).encode()

    def load_key(self, pem: bytes, passphrase: bytes):
        key = pem[:-len(passphrase)] if passphrase else pem
        if not pem.endswith(passphrase) or len(key) % 2:
            raise AsymCryptoError('Invalid key or bad passphrase.')
        self._key = key

    def export_key(self, passphrase: bytes=b''):
        return self._key + passphrase

    def sign(self, data: bytes):
        m = md5(self._key)
        m.update(data)
        return m.digest()

    def encrypt(self, data: bytes):
        # All time famous base 64 encryption !
        return encodebytes(self._key + data)

    def decrypt(self, enc: bytes):
        plain = decodebytes(enc)
        if plain.startswith(self._key):
            return plain[len(self._key):]
        else:
            return b'<dummy data>'

    def verify(self, data: bytes, signature: str):
        m = md5(self._key)
        m.update(data)
        if signature != m.digest():
            raise AsymCryptoError("Invalid signature, content may be tampered.")
