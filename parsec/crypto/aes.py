import base64
from Crypto.Cipher import AES
from Crypto import Random


class AESCipherError(Exception):
    pass


class AESCipher:
    KEY_SIZE = 32  # 256 bits long key

    def __init__(self):
        pass

    @staticmethod
    def pad(s):
        BS = AES.block_size
        return s + (BS - len(s) % BS) * chr(BS - len(s) % BS)

    @staticmethod
    def unpad(s):
        return s[:-ord(s[len(s) - 1:])]

    @staticmethod
    def _generate_key():
        return Random.new().read(AESCipher.KEY_SIZE)

    @staticmethod
    def encrypt(raw):
        key = AESCipher._generate_key()
        raw = AESCipher.pad(raw)
        iv = Random.new().read(AES.block_size)
        cipher = AES.new(key, AES.MODE_CBC, iv)
        # TODO : try/except then return
        return (key, base64.b64encode(iv + cipher.encrypt(raw)))

    @staticmethod
    def decrypt(enc, key):
        enc = base64.b64decode(enc)
        iv = enc[:16]
        cipher = AES.new(key, AES.MODE_CBC, iv)
        # TODO : try/except then return
        return AESCipher.unpad(cipher.decrypt(enc[16:]))
