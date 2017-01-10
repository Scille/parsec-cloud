from ..abstract import BaseClient
from ..exceptions import ParsecError
from ..broker import LocalClientMixin, ResRepClientMixin
from .crypto_pb2 import Request, Response
from .crypto import CryptoEngineService


__all__ = (
    'CryptoEngineService',
    'CryptoError',
    'BaseCryptoClient',
    'LocalCryptoClient',
    'ReqResVFSClient'
)


class CryptoError(ParsecError):
    pass


class BaseCryptoClient(BaseClient):

    @property
    def request_cls(self):
        return Request

    @property
    def response_cls(self):
        return Response

    def _communicate(self, **kwargs):
        response = self._ll_communicate(**kwargs)
        if response.status_code == Response.OK:
            return response
        else:
            raise CryptoError(response.error_msg)

    def encrypt(self, content: bytes=b'') -> Response:
        return self._communicate(type=Request.ENCRYPT, content=content)

    def decrypt(self, content: bytes=b'', key: bytes=b'',
                signature: bytes=b'', key_signature: bytes=b'') -> Response:
        return self._communicate(type=Request.DECRYPT, content=content,
                                 key=key, signature=signature, key_signature=key_signature)

    def load_key(self, key: bytes=b'', passphrase: bytes=b'')-> Response:
        return self._communicate(type=Request.LOAD_KEY, key=key, passphrase=passphrase)

    def genkey(self, passphrase: bytes=b'', key_size: int=4096) -> Response:
        return self._communicate(type=Request.GEN_KEY, passphrase=passphrase, key_size=key_size)


class LocalCryptoClient(BaseCryptoClient, LocalClientMixin):
    pass


class ReqResVFSClient(BaseCryptoClient, ResRepClientMixin):
    pass
