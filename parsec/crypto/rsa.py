from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.primitives.hashes import SHA512
from cryptography.hazmat.primitives.asymmetric.padding import PSS, OAEP, MGF1
from cryptography.exceptions import InvalidSignature, UnsupportedAlgorithm


class RSACipherError(Exception):
    pass


class RSACipher:

    @staticmethod
    def generate_key(key_size=4096):
        if key_size < 2048:
            raise RSACipherError("Generation error : Key size must be >= 2048 bits")
        try:
            key = rsa.generate_private_key(public_exponent=65537,
                                           key_size=key_size,
                                           backend=openssl)
        except UnsupportedAlgorithm:
            raise RSACipherError("Generation error : RSA is not supported by backend")
        return key

    @staticmethod
    def load_key(pem, passphrase):
        key = None
        if passphrase == b'':
            passphrase = None
        try:
            key = serialization.load_pem_private_key(pem,
                                                     password=passphrase,
                                                     backend=openssl)
        except (ValueError, IndexError, TypeError):
            raise RSACipherError('Cannot import key : wrong format or bad passphrase')
        if key.key_size < 2048:
            raise RSACipherError("Loading error : Key size must be >= 2048 bits")
        return key

    @staticmethod
    def sign(key, data):
        signer = key.signer(PSS(mgf=MGF1(SHA512()), salt_length=PSS.MAX_LENGTH), SHA512())
        signer.update(data)
        return signer.finalize()

    @staticmethod
    def encrypt(key, data):
        pub_key = key.public_key()
        enc = pub_key.encrypt(data,
                              OAEP(mgf=MGF1(algorithm=SHA512()),
                                   algorithm=SHA512(),
                                   label=None))
        return enc

    @staticmethod
    def decrypt(key, enc):
        dec = key.decrypt(enc,
                          OAEP(mgf=MGF1(algorithm=SHA512()), algorithm=SHA512(), label=None))
        return dec

    @staticmethod
    def verify(key, data, signature):
        pub_key = key.public_key()
        verifier = pub_key.verifier(signature,
                                    PSS(mgf=MGF1(SHA512()), salt_length=PSS.MAX_LENGTH),
                                    SHA512())
        verifier.update(data)
        try:
            verifier.verify()
        except InvalidSignature:
            raise RSACipherError("Invalid Signature")
