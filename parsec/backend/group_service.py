from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class ShareError(ParsecError):
    pass


class cmd_CREATE_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_READ_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class cmd_UPDATE_Schema(BaseCmdSchema):
    name = fields.String(required=True)


class BaseGroupService(BaseService):

    name = 'GroupService'

    @cmd('group_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)

    @cmd('group_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)

    @cmd('group_update')
    async def _cmd_UPDATE(self, session, msg):
        msg = cmd_UPDATE_Schema().load(msg)


class GroupService(BaseGroupService):

    def __init__(self):
        super().__init__()
        self.admins = {}
        self.read_write = {}  # TODO separate read only and write only

    async def create(self):
        pass

    async def read(self):
        pass

    async def update(self):
        pass
