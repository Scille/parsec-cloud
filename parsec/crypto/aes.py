from os import urandom
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives.ciphers.algorithms import AES
from cryptography.hazmat.primitives.ciphers.modes import GCM
from cryptography.hazmat.primitives.ciphers import Cipher
from cryptography.exceptions import InvalidTag


class AESCipherError(Exception):
    pass


class AESCipher:
    KEY_SIZE = 32  # 256 bits long key

    @staticmethod
    def encrypt(raw):
        # Generate key for AES encryption
        key = urandom(AESCipher.KEY_SIZE)

        # No need for padding as we are using GCM
        # Get a new iv for GCM
        iv = urandom(int(AES.block_size / 8))

        cipher = Cipher(AES(key), GCM(iv), backend=openssl)
        encryptor = cipher.encryptor()

        enc = encryptor.update(raw) + encryptor.finalize()
        return (key, iv + enc + encryptor.tag)

    @staticmethod
    def decrypt(enc, key):
        iv = enc[:int(AES.block_size / 8)]
        tag = enc[-16:]
        dec = None
        try:
            cipher = Cipher(AES(key), GCM(iv, tag), backend=openssl).decryptor()
            dec = cipher.update(enc[16:-16]) + cipher.finalize()
        except ValueError:
            raise AESCipherError("Cannot decrypt data")
        except InvalidTag:
            raise AESCipherError("GMAC verification failed")
        return dec
