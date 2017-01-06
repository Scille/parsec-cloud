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
        return s + bytes([(BS - len(s) % BS) for _ in range((BS - len(s) % BS))])

    @staticmethod
    def unpad(s):
        return s[:-s[-1]]

    @staticmethod
    def _generate_key():
        return Random.new().read(AESCipher.KEY_SIZE)

    @staticmethod
    def encrypt(raw):
        key = AESCipher._generate_key()
        raw = AESCipher.pad(raw)
        iv = Random.new().read(AES.block_size)
        cipher = AES.new(key, AES.MODE_CBC, iv)
        enc = cipher.encrypt(raw)
        return (key, iv + enc)

    @staticmethod
    def decrypt(enc, key):
        iv = enc[:16]
        dec = None
        try:
            cipher = AES.new(key, AES.MODE_CBC, iv)
            dec = AESCipher.unpad(cipher.decrypt(enc[16:]))
        except ValueError:
            raise AESCipherError("Cannot decrypt data")
        return dec
