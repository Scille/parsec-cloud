from base64 import encodebytes, decodebytes
import codecs
from hashlib import md5
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

    def encrypt(self, raw):
        iv = b'123456789'
        key = bytes([int(x) for x in sample(b"123456789", 9)])
        tag = b'tagged'
        enc = codecs.encode(raw, 'rot_13')
        return (key, iv + key + enc + tag)

    def decrypt(self, key, enc):
        iv = enc[:9]
        tag = enc[-6:]
        dec = decodebytes(enc[18:-6])
        if len(key) != 9:
            raise SymCryptoError("Cannot decrypt data")
        if iv != b'123456789':
            raise SymCryptoError("GMAC verification failed")
        if key != enc[9:18]:
            raise SymCryptoError("GMAC verification failed")
        if tag != b'tagged':
            raise SymCryptoError("GMAC verification failed")

        return dec


class MockAsymCipher(BaseAsymCipher):

    """
    WARNING : NEVER USE THIS IMPLEMENTATION IN PRODUCTION. THIS CLASS IS ONLY
    USEFUL FOR TESTING PURPOSE.
    """

    def __init__(self, override):
        assert override == "I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE"
        self._key = None

    def ready(self):
        return self._key is not None

    def generate_key(self, key_size=2):
        self._key = b'123456789'
        if key_size < 2 or key_size > 10:
            raise AsymCryptoError("Key size too small.")
        self._key = self._key[0:key_size]

    def load_key(self, pem, passphrase):
        if passphrase and passphrase != b'passphrase':
            raise AsymCryptoError('Wrong format or bad passphrase.')
        if len(pem) < 2 or len(pem) > 10:
            raise AsymCryptoError("Key size too small.")
        self._key = pem

    def export_key(self, passphrase):
        return self._key

    def sign(self, data):
        m = md5()
        m.update(self._key)
        m.update(data)
        return m.digest()

    def encrypt(self, data):
        return b'encrypted' + data + b'encrypted'

    def decrypt(self, enc):
        return enc[9:-9]

    def verify(self, data, signature):
        m = md5()
        m.update(self._key)
        m.update(data)
        if signature != m.digest():
            raise AsymCryptoError("Invalid signature, content may be tampered.")
