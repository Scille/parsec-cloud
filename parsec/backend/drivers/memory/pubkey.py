from nacl.public import PublicKey
from nacl.signing import VerifyKey

import attr

from parsec.backend.pubkey import BasePubKeyComponent

from parsec.backend.exceptions import (AlreadyExistsError, NotFoundError)


@attr.s
class MemoryPubKeyComponent(BasePubKeyComponent):
    _keys = attr.ib(default=attr.Factory(dict))

    async def add(self, id, pubkey, verifykey):
        assert isinstance(pubkey, (bytes, bytearray))
        assert isinstance(verifykey, (bytes, bytearray))
        if id in self._keys:
            raise AlreadyExistsError("Identity `%s` already has a public key" % id)

        else:
            self._keys[id] = (pubkey, verifykey)

    async def get(self, id, cooked=False):
        try:
            keys = self._keys[id]
            return (PublicKey(keys[0]), VerifyKey(keys[1])) if cooked else keys

        except KeyError:
            raise NotFoundError("No public key for identity `%s`" % id)
