from Crypto.Hash import SHA512
from Crypto.PublicKey import RSA


class RSACipherError(Exception):
    pass


class RSACipher:

    def __init__(self, key):
        pass

    @staticmethod
    def generate_key(key_size=4096):
        key = RSA.generate(key_size)
        return key

    @staticmethod
    def load_key(key, passphrase):
        try:
            return RSA.importKey(key, passphrase)
        except (ValueError, IndexError, TypeError):
            raise RSACipherError('Cannot import key : wrong format or bad passphrase')

    @staticmethod
    def sign(key, data):
        data_hash = SHA512.new(data.encode()).digest()
        signature = key.sign(data_hash, '')
        return signature

    @staticmethod
    def encrypt(key, data):
        if not key.can_encrypt():
            raise RSACipherError('This key cannot encrypt')
        if not key.can_sign():
            raise RSACipherError('This key cannot sign')
        # Must change the value of 32 but need to be sure how this random value is used
        enc = key.publickey().encrypt(data, 32)
        return enc[0]

    @staticmethod
    def decrypt(key, enc, signature=None):
        dec = key.decrypt(enc)
        return dec

    @staticmethod
    def verify(key, data, signature):
        data_hash = SHA512.new(data).digest()
        return key.publickey().verify(data_hash, signature)
