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
        pass

    @abstractmethod
    def encrypt(self, raw):
        pass

    @abstractmethod
    def decrypt(self, key, enc):
        pass


class AsymetricEncryption(metaclass=ABCMeta):

    @abstractmethod
    def __init__(self, **kwargs):
        pass

    @abstractmethod
    def generate_key(self, key_size):
        pass

    @abstractmethod
    def load_key(self, pem, passphrase):
        pass

    @abstractmethod
    def export_key(self, key, passphrase):
        pass

    @abstractmethod
    def sign(self, key, data):
        pass

    @abstractmethod
    def verify(self, key, data, signature):
        pass

    @abstractmethod
    def encrypt(self, key, data):
        pass

    @abstractmethod
    def decrypt(self, key, enc):
        pass
