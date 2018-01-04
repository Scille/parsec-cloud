import attr
from marshmallow import fields

from parsec.utils import UnknownCheckedSchema
from parsec.backend.exceptions import AlreadyExistsError, NotFoundError


@attr.s
class Group:
    name = attr.ib()
    admins = attr.ib(default=attr.Factory(set), convert=set)
    users = attr.ib(default=attr.Factory(set), convert=set)


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


class BaseGroupComponent:

    def __init__(self, signal_ns):
        pass

    async def api_group_create(self, client_ctx, msg):
        msg = cmd_CREATE_Schema().load(msg)
        await self.create(**msg)
        return {'status': 'ok'}


    async def api_group_read(self, client_ctx, msg):
        msg = cmd_READ_Schema().load(msg)
        group = await self.read(**msg)
        return {
            'status': 'ok',
            'name': group.name,
            'admins': list(group.admins),
            'users': list(group.users)
        }


    async def api_group_add_identities(self, client_ctx, msg):
        msg = cmd_ADD_IDENTITIES_Schema().load(msg)
        msg['identities'] = set(msg['identities'])
        await self.add_identities(**msg)
        return {'status': 'ok'}


    async def api_group_remove_identities(self, client_ctx, msg):
        msg = cmd_REMOVE_IDENTITIES_Schema().load(msg)
        msg['identities'] = set(msg['identities'])
        await self.remove_identities(**msg)
        return {'status': 'ok'}


class MockedGroupComponent(BaseGroupComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self._groups = {}

    async def perform_group_create(self, name):
        if name in self._groups:
            raise AlreadyExistsError('Group Already exists.')
        self._groups[name] = Group(name)

    async def perform_group_read(self, name):
        try:
            return self._groups[name]
        except KeyError:
            raise NotFoundError('Group not found.')

    async def perform_group_add_identities(self, name, identities, admin):
        try:
            group = self._groups[name]
        except KeyError:
            raise NotFoundError('Group not found.')
        if admin:
            group.admins |= identities
        else:
            group.users |= identities
        # TODO: send message to added user

    async def perform_group_remove_identities(self, name, identities, admin):
        try:
            group = self._groups[name]
        except KeyError:
            raise NotFoundError('Group not found.')
        if admin:
            group.admins -= identities
        else:
            group.users -= identities
        # TODO: send message to removed user
