from base64 import decodebytes
from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.tools import BaseCmdSchema


class cmd_SYNCHRONIZE_MANIFEST_Schema(BaseCmdSchema):
    pass


class cmd_CREATE_GROUP_MANIFEST_Schema(BaseCmdSchema):
    group = fields.String()


class cmd_SHOW_dustbin_Schema(BaseCmdSchema):
    path = fields.String(missing=None)
    group = fields.String(missing=None)


class cmd_HISTORY_Schema(BaseCmdSchema):
    first_version = fields.Integer(missing=1, validate=lambda n: n >= 1)
    last_version = fields.Integer(missing=None, validate=lambda n: n >= 1)
    summary = fields.Boolean(missing=False)
    group = fields.String(missing=None)


class cmd_RESTORE_MANIFEST_Schema(BaseCmdSchema):
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)
    group = fields.String(missing=None)


class ManifestAPIMixin(BaseService):

    @cmd('synchronize')
    async def _cmd_SYNCHRONIZE_MANIFEST(self, session, msg):
        msg = cmd_SYNCHRONIZE_MANIFEST_Schema().load(msg)
        await self.synchronize()
        return {'status': 'ok'}

    @cmd('create_group_manifest')
    async def _cmd_CREATE_GROUP_MANIFEST(self, session, msg):
        msg = cmd_CREATE_GROUP_MANIFEST_Schema().load(msg)
        await self.create_group_manifest(msg['group'])
        return {'status': 'ok'}

    @cmd('show_dustbin')
    async def _cmd_SHOW_dustbin(self, session, msg):
        msg = cmd_SHOW_dustbin_Schema().load(msg)
        dustbin = await self.show_dustbin(msg['path'], msg['group'])
        return {'status': 'ok', 'dustbin': dustbin}

    @cmd('history')
    async def _cmd_MANIFEST_HISTORY(self, session, msg):
        msg = cmd_HISTORY_Schema().load(msg)
        history = await self.history(msg['first_version'],
                                     msg['last_version'],
                                     msg['summary'],
                                     msg['group'])
        history['status'] = 'ok'
        return history

    @cmd('restore')
    async def _cmd_RESTORE_MANIFEST(self, session, msg):
        msg = cmd_RESTORE_MANIFEST_Schema().load(msg)
        await self.restore_manifest(msg['version'], msg['group'])
        return {'status': 'ok'}

    async def synchronize(self):
        manifest = await self.get_manifest()
        await manifest.commit()
        await self.synchronizer.synchronize()

    async def create_group_manifest(self, group):
        manifest = await self.get_manifest()
        vlob = await manifest.create_group_manifest(group)
        await manifest.commit()
        return vlob

    async def show_dustbin(self, path, group):
        manifest = await self.get_manifest(group)
        return await manifest.show_dustbin(path)

    async def history(self, first_version, last_version, summary, group):
        manifest = await self.get_manifest(group)
        return await manifest.history(first_version, last_version, summary)

    async def restore_manifest(self, version, group):
        manifest = await self.get_manifest(group)
        await manifest.restore(version)
