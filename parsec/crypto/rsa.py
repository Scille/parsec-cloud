from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.primitives.hashes import SHA512
from cryptography.hazmat.primitives.asymmetric.padding import PSS, OAEP, MGF1
from cryptography.exceptions import InvalidSignature, UnsupportedAlgorithm

from parsec.crypto.abstract import BaseAsymCipher
from parsec.crypto.base import AsymCryptoError


class RSACipher(BaseAsymCipher):

    def __init__(self, key_file: str = None, key_pem: bytes = None, passphrase: bytes = b''):
        if key_file:
            with open(key_file, 'rb') as f:
                self.load_key(f.read(), passphrase)
        elif key_pem:
            self.load_key(key_pem, passphrase)
        else:
            self._key = None

    def ready(self):
        return self._key is not None

    def generate_key(self, key_size=4096):
        if key_size < 2048:
            raise AsymCryptoError("Invalid key size.")
        try:
            self._key = rsa.generate_private_key(public_exponent=65537,
                                                 key_size=key_size,
                                                 backend=openssl)
        except UnsupportedAlgorithm:
            raise AsymCryptoError("Generation error : RSA is not supported by backend.")

    def load_key(self, pem, passphrase):
        self._key = None
        if passphrase == b'':
            passphrase = None
        try:
            self._key = serialization.load_pem_private_key(pem,
                                                           password=passphrase,
                                                           backend=openssl)
        except (ValueError, IndexError, TypeError):
            pass
        if not self._key or self._key.key_size < 2048:
            raise AsymCryptoError('Invalid key or bad passphrase.')

    def export_key(self, passphrase):
        if passphrase:
            pem = self._key.private_bytes(
                encoding=serialization.Encoding.PEM,
                format=serialization.PrivateFormat.PKCS8,
                encryption_algorithm=serialization.BestAvailableEncryption(passphrase))
        else:
            pem = self._key.private_bytes(encoding=serialization.Encoding.PEM,
                                          format=serialization.PrivateFormat.PKCS8,
                                          encryption_algorithm=serialization.NoEncryption())
        return pem

    def sign(self, data):
        signer = self._key.signer(PSS(mgf=MGF1(SHA512()), salt_length=PSS.MAX_LENGTH), SHA512())
        signer.update(data)
        return signer.finalize()

    def encrypt(self, data):
        pub_key = self._key.public_key()
        enc = pub_key.encrypt(data,
                              OAEP(mgf=MGF1(algorithm=SHA512()),
                                   algorithm=SHA512(),
                                   label=None))
        return enc

    def decrypt(self, enc):
        dec = self._key.decrypt(enc,
                                OAEP(mgf=MGF1(algorithm=SHA512()), algorithm=SHA512(), label=None))
        return dec

    def verify(self, data, signature):
        pub_key = self._key.public_key()
        verifier = pub_key.verifier(signature,
                                    PSS(mgf=MGF1(SHA512()), salt_length=PSS.MAX_LENGTH),
                                    SHA512())
        verifier.update(data)
        try:
            verifier.verify()
        except InvalidSignature:
            raise AsymCryptoError("Invalid signature, content may be tampered.")
