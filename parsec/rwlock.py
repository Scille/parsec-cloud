import attr
import trio


@attr.s(slots=True)
class ReadLockManager:

    rwlock = attr.ib()

    async def __aenter__(self):
        async with self.rwlock.r:
            self.rwlock.b += 1
            if self.rwlock.b == 1:
                await self.rwlock.g.acquire()
                # Don't worry, I known what I'm doing...
                self.rwlock.g._owner = 42

    async def __aexit__(self, *args):
        async with self.rwlock.r:
            self.rwlock.b -= 1
            if self.rwlock.b == 0:
                # Don't worry, I known what I'm doing...
                self.rwlock.g._owner = trio._core.current_task()
                self.rwlock.g.release()


@attr.s(slots=True)
class WriteLockManager:

    rwlock = attr.ib()

    async def __aenter__(self):
        await self.rwlock.g.acquire()

    async def __aexit__(self, *args):
        self.rwlock.g.release()


@attr.s(slots=True)
class RWLock:
    r = attr.ib(default=attr.Factory(trio.Lock))
    g = attr.ib(default=attr.Factory(trio.Lock))
    b = attr.ib(default=0)

    def acquire_read(self):
        return ReadLockManager(self)

    def acquire_write(self):
        return WriteLockManager(self)
