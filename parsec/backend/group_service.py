from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class GroupError(ParsecError):
    pass


class GroupNotFound(GroupError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_READ_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_ADD_IDENTITIES_Schema(BaseCmdSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String())
    admin = fields.Boolean(missing=False)


class cmd_REMOVE_IDENTITIES_Schema(BaseCmdSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String())
    admin = fields.Boolean(missing=False)


class BaseGroupService(BaseService):

    name = 'GroupService'

    @cmd('group_create')
    async def _cmd__CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        await self.create(msg['name'])
        return {'status': 'ok'}

    @cmd('group_read')
    async def _cmd__READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        group = await self.read(msg['name'])
        return {'status': 'ok', 'group': group}

    @cmd('group_add_identities')
    async def _cmd_ADD_IDENTITIES(self, session, msg):
        msg = cmd_ADD_IDENTITIES_Schema().load(msg)
        await self.add_identities(msg['name'], msg['identities'], msg['admin'])
        return {'status': 'ok'}

    @cmd('group_remove_identities')
    async def _cmd__REMOVE_IDENTITIES(self, session, msg):
        msg = cmd_REMOVE_IDENTITIES_Schema().load(msg)
        await self.remove_identities(msg['name'], msg['identities'], msg['admin'])
        return {'status': 'ok'}

    async def create(self, name):
        raise NotImplementedError()

    async def read(self, name):
        raise NotImplementedError()

    async def add_identities(self, name, identities, admin=False):
        raise NotImplementedError()

    async def remove_identities(self, name, identities, admin=False):
        raise NotImplementedError()


class MockedGroupService(BaseGroupService):

    def __init__(self):
        super().__init__()
        self.groups = {}

    async def create(self, name):
        if name in self.groups:
            raise(GroupError('already_exist', 'Group already exist.'))
        self.groups[name] = {'admins': [], 'users': []}

    async def read(self, name):
        try:
            return self.groups[name]
        except Exception:
            raise(GroupNotFound('Group not found.'))

    async def add_identities(self, name, identities, admin=False):
        subgroup = 'admins' if admin else 'users'
        try:
            group = self.groups[name][subgroup]
        except Exception:
            raise(GroupNotFound('Group not found.'))
        if not isinstance(identities, list):
            identities = [identities]
        group += identities
        group = list(set(group))
        self.groups[name][subgroup] = group

    async def remove_identities(self, name, identities, admin=False):
        subgroup = 'admins' if admin else 'users'
        try:
            group = self.groups[name][subgroup]
        except Exception:
            raise(GroupNotFound('Group not found.'))
        if not isinstance(identities, list):
            identities = [identities]
        self.groups[name][subgroup] = [identity for identity in group if identity not in identities]
