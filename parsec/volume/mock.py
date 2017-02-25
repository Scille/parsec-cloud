import os

from parsec.volume.base import BaseVolumeService, VolumeError


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class VolumeServiceMock(BaseVolumeService):
    def __init__(self, mock_path: str):
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = _clean_path(mock_path)

    def _get_path(self, path):
        return _clean_path('%s/%s' % (self.mock_path, path))

    async def read_file(self, vid: str):
        try:
            with open(self._get_path(vid), 'rb') as fd:
                return fd.read()
        except FileNotFoundError:
            raise VolumeError('not_found', 'File not found.')

    async def write_file(self, vid: str, content: bytes):
        with open(self._get_path(vid), 'wb') as fd:
            return fd.write(content)

    async def delete_file(self, vid: str):
        try:
            os.unlink(self._get_path(vid))
        except FileNotFoundError:
            raise VolumeError('not_found', 'File not found.')


class VolumeServiceInMemoryMock(BaseVolumeService):
    def __init__(self):
        self._dir = {}

    async def read_file(self, vid: str):
        try:
            return self._dir[vid]
        except KeyError:
            raise VolumeError('not_found', 'File not found.')

    async def write_file(self, vid: str, content: bytes):
        self._dir[vid] = content
        return len(content)

    async def delete_file(self, vid: str):
        try:
            del self._dir[vid]
        except KeyError:
            raise VolumeError('not_found', 'File not found.')
