from base64 import decodebytes
import json

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError


class UserManifestError(ParsecError):
    pass


class UserManifestNotFound(UserManifestError):
    status = 'not_found'


class UserManifestService(BaseService):

    file_service = service('FileService')
    identity_service = service('IdentityService')

    def __init__(self):
        super().__init__()
        self.manifest = {}
        self.manifest['/'] = {'id': None,
                              'read_trust_seed': None,
                              'write_trust_seed': None,
                              'key': None}  # TODO call make_dir

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
            raise UserManifestError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise UserManifestError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise UserManifestError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise UserManifestError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise UserManifestError('bad_params', 'Param `%s` must be of type `%s`'
                                    % (field, type_))
        return value

    @cmd('create_file')
    async def _cmd_CREATE_FILE(self, msg):
        path = self._get_field(msg, 'path')
        file = await self.create_file(path)
        return {'status': 'ok', 'file': file}

    @cmd('rename_file')
    async def _cmd_RENAME_FILE(self, msg):
        old_path = self._get_field(msg, 'old_path')
        new_path = self._get_field(msg, 'new_path')
        await self.rename_file(old_path, new_path)
        return {'status': 'ok'}

    @cmd('delete_file')
    async def _cmd_DELETE_FILE(self, msg):
        path = self._get_field(msg, 'path')
        await self.delete_file(path)
        return {'status': 'ok'}

    @cmd('list_dir')
    async def _cmd_LIST_DIR(self, msg):
        path = self._get_field(msg, 'path')
        current, childrens = await self.list_dir(path)
        return {'status': 'ok', 'current': current, 'childrens': childrens}

    @cmd('make_dir')
    async def _cmd_MAKE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.make_dir(path)
        return {'status': 'ok'}

    @cmd('remove_dir')
    async def _cmd_REMOVE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.remove_dir(path)
        return {'status': 'ok'}

    @cmd('history')
    async def _cmd_HISTORY(self, msg):
        path = self._get_field(msg, 'path')
        history = await self.history(path)
        return {'status': 'ok', 'history': history}

    @cmd('load_from_file')
    async def _cmd_LOAD_FROM_FILE(self, msg):
        user_manifest_file = self._get_field(msg, 'user_manifest_file')
        if await self.load_from_file(user_manifest_file):
            return {'status': 'ok'}
        else:
            raise UserManifestError()

    @cmd('dump_to_file')
    async def _cmd_DUMP_TO_FILE(self, msg):
        user_manifest_file = self._get_field(msg, 'user_manifest_file')
        await self.dump_to_file(user_manifest_file)

    async def create_file(self, path):
        if path in self.manifest:
            raise UserManifestError('already_exist', 'Target already exists.')
        else:
            ret = await self.file_service.create()
            file = {}
            for key in ('id', 'read_trust_seed', 'write_trust_seed'):
                file[key] = ret[key]
            file['key'] = None  # TODO set value
            self.manifest[path] = file
        return self.manifest[path]

    async def rename_file(self, old_path, new_path):
        self.manifest[new_path] = self.manifest[old_path]
        del self.manifest[old_path]

    async def delete_file(self, path):
        try:
            del self.manifest[path]
        except KeyError:
            raise UserManifestNotFound('File not found.')

    async def list_dir(self, path):
        if path != '/' and path not in self.manifest:
            raise UserManifestNotFound('Directory not found.')
        results = {}
        for entry in self.manifest:
            if entry != path and entry.startswith(path) and entry.count('/', len(path) + 1) == 0:
                results[entry.split('/')[-1]] = self.manifest[entry]
        return self.manifest[path], results

    async def make_dir(self, path):
        if path in self.manifest:
            raise UserManifestError('already_exist', 'Target already exists.')
        else:
            self.manifest[path] = {'id': None,
                                   'read_trust_seed': None,
                                   'write_trust_seed': None,
                                   'key': None}  # TODO set correct values
        return self.manifest[path]

    async def remove_dir(self, path):
        if path == '/':
            raise UserManifestError('cannot_remove_root', 'Cannot remove root directory.')
        for entry in self.manifest:
            if entry != path and entry.startswith(path):
                raise UserManifestError('directory_not_empty', 'Directory not empty.')
        try:
            del self.manifest[path]
        except KeyError:
            raise UserManifestNotFound('Directory not found.')

    async def load_from_file(self, manifest_file):
        with open(manifest_file, 'rb') as manifest_file:
            new_manifest = await self.identity_service.decrypt(manifest_file.read())
        if new_manifest:
            new_manifest = json.load(manifest_file)
            consistency = await self.check_consistency(new_manifest)
            if consistency:
                self.manifest = new_manifest
                return True
        return False

    async def dump_to_file(self, manifest_file):
        manifest = json.dumps(self.manifest,
                              sort_keys=True,
                              indent=4,
                              separators=(',', ': '))  # TODO remove indentation ?
        manifest = bytes(manifest, encoding='UTF-8')
        crypted_manifest = await self.identity_service.encrypt(manifest)
        with open(manifest_file, 'wb') as outfile:
            outfile.write(crypted_manifest)

    async def check_consistency(self, manifest):
        for _, entry in manifest.items():
            if entry['id']:
                try:
                    await self.file_service.stat(entry)
                except Exception:
                    return False
        return True

    async def history(self):
        # TODO raise ParsecNotImplementedError
        pass
