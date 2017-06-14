import arrow

from parsec.core2.fs_api import BaseFSAPIMixin
from parsec.core2.workspace import Folder, File, Reader
from parsec.exceptions import InvalidPath


class FSAPIMixin(BaseFSAPIMixin):

    def __init__(self):
        self._fs = Folder()
        self._reader = Reader()

    def _retrieve_file(self, path):
        fileobj = self._retrieve_path(path)
        if not isinstance(fileobj, File):
            raise InvalidPath("Path `%s` is not a file" % path)
        return fileobj

    def _check_path(self, path, should_exists=True, type=None):
        if path == '/':
            if not should_exists or type not in ('folder', None):
                raise InvalidPath('Root `/` folder always exists')
            else:
                return
        dirpath, leafname = path.rsplit('/', 1)
        try:
            obj = self._retrieve_path(dirpath)
            if not isinstance(obj, Folder):
                raise InvalidPath("Path `%s` is not a folder" % path)
            try:
                leafobj = obj.children[leafname]
                if not should_exists:
                    raise InvalidPath("Path `%s` already exist" % path)
                if (type == 'file' and not isinstance(leafobj, File) or
                        type == 'folder' and not isinstance(leafobj, Folder)):
                    raise InvalidPath("Path `%s` is not a %s" % (path, type))
            except KeyError:
                if should_exists:
                    raise InvalidPath("Path `%s` doesn't exist" % path)
        except InvalidPath:
            raise InvalidPath("Path `%s` doesn't exist" % (path if should_exists else dirpath))

    def _retrieve_path(self, path):
        if not path:
            return self._fs
        if not path.startswith('/'):
            raise InvalidPath("Path must start with `/`")
        parent_dir = cur_dir = self._fs
        reps = path.split('/')
        for rep in reps:
            if not rep or rep == '.':
                continue
            elif rep == '..':
                cur_dir = parent_dir
            else:
                try:
                    parent_dir, cur_dir = cur_dir, cur_dir.children[rep]
                except KeyError:
                    raise InvalidPath("Path `%s` doesn't exist" % path)
        return cur_dir

    async def file_create(self, path: str):
        self._check_path(path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        dirobj.children[name] = File()

    async def file_write(self, path: str, content: bytes, offset: int=0):
        self._check_path(path, should_exists=True, type='file')
        fileobj = self._retrieve_file(path)
        fileobj.data = (fileobj.data[:offset] + content +
                           fileobj.data[offset + len(content):])
        fileobj.updated = arrow.get()

    async def file_read(self, path: str, offset: int=0, size: int=-1):
        return await self._reader.file_read(self._fs, path, offset, size)

    async def stat(self, path: str):
        return await self._reader.stat(self._fs, path)

    async def folder_create(self, path: str):
        self._check_path(path, should_exists=False)
        dirpath, name = path.rsplit('/', 1)
        dirobj = self._retrieve_path(dirpath)
        dirobj.children[name] = Folder()

    async def move(self, src: str, dst: str):
        self._check_path(src, should_exists=True)
        self._check_path(dst, should_exists=False)

        srcdirpath, scrfilename = src.rsplit('/', 1)
        dstdirpath, dstfilename = dst.rsplit('/', 1)

        srcobj = self._retrieve_path(srcdirpath)
        dstobj = self._retrieve_path(dstdirpath)
        dstobj.children[dstfilename] = srcobj.children[scrfilename]
        del srcobj.children[scrfilename]

    async def delete(self, path: str):
        self._check_path(path, should_exists=True)
        dirpath, leafname = path.rsplit('/', 1)
        obj = self._retrieve_path(dirpath)
        del obj.children[leafname]

    async def file_truncate(self, path: str, length: int):
        self._check_path(path, should_exists=True, type='file')
        fileobj = self._retrieve_file(path)
        fileobj.data = fileobj.data[:length]
        fileobj.updated = arrow.get()
