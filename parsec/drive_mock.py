import os
from stat import S_ISDIR, S_IFREG, S_IFDIR
import base64
import zmq


class DriveMockError(Exception):
    pass


def _content_wrap(content):
    return base64.encodebytes(content).decode()


def _content_unwrap(wrapped_content):
    return base64.decodebytes(wrapped_content.encode())


def _clean_path(path):
    return '/' + '/'.join([e for e in path.split('/') if e])


class DriveMock:
    def __init__(self, mock_path):
        assert mock_path.startswith('/'), '`mock_path` must be absolute'
        self.mock_path = _clean_path(mock_path)

    def _get_path(self, path):
        return _clean_path('%s/%s' % (self.mock_path, path))

    def cmd_READ_FILE(self, path):
        try:
            with open(self._get_path(path), 'rb') as fd:
                return {'content': _content_wrap(fd.read())}
        except FileNotFoundError:
            raise DriveMockError('File not found')

    def cmd_CREATE_FILE(self, path, content):
        return self.cmd_WRITE_FILE(path, content)

    def cmd_WRITE_FILE(self, path, content):
        try:
            with open(self._get_path(path), 'wb') as fd:
                return fd.write(_content_unwrap(content))
        except FileNotFoundError:
            raise DriveMockError('File not found')

    def cmd_DELETE_FILE(self, path):
        try:
            os.unlink(self._get_path(path))
        except FileNotFoundError:
            raise DriveMockError('File not found')

    def cmd_STAT(self, path):
        try:
            stat = os.stat(self._get_path(path))
        except FileNotFoundError:
            raise DriveMockError('File not found')
        data = {'st_size': stat.st_size, 'st_ctime': stat.st_ctime, 'st_mtime': stat.st_mtime}
        # File or directory ?
        if S_ISDIR(stat.st_mode):
            data['st_mode'] = S_IFDIR
        else:
            data['st_mode'] = S_IFREG
        return data

    def cmd_LIST_DIR(self, path):
        try:
            return {'_items': os.listdir(self._get_path(path))}
        except (FileNotFoundError, NotADirectoryError) as exc:
            raise DriveMockError(str(exc))

    def cmd_dispach(self, cmd, params):
        attr_name = 'cmd_%s' % cmd.upper()
        if hasattr(self, attr_name):
            try:
                ret = getattr(self, attr_name)(**params)
            except TypeError:
                raise DriveMockError('Bad params for cmd `%s`' % cmd)
        else:
            raise DriveMockError('Unknown cmd `%s`' % cmd)
        return ret


def main(addr='tcp://127.0.0.1:5000', mock_path='/tmp'):
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind(addr)
    drive = DriveMock(mock_path)
    while True:
        msg = socket.recv_json()
        cmd = msg.get('cmd')
        params = msg.get('params')
        try:
            print('==>', cmd, params)
            data = drive.cmd_dispach(cmd, params)
        except DriveMockError as exc:
            ret = {'ok': False, 'reason': str(exc)}
        else:
            ret = {'ok': True}
            if data:
                ret['data'] = data
        print('<==', ret)
        socket.send_json(ret)


if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        raise SystemExit("usage: %s <directory>" % sys.argv[0])
    main(mock_path=sys.argv[1])
