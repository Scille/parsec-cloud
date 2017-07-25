import attr
import random
import string
from uuid import uuid4
from marshmallow import fields
from effect2 import TypeDispatcher, Effect, do

from parsec.base import EEvent
from parsec.exceptions import VlobNotFound, TrustSeedError
from parsec.tools import UnknownCheckedSchema, to_jsonb64


TRUST_SEED_LENGTH = 12


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


@attr.s
class EVlobCreate:
    id = attr.ib(default=None)
    blob = attr.ib(default=b'')


@attr.s
class EVlobRead:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib(default=None)


@attr.s
class EVlobUpdate:
    id = attr.ib()
    version = attr.ib()
    trust_seed = attr.ib()
    blob = attr.ib()


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


@do
def api_vlob_create(msg):
    msg = cmd_CREATE_Schema().load(msg)
    atom = yield Effect(EVlobCreate(**msg))
    return {
        'status': 'ok',
        'id': atom.id,
        'read_trust_seed': atom.read_trust_seed,
        'write_trust_seed': atom.write_trust_seed
    }


@do
def api_vlob_read(msg):
    msg = cmd_READ_Schema().load(msg)
    atom = yield Effect(EVlobRead(**msg))
    return {
        'status': 'ok',
        'id': atom.id,
        'blob': to_jsonb64(atom.blob),
        'version': atom.version
    }


@do
def api_vlob_update(msg):
    msg = cmd_UPDATE_Schema().load(msg)
    yield Effect(EVlobUpdate(**msg))
    return {'status': 'ok'}


class MockedVlob:
    def __init__(self, *args, **kwargs):
        atom = VlobAtom(*args, **kwargs)
        self.id = atom.id
        self.read_trust_seed = atom.read_trust_seed
        self.write_trust_seed = atom.write_trust_seed
        self.blob_versions = [atom.blob]


@attr.s
class MockedVlobComponent:
    vlobs = attr.ib(default=attr.Factory(dict))

    @do
    def perform_vlob_create(self, intent):
        # Generate opaque id if not provided
        if not intent.id:
            intent.id = uuid4().hex
        vlob = MockedVlob(id=intent.id, blob=intent.blob)
        self.vlobs[vlob.id] = vlob
        return VlobAtom(id=vlob.id,
                        read_trust_seed=vlob.read_trust_seed,
                        write_trust_seed=vlob.write_trust_seed,
                        blob=vlob.blob_versions[0])

    @do
    def perform_vlob_read(self, intent):
        try:
            vlob = self.vlobs[intent.id]
            if vlob.read_trust_seed != intent.trust_seed:
                raise TrustSeedError('Invalid read trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        version = intent.version or len(vlob.blob_versions)
        try:
            return VlobAtom(id=vlob.id,
                            read_trust_seed=vlob.read_trust_seed,
                            write_trust_seed=vlob.write_trust_seed,
                            blob=vlob.blob_versions[version - 1],
                            version=version)
        except IndexError:
            raise VlobNotFound('Wrong blob version.')

    @do
    def perform_vlob_update(self, intent):
        try:
            vlob = self.vlobs[intent.id]
            if vlob.write_trust_seed != intent.trust_seed:
                raise TrustSeedError('Invalid write trust seed.')
        except KeyError:
            raise VlobNotFound('Vlob not found.')
        if intent.version - 1 == len(vlob.blob_versions):
            vlob.blob_versions.append(intent.blob)
        else:
            raise VlobNotFound('Wrong blob version.')
        yield Effect(EEvent('vlob_updated', intent.id))

    def get_dispatcher(self):
        return TypeDispatcher({
            EVlobCreate: self.perform_vlob_create,
            EVlobRead: self.perform_vlob_read,
            EVlobUpdate: self.perform_vlob_update,
        })
