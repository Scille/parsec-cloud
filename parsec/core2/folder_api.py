from marshmallow import fields

from parsec.service import event, cmd
from parsec.tools import BaseCmdSchema


class cmd_PATH_INFO_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_FOLDER_CREATE_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_FOLDER_DELETE_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class FolderAPIMixin:

    on_folder_changed = event('folder_changed')

    @cmd('path_info')
    async def _cmd_PATH_INFO(self, session, msg):
        msg = cmd_PATH_INFO_Schema().load(msg)
        await self.path_info(msg['path'])
        return {'status': 'ok'}

    @cmd('folder_create')
    async def _cmd_FOLDER_CREATE(self, session, msg):
        msg = cmd_FOLDER_CREATE_Schema().load(msg)
        await self.folder_create(msg['name'])
        return {'status': 'ok'}

    @cmd('folder_delete')
    async def _cmd_FOLDER_DELETE(self, session, msg):
        msg = cmd_FOLDER_DELETE_Schema().load(msg)
        await self.folder_delete(msg['name'])
        return {'status': 'ok'}

    async def path_info(self, path):
        # Should provide info for both folder and files !
        # Return examples:
        # File: {'status': 'ok', 'type': 'file', 'ctime': ..., 'mtime': ..., 'size': ...}
        # Folder: {'status': 'ok', 'type': 'folder', 'ctime': ..., 'items': ['foo.txt', 'bar']}
        # Skip mtime and size given they are too complicated to provide for folder
        raise NotImplementedError()

    async def folder_create(self, name):
        raise NotImplementedError()

    async def folder_delete(self, name, identities, admin=False):
        raise NotImplementedError()
