from base64 import decodebytes, encodebytes

from parsec.base import BaseService, cmd, ParsecError


class VolumeError(ParsecError):
    pass


class BaseVolumeService(BaseService):

    @staticmethod
    def _get_field(msg, field, type_=str):
        value = msg.get(field)
        if value is None:
            raise VolumeError('bad_params', 'Param `%s` is required' % field)
        if type_ is bytes:
            try:
                value = decodebytes(value.encode())
            except TypeError:
                raise VolumeError('bad_params', 'Param `%s` is not valid base64 data' % field)
        if not isinstance(value, type_):
            raise VolumeError('bad_params', 'Param `%s` must be of type `%s`' % (field, type_))
        return value

    @cmd('read_file')
    async def cmd_READ_FILE(self, msg):
        vid = self._get_field(msg, 'vid')
        content = await self.read_file(vid)
        return {'status': 'ok', 'content': encodebytes(content).decode()}

    @cmd('write_file')
    async def cmd_WRITE_FILE(self, msg):
        vid = self._get_field(msg, 'vid')
        content = self._get_field(msg, 'content', bytes)
        size = await self.write_file(vid, content)
        return {'status': 'ok', 'size': size}

    @cmd('delete_file')
    async def cmd_DELETE_FILE(self, msg):
        vid = self._get_field(msg, 'vid')
        await self.delete_file(vid)
        return {'status': 'ok'}
