import attr
from effect2 import Effect, do

from parsec.tools import to_jsonb64, from_jsonb64
from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class EBackendVlobCreate:
    blob = attr.ib(default=b'')


@attr.s
class EBackendVlobRead:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib(default=None)


@attr.s
class EBackendVlobUpdate:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib()
    blob = attr.ib(default=b'')


@attr.s
class VlobAccess:
    id = attr.ib()
    read_trust_seed = attr.ib()
    write_trust_seed = attr.ib()


@attr.s
class VlobAtom:
    id = attr.ib()
    version = attr.ib()
    blob = attr.ib()


@do
def perform_vlob_create(intent):
    msg = {'blob': to_jsonb64(intent.blob)}
    ret = yield Effect(BackendCmd('vlob_create', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return VlobAccess(ret['id'], ret['read_trust_seed'], ret['write_trust_seed'])


@do
def perform_vlob_read(intent):
    assert isinstance(intent.id, str)
    msg = {'id': intent.id, 'trust_seed': intent.trust_seed}
    if intent.version is not None:
        msg['version'] = intent.version
    ret = yield Effect(BackendCmd('vlob_read', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return VlobAtom(ret['id'], ret['version'], from_jsonb64(ret['blob']))


@do
def perform_vlob_update(intent):
    msg = {
        'id': intent.id,
        'version': intent.version,
        'trust_seed': intent.trust_seed,
        'blob': to_jsonb64(intent.blob)
    }
    ret = yield Effect(BackendCmd('vlob_update', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
