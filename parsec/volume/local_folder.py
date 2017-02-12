import os
from uuid import uuid4
from json import dumps, loads
from google.protobuf.message import DecodeError

from ..abstract import BaseService
from .volume_pb2 import Request, Response


class CmdError(Exception):

    def __init__(self, error_msg, status_code=Response.BAD_REQUEST):
        self.error_msg = error_msg
        self.status_code = status_code


class LocalFolderServiceException(CmdError):
    pass


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class LocalFolderVolumeService(BaseService):

    """ This service might be used with a local folder you wan tto encrypt.
    It could also be used by mapping a dropbox or a google drive folder to an other part
    of the FS in order to let the third party softwar handle the synchronisation between
    the local storage and the cloud.
    """

    def __init__(self, path: str):
        self._initialized = False
        path = _clean_path(path)
        self._path = os.path.abspath(os.path.expanduser(path)) + '/'
        self._initialize_driver()

    def _initialize_driver(self):
        """ Looks up Parsec system files one the cloud and load their ID into
        the Driver class instance. This Function also loads the mapping between
        virtual IDs used by the VFS and the Physical IDs used on the cloud side.
        Sets the initialized attribute to the according boolean value.
        WARNING: You MUST not use the driver if not successfully initialized.
            Returns:
                None
            Raises:
                LocalFolderServiceException if a system file is not found
        """
        if self._initialized:
            return
        # Load the mapping file
        try:
            with open(self._path + 'MAP', 'br') as mapf:
                self._mapping = loads(mapf.read().decode())
        except (EnvironmentError, ValueError):
            self._mapping = {}
        self._sync()
        self._initialized = True

    def _sync(self):
        try:
            with open(self._path + 'MAP', 'bw') as mapf:
                mapf.write(dumps(self._mapping).encode())
        except (EnvironmentError, ValueError):
            raise LocalFolderServiceException('Cannot save mapping file')

    def _create_driver_files(self):
        pass

    def initialize_driver(self, force=False):
        try:
            self._initialize_driver()
        except LocalFolderServiceException:
            if force:
                self._create_driver_files()
        finally:
            self._initialize_driver()

    def _read_file(self, msg):
        file_id = self._mapping.get(msg.vid)
        if file_id is None:
            return Response(status_code=Response.FILE_NOT_FOUND)
        try:
            with open(self._path + file_id, 'rb') as f:
                file_content = f.read()
        except IOError:
            return Response(status_code=Response.FILE_NOT_FOUND)
        return Response(status_code=Response.OK, content=file_content)

    def _write_file(self, msg):
        vid = msg.vid
        if vid is None:
            raise LocalFolderServiceException('A VID is mandatory')
        file_id = self._mapping.get(vid)
        if file_id is None:
            file_id = uuid4().hex
            self._mapping[msg.vid] = file_id
        try:
            with open(self._path + file_id, 'wb')as f:
                f.write(msg.content)
        except IOError:
            raise LocalFolderServiceException('Cannot write file')
        self._sync()
        return Response(status_code=Response.OK)

    def _delete_file(self, msg):
        file_id = self._mapping.get(msg.vid)
        if file_id is not None:
            os.remove(self._path + file_id)
            del self._mapping[msg.vid]
            return Response(status_code=Response.OK)
        self._sync()
        return Response(status_code=Response.FILE_NOT_FOUND)

    _CMD_MAP = {
        Request.READ_FILE: _read_file,
        Request.WRITE_FILE: _write_file,
        Request.DELETE_FILE: _delete_file,
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
