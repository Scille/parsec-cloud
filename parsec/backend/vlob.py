import attr
import random
import string
from uuid import uuid4
from marshmallow import fields

from parsec.utils import UnknownCheckedSchema, to_jsonb64, ParsecError


TRUST_SEED_LENGTH = 12


class VlobError(ParsecError):
    status = 'vlob_error'


class VlobNotFound(VlobError):
    status = 'vlob_not_found'


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'



def generate_trust_seed():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                   for _ in range(TRUST_SEED_LENGTH))


@attr.s
class VlobAtom:
    id = attr.ib()
    read_trust_seed = attr.ib(default=attr.Factory(generate_trust_seed))
    write_trust_seed = attr.ib(default=attr.Factory(generate_trust_seed))
    blob = attr.ib(default=b'')
    version = attr.ib(default=1)


class cmd_CREATE_Schema(UnknownCheckedSchema):
    id = fields.String(missing=None, validate=lambda n: 0 < len(n) <= 32)
    blob = fields.Base64Bytes(missing=to_jsonb64(b''))


class cmd_READ_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    version = fields.Int(validate=lambda n: n >= 1)
    trust_seed = fields.String(required=True)


class cmd_UPDATE_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    version = fields.Int(validate=lambda n: n > 1)
    trust_seed = fields.String(required=True)
    blob = fields.Base64Bytes(required=True)


class MockedVlob:
    def __init__(self, *args, **kwargs):
        atom = VlobAtom(*args, **kwargs)
        self.id = atom.id
        self.read_trust_seed = atom.read_trust_seed
        self.write_trust_seed = atom.write_trust_seed
        self.blob_versions = [atom.blob]


class BaseVlobComponent:

    async def api_vlob_create(self, client_ctx, msg):
        msg = cmd_CREATE_Schema().load(msg)
        atom = await self.create(**msg)
        return {
            'status': 'ok',
            'id': atom.id,
            'read_trust_seed': atom.read_trust_seed,
            'write_trust_seed': atom.write_trust_seed
        }

    async def api_vlob_read(self, client_ctx, msg):
        msg = cmd_READ_Schema().load(msg)
        atom = await self.read(**msg)
        return {
            'status': 'ok',
            'id': atom.id,
            'blob': to_jsonb64(atom.blob),
            'version': atom.version
        }

    async def api_vlob_update(self, client_ctx, msg):
        msg = cmd_UPDATE_Schema().load(msg)
        await self.update(**msg)
        return {'status': 'ok'}

    async def create(self, blob, id=None):
        raise NotImplementedError()

    async def read(self, id, trust_seed, version=None):
        raise NotImplementedError()

    async def update(self, id, trust_seed, version, blob):
        raise NotImplementedError()


@attr.s
class MockedVlobComponent(BaseVlobComponent):
    vlobs = attr.ib(default=attr.Factory(dict))

    async def create(self, blob, id=None):
        # Generate opaque id if not provided
        if not id:
            id = uuid4().hex
        vlob = MockedVlob(id=id, blob=blob)
        self.vlobs[vlob.id] = vlob
        return VlobAtom(id=vlob.id,
                        read_trust_seed=vlob.read_trust_seed,
                        write_trust_seed=vlob.write_trust_seed,
                        blob=vlob.blob_versions[0])

    async def read(self, id, trust_seed, version=None):
        try:
            vlob = self.vlobs[id]
            if vlob.read_trust_seed != trust_seed:
                raise TrustSeedError('Invalid read trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        version = version or len(vlob.blob_versions)
        try:
            return VlobAtom(id=vlob.id,
                            read_trust_seed=vlob.read_trust_seed,
                            write_trust_seed=vlob.write_trust_seed,
                            blob=vlob.blob_versions[version - 1],
                            version=version)
        except IndexError:
            raise VlobNotFound('Wrong blob version.')

    async def update(self, id, trust_seed, version, blob):
        try:
            vlob = self.vlobs[id]
            if vlob.write_trust_seed != trust_seed:
                raise TrustSeedError('Invalid write trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        if version - 1 == len(vlob.blob_versions):
            vlob.blob_versions.append(blob)
        else:
            raise VlobNotFound('Wrong blob version.')
        # TODO: trigger event
        # await Effect(EEvent('vlob_updated', id))
