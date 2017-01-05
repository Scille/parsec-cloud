import os
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


class VFSServiceMock(BaseService):
    def __init__(self, mock_path: str):
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = _clean_path(mock_path)

    def _get_path(self, path):
        return _clean_path('%s/%s' % (self.mock_path, path))

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

    _CMD_MAP = {
        Request.CREATE_FILE: cmd_CREATE_FILE,
        Request.READ_FILE: cmd_READ_FILE,
        Request.WRITE_FILE: cmd_WRITE_FILE,
        Request.DELETE_FILE: cmd_DELETE_FILE,
        Request.STAT: cmd_STAT,
        Request.LIST_DIR: cmd_LIST_DIR,
        Request.MAKE_DIR: cmd_MAKE_DIR,
        Request.REMOVE_DIR: cmd_REMOVE_DIR
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
