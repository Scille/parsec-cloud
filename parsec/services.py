from os.path import normpath
from datetime import datetime
from base64 import decodebytes, encodebytes


def cmd(param):

    def patcher(name, callback):
        if not hasattr(callback, '_cmds'):
            callback._cmds = []
        callback._cmds.append(name)
        return callback

    if callable(param):
        patcher(param.__name__, param)
        return param
    else:
        return lambda fn: patcher(param, fn)


class BaseService:
    async def bootstrap(self):
        pass

    def get_cmds(self):
        cmds = {}
        for key in dir(self):
            value = getattr(self, key)
            if hasattr(value, '_cmds'):
                for name in value._cmds:
                    cmds[name] = value
        return cmds


class BaseVFSService(BaseService):

    @staticmethod
    def _pack_vfs_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise VFSError(message='Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise VFSError(message='Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise VFSError(status='bad_params', label='Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise VFSError(status='bad_params', label='Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise VFSError(status='bad_params', label='Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('create_file')
    async def __CREATE_FILE(self, msg):
        return await self.__WRITE_FILE(msg)

    @cmd('read_file')
    async def __READ_FILE(self, msg):
        try:
            path = self._get_field(msg, 'path')
            content = await self.read_file(path)
            return {'status': 'ok', 'content': encodebytes(content).decode()}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('write_file')
    async def __WRITE_FILE(self, msg):
        try:
            path = self._get_field(msg, 'path')
            content = self._get_field(msg, 'content', bytes)
            count = await self.write_file(path, content)
            return {'status': 'ok', 'count': count}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('delete_file')
    async def __DELETE_FILE(self, msg):
        try:
            path = self._get_field(msg, 'path')
            await self.delete_file(path)
            return {'status': 'ok'}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('stat')
    async def __STAT(self, msg):
        try:
            path = self._get_field(msg, 'path')
            stat = await self.stat(path)
            return {'status': 'ok', 'stat': stat}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('list_dir')
    async def __LIST_DIR(self, msg):
        try:
            path = self._get_field(msg, 'path')
            listing = await self.list_dir(path)
            return {'status': 'ok', 'listing': listing}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('make_dir')
    async def __MAKE_DIR(self, msg):
        try:
            path = self._get_field(msg, 'path')
            await self.make_dir(path)
            return {'status': 'ok'}
        except VFSError as exc:
            return exc.to_dict()

    @cmd('remove_dir')
    async def __REMOVE_DIR(self, msg):
        try:
            path = self._get_field(msg, 'path')
            await self.remove_dir(path)
            return {'status': 'ok'}
        except VFSError as exc:
            return exc.to_dict()


class VFSError(Exception):
    def __init__(self, status='error', label=''):
        self.status = status
        self.label = label

    def to_dict(self):
        return {'status': self.status, 'label': self.label}


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
            raise VFSError(status='not_found')

    async def write_file(self, path: str, content: bytes) -> int:
        path = normpath(path)
        if self._is_valid_file(path, missing_is_ok=True):
            self._dir[path] = self.Node(content=content)
            return len(content)
        else:
            raise VFSError(status='not_found')

    async def delete_file(self, path: str):
        path = normpath(path)
        try:
            del self._dir[path]
        except KeyError:
            raise VFSError(status='not_found')

    async def stat(self, path: str) -> dict:
        path = normpath(path)
        try:
            return self._dir[path].stat
        except KeyError:
            raise VFSError(status='not_found')

    async def list_dir(self, path: str) -> list:
        path = normpath(path)
        if path not in self._dir:
            raise VFSError(status='not_found')
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
            raise VFSError(status='already_exist')

    async def remove_dir(self, path: str):
        path = normpath(path)
        if path == '/':
            raise VFSError(status='cannot_remove_root')
        elif self._is_valid_dir(path):
            if any(p for p in self._dir.keys() if p.startswith(path) and p != path):
                raise VFSError(status='directory_not_empty')
            del self._dir[path]
        else:
            raise VFSError(status='not_found')
