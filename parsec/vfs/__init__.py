from ..abstract import BaseClient
from ..exceptions import ParsecError
from ..broker import LocalClientMixin, ResRepClientMixin
from .vfs_pb2 import Request, Response
from .mock import VFSServiceMock


__all__ = (
    'VFSServiceMock',
    'VFSError',
    'VFSFileNotFoundError',
    'BaseVFSClient',
    'LocalVFSClient',
    'ReqResVFSClient'
)


class VFSError(ParsecError):
    pass


class VFSFileNotFoundError(VFSError):
    pass


class BaseVFSClient(BaseClient):
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
        elif response.status_code == Response.FILE_NOT_FOUND:
            raise VFSFileNotFoundError(response.error_msg)
        else:
            raise VFSError(response.error_msg)

    def create_file(self, path: str, content: bytes=b'') -> Response:
        return self._communicate(type=Request.CREATE_FILE, path=path, content=content)

    def read_file(self, path: str) -> Response:
        return self._communicate(type=Request.READ_FILE, path=path)

    def write_file(self, path: str, content: bytes) -> Response:
        return self._communicate(type=Request.WRITE_FILE, path=path, content=content)

    def delete_file(self, path: str) -> Response:
        return self._communicate(type=Request.DELETE_FILE, path=path)

    def stat(self, path: str) -> Response:
        return self._communicate(type=Request.STAT, path=path)

    def list_dir(self, path: str) -> Response:
        return self._communicate(type=Request.LIST_DIR, path=path)

    def make_dir(self, path: str) -> Response:
        return self._communicate(type=Request.MAKE_DIR, path=path)

    def remove_dir(self, path: str) -> Response:
        return self._communicate(type=Request.REMOVE_DIR, path=path)


class LocalVFSClient(BaseVFSClient, LocalClientMixin):
    pass


class ReqResVFSClient(BaseVFSClient, ResRepClientMixin):
    pass
