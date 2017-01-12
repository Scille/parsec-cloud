from google.protobuf.message import DecodeError
from ..abstract import BaseService
from .abstract import SymetricEncryptionError, AsymetricEncryptionError
from .crypto_pb2 import Request, Response


class CmdError(Exception):

    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


class CryptoEngineError(CmdError):
    pass


class CryptoEngineService(BaseService):

    def __init__(self, symetric_cls, asymetric_cls, **kwargs):
        self._key = None
        self._asym = asymetric_cls(**kwargs.get('asymetric_parameters', {}))
        self._sym = symetric_cls(**kwargs.get('symetric_parameters', {}))

    def _generate_key(self, msg):
        try:
            self._key = self._asym.generate_key(key_size=msg.key_size)
        except AsymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_KEY_ERROR,
                                    error_msg=e.error_msg)
        pem = self._asym.export_key(self._key, msg.passphrase)

        return Response(status_code=Response.OK, key=pem)

    def _load_key(self, msg):
        try:
            self._key = self._asym.load_key(pem=msg.key, passphrase=msg.passphrase)
        except AsymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_KEY_ERROR,
                                    error_msg=e.error_msg)
        return Response(status_code=Response.OK)

    def _encrypt(self, msg):
        if not self._key:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_KEY_ERROR,
                                    error_msg='No private key loaded')

        aes_key, enc = self._sym.encrypt(msg.content)
        signature = self._asym.sign(self._key, enc)
        encrypted_key = self._asym.encrypt(self._key, aes_key)
        key_sig = self._asym.sign(self._key, encrypted_key)

        return Response(status_code=Response.OK, key=encrypted_key,
                        content=enc, signature=signature, key_signature=key_sig)

    def _decrypt(self, msg):
        if not self._key:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_KEY_ERROR,
                                    error_msg='No private key loaded')
        # Check if the key and its signature match
        try:
            self._asym.verify(self._key, msg.key, msg.key_signature)
        except AsymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_KEY_SIGN_ERROR,
                                    error_msg=e.error_msg)
        # Decrypt the AES key
        try:
            aes_key = self._asym.decrypt(self._key, msg.key)
        except AsymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.ASYMETRIC_DECRYPT_FAILED,
                                    error_msg=e.error_msg)

        # Check if the encrypted content matches its signature
        try:
            self._asym.verify(self._key, msg.content, msg.signature)
        except AsymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.VERIFY_FAILED,
                                    error_msg=e.error_msg)
        try:
            dec = self._sym.decrypt(aes_key, msg.content)
        except SymetricEncryptionError as e:
            raise CryptoEngineError(status_code=Response.SYMETRIC_DECRYPT_FAILED,
                                    error_msg=e.error_msg)

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
