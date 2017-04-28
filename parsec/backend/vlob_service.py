import random
import string
from uuid import uuid4
from marshmallow import fields

from parsec.service import BaseService, cmd, event
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


TRUST_SEED_LENGTH = 12


def generate_trust_seed():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                   for _ in range(TRUST_SEED_LENGTH))


class VlobError(ParsecError):
    status = 'vlob_error'


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'


class VlobNotFound(VlobError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    blob = fields.String(missing='')


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Int(validate=lambda n: n >= 1)
    trust_seed = fields.String(required=True)


class cmd_UPDATE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Int(validate=lambda n: n > 1)
    trust_seed = fields.String(required=True)
    blob = fields.String(required=True)


class BaseVlobService(BaseService):

    name = 'VlobService'

    on_updated = event('on_vlob_updated')

    @cmd('vlob_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        atom = await self.create(msg['blob'])
        return {
            'status': 'ok',
            'id': atom.id,
            'read_trust_seed': atom.read_trust_seed,
            'write_trust_seed': atom.write_trust_seed
        }

    @cmd('vlob_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        atom = await self.read(msg['id'], msg.get('version'), check_trust_seed=msg['trust_seed'])
        return {'status': 'ok', 'id': atom.id, 'blob': atom.blob, 'version': atom.version}

    @cmd('vlob_update')
    async def _cmd_UPDATE(self, session, msg):
        msg = cmd_UPDATE_Schema().load(msg)
        vlob = await self.read(msg['id'])
        if vlob.write_trust_seed != msg['trust_seed']:
            raise TrustSeedError('Invalid write trust seed.')
        await self.update(msg['id'], msg['version'], msg['blob'], check_trust_seed=msg['trust_seed'])
        return {'status': 'ok'}

    async def create(self, blob=None):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def update(self, id, version, blob):
        raise NotImplementedError()


class VlobAtom:
    def __init__(self, id=None, version=1, blob=None, read_trust_seed=None, write_trust_seed=None):
        # Generate opaque id if not provided
        self.id = id or uuid4().hex  # TODO uuid4 or trust seed?
        self.read_trust_seed = read_trust_seed or generate_trust_seed()
        self.write_trust_seed = write_trust_seed or generate_trust_seed()
        self.blob = blob or ''
        self.version = version


class MockedVlob:
    def __init__(self, *args, **kwargs):
        atom = VlobAtom(*args, **kwargs)
        self.id = atom.id
        self.read_trust_seed = atom.read_trust_seed
        self.write_trust_seed = atom.write_trust_seed
        self.blob_versions = [atom.blob]


class MockedVlobService(BaseVlobService):
    def __init__(self):
        super().__init__()
        self._vlobs = {}

    async def create(self, blob=None):
        vlob = MockedVlob(blob=blob)
        self._vlobs[vlob.id] = vlob
        return VlobAtom(id=vlob.id,
                        read_trust_seed=vlob.read_trust_seed,
                        write_trust_seed=vlob.write_trust_seed,
                        blob=vlob.blob_versions[0])

    async def read(self, id, version=None, check_trust_seed=False):
        try:
            vlob = self._vlobs[id]
            if check_trust_seed and vlob.read_trust_seed != check_trust_seed:
                raise TrustSeedError('Invalid read trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        if not version:
            version = len(vlob.blob_versions)
        try:
            return VlobAtom(id=vlob.id,
                            read_trust_seed=vlob.read_trust_seed,
                            write_trust_seed=vlob.write_trust_seed,
                            blob=vlob.blob_versions[version - 1],
                            version=version)
        except IndexError:
            raise VlobNotFound('Wrong blob version.')

    async def update(self, id, version, blob, check_trust_seed=False):
        try:
            vlob = self._vlobs[id]
            if check_trust_seed and vlob.write_trust_seed != check_trust_seed:
                raise TrustSeedError('Invalid write trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        if version - 1 == len(vlob.blob_versions):
            vlob.blob_versions.append(blob)
        else:
            raise VlobNotFound('Wrong blob version.')
        self.on_updated.send(id)
