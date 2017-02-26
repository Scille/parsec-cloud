from binascii import hexlify, unhexlify

from parsec.crypto.base import BaseCryptoService, CryptoError
from parsec.crypto.abstract import BaseSymCipher, BaseAsymCipher


def _bytes_to_str(raw):
    return hexlify(raw).decode()


def _str_to_bytes(cooked):
    try:
        return unhexlify(cooked.encode())
    except:
        raise CryptoError('Invalid hex value `%s`' % cooked)


class CryptoService(BaseCryptoService):

    def __init__(self, symetric: BaseSymCipher, asymetric: BaseAsymCipher):
        self._asym = asymetric
        self._sym = symetric

    async def gen_key(self, key_size: int, passphrase: str) -> str:
        self._asym.generate_key(key_size=key_size)
        return self._asym.export_key(passphrase.encode()).decode()

    async def load_key(self, key: str, passphrase: str):
        self._asym.load_key(pem=key.encode(), passphrase=passphrase.encode())

    async def encrypt(self, content: bytes):
        sym_key, enc = self._sym.encrypt(content)
        signature = self._asym.sign(enc)
        encrypted_key = self._asym.encrypt(sym_key)
        key_sig = self._asym.sign(encrypted_key)
        return {
            'key': _bytes_to_str(encrypted_key),
            'content': enc,
            'signature': _bytes_to_str(signature),
            'key_signature': _bytes_to_str(key_sig)
        }

    async def decrypt(self, key: str, key_signature: str, content: bytes, signature: str):
        key = _str_to_bytes(key)
        key_signature = _str_to_bytes(key_signature)
        signature = _str_to_bytes(signature)
        # Check if the key and its signature match
        self._asym.verify(key, key_signature)
        # Decrypt the AES key
        aes_key = self._asym.decrypt(key)
        # Check if the encrypted content matches its signature
        self._asym.verify(content, signature)
        dec = self._sym.decrypt(aes_key, content)
        return dec
