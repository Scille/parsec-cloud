import attr
from effect2 import TypeDispatcher, Effect

from parsec.core.backend_start_api import (
    EBackendCipherKeyGet, EBackendCipherKeyAdd, EBackendIdentityRegister)
from parsec.base import EEvent
from parsec.exceptions import IdentityNotLoadedError, IdentityError
from parsec.crypto import load_private_key, generate_asym_key


@attr.s
class Identity:
    id = attr.ib()
    private_key = attr.ib()
    public_key = attr.ib()


@attr.s
class EIdentityLoad:
    id = attr.ib()
    key = attr.ib()
    password = attr.ib(default=None)


@attr.s
class EIdentityUnload:
    pass


@attr.s
class EIdentityGet:
    pass


@attr.s
class EIdentitySignup:
    id = attr.ib()
    password = attr.ib()
    key_size = attr.ib(default=2048)


@attr.s
class EIdentityLogin:
    id = attr.ib()
    password = attr.ib()


@attr.s
class IdentityComponent:
    identity = attr.ib(default=None)

    async def perform_identity_load(self, intent):
        from parsec.core.fs import EFSInit
        if self.identity:
            raise IdentityError('Identity already loaded')
        # TODO: handle invalid key with more precise exception
        try:
            private_key = load_private_key(intent.key, intent.password)
        except Exception as e:
            raise IdentityError('Invalid private key (%s)' % e)
        self.identity = Identity(intent.id, private_key, private_key.pub_key)
        await Effect(EEvent('identity_loaded', self.identity.id))
        await Effect(EFSInit())
        return self.identity

    async def perform_identity_unload(self, intent):
        from parsec.core.backend import EBackendReset
        from parsec.core.block import EBlockReset
        from parsec.core.fs import EFSReset
        if not self.identity:
            raise IdentityNotLoadedError('Identity not loaded')
        # TODO: make block&backend reset event triggered
        await Effect(EFSReset())
        await Effect(EBlockReset())
        await Effect(EBackendReset())
        await Effect(EEvent('identity_unloaded', None))
        self.identity = None

    async def perform_identity_get(self, intent):
        if not self.identity:
            raise IdentityNotLoadedError('Identity not loaded')
        return self.identity

    async def perform_identity_signup(self, intent):
        private_key = generate_asym_key(intent.key_size)
        cipherkey = private_key.export(intent.password)
        pubkey = private_key.pub_key.export()
        # Saving the cipherkey first prevent us from registering a public key
        # then crashing without the corresponding private key saved somewhere
        # TODO: handle compute the hash here instead of in BackendCipherkeyAdd ?
        await Effect(EBackendCipherKeyAdd(intent.id, intent.password, cipherkey))
        await Effect(EBackendIdentityRegister(intent.id, pubkey))

    async def perform_identity_login(self, intent):
        cipherkey = await Effect(EBackendCipherKeyGet(intent.id, intent.password))
        await Effect(EIdentityLoad(intent.id, cipherkey, intent.password))

    def get_dispatcher(self):
        return TypeDispatcher({
            EIdentityLoad: self.perform_identity_load,
            EIdentityUnload: self.perform_identity_unload,
            EIdentityGet: self.perform_identity_get,
            EIdentityLogin: self.perform_identity_login,
            EIdentitySignup: self.perform_identity_signup
        })
