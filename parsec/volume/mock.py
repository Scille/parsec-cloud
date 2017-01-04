import os
from google.protobuf.message import DecodeError

from ..abstract import BaseService
from .volume_pb2 import Request, Response


class CmdError(Exception):
    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class VolumeServiceBaseMock(BaseService):

    def cmd_READ_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_WRITE_FILE(self, cmd):
        raise NotImplementedError()

    def cmd_DELETE_FILE(self, path):
        raise NotImplementedError()

    def dispatch_msg(self, msg):
        try:
            if msg.type == Request.READ_FILE:
                return self.cmd_READ_FILE(msg)
            elif msg.type == Request.WRITE_FILE:
                return self.cmd_WRITE_FILE(msg)
            elif msg.type == Request.DELETE_FILE:
                return self.cmd_DELETE_FILE(msg)
            else:
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


class VolumeServiceMock(VolumeServiceBaseMock):
    def __init__(self, mock_path: str):
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = _clean_path(mock_path)

    def _get_path(self, path):
        return _clean_path('%s/%s' % (self.mock_path, path))

    def cmd_READ_FILE(self, cmd):
        try:
            with open(self._get_path(cmd.vid), 'rb') as fd:
                return Response(status_code=Response.OK, content=fd.read())
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_WRITE_FILE(self, cmd):
        with open(self._get_path(cmd.vid), 'wb') as fd:
            fd.write(cmd.content)
            return Response(status_code=Response.OK)

    def cmd_DELETE_FILE(self, cmd):
        try:
            os.unlink(self._get_path(cmd.vid))
            return Response(status_code=Response.OK)
        except FileNotFoundError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)


class VolumeServiceInMemoryMock(VolumeServiceBaseMock):
    def __init__(self):
        self._dir = {}

    def cmd_READ_FILE(self, cmd):
        try:
            return Response(status_code=Response.OK, content=self._dir[cmd.vid])
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_WRITE_FILE(self, cmd):
        self._dir[cmd.vid] = cmd.content
        return Response(status_code=Response.OK)

    def cmd_DELETE_FILE(self, cmd):
        try:
            del self._dir[cmd.vid]
            return Response(status_code=Response.OK)
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)
