# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import threading
from inspect import iscoroutinefunction
from contextlib import contextmanager

from parsec.utils import start_task


@pytest.fixture
def backend_service_factory(backend_factory):
    """
    Run a trio loop with backend in a separate thread to allow blocking
    operations from the Qt loop.
    """

    class BackendService:
        def __init__(self):
            self.port = None
            self._portal = None
            self._task = None
            self._need_stop = trio.Event()
            self._stopped = trio.Event()
            self._need_start = trio.Event()
            self._started = trio.Event()
            self._ready = threading.Event()

        def get_url(self):
            if self.port is None:
                raise RuntimeError("Port not yet available, is service started ?")
            return f"ws://127.0.0.1:{self.port}"

        def execute(self, cb):
            if iscoroutinefunction(cb):
                self._portal.run(cb, self._task.value)
            else:
                self._portal.run_sync(cb, self._task.value)

        def start(self, **backend_factory_params):
            async def _start():
                self._need_start.set()
                await self._started.wait()

            self._backend_factory_params = backend_factory_params
            self._portal.run(_start)

        def stop(self):
            async def _stop():
                if self._started.is_set():
                    self._need_stop.set()
                    await self._stopped.wait()

            self._portal.run(_stop)

        def init(self):
            self._thread = threading.Thread(target=trio.run, args=(self._service,))
            self._thread.setName("BackendService")
            self._thread.start()
            self._ready.wait()

        async def _teardown(self):
            self._nursery.cancel_scope.cancel()

        def teardown(self):
            if not self._portal:
                return
            self.stop()
            self._portal.run(self._teardown)
            self._thread.join()

        async def _service(self):
            self._portal = trio.BlockingTrioPortal()

            async def _backend_controlled_cb(*, task_status=trio.TASK_STATUS_IGNORED):
                async with backend_factory(**self._backend_factory_params) as backend:
                    async with trio.open_nursery() as nursery:
                        listeners = await nursery.start(trio.serve_tcp, backend.handle_client, 0)
                        self.port = listeners[0].socket.getsockname()[1]
                        task_status.started(backend)
                        await trio.sleep_forever()

            async with trio.open_nursery() as self._nursery:
                self._ready.set()
                while True:
                    await self._need_start.wait()
                    self._need_stop.clear()
                    self._stopped.clear()

                    self._task = await start_task(self._nursery, _backend_controlled_cb)
                    self._started.set()

                    await self._need_stop.wait()
                    self._need_start.clear()
                    self._started.clear()
                    await self._task.cancel_and_join()
                    self._stopped.set()

    @contextmanager
    def _backend_service_factory():
        backend_service = BackendService()
        backend_service.init()
        try:
            yield backend_service

        finally:
            backend_service.teardown()

    return _backend_service_factory


@pytest.fixture
def backend_service(backend_service_factory):
    with backend_service_factory() as backend:
        yield backend


@pytest.fixture
def autoclose_dialog(monkeypatch):
    class DialogSpy:
        def __init__(self):
            self.dialogs = []

    spy = DialogSpy()

    def _dialog_exec(dialog):
        spy.dialogs.append((dialog.label_title.text(), dialog.label_message.text()))

    monkeypatch.setattr(
        "parsec.core.gui.custom_widgets.MessageDialog.exec_", _dialog_exec, raising=False
    )
    yield spy
