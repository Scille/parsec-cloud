import attr
import random
import string
from uuid import uuid4

from parsec.utils import to_jsonb64
from parsec.schema import _BaseCmdSchema, fields


TRUST_SEED_LENGTH = 12


def generate_trust_seed():
    # Use SystemRandom to get cryptographically secure seeds
    return "".join(
        random.SystemRandom().choice(string.ascii_uppercase + string.digits)
        for _ in range(TRUST_SEED_LENGTH)
    )


@attr.s
class VlobAtom:
    id = attr.ib()
    read_trust_seed = attr.ib(default=attr.Factory(generate_trust_seed))
    write_trust_seed = attr.ib(default=attr.Factory(generate_trust_seed))
    blob = attr.ib(default=b"")
    version = attr.ib(default=1)


class _cmd_CREATE_Schema(_BaseCmdSchema):
    # TODO: blob must be present
    blob = fields.Base64Bytes(missing=to_jsonb64(b""))


class _cmd_READ_Schema(_BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(validate=lambda n: n >= 1)
    trust_seed = fields.String(required=True)


class _cmd_UPDATE_Schema(_BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Integer(validate=lambda n: n > 1)
    trust_seed = fields.String(required=True)
    blob = fields.Base64Bytes(required=True)


cmd_CREATE_Schema = _cmd_CREATE_Schema()
cmd_READ_Schema = _cmd_READ_Schema()
cmd_UPDATE_Schema = _cmd_UPDATE_Schema()


class BaseVlobComponent:
    def __init__(self, signal_ns):
        self._signal_vlob_updated = signal_ns.signal("vlob_updated")

    async def api_vlob_create(self, client_ctx, msg):
        msg = cmd_CREATE_Schema.load_or_abort(msg)
        id = uuid4().hex
        rts = uuid4().hex
        wts = uuid4().hex
        atom = await self.create(id, rts, wts, msg["blob"])
        return {
            "status": "ok",
            "id": atom.id,
            "read_trust_seed": atom.read_trust_seed,
            "write_trust_seed": atom.write_trust_seed,
        }

    async def api_vlob_read(self, client_ctx, msg):
        msg = cmd_READ_Schema.load_or_abort(msg)
        atom = await self.read(**msg)
        return {
            "status": "ok",
            "id": atom.id,
            "blob": to_jsonb64(atom.blob),
            "version": atom.version,
        }

    async def api_vlob_update(self, client_ctx, msg):
        msg = cmd_UPDATE_Schema.load_or_abort(msg)
        await self.update(**msg)
        return {"status": "ok"}

    async def create(self, rts, wts, blob):
        raise NotImplementedError()

    async def read(self, id, trust_seed, version=None):
        raise NotImplementedError()

    async def update(self, id, trust_seed, version, blob):
        raise NotImplementedError()
