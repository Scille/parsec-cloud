from ..abstract import BaseClient
from ..exceptions import ParsecError
from ..broker import LocalClientMixin, ResRepClientMixin
from .mock import VolumeServiceMock, VolumeServiceInMemoryMock
from .google_drive import GoogleDriveVolumeService
from .volume_pb2 import Request, Response


__all__ = (
    'VolumeServiceMock',
    'VolumeServiceInMemoryMock',
    'GoogleDriveVolumeService',
    'VolumeError',
    'VolumeFileNotFoundError',
    'BaseVolumeClient',
    'LocalVolumeClient',
    'ReqResVolumeClient'
)


class VolumeError(ParsecError):
    pass


class VolumeFileNotFoundError(VolumeError):
    pass


class BaseVolumeClient(BaseClient):

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
            raise VolumeFileNotFoundError(response.error_msg)
        else:
            raise VolumeError(response.error_msg)

    def read_file(self, vid: int) -> Response:
        return self._communicate(type=Request.READ_FILE, vid=vid)

    def write_file(self, vid: int, content: bytes) -> Response:
        return self._communicate(type=Request.WRITE_FILE, vid=vid, content=content)

    def delete_file(self, vid: int) -> Response:
        return self._communicate(type=Request.DELETE_FILE, vid=vid)


class LocalVolumeClient(BaseVolumeClient, LocalClientMixin):
    pass


class ReqResVolumeClient(BaseVolumeClient, ResRepClientMixin):
    pass
