# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from contextlib import contextmanager
import trio
import threading
from inspect import iscoroutinefunction
from structlog import get_logger


logger = get_logger()


class JobResultError(Exception):
    def __init__(self, status, **kwargs):
        self.status = status
        self.params = kwargs


class QtToTrioJob:
    def __init__(self, qt_on_success, qt_on_error, fn, *args, **kwargs):
        self._qt_on_success = qt_on_success
        self._qt_on_error = qt_on_error
        self._fn = fn
        self._args = args
        self._kwargs = kwargs
        self.cancel_scope = None
        self._done = threading.Event()
        self.status = None
        self.ret = None
        self.exc = None

    def is_finished(self):
        return self._done.is_set()

    async def _run_fn(self, *, task_status=trio.TASK_STATUS_IGNORED):
        try:
            with trio.CancelScope() as self.cancel_scope:
                task_status.started()

                try:
                    if iscoroutinefunction(self._fn):
                        self.ret = await self._fn(*self._args, **self._kwargs)
                    else:
                        self.ret = self._fn(*self._args, **self._kwargs)
                    self.status = "ok"

                except JobResultError as exc:
                    self.status = exc.status
                    self.exc = exc

                except Exception as exc:
                    logger.exception("Uncatched error", exc_info=exc)
                    self.status = "crashed"
                    self.exc = JobResultError(
                        "crashed", exc=exc, info=f"Unexpected error: {repr(exc)}"
                    )

        finally:
            self._done.set()
            if self.status == "ok":
                self._qt_on_success.emit()

            else:
                self._qt_on_error.emit()

    def cancel_and_join(self):
        assert self.cancel_scope
        self._portal.run_sync(self.cancel_scope.cancel)
        self._done.wait()


class QtToTrioJobScheduler:
    def __init__(self):
        self._portal = None
        self._cancel_scope = None
        self.started = threading.Event()

    async def _start(self, *, task_status=trio.TASK_STATUS_IGNORED):
        assert not self.started.is_set()

        self._portal = trio.BlockingTrioPortal()
        self._send_job_channel, recv_job_channel = trio.open_memory_channel(100)
        async with trio.open_nursery() as nursery, recv_job_channel:
            self._cancel_scope = nursery.cancel_scope
            self.started.set()
            task_status.started()
            while True:
                job = await recv_job_channel.receive()
                assert job.status is None
                await nursery.start(job._run_fn)

    async def _teardown(self):
        self._cancel_scope.cancel()
        await self._send_job_channel.aclose()

    def run(self, afn, *args):
        return self._portal.run(afn, *args)

    def run_sync(self, fn, *args):
        return self._portal.run(fn, *args)

    def submit_job(self, qt_on_success, qt_on_error, fn, *args, **kwargs):
        job = QtToTrioJob(qt_on_success, qt_on_error, fn, *args, **kwargs)
        self._portal.run_sync(self._send_job_channel.send_nowait, job)
        return job


@contextmanager
def run_trio_thread():
    job_scheduler = QtToTrioJobScheduler()
    thread = threading.Thread(target=trio.run, args=[job_scheduler._start])
    thread.setName("TrioLoop")
    thread.start()
    job_scheduler.started.wait()

    try:
        yield job_scheduler

    finally:
        job_scheduler._portal.run(job_scheduler._teardown)
        thread.join()
