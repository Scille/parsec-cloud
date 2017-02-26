from parsec.crypto.base import BaseCryptoService
from parsec.crypto.abstract import BaseSymCipher, BaseAsymCipher


class CryptoService(BaseCryptoService):

    def __init__(self, symetric: BaseSymCipher, asymetric: BaseAsymCipher):
        self._asym = asymetric
        self._sym = symetric

    async def gen_key(self, key_size: int, passphrase: str):
        self._asym.generate_key(key_size=key_size)
        return self._asym.export_key(passphrase)

    async def load_key(self, key: str, passphrase: str):
        self._asym.load_key(pem=key, passphrase=passphrase)

    async def encrypt(self, content: bytes):
        aes_key, enc = self._sym.encrypt(content)
        signature = self._asym.sign(enc)
        encrypted_key = self._asym.encrypt(aes_key)
        key_sig = self._asym.sign(encrypted_key)
        return {
            'key': encrypted_key,
            'content': enc,
            'signature': signature,
            'key_signature': key_sig
        }

    async def decrypt(self, key: str, key_signature: str, content: bytes, signature: str):
        # Check if the key and its signature match
        self._asym.verify(key, key_signature)
        # Decrypt the AES key
        aes_key = self._asym.decrypt(key)
        # Check if the encrypted content matches its signature
        self._asym.verify(content, signature)
        dec = self._sym.decrypt(aes_key, content)
        return dec
