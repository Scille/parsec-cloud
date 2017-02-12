from abc import ABCMeta, abstractmethod


class EncryptionError(Exception):

    def __init__(self, error_code=None, error_msg=''):
        self.error_msg = error_msg
        self.error_code = error_code


class SymetricEncryptionError(EncryptionError):
    pass


class AsymetricEncryptionError(EncryptionError):
    pass


class SymetricEncryption(metaclass=ABCMeta):

    @abstractmethod
    def __init__(self, **kwargs):
        pass  # pragma: no cover

    @abstractmethod
    def encrypt(self, raw):
        pass  # pragma: no cover

    @abstractmethod
    def decrypt(self, key, enc):
        pass  # pragma: no cover


class AsymetricEncryption(metaclass=ABCMeta):

    @abstractmethod
    def __init__(self, **kwargs):
        pass  # pragma: no cover

    @abstractmethod
    def ready(self, key_size):
        pass  # pragma: no cover

    @abstractmethod
    def generate_key(self, key_size):
        pass  # pragma: no cover

    @abstractmethod
    def load_key(self, pem, passphrase):
        pass  # pragma: no cover

    @abstractmethod
    def export_key(self, key, passphrase):
        pass  # pragma: no cover

    @abstractmethod
    def sign(self, key, data):
        pass  # pragma: no cover

    @abstractmethod
    def verify(self, key, data, signature):
        pass  # pragma: no cover

    @abstractmethod
    def encrypt(self, key, data):
        pass  # pragma: no cover

    @abstractmethod
    def decrypt(self, key, enc):
        pass  # pragma: no cover
