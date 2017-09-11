import attr
from effect2 import ComposedDispatcher, asyncio_perform

from parsec.base import base_dispatcher, EventComponent
from parsec.core.core_api import register_core_api
from parsec.core.backend import BackendComponent
from parsec.core.identity import IdentityComponent
from parsec.core.fs import FSComponent
from parsec.core.synchronizer import SynchronizerComponent
from parsec.core.block import BlockComponent


@attr.s
class CoreComponents:
    event = attr.ib()
    backend = attr.ib()
    block = attr.ib()
    fs = attr.ib()
    identity = attr.ib()
    synchronizer = attr.ib()

    def get_dispatcher(self):
        return ComposedDispatcher([
            base_dispatcher,
            self.event.get_dispatcher(),
            self.backend.get_dispatcher(),
            self.block.get_dispatcher(),
            self.fs.get_dispatcher(),
            self.identity.get_dispatcher(),
            self.synchronizer.get_dispatcher()
        ])

    async def shutdown(self, app):
        async def _shutdown():
            await self.backend.shutdown(app)
            await self.block.shutdown(app)
            await self.synchronizer.shutdown(app)
        await asyncio_perform(self.get_dispatcher(), _shutdown())

    async def startup(self, app):
        async def _startup():
            await self.synchronizer.startup(app)
            await self.fs.startup(app)
        await asyncio_perform(self.get_dispatcher(), _startup())


def components_factory(app, backend_host, backend_watchdog=False, cache_size=4000):
    backend = BackendComponent(backend_host, backend_watchdog)
    block = BlockComponent()
    core_components = CoreComponents(
        event=EventComponent(),
        block=block,
        backend=backend,
        fs=FSComponent(),
        identity=IdentityComponent(),
        synchronizer=SynchronizerComponent()
    )
    app.components = core_components
    app.on_startup.append(core_components.startup)
    app.on_shutdown.append(core_components.shutdown)
    return core_components


__all__ = ('register_core_api', 'components_factory')
