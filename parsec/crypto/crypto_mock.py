from .abstract import (SymetricEncryption, SymetricEncryptionError,
                       AsymetricEncryption, AsymetricEncryptionError)
from base64 import b64decode, b64encode
from hashlib import md5
from random import sample


class MockSymCipherError(SymetricEncryptionError):
    pass


class MockAsymCipherError(AsymetricEncryptionError):
    pass


class MockSymCipher(SymetricEncryption):

    """
    WARNING : NEVER USE THIS IMPLEMENTATION IN PRODUCTION. THIS CLASS IS ONLY
    USEFUL FOR TESTING PURPOSE.
    """

    def __new__(cls, *args, **kwargs):
        if 'override' in kwargs:
            if kwargs['override'] == "I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE":
                return object.__new__(cls)
        return None

    def __init__(self, **kwargs):
        pass

    def encrypt(self, raw):
        iv = b'123456789'
        key = bytes([int(x) for x in sample(b"123456789", 9)])
        tag = b'tagged'
        enc = b64encode(raw)

        return (key, iv + key + enc + tag)

    def decrypt(self, key, enc):
        iv = enc[:9]
        tag = enc[-6:]
        dec = b64decode(enc[18:-6])
        if len(key) != 9:
            raise MockSymCipherError(1, "Cannot decrypt data")
        if iv != b'123456789':
            raise MockSymCipherError(2, "GMAC verification failed")
        if key != enc[9:18]:
            raise MockSymCipherError(2, "GMAC verification failed")
        if tag != b'tagged':
            raise MockSymCipherError(2, "GMAC verification failed")

        return dec


class MockAsymCipher(AsymetricEncryption):

    """
    WARNING : NEVER USE THIS IMPLEMENTATION IN PRODUCTION. THIS CLASS IS ONLY
    USEFUL FOR TESTING PURPOSE.
    """

    def __new__(cls, *args, **kwargs):
        if 'override' in kwargs:
            if kwargs['override'] == "I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE":
                return object.__new__(cls)
        return None

    def __init__(self, **kwargs):
        pass

    def generate_key(self, key_size=2):
        key = b'123456789'
        if key_size < 2 or key_size > 9:
            raise MockAsymCipherError(1, "Generation error : Key size must be between 1 and 9")
        return key[0:key_size]

    def load_key(self, pem, passphrase):
        if passphrase and passphrase != b'passphrase':
            raise MockAsymCipherError(3, 'Cannot import key : wrong format or bad passphrase')
        if len(pem) < 1 or len(pem) > 9:
            raise MockAsymCipherError(4, "Loading error : Key size must be >= 2048 bits")
        return pem

    def export_key(self, key, passphrase):
        return key

    def sign(self, key, data):
        m = md5()
        m.update(key)
        m.update(data)
        return m.digest()

    def encrypt(self, key, data):
        return b'encrypted' + data + b'encrypted'

    def decrypt(self, key, enc):
        return enc[9:-9]

    def verify(self, key, data, signature):
        m = md5()
        m.update(key)
        m.update(data)
        if signature != m.digest():
            raise MockAsymCipherError(5, "Invalid signature, content may be tampered")
