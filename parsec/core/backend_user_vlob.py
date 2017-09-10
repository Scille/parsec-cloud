import attr
from effect2 import Effect

from parsec.tools import to_jsonb64, from_jsonb64
from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class EBackendUserVlobRead:
    version = attr.ib(default=None)


@attr.s
class EBackendUserVlobUpdate:
    version = attr.ib()
    blob = attr.ib(default=b'')


@attr.s
class UserVlobAtom:
    version = attr.ib()
    blob = attr.ib()


async def perform_user_vlob_read(intent):
    msg = {}
    if intent.version is not None:
        msg['version'] = intent.version
    ret = await Effect(BackendCmd('user_vlob_read', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return UserVlobAtom(ret['version'], from_jsonb64(ret['blob']))


async def perform_user_vlob_update(intent):
    msg = {
        'version': intent.version,
        'blob': to_jsonb64(intent.blob)
    }
    ret = await Effect(BackendCmd('user_vlob_update', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
