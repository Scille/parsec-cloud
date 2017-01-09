from Crypto.Hash import SHA512
from Crypto.PublicKey import RSA
from Crypto.Signature import PKCS1_v1_5


class RSACipherError(Exception):
    pass


class RSACipher:

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
        data_hash = SHA512.new(data)
        signer = PKCS1_v1_5.new(key)
        signature = signer.sign(data_hash)
        return signature

    @staticmethod
    def encrypt(key, data):
        # Must change the value of 32 but need to be sure how this random value is used
        enc = key.publickey().encrypt(data, 32)
        return enc[0]

    @staticmethod
    def decrypt(key, enc, signature=None):
        if enc in (None, b''):
            raise RSACipherError('No AES key provided')
        dec = key.decrypt(enc)
        return dec

    @staticmethod
    def verify(key, data, signature):
        data_hash = SHA512.new(data)
        verifier = PKCS1_v1_5.new(key)
        return verifier.verify(data_hash, signature)
