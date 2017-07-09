import attr
from effect2 import TypeDispatcher, Effect, do
import hashlib

from parsec.exceptions import PrivKeyNotFound
from parsec.crypto import load_sym_key
from parsec.core.identity import EIdentityLoad


@attr.s
class EPrivkeyAdd:
    id = attr.ib()
    password = attr.ib()
    key = attr.ib()


@attr.s
class EPrivkeyGet:
    id = attr.ib()
    password = attr.ib()


@attr.s
class EPrivkeyLoad:
    id = attr.ib()
    password = attr.ib()


@attr.s
class PrivKeyComponent:
    encrypted_keys = attr.ib(default={})  # TODO in __init__?

    @do
    def perform_add_privkey(self, intent):
        assert isinstance(intent.key, (bytes, bytearray))
        id_password_hash = hashlib.sha256((intent.id + ':' + intent.password).encode()).digest()
        # if id_password_hash in self.encrypted_keys:
        #     raise PrivKeyError('Identity already has an encrypted private key.')
        password_digest = hashlib.sha256(intent.password.encode()).digest()
        encryptor = load_sym_key(password_digest)
        encrypted_key = encryptor.encrypt(intent.key)
        self.encrypted_keys[id_password_hash] = encrypted_key

    @do
    def perform_get_privkey(self, intent):
        return self._fetch_privkey(intent.id, intent.password)

    @do
    def perform_privkey_load_identity(self, intent):
        privkey = self._fetch_privkey(intent.id, intent.password)
        yield Effect(EIdentityLoad(intent.id, privkey))

    def _fetch_privkey(self, id, password):
        id_password_hash = hashlib.sha256(('%s:%s' % (id, password)).encode()).digest()
        try:
            encrypted_privkey = self.encrypted_keys[id_password_hash]
        except KeyError:
            raise PrivKeyNotFound('Private key not found.')
        password_digest = hashlib.sha256(password.encode()).digest()
        encryptor = load_sym_key(password_digest)
        return encryptor.decrypt(encrypted_privkey)

    def get_dispatcher(self):
        return TypeDispatcher({
            EPrivkeyAdd: self.perform_add_privkey,
            EPrivkeyGet: self.perform_get_privkey,
            EPrivkeyLoad: self.perform_privkey_load_identity,
        })
