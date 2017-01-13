import os
from os.path import normpath
from datetime import datetime
from stat import S_ISDIR
from google.protobuf.message import DecodeError

from ..abstract import BaseService
from .vfs_pb2 import Request, Response, Stat


class CmdError(Exception):
    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


def _clean_path(path):

    return '/' + '/'.join([e for e in path.split('/') if e])


def _check_required(cmd, *fields):
    for field in fields:
        if getattr(cmd, field) is None:
            raise CmdError('field `%s` is mandatory' % field)


class VFSServiceBaseMock(BaseService):

    def __init__(self):
        self._CMD_MAP = {
            Request.CREATE_FILE: self.cmd_CREATE_FILE,
            Request.READ_FILE: self.cmd_READ_FILE,
            Request.WRITE_FILE: self.cmd_WRITE_FILE,
            Request.DELETE_FILE: self.cmd_DELETE_FILE,
            Request.STAT: self.cmd_STAT,
            Request.LIST_DIR: self.cmd_LIST_DIR,
            Request.MAKE_DIR: self.cmd_MAKE_DIR,
            Request.REMOVE_DIR: self.cmd_REMOVE_DIR
        }

    def cmd_READ_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_CREATE_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_WRITE_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_DELETE_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_STAT(self, cmd):
        raise NotImplementedError()

    def cmd_LIST_DIR(self, cmd):
        raise NotImplementedError()

    def cmd_MAKE_DIR(self, cmd):
        raise NotImplementedError()

    def cmd_REMOVE_DIR(self, cmd):
        raise NotImplementedError()

    def dispatch_msg(self, msg):
        try:
            try:
                return self._CMD_MAP[msg.type](msg)
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


class VFSServiceMock(VFSServiceBaseMock):
    def __init__(self, mock_path: str):
        super().__init__()
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = normpath(mock_path)

    def _get_path(self, path):
        return normpath('%s/%s' % (self.mock_path, path))

    def cmd_READ_FILE(self, cmd):
        _check_required(cmd, 'path')
        try:
            with open(self._get_path(cmd.path), 'rb') as fd:
                return Response(status_code=Response.OK, content=fd.read())
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_CREATE_FILE(self, cmd):
        return self.cmd_WRITE_FILE(cmd)

    def cmd_WRITE_FILE(self, cmd):
        _check_required(cmd, 'path', 'content')
        try:
            with open(self._get_path(cmd.path), 'wb') as fd:
                return Response(status_code=Response.OK, size=fd.write(cmd.content))
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_DELETE_FILE(self, cmd):
        _check_required(cmd, 'path')
        try:
            os.unlink(self._get_path(cmd.path))
            return Response(status_code=Response.OK)
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_STAT(self, cmd):
        _check_required(cmd, 'path')
        try:
            stat = os.stat(self._get_path(cmd.path))
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)
        kwargs = {'atime': stat.st_atime, 'ctime': stat.st_ctime, 'mtime': stat.st_mtime}
        # File or directory ?
        if S_ISDIR(stat.st_mode):
            kwargs['type'] = Stat.DIRECTORY
            kwargs['size'] = 0
        else:
            kwargs['type'] = Stat.FILE
            kwargs['size'] = stat.st_size
        return Response(status_code=Response.OK, stat=Stat(**kwargs))

    def cmd_LIST_DIR(self, cmd):
        _check_required(cmd, 'path')
        try:
            return Response(status_code=Response.OK, list_dir=os.listdir(self._get_path(cmd.path)))
        except (FileNotFoundError, NotADirectoryError) as exc:
            raise CmdError(str(exc))

    def cmd_MAKE_DIR(self, cmd):
        _check_required(cmd, 'path')
        try:
            os.mkdir(self._get_path(cmd.path))
            return Response(status_code=Response.OK)
        except FileExistsError:
            raise CmdError('Target already exists')

    def cmd_REMOVE_DIR(self, cmd):
        _check_required(cmd, 'path')
        if cmd.path == '/':
            raise CmdError('Cannot remove root')
        try:
            os.rmdir(self._get_path(cmd.path))
            return Response(status_code=Response.OK)
        except FileNotFoundError:
            raise CmdError('Directory not found', status_code=Response.FILE_NOT_FOUND)
        except OSError:
            raise CmdError('Directory not empty')


