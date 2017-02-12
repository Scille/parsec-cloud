import json
from os.path import normpath
from uuid import uuid4
from base64 import b64decode, b64encode
from datetime import datetime
from google.protobuf.message import DecodeError

from ..abstract import BaseService
from ..volume import VolumeFileNotFoundError, BaseVolumeClient
from ..crypto import CryptoError, BaseCryptoClient
from .vfs_pb2 import Request, Response, Stat


class CmdError(Exception):

    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class VFSService(BaseService):

    def __init__(self, volume: BaseVolumeClient, crypto: BaseCryptoClient):
        self._volume = volume
        self._crypto = crypto
        self._build_root()

    def _init_metadata(self, isdir=False):
        now = datetime.utcnow().timestamp()
        return {
            'ctime': now,
            'mtime': now,
            'atime': now,
            'size': 0,
            'type': Stat.DIRECTORY if isdir else Stat.FILE
        }

    def _update_metadata(self, metadata):
        now = datetime.utcnow().timestamp()
        metadata.update({
            'ctime': now,
            'mtime': now,
            'atime': now
        })

    def _build_root(self):
        """
        Ask the driver for the manifest file and loads it into memory.
        This method can be called as many time as required.
        """
        try:
            ret = self._volume.read_file('0')
            # ret = self._crypto.decrypt(content=ret.content)
            self._root = json.loads(ret.content.decode())
        except (VolumeFileNotFoundError, CryptoError):
            self._root = {'/': {'metadata': self._init_metadata(isdir=True)}}

    def _save_manifest(self):
        content = json.dumps(self._root).encode()
        # ret = self._crypto.encrypt(content)
        self._volume.write_file('0', content)

    def _get_path(self, path):
        return _clean_path('%s/%s' % (self.mock_path, path))

    def cmd_READ_FILE(self, cmd):
        try:
            path = normpath(cmd.path)
            file = self._root[path]
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)
        if file['vid'] is None:
            raise CmdError('File is a directory')
        now = datetime.utcnow().timestamp()
        file['metadata'].update({'atime': now, })
        self._save_manifest()
        ret = self._volume.read_file(file['vid'])
        ret = self._crypto.decrypt(
            content=ret.content,
            key=b64decode(file['metadata']['key'].encode()),
            signature=b64decode(file['metadata']['signature'].encode()),
            key_signature=b64decode(file['metadata']['key_signature'].encode()))
        return Response(status_code=Response.OK, content=ret.content)

    def cmd_CREATE_FILE(self, cmd):
        path = normpath(cmd.path)
        now = datetime.utcnow().timestamp()
        file = self._root.get(path, None)
        if not file:
            file = {'metadata': self._init_metadata(), 'vid': uuid4().hex}
            # TODO: update metadata (dates, etc.)
            self._root[path] = file
        if file['vid'] is None:
            raise CmdError('File is a directory')
        file_size = len(cmd.content)
        file['metadata'].update({'mtime': now, 'size': file_size})
        ret = self._crypto.encrypt(cmd.content)
        file['metadata'].update({'key': b64encode(ret.key).decode(),
                                 'signature': b64encode(ret.signature).decode(),
                                 'key_signature': b64encode(ret.key_signature).decode()})
        self._volume.write_file(file['vid'], ret.content)
        self._save_manifest()
        return Response(status_code=Response.OK, size=file_size)

    def cmd_WRITE_FILE(self, cmd):
        return self.cmd_CREATE_FILE(cmd)

    def cmd_DELETE_FILE(self, cmd):
        path = normpath(cmd.path)
        file = self._root.get(path, None)
        if file is not None:
            self._volume.delete_file(file['vid'])
            del self._root[path]
            self._save_manifest()
            return Response(status_code=Response.OK)
        else:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def cmd_STAT(self, cmd):
        path = normpath(cmd.path)
        try:
            meta = self._root[path]['metadata']
            stat = Stat(type=meta['type'], ctime=meta['ctime'], mtime=meta['mtime'],
                        atime=meta['atime'], size=meta['size'])
            return Response(status_code=Response.OK, stat=stat)
        except KeyError:
            raise CmdError('File not found', status_code=Response.FILE_NOT_FOUND)

    def _list_dir(self, path):
        files = []
        if path not in self._root:
            raise CmdError('Directory not found', status_code=Response.FILE_NOT_FOUND)
        # Cook path to support single '/' comparison
        path = path[:-1] if path.endswith('/') else path
        for key in self._root.keys():
            head, tail = key.rsplit('/', 1)
            if head == path and tail not in (None, ''):
                files.append(tail)
        return files

    def cmd_LIST_DIR(self, cmd):
        path = normpath(cmd.path)
        files = self._list_dir(path)
        return Response(status_code=Response.OK, list_dir=files)

    def cmd_MAKE_DIR(self, cmd):
        path = normpath(cmd.path)
        if path in self._root:
            raise CmdError('Target already exists')
        else:
            metadata = self._init_metadata(True)
            self._root[cmd.path] = {'vid': None, 'path': cmd.path, 'metadata': metadata}
        return Response(status_code=Response.OK)

    def cmd_REMOVE_DIR(self, cmd):
        path = normpath(cmd.path)
        if path == '/':
            raise CmdError('Cannot remove root')
        if self._list_dir(path):
            raise CmdError('Directory not empty')
        try:
            del self._root[path]
        except KeyError:
            raise CmdError('Directory not found', status_code=Response.FILE_NOT_FOUND)
        return Response(status_code=Response.OK)

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
