import json

from marshmallow import fields

from parsec.crypto import RSAPublicKey
from parsec.exceptions import ShareError
from parsec.service import BaseService, cmd
from parsec.tools import BaseCmdSchema


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
    identities = fields.List(fields.String(), required=True)
    admin = fields.Boolean(missing=False)


class cmd_GROUP_REMOVE_IDENTITIES_Schema(BaseCmdSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String(), required=True)
    admin = fields.Boolean(missing=False)


class ShareAPIMixin(BaseService):

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
        await self.share_stop(msg['path'])
        return {'status': 'ok'}

    @cmd('group_create')
    async def _cmd_GROUP_CREATE(self, session, msg):
        msg = cmd_GROUP_CREATE_Schema().load(msg)
        await self.group_create(msg['name'])
        return {'status': 'ok'}

    @cmd('group_read')
    async def _cmd_GROUP_READ(self, session, msg):
        msg = cmd_GROUP_READ_Schema().load(msg)
        group = await self.group_read(msg['name'])
        return {'status': 'ok', 'admins': group['admins'], 'users': group['users']}

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
        vlob = await self.get_properties(path=path)
        encryptor = RSAPublicKey('KEY FROM PUBKEY SERVICE')  # TODO
        encrypted_vlob = await encryptor.encrypt(json.dumps(vlob), identity)
        await self.backend.message_new(identity, encrypted_vlob.decode())

    async def share_with_group(self, path, group):
        group = await self.backend.group_read(group)
        for identity in group['users']:
            await self.share_with_identity(path, identity)  # TODO bug everyone notified

    async def share_stop(self, path):
        core = await self.get_manifest()
        await core.reencrypt_file(path)

    async def import_shared_vlob(self):
        identity = await self.identity.get_identity()
        messages = await self.backend.message_get(identity)  # TODO get last
        if not messages:
            raise ShareError('No shared vlob in messages queue.')
        message = await self.identity.decrypt(messages[-1])
        message = json.loads(message.decode())
        if 'group' in message and not isinstance(message['group'], dict):  # TODO message format?
            await self.import_group_vlob(message['group'], message['vlob'])
        else:
            path = '/share-' + message['id']
            await self.import_file_vlob(path, message)

    async def group_create(self, name):
        await self.backend.group_create(name)
        await self.create_group_manifest(name)

    async def group_read(self, name):
        return await self.backend.group_read(name)

    async def group_add_identities(self, name, identities, admin=False):
        # TODO check admin
        await self.backend.group_add_identities(name, identities, admin)
        vlob = await self.get_properties(group=name)
        message = {'group': name, 'vlob': vlob}
        for identity in identities:
            # TODO use pub key service ?
            encrypted_msg = await encryptor.encrypt(json.dumps(message), identity)
            await self.backend.message_new(identity, encrypted_msg.decode())

    async def group_remove_identities(self, name, identities, admin=False):
        # TODO check admin
        group = await self.backend.group_read(name)
        old_identities = group['admins'] if admin else group['users']
        await self.backend.group_remove_identities(name, identities, admin)
        await self.reencrypt_group_manifest(name)
        vlob = await self.get_properties(group=name)
        message = {'group': name, 'vlob': vlob}
        for identity in [identity for identity in old_identities if identity not in identities]:
            encrypted_msg = await encryptor.encrypt(json.dumps(message), identity)
            await self.backend.message_new(identity, encrypted_msg.decode())
