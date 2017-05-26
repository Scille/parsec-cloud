from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding


def load_private_key(raw_key):
    # Only support RSA so far
    return RSAPrivateKey(raw_key)


def load_public_key(raw_key):
    return RSAPublicKey(raw_key)


class BasePrivateAsymKey:
    @classmethod
    def load(self, key):
        raise NotImplementedError()

    def sign(self, message):
        raise NotImplementedError()

    def signer(self):
        raise NotImplementedError()

    def decrypt(self, message):
        raise NotImplementedError()


class BasePublicAsymKey:
    @classmethod
    def load(self, key):
        raise NotImplementedError()

    def verify(self, signature, message):
        raise NotImplementedError()

    def verifier(self, signature):
        raise NotImplementedError()

    def encrypt(self, message):
        raise NotImplementedError()


class RSAPublicKey(BasePublicAsymKey):
    def __init__(self, key: bytes):
        public_key = serialization.load_pem_public_key(key, backend=default_backend())
        self._hazmat_public_key = public_key

    def verify(self, signature: bytes, message: bytes):
        return self._hazmat_public_key.verify(
            signature,
            message,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )

    def verifier(self, signature: bytes):
        return self._hazmat_public_key.verifier(
            signature,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )

    def encrypt(self, message: bytes):
            return self._hazmat_public_key.encrypt(
                message,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA1()),
                    algorithm=hashes.SHA1(),
                    label=None
                )
            )


class RSAPrivateKey(BasePrivateAsymKey):
    def __init__(self, key: bytes):
        private_key = serialization.load_pem_private_key(
            key,
            password=None,
            backend=default_backend()
        )
        self._hazmat_private_key = private_key

    def sign(self, message: bytes):
        return self._hazmat_private_key.sign(
            message,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )

    def signer(self):
        return self._hazmat_private_key.signer(
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )

    def decrypt(self, ciphertext: bytes):
        return self._hazmat_private_key.decrypt(
            ciphertext,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA1()),
                algorithm=hashes.SHA1(),
                label=None
            )
        )

    @property
    def pub_key(self):
        raise NotImplementedError('TODO !')
