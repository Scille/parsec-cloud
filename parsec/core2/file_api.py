from marshmallow import fields

from parsec.service import event, cmd
from parsec.tools import BaseCmdSchema


class cmd_FILE_OPEN_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_FILE_READ_Schema(BaseCmdSchema):
    fd = fields.Int(required=True)
    count = fields.Int(missing=-1)


class cmd_FILE_WRITE_Schema(BaseCmdSchema):
    fd = fields.Int(required=True)
    payload = fields.String(required=True)


class cmd_FILE_CLOSE_Schema(BaseCmdSchema):
    fd = fields.Int(required=True)


class FileAPIMixin:

    on_file_changed = event('file_changed')

    @cmd('file_open')
    async def _cmd_FILE_OPEN(self, session, msg):
        msg = cmd_FILE_OPEN_Schema().load(msg)
        await self.file_open(msg['name'])
        return {'status': 'ok'}

    @cmd('file_read')
    async def _cmd_FILE_READ(self, session, msg):
        msg = cmd_FILE_READ_Schema().load(msg)
        await self.file_read(msg['name'])
        return {'status': 'ok'}

    @cmd('file_close')
    async def _cmd_FILE_CLOSE(self, session, msg):
        msg = cmd_FILE_CLOSE_Schema().load(msg)
        await self.file_close(msg['name'])
        return {'status': 'ok'}

    async def file_open(self, name):
        raise NotImplementedError()

    async def file_read(self, name):
        raise NotImplementedError()

    async def file_write(self, name, identities, admin=False):
        raise NotImplementedError()

    async def file_close(self, name, identities, admin=False):
        raise NotImplementedError()
