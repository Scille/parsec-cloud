import attr
from effect2 import Effect

from parsec.tools import to_jsonb64, from_jsonb64
from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class EBackendVlobCreate:
    id = attr.ib(default=None)
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


async def perform_vlob_create(intent):
    msg = {'blob': to_jsonb64(intent.blob)}
    if intent.id:
        msg['id'] = intent.id
    ret = await Effect(BackendCmd('vlob_create', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return VlobAccess(ret['id'], ret['read_trust_seed'], ret['write_trust_seed'])


async def perform_vlob_read(intent):
    assert isinstance(intent.id, str)
    msg = {'id': intent.id, 'trust_seed': intent.trust_seed}
    if intent.version is not None:
        msg['version'] = intent.version
    ret = await Effect(BackendCmd('vlob_read', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return VlobAtom(ret['id'], ret['version'], from_jsonb64(ret['blob']))


async def perform_vlob_update(intent):
    msg = {
        'id': intent.id,
        'version': intent.version,
        'trust_seed': intent.trust_seed,
        'blob': to_jsonb64(intent.blob)
    }
    ret = await Effect(BackendCmd('vlob_update', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
