from blinker import signal
import random
import string
from uuid import uuid4
from collections import defaultdict

from parsec.service import BaseService, cmd, event


TRUST_SEED_LENGTH = 12


def generate_trust_seed():
    # Use SystemRandom to get cryptographically secure seeds
    return ''.join(random.SystemRandom().choice(string.ascii_uppercase + string.digits)
                   for _ in range(TRUST_SEED_LENGTH))


class BaseVlobService(BaseService):

    on_vlob_updated = event('updated')

    @cmd('create')
    async def _cmd_CREATE(self, msg):
        vlob = await self.create(msg.get('id'), msg.get('blob'))
        return {
            'status': 'ok',
            'id': vlob.id,
            'read_trust_seed': vlob.read_trust_seed,
            'write_trust_seed': vlob.write_trust_seed
        }

    @cmd('read')
    async def _cmd_READ(self, msg):
        vlob = await self.read(msg['id'], msg['trust_seed'])
        version = msg.get('version', len(vlob.blob_versions))
        return {'status': 'ok', 'content': vlob.blob_versions[version - 1], 'version': version}

    @cmd('update')
    async def _cmd_UPDATE(self, msg):
        await self.update(msg['id'], msg['trust_seed'], msg['version'], msg['blob'])
        return {'status': 'ok'}


class Vlob:
    def __init__(self, id=None, blob=None):
        # Generate opaque id if not provided
        self.id = id or uuid4().hex
        self.read_trust_seed = generate_trust_seed()
        self.write_trust_seed = generate_trust_seed()
        self.blob_versions = [blob or '']


class VlobService(BaseVlobService):
    def __init__(self):
        self._vlobs = {}

    async def create(self, id=None, blob=None):
        vlob = Vlob(id=id, blob=blob)
        # TODO: who cares about hash collision ?
        self._vlobs[vlob.id] = vlob
        return vlob

    async def update(self, id, trust_seed, next_version, blob):
        vlob = self._vlobs[id]
        assert trust_seed == vlob.write_trust_seed
        assert next_version == len(vlob.blob_versions) + 1
        vlob.blob_versions.append(blob)
        self.on_vlob_updated.send(id)

    async def read(self, id, trust_seed):
        vlob = self._vlobs[id]
        assert trust_seed == vlob.read_trust_seed
        return vlob
