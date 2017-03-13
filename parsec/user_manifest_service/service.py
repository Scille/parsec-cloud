from base64 import decodebytes

from parsec.base import BaseService, cmd, ParsecError
from parsec.file_service import FileService


class ManifestError(ParsecError):
    pass


class ManifestNotFound(ManifestError):
    status = 'not_found'


class ManifestService(BaseService):

    def __init__(self):
        self.file_service = FileService()  # TODO register file service
        self.manifest = {}

    @staticmethod
    def _pack_manifest_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise ManifestError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise ManifestError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise ManifestError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise ManifestError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise ManifestError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('create_file')
    async def cmd_CREATE_FILE(self, msg):
        if 'path' not in msg:
            raise ManifestError('bad_params', 'Invalid parameters')
        await self.create_file(msg['path'])
        return {'status': 'ok'}

    @cmd('delete_file')
    async def cmd_DELETE_FILE(self, msg):
        path = self._get_field(msg, 'path')
        await self.delete_file(path)
        return {'status': 'ok'}

    @cmd('list_dir')
    async def cmd_LIST_DIR(self, msg):
        path = self._get_field(msg, 'path')
        listing = await self.list_dir(path)
        return {'status': 'ok', 'list': listing}

    @cmd('make_dir')
    async def cmd_MAKE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.make_dir(path)
        return {'status': 'ok'}

    @cmd('remove_dir')
    async def cmd_REMOVE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.remove_dir(path)
        return {'status': 'ok'}

    @cmd('history')
    async def cmd_HISTORY(self, msg):
        path = self._get_field(msg, 'path')
        history = await self.history(path)
        return {'status': 'ok', 'history': history}

    async def create_file(self, path):
        if path in self.manifest:
            raise ManifestError('already_exist', 'Target already exists.')
        else:
            id = await self.file_service.create()
            # TODO set correct values
            self.manifest[path] = {'id': id, 'read_seed': None, 'write_seed': None, 'key': None}
        return self.manifest[path]

    async def delete_file(self, path):
        try:
            del self.manifest[path]
        except KeyError:
            raise ManifestNotFound('File not found.')

    async def list_dir(self, path):
        if path != '/' and path not in self.manifest:
            raise ManifestNotFound('Directory not found.')
        results = []
        for entry in self.manifest:
            if entry != path and entry.startswith(path) and entry.count('/', len(path) + 1) == 0:
                results.append(entry.split('/')[-1])
        return results

    async def make_dir(self, path):
        if path in self.manifest:
            raise ManifestError('already_exist', 'Target already exists.')
        else:
            # TODO set correct values
            self.manifest[path] = {'id': None, 'read_seed': None, 'write_seed': None, 'key': None}
        return self.manifest[path]

    async def remove_dir(self, path):
        if path == '/':
            raise ManifestError('cannot_remove_root', 'Cannot remove root directory.')
        for entry in self.manifest:
            if entry != path and entry.startswith(path):
                raise ManifestError('directory_not_empty', 'Directory not empty.')
        try:
            del self.manifest[path]
        except KeyError:
            raise ManifestNotFound('Directory not found.')

    async def history(self):
        # TODO raise ParsecNotImplementedError
        pass
