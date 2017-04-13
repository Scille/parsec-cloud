import json

from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class ShareError(ParsecError):
    pass


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    recipient = fields.String(required=True)


class cmd_STOP_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_CREATE_GROUP_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class BaseShareService(BaseService):

    name = 'ShareService'

    @cmd('share')
    async def _cmd_SHARE(self, session, msg):
        msg = cmd_SHARE_Schema().load(msg)
        await self.share(msg['path'], msg['recipient'])
        return {'status': 'ok'}

    @cmd('stop_share')
    async def _cmd_STOP_SHARE(self, session, msg):
        msg = cmd_STOP_SHARE_Schema().load(msg)

    @cmd('create_group')
    async def _cmd_CREATE_GROUP(self, session, msg):
        msg = cmd_CREATE_GROUP_Schema().load(msg)


class ShareService(BaseShareService):

    backend_api_service = service('BackendAPIService')
    crypto_service = service('CryptoService')
    pub_keys_service = service('PubKeysService')
    user_manifest_service = service('UserManifestService')

    def __init__(self):
        super().__init__()

    async def share(self, path, recipient):
        vlob = await self.user_manifest_service.get_properties(path=path)
        # TODO use pub key service ?
        encrypted_vlob = await self.crypto_service.asym_encrypt(json.dumps(vlob), recipient)
        await self.backend_api_service.message_new(recipient, encrypted_vlob)

    async def stop_share(self, path):
        # vlob = await self.user_manifest_service.get_properties(path=path)
        # # TODO create a new group manifest
        # identities = []  # TODO users in group
        # vlob = None  # TODO create group manifest
        # for identity in identities:
        #     await self.backend_api_service.message_service.new(identity, vlob)
        pass

    async def create_group(self, name):
        # await self.backend_api_service.group_service.new(name)
        pass
