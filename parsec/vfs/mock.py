import os
from os.path import normpath
from stat import S_ISDIR
from datetime import datetime

from parsec.vfs.base import BaseVFSService, VFSError, VFSNotFound


class VFSServiceMock(BaseVFSService):
    def __init__(self, mock_path: str):
        super().__init__()
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = normpath(mock_path)

    def _get_path(self, path):
        return normpath('%s/%s' % (self.mock_path, path))

    async def create_file(self, path: str, content: bytes=b''):
        await self.write_file(path, content)

    async def read_file(self, path: str) -> bytes:
        try:
            with open(self._get_path(path), 'rb') as fd:
                return fd.read()
        except FileNotFoundError:
            raise VFSNotFound('File not found.')

    async def write_file(self, path: str, content: bytes) -> int:
        try:
            with open(self._get_path(path), 'wb') as fd:
                return fd.write(content)
        except FileNotFoundError:
            raise VFSNotFound('File not found.')

    async def delete_file(self, path: str):
        try:
            os.unlink(self._get_path(path))
        except FileNotFoundError:
            raise VFSNotFound('File not found.')

    async def stat(self, path: str) -> dict:
        try:
            osstat = os.stat(self._get_path(path))
        except FileNotFoundError:
            raise VFSNotFound('Target not found.')
        stat = {'atime': osstat.st_atime, 'ctime': osstat.st_ctime, 'mtime': osstat.st_mtime}
        # File or directory ?
        if S_ISDIR(osstat.st_mode):
            stat['is_dir'] = True
            stat['size'] = 0
        else:
            stat['is_dir'] = False
            stat['size'] = osstat.st_size
        return stat

    async def list_dir(self, path: str) -> list:
        try:
            return os.listdir(self._get_path(path))
        except (FileNotFoundError, NotADirectoryError) as exc:
            raise VFSNotFound('Directory not found.')

    async def make_dir(self, path: str):
        try:
            os.mkdir(self._get_path(path))
        except FileExistsError:
            raise VFSError('already_exist', 'Target already exists.')

    async def remove_dir(self, path: str):
        if path == '/':
            raise VFSError('cannot_remove_root', 'Cannot remove root directory.')
        try:
            os.rmdir(self._get_path(path))
        except FileNotFoundError:
            raise VFSNotFound('Directory not found.')
        except OSError:
            raise VFSError('directory_not_empty', 'Directory not empty.')


class VFSServiceInMemoryMock(BaseVFSService):

    def __init__(self):
        super().__init__()
        self._dir = {'/': self.Node(is_dir=True)}

    def _get_path(self, path):
        return normpath('/%s' % path)

    class Node:
        def __init__(self, content=None, is_dir=False):
            now = datetime.utcnow().timestamp()
            self.stat = {
                'is_dir': is_dir,
                'size': len(content) if content is not None else 0,
                'ctime': now,
                'mtime': now,
                'atime': now,
            }
            self.content = content

    def _is_valid_dir(self, basedir):
        try:
            return self._dir[basedir].stat['is_dir']
        except:
            return False

    def _is_valid_file(self, path, missing_is_ok=False):
        basedir = path.rsplit('/', 1)[0] or '/'
        if self._is_valid_dir(basedir):
            if ((path not in self._dir and missing_is_ok) or
                    (not self._dir[path].stat['is_dir'])):
                return True
        return False

    async def create_file(self, path: str, content: bytes=b''):
        await self.write_file(path, content)

    async def read_file(self, path: str) -> bytes:
        try:
            path = normpath(path)
            return self._dir[path].content
        except KeyError:
            raise VFSNotFound('File not found.')

    async def write_file(self, path: str, content: bytes) -> int:
        path = normpath(path)
        if self._is_valid_file(path, missing_is_ok=True):
            self._dir[path] = self.Node(content=content)
            return len(content)
        else:
            raise VFSNotFound('File not found.')

    async def delete_file(self, path: str):
        path = normpath(path)
        try:
            del self._dir[path]
        except KeyError:
            raise VFSNotFound('File not found.')

    async def stat(self, path: str) -> dict:
        path = normpath(path)
        try:
            return self._dir[path].stat
        except KeyError:
            raise VFSNotFound('Target not found.')

    async def list_dir(self, path: str) -> list:
        path = normpath(path)
        if path not in self._dir:
            raise VFSNotFound('Directory not found.')
        elif not self._dir[path].stat['is_dir']:
            raise VFSError(status='not_a_directory')
        listing = []
        if not path.endswith('/'):
            path = path + '/'
        for p in self._dir.keys():
            relative_p = p[len(path):]
            if p.startswith(path) and '/' not in relative_p and p != path:
                listing.append(relative_p)
        return listing

    async def make_dir(self, path: str):
        path = normpath(path)
        if path not in self._dir:
            self._dir[path] = self.Node(is_dir=True)
        else:
            raise VFSError('already_exist', 'Target already exists.')

    async def remove_dir(self, path: str):
        path = normpath(path)
        if path == '/':
            raise VFSError('cannot_remove_root', 'Cannot remove root directory.')
        elif self._is_valid_dir(path):
            if any(p for p in self._dir.keys() if p.startswith(path) and p != path):
                raise VFSError('directory_not_empty', 'Directory not empty.')
            del self._dir[path]
        else:
            raise VFSNotFound('Directory not found.')
