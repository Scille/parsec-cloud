from .abstract import SymetricEncryption, SymetricEncryptionError
from os import urandom
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives.ciphers.algorithms import AES
from cryptography.hazmat.primitives.ciphers.modes import GCM
from cryptography.hazmat.primitives.ciphers import Cipher
from cryptography.exceptions import InvalidTag


class AESCipherError(SymetricEncryptionError):
    pass


class AESCipher(SymetricEncryption):
    KEY_SIZE = 32  # 256 bits long key

    def __init__(self, **kwargs):
        pass

    def encrypt(self, raw):
        # Generate key for AES encryption
        key = urandom(self.KEY_SIZE)

        # No need for padding as we are using GCM
        # Get a new iv for GCM
        iv = urandom(int(AES.block_size / 8))

        cipher = Cipher(AES(key), GCM(iv), backend=openssl)
        encryptor = cipher.encryptor()

        enc = encryptor.update(raw) + encryptor.finalize()
        return (key, iv + enc + encryptor.tag)

    def decrypt(self, key, enc):
        iv = enc[:int(AES.block_size / 8)]
        tag = enc[-16:]
        dec = None
        try:
            cipher = Cipher(AES(key), GCM(iv, tag), backend=openssl).decryptor()
            dec = cipher.update(enc[16:-16]) + cipher.finalize()
        except ValueError:
            raise AESCipherError(1, "Cannot decrypt data")
        except InvalidTag:
            raise AESCipherError(2, "GMAC verification failed")
        return dec
