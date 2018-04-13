import attr
from uuid import uuid4

from parsec.utils import generate_sym_key


@attr.s(slots=True, frozen=True)
class BaseAccess:

    @property
    def _fs(self):
        raise NotImplementedError()

    async def fetch(self):
        raise NotImplementedError()

    def dump(self):
        raise NotImplementedError()


@attr.s(slots=True, frozen=True)
class BaseUserVlobAccess(BaseAccess):
    # TODO: remove privkey (only manifests_manager deals with it)
    privkey = attr.ib()

    async def fetch(self):
        return await self._fs.manifests_manager.fetch_user_manifest_from_backend(
            self.privkey
        )

    def dump(self, with_type=True):
        # TODO: shouldn't be called, so raise exception here
        dumped = {"privkey": self.privkey}
        if with_type:
            dumped["type"] = "user_vlob"
        return dumped


@attr.s(slots=True, frozen=True)
class BaseVlobAccess(BaseAccess):
    id = attr.ib()
    rts = attr.ib()
    wts = attr.ib()
    key = attr.ib()

    async def fetch(self):
        # First look into local storage
        manifest = await self._fs.manifests_manager.fetch_from_local(self.id, self.key)
        if not manifest:
            # Go the much slower way of asking backend
            # Note this can raise a `BackendOfflineError` exception
            manifest = await self._fs.manifests_manager.fetch_from_backend(
                self.id, self.rts, self.key
            )
            if not manifest:
                raise RuntimeError("No manifest with access %s" % self)

        return manifest

    def dump(self, with_type=True):
        dumped = {"id": self.id, "rts": self.rts, "wts": self.wts, "key": self.key}
        if with_type:
            dumped["type"] = "vlob"
        return dumped


@attr.s(slots=True, frozen=True)
class BasePlaceHolderAccess(BaseAccess):
    id = attr.ib(default=attr.Factory(lambda: uuid4().hex))
    key = attr.ib(default=attr.Factory(generate_sym_key))

    async def fetch(self):
        return await self._fs.manifests_manager.fetch_from_local(self.id, self.key)

    def dump(self, with_type=True):
        dumped = {"id": self.id, "key": self.key}
        if with_type:
            dumped["type"] = "placeholder"
        return dumped
