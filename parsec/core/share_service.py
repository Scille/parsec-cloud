import json

from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class ShareError(ParsecError):
    pass


class cmd_SHARE_WITH_IDENTITY_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    identity = fields.String(required=True)


class cmd_SHARE_WITH_GROUP_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    group = fields.String(required=True)


class cmd_SHARE_STOP_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_GROUP_CREATE_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_GROUP_READ_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_GROUP_ADD_IDENTITIES_Schema(BaseCmdSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String())
    admin = fields.Boolean(missing=False)


class cmd_GROUP_REMOVE_IDENTITIES_Schema(BaseCmdSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String())
    admin = fields.Boolean(missing=False)


class BaseShareService(BaseService):

    name = 'ShareService'

    @cmd('share_with_identity')
    async def _cmd_SHARE_WITH_IDENTITY(self, session, msg):
        msg = cmd_SHARE_WITH_IDENTITY_Schema().load(msg)
        await self.share_with_identity(msg['path'], msg['identity'])
        return {'status': 'ok'}

    @cmd('share_with_group')
    async def _cmd_SHARE_WITH_GROUP(self, session, msg):
        msg = cmd_SHARE_WITH_GROUP_Schema().load(msg)
        await self.share_with_group(msg['path'], msg['group'])
        return {'status': 'ok'}

    @cmd('share_stop')
    async def _cmd_SHARE_STOP(self, session, msg):
        msg = cmd_SHARE_STOP_Schema().load(msg)
        # TODO

    @cmd('group_create')
    async def _cmd_GROUP_CREATE(self, session, msg):
        msg = cmd_GROUP_CREATE_Schema().load(msg)
        await self.group_create(msg['name'])
        return {'status': 'ok'}

    @cmd('group_read')
    async def _cmd_GROUP_READ(self, session, msg):
        msg = cmd_GROUP_READ_Schema().load(msg)
        group = await self.group_read(msg['name'])
        return {'status': 'ok', 'group': group}

    @cmd('group_add_identities')
    async def _cmd_GROUP_ADD_IDENTITIES(self, session, msg):
        msg = cmd_GROUP_ADD_IDENTITIES_Schema().load(msg)
        await self.group_add_identities(msg['name'], msg['identities'], msg['admin'])
        return {'status': 'ok'}

    @cmd('group_remove_identities')
    async def _cmd_GROUP_REMOVE_IDENTITIES(self, session, msg):
        msg = cmd_GROUP_REMOVE_IDENTITIES_Schema().load(msg)
        await self.group_remove_identities(msg['name'], msg['identities'], msg['admin'])
        return {'status': 'ok'}

    async def share_with_identity(self, path, identity):
        raise NotImplementedError()

    async def share_with_group(self, path, group):
        raise NotImplementedError()

    async def stop_share(self, path):
        raise NotImplementedError()

    async def create_group(self, name):
        raise NotImplementedError()

    async def read_group(self, name):
        raise NotImplementedError()

    async def add_identities_to_group(self, name, identities, admin=False):
        raise NotImplementedError()

    async def remove_identities_from_group(self, name, identities, admin=False):
        raise NotImplementedError()


class ShareService(BaseShareService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    pub_keys_service = service('PubKeysService')
    user_manifest_service = service('UserManifestService')

    def __init__(self):
        super().__init__()

    async def share_with_identity(self, path, identity):
        vlob = await self.user_manifest_service.get_properties(path=path)
        # TODO use pub key service ?
        encrypted_vlob = await self.crypto_service.asym_encrypt(json.dumps(vlob), identity)
        await self.backend_api_service.message_new(identity, encrypted_vlob)

    async def share_with_group(self, path, group):
        group = await self.backend_api_service.group_read(group)
        for identity in group['users']:
            await self.share_with_identity(path, identity)  # TODO bug everyone notified

    async def share_stop(self, path):
        # vlob = await self.user_manifest_service.get_properties(path=path)
        # # TODO create a new group manifest
        # identities = []  # TODO users in group
        # vlob = None  # TODO create group manifest
        # for identity in identities:
        #     await self.backend_api_service.message_service.new(identity, vlob)
        pass

    async def group_create(self, name):
        await self.backend_api_service.group_create(name)

    async def group_read(self, name):
        return await self.backend_api_service.group_read(name)

    async def group_add_identities(self, name, identities, admin=False):
        await self.backend_api_service.group_add_identities(name, identities, admin)

    async def group_remove_identities(self, name, identities, admin=False):
        await self.backend_api_service.group_remove_identities(name, identities, admin)
