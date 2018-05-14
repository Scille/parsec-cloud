from parsec.rwlock import RWLock
from parsec.utils import ParsecError
from parsec.core.fs.access import BasePlaceHolderAccess


class SecurityError(ParsecError):
    pass


class FSInvalidPath(ParsecError):
    status = "invalid_path"


class FSError(Exception):
    pass


class FSLoadError(Exception):
    pass


class BaseEntry:
    def __init__(self, access, name="", parent=None):
        self._access = access
        self._name = name
        self._parent = parent
        self._rwlock = RWLock()

    @property
    def id(self):
        return self._access.id

    @property
    def is_loaded(self):
        return True

    def __repr__(self):
        return "<%s(path=%r)>" % (self.__class__.__name__, self.path)

    def acquire_read(self):
        return self._rwlock.acquire_read()

    def acquire_write(self):
        return self._rwlock.acquire_write()

    @property
    def path(self):
        path_items = []
        item = self
        while item:
            path_items.append(item.name)
            item = item.parent
        path = "/" + "/".join(reversed(path_items))
        # TODO: too cumbersome to handle the root-without-name case this way
        if path.startswith("//"):
            path = path[1:]
        if path == "/":
            return ""

        else:
            return path

    @property
    def name(self):
        return self._name

    @property
    def parent(self):
        return self._parent

    @property
    def is_placeholder(self):
        return isinstance(self._access, BasePlaceHolderAccess)

    @property
    def need_sync(self):
        raise NotImplementedError()

    @property
    def need_flush(self):
        raise NotImplementedError()

    @property
    def _fs(self):
        raise NotImplementedError()

    async def flush(self, recursive=False):
        raise NotImplementedError()

    async def sync(self, recursive=False):
        raise NotImplementedError()

    async def minimal_sync_if_placeholder(self):
        raise NotImplementedError()


class BaseNotLoadedEntry(BaseEntry):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._loaded = None

    @property
    def is_loaded(self):
        return False

    @property
    def need_sync(self):
        return False

    @property
    def need_flush(self):
        return False

    async def load(self):
        if self._loaded:
            return self._loaded

        async with self.acquire_write():
            # Make sure this entry hasn't been loaded while we were waiting
            if self._loaded:
                return self._loaded

            manifest = await self._access.fetch()
            if manifest:
                # The idea here is to create a new entry object that will replace the
                # current one and represent the fact it is now loaded in memory.
                # Hence we must share `_access` and `_rwlock` between the two.
                self._loaded = self._fs._load_entry(
                    self._access,
                    manifest["user_id"],
                    manifest["device_name"],
                    self.name,
                    self.parent,
                    manifest,
                )
                self._loaded._rwlock = self._rwlock
                return self._loaded

            else:
                raise FSLoadError("%s: cannot fetch access %r" % (self.path, self._access))

    async def flush(self, recursive=False):
        # If the entry is not loaded, then there is nothing to be flushed
        pass

    async def sync(self, recursive=False):
        loaded_entry = await self.load()
        await loaded_entry.sync(recursive=recursive)
        # TODO: should be modified while lock is held
        # in case access was a placeholder
        self._access = loaded_entry._access

    async def minimal_sync_if_placeholder(self):
        if not self.is_placeholder:
            return

        # TODO: A bit too cumbersome to acquire multiple times the lock and modify ?
        loaded_entry = await self.load()
        await loaded_entry.minimal_sync_if_placeholder()
        async with self.acquire_write():
            self._access = loaded_entry._access
