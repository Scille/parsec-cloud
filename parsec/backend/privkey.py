import attr
from effect2 import TypeDispatcher

from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound


@attr.s
class EPrivKeyGet:
    hash = attr.ib()


@attr.s
class EPrivKeyAdd:
    hash = attr.ib()
    cipherkey = attr.ib()


@attr.s
class MockedPrivKeyComponent:
    _keys = attr.ib(default=attr.Factory(dict))

    async def perform_privkey_add(self, intent):
        # TODO: should check for authorization token to avoid impersonation
        assert isinstance(intent.cipherkey, (bytes, bytearray))
        if intent.hash in self._keys:
            raise PrivKeyHashCollision('Hash collision, change your password and retry.')
        else:
            self._keys[intent.hash] = intent.cipherkey

    async def perform_privkey_get(self, intent):
        try:
            return self._keys[intent.hash]
        except KeyError:
            raise PrivKeyNotFound('No entry with this hash')

    def get_dispatcher(self):
        return TypeDispatcher({
            EPrivKeyGet: self.perform_privkey_get,
            EPrivKeyAdd: self.perform_privkey_add
        })
