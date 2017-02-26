from os import urandom
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives.ciphers.algorithms import AES
from cryptography.hazmat.primitives.ciphers.modes import GCM
from cryptography.hazmat.primitives.ciphers import Cipher
from cryptography.exceptions import InvalidTag

from parsec.crypto.abstract import BaseSymCipher
from parsec.crypto.base import SymCryptoError


class AESCipher(BaseSymCipher):
    KEY_SIZE = 32  # 256 bits long key

    def encrypt(self, cleartext: bytes):
        # Generate key for AES encryption
        key = urandom(self.KEY_SIZE)

        # No need for padding as we are using GCM
        # Get a new iv for GCM
        iv = urandom(int(AES.block_size / 8))

        cipher = Cipher(AES(key), GCM(iv), backend=openssl)
        encryptor = cipher.encryptor()

        enc = encryptor.update(cleartext) + encryptor.finalize()
        return (key, iv + enc + encryptor.tag)

    def decrypt(self, key: bytes, enc: bytes):
        iv = enc[:AES.block_size // 8]
        tag = enc[-16:]
        try:
            cipher = Cipher(AES(key), GCM(iv, tag), backend=openssl).decryptor()
            return cipher.update(enc[16:-16]) + cipher.finalize()
        except ValueError:
            raise SymCryptoError("Cannot decrypt data.")
        except InvalidTag:
            raise SymCryptoError("GMAC verification failed.")
