from google.protobuf.message import DecodeError
from cryptography.hazmat.primitives import serialization
from ..abstract import BaseService
from .aes import AESCipher, AESCipherError
from .rsa import RSACipher, RSACipherError
from .crypto_pb2 import Request, Response


class CmdError(Exception):

    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


class CryptoEngineError(CmdError):
    pass


# TODO : Make this class able to load other crypto drivers than RSA (DSA, custom ones, etc.)
class CryptoEngineService(BaseService):

    def __init__(self):
        self._key = None

    def _generate_key(self, msg):
        try:
            self._key = RSACipher.generate_key(key_size=msg.key_size)
        except RSACipherError:
            raise CryptoEngineError(status_code=Response.RSA_KEY_ERROR,
                                    error_msg='Incorrect RSA key size')

        if msg.passphrase:
            pem = self._key.private_bytes(
                encoding=serialization.Encoding.PEM,
                format=serialization.PrivateFormat.PKCS8,
                encryption_algorithm=serialization.BestAvailableEncryption(msg.passphrase))
        else:
            pem = self._key.private_bytes(encoding=serialization.Encoding.PEM,
                                          format=serialization.PrivateFormat.PKCS8,
                                          encryption_algorithm=serialization.NoEncryption())

        return Response(status_code=Response.OK,
                        key=pem)

    def _load_key(self, msg):
        try:
            self._key = RSACipher.load_key(pem=msg.key, passphrase=msg.passphrase)
        except RSACipherError:
            raise CryptoEngineError(status_code=Response.RSA_KEY_ERROR,
                                    error_msg='Cannot Load RSA key, wrong pasphrase ?')
        return Response(status_code=Response.OK)

    def _encrypt(self, msg):
        if not self._key:
            raise CryptoEngineError(status_code=Response.RSA_KEY_ERROR,
                                    error_msg='Not RSA key loaded')

        aes_key, enc = AESCipher.encrypt(msg.content)
        signature = RSACipher.sign(self._key, enc)
        encrypted_key = RSACipher.encrypt(self._key, aes_key)
        key_sig = RSACipher.sign(self._key, encrypted_key)

        return Response(status_code=Response.OK, key=encrypted_key,
                        content=enc, signature=signature, key_signature=key_sig)

    def _decrypt(self, msg):
        if not self._key:
            raise CryptoEngineError(status_code=Response.RSA_KEY_ERROR,
                                    error_msg='Not RSA key loaded')
        # Check if the key and its signature match
        try:
            RSACipher.verify(self._key, msg.key, msg.key_signature)
        except RSACipherError:
            raise CryptoEngineError(status_code=Response.RSA_KEY_SIGN_ERROR,
                                    error_msg='Incorrect RSA key signature')
        # Decrypt the AES key
        try:
            aes_key = RSACipher.decrypt(self._key, msg.key)
        except RSACipherError:
            raise CryptoEngineError(status_code=Response.RSA_DECRYPT_FAILED,
                                    error_msg='Cannot Decrypt AES key')

        # Check if the encrypted content matches its signature
        try:
            RSACipher.verify(self._key, msg.content, msg.signature)
        except RSACipherError:
            raise CryptoEngineError(status_code=Response.VERIFY_FAILED,
                                    error_msg='Invalid signature, content may be tampered')
        try:
            dec = AESCipher.decrypt(msg.content, aes_key)
        except AESCipherError:
            raise CryptoEngineError(status_code=Response.AES_DECRYPT_FAILED,
                                    error_msg='Cannot Decrypt file content')

        return Response(status_code=Response.OK, content=dec)

    _CMD_MAP = {
        Request.ENCRYPT: _encrypt,
        Request.DECRYPT: _decrypt,
        Request.LOAD_KEY: _load_key,
        Request.GEN_KEY: _generate_key,
    }

    def dispatch_msg(self, msg):
        try:
            try:
                return self._CMD_MAP[msg.type](self, msg)
            except KeyError:
                raise CmdError('Unknown msg `%s`' % msg.type)
        except CmdError as exc:
            return Response(status_code=exc.status_code, error_msg=exc.error_msg)

    def dispatch_raw_msg(self, raw_msg):
        try:
            msg = Request()
            msg.ParseFromString(raw_msg)
            ret = self.dispatch_msg(msg)
        except DecodeError as exc:
            ret = Response(status_code=Response.BAD_REQUEST, error_msg='Invalid request format')
        return ret.SerializeToString()
