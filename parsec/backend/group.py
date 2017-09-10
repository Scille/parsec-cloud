import attr
from marshmallow import fields
from effect2 import TypeDispatcher, Effect

from parsec.exceptions import GroupAlreadyExist, GroupNotFound
from parsec.tools import UnknownCheckedSchema


@attr.s
class Group:
    name = attr.ib()
    admins = attr.ib(default=attr.Factory(set), convert=set)
    users = attr.ib(default=attr.Factory(set), convert=set)


@attr.s
class EGroupCreate:
    name = attr.ib()


@attr.s
class EGroupRead:
    name = attr.ib()


@attr.s
class EGroupAddIdentities:
    name = attr.ib()
    identities = attr.ib()
    admin = attr.ib()


@attr.s
class EGroupRemoveIdentities:
    name = attr.ib()
    identities = attr.ib()
    admin = attr.ib()


class cmd_CREATE_Schema(UnknownCheckedSchema):
    name = fields.String(required=True)


class cmd_READ_Schema(UnknownCheckedSchema):
    name = fields.String(required=True)


class cmd_ADD_IDENTITIES_Schema(UnknownCheckedSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String(), required=True)
    admin = fields.Boolean(missing=False)


class cmd_REMOVE_IDENTITIES_Schema(UnknownCheckedSchema):
    name = fields.String(required=True)
    identities = fields.List(fields.String(), required=True)
    admin = fields.Boolean(missing=False)


async def api_group_create(msg):
    msg = cmd_CREATE_Schema().load(msg)
    await Effect(EGroupCreate(**msg))
    return {'status': 'ok'}


async def api_group_read(msg):
    msg = cmd_READ_Schema().load(msg)
    group = await Effect(EGroupRead(**msg))
    return {
        'status': 'ok',
        'name': group.name,
        'admins': list(group.admins),
        'users': list(group.users)
    }


async def api_group_add_identities(msg):
    msg = cmd_ADD_IDENTITIES_Schema().load(msg)
    msg['identities'] = set(msg['identities'])
    await Effect(EGroupAddIdentities(**msg))
    return {'status': 'ok'}


async def api_group_remove_identities(msg):
    msg = cmd_REMOVE_IDENTITIES_Schema().load(msg)
    msg['identities'] = set(msg['identities'])
    await Effect(EGroupRemoveIdentities(**msg))
    return {'status': 'ok'}


@attr.s
class MockedGroupComponent:
    _groups = attr.ib(default=attr.Factory(dict))

    async def perform_group_create(self, intent):
        if intent.name in self._groups:
            raise GroupAlreadyExist('Group already exist.')
        self._groups[intent.name] = Group(intent.name)

    async def perform_group_read(self, intent):
        try:
            return self._groups[intent.name]
        except KeyError:
            raise GroupNotFound('Group not found.')

    async def perform_group_add_identities(self, intent):
        try:
            group = self._groups[intent.name]
        except KeyError:
            raise GroupNotFound('Group not found.')
        if intent.admin:
            group.admins |= intent.identities
        else:
            group.users |= intent.identities
        # TODO: send message to added user

    async def perform_group_remove_identities(self, intent):
        try:
            group = self._groups[intent.name]
        except KeyError:
            raise GroupNotFound('Group not found.')
        if intent.admin:
            group.admins -= intent.identities
        else:
            group.users -= intent.identities
        # TODO: send message to removed user

    def get_dispatcher(self):
        return TypeDispatcher({
            EGroupRead: self.perform_group_read,
            EGroupCreate: self.perform_group_create,
            EGroupAddIdentities: self.perform_group_add_identities,
            EGroupRemoveIdentities: self.perform_group_remove_identities
        })