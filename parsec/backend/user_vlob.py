import attr
from marshmallow import fields

from parsec.utils import UnknownCheckedSchema, to_jsonb64
from parsec.backend.exceptions import VersionError


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

    def __init__(self, signal_ns):
        self._signal_user_vlob_updated = signal_ns.signal('user_vlob_updated')

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
