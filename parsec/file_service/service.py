from base64 import decodebytes

from datetime import datetime
from uuid import uuid4

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError


class FileError(ParsecError):
    pass


class FileNotFound(FileError):
    status = 'not_found'


class FileService(BaseService):

    def __init__(self):
        self.files = {}

    @staticmethod
    def _pack_file_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise FileError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise FileError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise FileError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise FileError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise FileError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('create_file')
    async def cmd_CREATE(self, msg):
        id = await self.create()
        return {'status': 'ok', 'id': id}

    @cmd('read_file')
    async def cmd_READ(self, msg):
        if 'id' not in msg:
            raise FileError('bad_params', 'Invalid parameters')
        content = await self.read(msg['id'])
        return {'status': 'ok', 'content': content}

    @cmd('write_file')
    async def cmd_WRITE(self, msg):
        if 'id' not in msg:
            raise FileError('bad_params', 'Invalid parameters')
        await self.write(msg['id'], msg['content'])
        return {'status': 'ok'}

    @cmd('stat_file')
    async def cmd_STAT(self, msg):
        if 'id' not in msg:
            raise FileError('bad_params', 'Invalid parameters')
        stats = await self.stat(msg['id'])
        return {'status': 'ok', 'stats': stats}

    @cmd('history')
    async def cmd_HISTORY(self, msg):
        id = self._get_field(msg, 'id')
        history = await self.history(id)
        return {'status': 'ok', 'history': history}

    async def create(self):
        id = uuid4().hex
        file = {
            'ctime': datetime.utcnow().timestamp(),
            'mtime': datetime.utcnow().timestamp(),
            'atime': datetime.utcnow().timestamp(),
            'content': ''
        }
        self.files[id] = file
        return id

    async def read(self, id):
        if id not in self.files:
            raise FileNotFound('File not found.')
        self.files[id]['atime'] = datetime.utcnow().timestamp()
        return self.files[id]['content']

    async def write(self, id, content):
        if id not in self.files:
            raise FileNotFound('File not found.')
        self.files[id]['mtime'] = datetime.utcnow().timestamp()
        self.files[id]['content'] = content

    async def stat(self, id):
        if id not in self.files:
            raise FileNotFound('File not found.')
        file = self.files[id]
        return {'id': id,
                'ctime': file['ctime'],
                'mtime': file['mtime'],
                'atime': file['atime']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
