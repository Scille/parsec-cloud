from base64 import decodebytes, encodebytes

from parsec.base import BaseService, cmd, ParsecError


class VFSError(ParsecError):
    pass


class VFSNotFound(VFSError):
    status = 'not_found'


class BaseVFSService(BaseService):

    @staticmethod
    def _pack_vfs_error(error):
        if error.message:
            return ('%s %s' % (error.status, error.message)).encode()
        else:
            return error.status.encode()

    @staticmethod
    def _extract_params(raw_data, *types):
        number = len(types)
        splitted = raw_data.split(b' ', maxsplit=number - 1)
        if len(splitted) != number:
            raise VFSError('bad_params', 'Invalid parameters')
        try:
            for i, tp in enumerate(types):
                if tp is bytes:
                    continue
                elif tp is str:
                    splitted[i] = splitted[i].decode()
                else:
                    splitted[i] = tp(splitted[i])
        except TypeError:
            raise VFSError('bad_params', 'Parameter %s should be of type %s' % (i, tp))
        return splitted

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise VFSError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise VFSError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise VFSError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('create_file')
    async def __CREATE_FILE(self, msg):
        if 'content' not in msg:
            msg['content'] = ''
        return await self.__WRITE_FILE(msg)

    @cmd('read_file')
    async def __READ_FILE(self, msg):
        path = self._get_field(msg, 'path')
        content = await self.read_file(path)
        return {'status': 'ok', 'content': encodebytes(content).decode()}

    @cmd('write_file')
    async def __WRITE_FILE(self, msg):
        path = self._get_field(msg, 'path')
        content = self._get_field(msg, 'content', bytes)
        count = await self.write_file(path, content)
        return {'status': 'ok', 'count': count}

    @cmd('delete_file')
    async def __DELETE_FILE(self, msg):
        path = self._get_field(msg, 'path')
        await self.delete_file(path)
        return {'status': 'ok'}

    @cmd('stat')
    async def __STAT(self, msg):
        path = self._get_field(msg, 'path')
        stat = await self.stat(path)
        return {'status': 'ok', 'stat': stat}

    @cmd('list_dir')
    async def __LIST_DIR(self, msg):
        path = self._get_field(msg, 'path')
        listing = await self.list_dir(path)
        return {'status': 'ok', 'list': listing}

    @cmd('make_dir')
    async def __MAKE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.make_dir(path)
        return {'status': 'ok'}

    @cmd('remove_dir')
    async def __REMOVE_DIR(self, msg):
        path = self._get_field(msg, 'path')
        await self.remove_dir(path)
        return {'status': 'ok'}
