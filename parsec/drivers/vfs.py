import zmq
from json import loads, dumps
from uuid import uuid4
from datetime import datetime
from parsec.drivers.interface import DriverInterfaceException
from parsec.crypto.transport import SecurityTransportLayer, SecurityTransportLayerError
import base64


def _content_wrap(content):
    return base64.encodebytes(content).decode()


def _content_unwrap(wrapped_content):
    return base64.decodebytes(wrapped_content.encode())


class ParsecVFSException(Exception):
    pass


class ParsecVFS:

    """ Virtual FileSystem used by Parsec. In order to be used by other modules
    a glue layer must be implemented in ordre to link the module to the VFS.
    eg : map fuse functions and metadata to their equivalent in the VFS."""

    def __init__(self, driver_layer, security_layer=None):
        """ Initialise the filesystem from a driver. The driver must implment a function
        that return the plain manifest files as a StringIO. """
        self._driver = driver_layer
        self._driver.initialize_driver(force=True)
        self._security = security_layer
        self._build_root()

    def _init_metadata(self, isdir=False):
        now = datetime.utcnow().timestamp()
        return {'creation_time': now,
                'modification_time': now,
                'last_access': now,
                'size': 0,
                'directory': isdir}

    def _update_metadata(self, metadata):
        now = datetime.utcnow().timestamp()
        metadata.update({'creation_time': now,
                         'modification_time': now,
                         'last_access': now})

    def _build_root(self):
        """ Ask the driver for the manifest file and loads it into memory.
        This method can be called as many time as required.
            Returns : None
            Raises : None
            """
        try:
            self._root = loads(self._decrypt(self._driver.read_file('0')))
        except DriverInterfaceException:
            self._root = {'/': {'metadata': self._init_metadata(isdir=True)}}

    def _locate_file(self, path):
        """ Method aimed to find a file in the tree
            Returns:
                metadata
            Raises:
                ParsecVFSException : File not found
        """
        try:
            return self._root[path]['metadata']
        except KeyError:
            raise ParsecVFSException('File not found')

    def _save_manifest(self):
        self._driver.write_file('0', self._encrypt(dumps(self._root)))

    def _encrypt(self, content):
        try:
            content = self._security.encrypt(content)
        except AttributeError:
            # No security layer
            pass
        except SecurityTransportLayerError as e:
            raise ParsecVFSException("Security error. Reason: %s." % str(e))
        # TODO : set sig and return
        return content.encode()

    def _decrypt(self, content):
        try:
            content = self._security.decrypt(content)
        except AttributeError:
            # No security layer
            pass
        except SecurityTransportLayerError as e:
            raise ParsecVFSException("Security error. Reason: %s." % str(e))
        # TODO : check sig and return
        return content

    def create_file(self, path, content):
        now = datetime.utcnow().timestamp()
        file = self._root.get(path, None)
        if not file:
            file = {'metadata': self._init_metadata(), 'vid': uuid4().hex}
            # TODO : update metadata (dates, etc.)
            self._root[path] = file
        if file['vid'] is None:
            raise ParsecVFSException('File is a directory')
        file['metadata'].update({'modification_time': now,
                                 'size': len(content)})
        content = self._encrypt(content)
        self._driver.write_file(file['vid'], content)
        self._save_manifest()
        self._driver.sync()

    def read_file(self, path):
        try:
            file = self._root[path]
        except KeyError:
            raise ParsecVFSException('File not found')
        if file['vid'] is None:
            raise ParsecVFSException('File is Directory')
        now = datetime.utcnow().timestamp()
        file['metadata'].update({'last_access': now, })
        self._save_manifest()

        return {'content': self._decrypt(self._driver.read_file(file['vid']))}

    def write_file(self, path, content):
        self.create_file(path, content)

    def delete_file(self, path):
        file = self._root.get(path, None)
        if file is not None:
            self._driver.delete_file(file['vid'])
            del self._root[path]
            self._save_manifest()

    def stat(self, path):
        try:
            return self._root[path]['metadata']
        except KeyError:
            raise ParsecVFSException('File not found')

    def list_dir(self, path):
        files = []
        path = path[:-1] if path.endswith('/') else path
        for key in self._root.keys():
            head, tail = key.rsplit('/', 1)
            if head == path and tail not in (None, ''):
                files.append(tail)

        return files

    def make_dir(self, path):
        if self._root.get(path):
            raise ParsecVFSException('File exists')
        else:
            metadata = self._init_metadata(True)
            self._root[path] = {'vid': None, 'path': path, 'metadata': metadata}

    def remove_dir(self, path):
        if len(self.list_dir(path)):
            raise ParsecVFSException('Directory not empty')
        del self._root[path]

    def cmd_dispach(self, cmd, params):
        cmd = cmd.lower()
        if hasattr(self, cmd):
            try:
                ret = getattr(self, cmd)(**params)
            except TypeError:
                raise ParsecVFSException('Bad params for cmd `%s`' % cmd)
        else:
            raise ParsecVFSException('Unknown cmd `%s`' % cmd)

        return ret


def main(addr='tcp://127.0.0.1:5000'):
    from parsec.drivers.google import GoogleDriver
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind(addr)
    vfs = ParsecVFS(driver_layer=GoogleDriver(), security_layer=SecurityTransportLayer())

    while True:
        msg = socket.recv_json()
        cmd = msg.get('cmd')
        params = msg.get('params')
        try:
            print('==>', cmd, params)
            data = vfs.cmd_dispach(cmd, params)
        except ParsecVFSException as exc:
            ret = {'ok': False, 'reason': str(exc)}
        else:
            ret = {'ok': True}
            if data is not None:
                ret['data'] = data
        print('<==', ret)
        socket.send_json(ret)

if __name__ == "__main__":
    main()
