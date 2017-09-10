import attr
from collections import defaultdict
from marshmallow import fields
from effect2 import TypeDispatcher, Effect

from parsec.base import EEvent
from parsec.backend.session import EGetAuthenticatedUser
from parsec.exceptions import UserVlobError
from parsec.tools import UnknownCheckedSchema, to_jsonb64


@attr.s
class UserVlobAtom:
    # Generate opaque id if not provided
    id = attr.ib()
    version = attr.ib(default=0)
    blob = attr.ib(default=b'')


@attr.s
class EUserVlobRead:
    version = attr.ib(default=None)


@attr.s
class EUserVlobUpdate:
    version = attr.ib()
    blob = attr.ib()


class cmd_READ_Schema(UnknownCheckedSchema):
    version = fields.Int(validate=lambda n: n >= 0)


class cmd_UPDATE_Schema(UnknownCheckedSchema):
    version = fields.Int(validate=lambda n: n > 0)
    blob = fields.Base64Bytes(required=True)


async def api_user_vlob_read(msg):
    msg = cmd_READ_Schema().load(msg)
    atom = await Effect(EUserVlobRead(**msg))
    return {
        'status': 'ok',
        'blob': to_jsonb64(atom.blob),
        'version': atom.version
    }


async def api_user_vlob_update(msg):
    msg = cmd_UPDATE_Schema().load(msg)
    await Effect(EUserVlobUpdate(**msg))
    return {'status': 'ok'}


@attr.s
class MockedUserVlobComponent:
    vlobs = attr.ib(default=attr.Factory(lambda: defaultdict(list)))

    async def perform_user_vlob_read(self, intent):
        id = await Effect(EGetAuthenticatedUser())
        vlobs = self.vlobs[id]
        if intent.version == 0 or (intent.version is None and not vlobs):
            return UserVlobAtom(id=id)
        try:
            if intent.version is None:
                return vlobs[-1]
            else:
                return vlobs[intent.version - 1]
        except IndexError:
            raise UserVlobError('Wrong blob version.')

    async def perform_user_vlob_update(self, intent):
        id = await Effect(EGetAuthenticatedUser())
        vlobs = self.vlobs[id]
        if len(vlobs) != intent.version - 1:
            raise UserVlobError('Wrong blob version.')
        vlobs.append(UserVlobAtom(id=id, version=intent.version, blob=intent.blob))
        await Effect(EEvent('user_vlob_updated', id))

    def get_dispatcher(self):
        return TypeDispatcher({
            EUserVlobRead: self.perform_user_vlob_read,
            EUserVlobUpdate: self.perform_user_vlob_update,
        })