class VFSServiceInMemoryMock(VFSServiceBaseMock):
    def __init__(self):
        super().__init__()
        self._dir = {'/': self.Node(is_dir=True)}

    def _get_path(self, path):
        return normpath('/%s' % path)

    class Node:
        def __init__(self, content=None, is_dir=False):
            now = datetime.utcnow().timestamp()
            self.stat = Stat()
            self.stat.type = Stat.DIRECTORY if is_dir else Stat.FILE
            self.stat.size = len(content) if content is not None else 0
            self.stat.ctime = self.stat.mtime = self.stat.atime = now
            self.content = content

    def cmd_READ_FILE(self, cmd):
        _check_required(cmd, 'path')
        try:
            path = normpath(cmd.path)
            return Response(status_code=Response.OK, content=self._dir[path].content)
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_CREATE_FILE(self, cmd):
        return self.cmd_WRITE_FILE(cmd)

    def _is_valid_dir(self, basedir):
        try:
            return self._dir[basedir].stat.type == Stat.DIRECTORY
        except:
            return False

    def _is_valid_file(self, path, missing_is_ok=False):
        basedir = path.rsplit('/', 1)[0] or '/'
        if self._is_valid_dir(basedir):
            if ((path not in self._dir and missing_is_ok) or
                    (self._dir[path].stat.type == Stat.FILE)):
                return True
        return False

    def cmd_WRITE_FILE(self, cmd):
        _check_required(cmd, 'path', 'content')
        path = normpath(cmd.path)
        if self._is_valid_file(path, missing_is_ok=True):
            self._dir[path] = self.Node(content=cmd.content)
            return Response(status_code=Response.OK, size=len(cmd.content))
        else:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_DELETE_FILE(self, cmd):
        _check_required(cmd, 'path')
        path = normpath(cmd.path)
        try:
            del self._dir[path]
            return Response(status_code=Response.OK)
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_STAT(self, cmd):
        _check_required(cmd, 'path')
        path = normpath(cmd.path)
        try:
            return Response(status_code=Response.OK, stat=self._dir[path].stat)
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_LIST_DIR(self, cmd):
        _check_required(cmd, 'path')
        path = normpath(cmd.path)
        if path not in self._dir or self._dir[path].stat.type != Stat.DIRECTORY:
            raise CmdError("Directory doesn't exists")
        listing = []
        if not path.endswith('/'):
            path = path + '/'
        for p in self._dir.keys():
            relative_p = p[len(path):]
            if p.startswith(path) and '/' not in relative_p and p != path:
                listing.append(relative_p)
        return Response(status_code=Response.OK, list_dir=listing)

    def cmd_MAKE_DIR(self, cmd):
        _check_required(cmd, 'path')
        path = normpath(cmd.path)
        if path not in self._dir:
            self._dir[path] = self.Node(is_dir=True)
            return Response(status_code=Response.OK)
        else:
            raise CmdError('Target already exists')

    def cmd_REMOVE_DIR(self, cmd):
        _check_required(cmd, 'path')
        path = normpath(cmd.path)
        if cmd.path == '/':
            raise CmdError('Cannot remove root')
        elif self._is_valid_dir(path):
            if any(p for p in self._dir.keys() if p.startswith(path) and p != path):
                raise CmdError('Directory not empty')
            del self._dir[path]
            return Response(status_code=Response.OK)
        else:
            raise CmdError('Directory not found', status_code=Response.FILE_NOT_FOUND)
