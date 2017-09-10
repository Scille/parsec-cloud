import attr
from effect2 import Effect

from parsec.tools import from_jsonb64
from parsec.crypto import load_public_key
from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class PubKey:
    id = attr.ib()
    raw_key = attr.ib()
    _key = attr.ib(default=None)

    @property
    def key(self):
        if not self._key:
            self._key = load_public_key(self.raw_key)
        return self._key


@attr.s
class EBackendPubKeyGet:
    id = attr.ib()
    raw = attr.ib(default=False)


async def perform_pubkey_get(intent):
    msg = {'id': intent.id}
    ret = await Effect(BackendCmd('pubkey_get', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return PubKey(intent.id, from_jsonb64(ret['key']))
