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
    pass


class TrustSeedError(ParsecError):
    status = 'trust_seed_error'
    label = 'Invalid trust seed.'


class VlobNotFound(VlobError):
    status = 'not_found'


class VlobBadVersionError(VlobError):
    status = 'bad_version'
    label = 'Invalid blob version.'


class cmd_CREATE_Schema(BaseCmdSchema):
    blob = fields.String(missing='')


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    version = fields.Int(validate=lambda n: n > 1)
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
        vlob = await self.create(msg['blob'])
        return {
            'status': 'ok',
            'id': vlob.id,
            'read_trust_seed': vlob.read_trust_seed,
            'write_trust_seed': vlob.write_trust_seed
        }

    @cmd('vlob_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        vlob = await self.read(msg['id'])
        if vlob.read_trust_seed != msg['trust_seed']:
            raise TrustSeedError()
        max_version = len(vlob.blob_versions)
        version = msg.get('version', max_version)
        if version > max_version:
            raise VlobBadVersionError()
        blob = vlob.blob_versions[version - 1]
        return {'status': 'ok', 'blob': blob, 'version': version}

    @cmd('vlob_update')
    async def _cmd_UPDATE(self, session, msg):
        msg = cmd_UPDATE_Schema().load(msg)
        vlob = await self.read(msg['id'])
        if vlob.write_trust_seed != msg['trust_seed']:
            raise TrustSeedError()
        await self.update(msg['id'], msg['version'], msg['blob'])
        return {'status': 'ok'}

    async def create(self, blob=None):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def update(self, id, next_version, blob):
        raise NotImplementedError()


class Vlob:
    def __init__(self, id=None, blob=None, read_trust_seed=None, write_trust_seed=None):
        # Generate opaque id if not provided
        self.id = id or uuid4().hex  # TODO uuid4 or trust seed?
        self.read_trust_seed = read_trust_seed or generate_trust_seed()
        self.write_trust_seed = write_trust_seed or generate_trust_seed()
        self.blob_versions = [blob] if blob else []


class MockedVlobService(BaseVlobService):
    def __init__(self):
        super().__init__()
        self._vlobs = {}

    async def create(self, blob=None):
        vlob = Vlob(blob=blob)
        # TODO: who cares about hash collision ?
        self._vlobs[vlob.id] = vlob
        return vlob

    async def read(self, id):
        try:
            return self._vlobs[id]
        except KeyError:
            raise VlobNotFound('Vlob not found.')

    async def update(self, id, next_version, blob):
        try:
            vlob = self._vlobs[id]
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        if next_version == len(vlob.blob_versions) + 1:
            vlob.blob_versions.append(blob)
        else:
            raise VlobBadVersionError('Wrong blob version.')
        self.on_updated.send(id)
