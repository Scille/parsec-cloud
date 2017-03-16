from base64 import decodebytes

from datetime import datetime

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError
from parsec.backend import VlobService


class FileError(ParsecError):
    pass


class FileNotFound(FileError):
    status = 'not_found'


class FileService(BaseService):

    def __init__(self):
        super().__init__()
        self.vlob_service = VlobService()  # TODO call this remotely
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
    async def _cmd_CREATE(self, msg):
        id = await self.create()
        return {'status': 'ok', 'file': id}

    @cmd('read_file')
    async def _cmd_READ(self, msg):
        id = self._get_field(msg, 'id')
        trust_seed = self._get_field(msg, 'trust_seed')
        response = await self.read(id, trust_seed)
        response.update({'status': 'ok'})
        return response

    @cmd('write_file')
    async def _cmd_WRITE(self, msg):
        id = self._get_field(msg, 'id')
        trust_seed = self._get_field(msg, 'trust_seed')
        version = self._get_field(msg, 'version', int)
        content = self._get_field(msg, 'content')
        await self.write(id, trust_seed, version, content)
        return {'status': 'ok'}

    @cmd('stat_file')
    async def _cmd_STAT(self, msg):
        id = self._get_field(msg, 'id')
        stats = await self.stat(id)
        return {'status': 'ok', 'stats': stats}

    @cmd('history')
    async def _cmd_HISTORY(self, msg):
        id = self._get_field(msg, 'id')
        history = await self.history(id)
        return {'status': 'ok', 'history': history}

    async def create(self):
        ret = await self.vlob_service._cmd_CREATE({})  # TODO empty dict ?
        file = {
            'ctime': datetime.utcnow().timestamp(),
            'mtime': datetime.utcnow().timestamp(),
            'atime': datetime.utcnow().timestamp(),
            'size': 0
        }
        self.files[ret['id']] = file
        response = {}
        for key in ('id', 'read_trust_seed', 'write_trust_seed'):
            response[key] = ret[key]
        return ret

    async def read(self, id, trust_seed):
        if id not in self.files:
            raise FileNotFound('File not found.')
        self.files[id]['atime'] = datetime.utcnow().timestamp()
        ret = await self.vlob_service._cmd_READ({'id': id, 'trust_seed': trust_seed})
        file = {}
        for key in ('content', 'version'):
            file[key] = ret[key]
        return file

    async def write(self, id, trust_seed, version, content):
        if id not in self.files:
            raise FileNotFound('File not found.')
        self.files[id]['mtime'] = datetime.utcnow().timestamp()
        self.files[id]['size'] = len(content)
        await self.vlob_service._cmd_UPDATE({'id': id,
                                             'trust_seed': trust_seed,
                                             'version': version,
                                             'blob': content})

    async def stat(self, id):
        if id not in self.files:
            raise FileNotFound('File not found.')
        file = self.files[id]
        return {'id': id,
                'ctime': file['ctime'],
                'mtime': file['mtime'],
                'atime': file['atime'],
                'size': file['size']}

    async def history(self, id):
        # TODO raise ParsecNotImplementedError
        pass
