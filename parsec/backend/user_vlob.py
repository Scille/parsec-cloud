import attr
from collections import defaultdict
from marshmallow import fields

from parsec.utils import UnknownCheckedSchema, to_jsonb64, ParsecError


class UserVlobError(ParsecError):
    status = 'user_vlob_error'


@attr.s
class UserVlobAtom:
    # Generate opaque id if not provided
    id = attr.ib()
    version = attr.ib(default=0)
    blob = attr.ib(default=b'')


class cmd_READ_Schema(UnknownCheckedSchema):
    version = fields.Int(validate=lambda n: n >= 0)


class cmd_UPDATE_Schema(UnknownCheckedSchema):
    version = fields.Int(validate=lambda n: n > 0)
    blob = fields.Base64Bytes(required=True)


class BaseUserVlobComponent:
    async def api_user_vlob_read(self, client_ctx, msg):
        msg = cmd_READ_Schema().load(msg)
        atom = await self.read(client_ctx.id, **msg)
        return {
            'status': 'ok',
            'blob': to_jsonb64(atom.blob),
            'version': atom.version
        }

    async def api_user_vlob_update(self, client_ctx, msg):
        msg = cmd_UPDATE_Schema().load(msg)
        await self.update(client_ctx.id, **msg)
        return {'status': 'ok'}

    async def read(self, id, version):
        raise NotImplementedError()

    async def update(self, id, version, blob):
        raise NotImplementedError()


@attr.s
class MockedUserVlobComponent(BaseUserVlobComponent):
    vlobs = attr.ib(default=attr.Factory(lambda: defaultdict(list)))

    async def read(self, id, version):
        vlobs = self.vlobs[id]
        if version == 0 or (version is None and not vlobs):
            return UserVlobAtom(id=id)
        try:
            if version is None:
                return vlobs[-1]
            else:
                return vlobs[version - 1]
        except IndexError:
            raise UserVlobError('Wrong blob version.')

    async def update(self, id, version, blob):
        vlobs = self.vlobs[id]
        if len(vlobs) != version - 1:
            raise UserVlobError('Wrong blob version.')
        vlobs.append(UserVlobAtom(id=id, version=version, blob=blob))
        # TODO: trigger event
        # await Effect(EEvent('user_vlob_updated', id))
