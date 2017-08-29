import attr
import asyncio
from effect2 import Effect, TypeDispatcher, do, AsyncFunc, asyncio_perform

from parsec.tools import logger


@attr.s
class ESynchronizerPutJob:
    intent = attr.ib()


@attr.s
class ESynchronizerFileManifestUpdate:
    file_manifest = attr.ib()


@attr.s
class ESynchronizerBlockCreate:
    id = attr.ib()
    content = attr.ib()


@attr.s
class ESynchronizerBlockUpdate:
    block = attr.ib()


@attr.s
class ESynchronizerFlush:
    pass


class SynchronizerComponent:
    def __init__(self):
        self._job_queue = None
        self._job_processor_task = None
        self._app = None

    async def shutdown(self, app):
        if self._job_processor_task:
            self._job_processor_task.cancel()
            try:
                await self._job_processor_task
            except asyncio.CancelledError:
                pass

    async def startup(self, app):
        self._app = app
        self._job_queue = asyncio.Queue()
        self._job_processor_task = asyncio.ensure_future(self._job_processor())

    async def _job_processor(self):
        # TODO: find a better way to do this than using asyncio_perform...
        dispatcher = self._app.components.get_dispatcher()
        while True:
            intent = await self._job_queue.get()
            logger.debug('Synchronizing:', intent.intent)
            await asyncio_perform(dispatcher, Effect(intent.intent))
            self._job_queue.task_done()

    @do
    def push_to_job_queue(self, intent):
        self._job_queue.put_nowait(intent)

    @do
    def perform_flush(self, intent):
        yield AsyncFunc(self._job_queue.join())

    def get_dispatcher(self):
        return TypeDispatcher({
            ESynchronizerPutJob: self.push_to_job_queue,

            ESynchronizerFlush: self.perform_flush,
        })
